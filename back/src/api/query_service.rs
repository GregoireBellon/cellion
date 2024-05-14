use chrono::NaiveDateTime;
use diesel::{
    result::Error as DieselError, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl,
    RunQueryDsl, SqliteConnection,
};

use crate::models::schema;

use super::solutions_dto::{ShortCourseInfo, ShortPartInfo, ShortSessionInfo};

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
            schema::sessions_rooms::table.on(schema::sessions_rooms::session_rank
                .eq(schema::sessions::rank)
                .and(schema::sessions_rooms::solution_id.eq(schema::sessions::solution_id))
                .and(schema::sessions_rooms::class_id.eq(schema::sessions::class_id))),
        )
        .inner_join(
            schema::rooms::table.on(schema::sessions_rooms::room_id
                .eq(schema::rooms::id)
                .and(schema::sessions_rooms::solution_id.eq(schema::rooms::solution_id))),
        )
        .inner_join(
            schema::sessions_teachers::table.on(schema::sessions_teachers::session_rank
                .eq(schema::sessions::rank)
                .and(schema::sessions_teachers::solution_id.eq(schema::sessions::solution_id))
                .and(schema::sessions_teachers::class_id.eq(schema::sessions::class_id))),
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

    let sessions_return: Vec<(String, NaiveDateTime, String, String, i32)> = query
        .select((
            schema::sessions::uuid,
            schema::sessions::starting_date,
            schema::courses::id,
            schema::parts::id,
            schema::parts::session_length,
        ))
        .load::<(String, NaiveDateTime, String, String, i32)>(conn)?;

    return Ok(sessions_return
        .into_iter()
        .map(|elem| ShortSessionInfo {
            id: elem.0,
            from: elem.1,
            to: elem
                .1
                .checked_add_signed(chrono::Duration::minutes(elem.4 as i64))
                .unwrap(),
            course: ShortCourseInfo { id: elem.2 },
            part: ShortPartInfo { id: elem.3 },
        })
        .collect());
}
