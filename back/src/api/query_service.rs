use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use diesel::{
    result::Error as DieselError, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl,
    RunQueryDsl, SqliteConnection,
};

use crate::models::schema;

use super::{
    query_controller::FilterList,
    solutions_dto::{
        ShortCourseInfo, ShortGroupInfo, ShortPartInfo, ShortRoomInfo, ShortSessionInfo,
        ShortTeacherInfo,
    },
};

struct ShortSessionInfoMap {
    pub id: String,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub course: ShortCourseInfo,
    pub part: ShortPartInfo,
    pub rooms: HashSet<ShortRoomInfo>,
    pub groups: HashSet<ShortGroupInfo>,
    pub teachers: HashSet<ShortTeacherInfo>,
}

impl From<(String, NaiveDateTime, String, String, i32)> for ShortSessionInfoMap {
    fn from(value: (String, NaiveDateTime, String, String, i32)) -> Self {
        ShortSessionInfoMap {
            id: value.0,
            from: value.1,
            to: value
                .1
                .checked_add_signed(chrono::Duration::minutes(value.4 as i64))
                .unwrap(),
            course: ShortCourseInfo { id: value.2 },
            part: ShortPartInfo { id: value.3 },
            rooms: HashSet::new(),
            groups: HashSet::new(),
            teachers: HashSet::new(),
        }
    }
}

impl Into<ShortSessionInfo> for ShortSessionInfoMap {
    fn into(self) -> ShortSessionInfo {
        ShortSessionInfo {
            id: self.id,
            from: self.from,
            to: self.to,
            course: self.course,
            part: self.part,
            rooms: Vec::from_iter(self.rooms.into_iter()),
            groups: Vec::from_iter(self.groups.into_iter()),
            teachers: Vec::from_iter(self.teachers.into_iter()),
        }
    }
}

pub fn get_sessions_with_filters(
    conn: &mut SqliteConnection,
    query_solution_id: i32,
    courses_id: Vec<String>,
    parts_id: Vec<String>,
    teachers_id: Vec<String>,
    rooms_id: Vec<String>,
    groups_id: Vec<String>,
) -> Result<Vec<ShortSessionInfo>, DieselError> {
    let mut query = schema::solutions::table
        .filter(schema::solutions::id.eq(query_solution_id))
        .inner_join(schema::sessions::table)
        .inner_join(
            schema::classes::table.on(schema::classes::id
                .eq(schema::sessions::class_id)
                .and(schema::classes::solution_id.eq(schema::sessions::solution_id))),
        )
        .inner_join(
            schema::parts::table.on(schema::classes::part_id
                .eq(schema::parts::id)
                .and(schema::classes::solution_id.eq(schema::parts::solution_id))),
        )
        .inner_join(
            schema::courses::table.on(schema::parts::course_id
                .eq(schema::courses::id)
                .and(schema::parts::solution_id.eq(schema::courses::solution_id))),
        )
        .inner_join(
            schema::sessions_rooms::table
                .on(schema::sessions_rooms::session_id.eq(schema::sessions::id)),
        )
        .inner_join(
            schema::rooms::table.on(schema::sessions_rooms::room_id
                .eq(schema::rooms::id)
                .and(schema::sessions_rooms::solution_id.eq(schema::rooms::solution_id))),
        )
        .inner_join(
            schema::sessions_teachers::table
                .on(schema::sessions_teachers::session_id.eq(schema::sessions::id)),
        )
        .inner_join(
            schema::teachers::table.on(schema::sessions_teachers::teacher_id
                .eq(schema::teachers::name)
                .and(schema::sessions_teachers::solution_id.eq(schema::teachers::solution_id))),
        )
        .inner_join(
            schema::classes_groups::table.on(schema::classes_groups::class_id
                .eq(schema::classes::id)
                .and(schema::classes::solution_id.eq(schema::classes_groups::solution_id))),
        )
        .inner_join(
            schema::groups::table.on(schema::classes_groups::group_id
                .eq(schema::groups::id)
                .and(schema::classes_groups::solution_id.eq(schema::groups::solution_id))),
        )
        .into_boxed();

    if !teachers_id.is_empty() {
        query = query.filter(schema::teachers::name.eq_any(teachers_id));
    }
    if !courses_id.is_empty() {
        query = query.filter(schema::courses::id.eq_any(courses_id));
    }
    if !parts_id.is_empty() {
        query = query.filter(schema::parts::id.eq_any(parts_id));
    }
    if !rooms_id.is_empty() {
        query = query.filter(schema::rooms::id.eq_any(rooms_id));
    }
    if !groups_id.is_empty() {
        query = query.filter(schema::groups::id.eq_any(groups_id));
    }
    let mut sessions_map: HashMap<i32, ShortSessionInfoMap> = HashMap::new();

    query
        .select((
            schema::sessions::id,
            schema::sessions::uuid,
            schema::sessions::starting_date,
            schema::courses::id,
            schema::parts::id,
            schema::parts::session_length,
            schema::rooms::id,
            schema::groups::id,
            schema::teachers::name,
        ))
        .load::<(
            i32,
            String,
            NaiveDateTime,
            String,
            String,
            i32,
            String,
            String,
            String,
        )>(conn)?
        .into_iter()
        .for_each(|sess| {
            let entry = sessions_map
                .entry(sess.0)
                .or_insert(ShortSessionInfoMap::from((
                    sess.1, sess.2, sess.3, sess.4, sess.5,
                )));

            entry.rooms.insert(ShortRoomInfo { id: sess.6 });
            entry.groups.insert(ShortGroupInfo { id: sess.7 });
            entry.teachers.insert(ShortTeacherInfo { id: sess.8 });
        });

    return Ok(sessions_map
        .into_values()
        .map(ShortSessionInfoMap::into)
        .collect::<Vec<ShortSessionInfo>>());
}

pub fn get_filter_list(
    conn: &mut SqliteConnection,
    request_solution_id: i32,
) -> Result<FilterList, DieselError> {
    let courses = schema::classes::table
        .filter(schema::classes::solution_id.eq(request_solution_id))
        .select(schema::classes::id)
        .get_results::<String>(conn)?;

    let parts = schema::parts::table
        .filter(schema::parts::solution_id.eq(request_solution_id))
        .select(schema::parts::id)
        .get_results::<String>(conn)?;

    let teachers = schema::teachers::table
        .filter(schema::teachers::solution_id.eq(request_solution_id))
        .select(schema::teachers::name)
        .get_results::<String>(conn)?;

    let rooms = schema::rooms::table
        .filter(schema::rooms::solution_id.eq(request_solution_id))
        .select(schema::rooms::id)
        .get_results::<String>(conn)?;

    let groups = schema::groups::table
        .filter(schema::groups::solution_id.eq(request_solution_id))
        .select(schema::groups::id)
        .get_results::<String>(conn)?;

    Ok(FilterList {
        courses: courses,
        parts: parts,
        teachers: teachers,
        rooms: rooms,
        groups: groups,
    })
}
