use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Hash)]
pub struct ShortRoomInfo {
    pub id: String,
}
#[derive(Serialize, PartialEq, Eq, Hash)]
pub struct ShortGroupInfo {
    pub id: String,
}
#[derive(Serialize, PartialEq, Eq, Hash)]
pub struct ShortTeacherInfo {
    pub id: String,
}

#[derive(Serialize)]
pub struct ShortCourseInfo {
    pub id: String,
}

#[derive(Serialize)]
pub struct ShortPartInfo {
    pub id: String,
}

#[derive(Serialize)]
pub struct ShortSessionInfo {
    pub id: String,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub course: ShortCourseInfo,
    pub part: ShortPartInfo,
    pub rooms: Vec<ShortRoomInfo>,
    pub groups: Vec<ShortGroupInfo>,
    pub teachers: Vec<ShortTeacherInfo>,
}
