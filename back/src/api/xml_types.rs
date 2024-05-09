use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XmlRoom {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@capacity")]
    pub capacity: i32,
    #[serde(rename = "@label")]
    pub label: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct XmlTeacher {
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct XmlCourse {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,

    #[serde(rename = "part", default)]
    pub parts: Vec<XmlPart>,
}

#[derive(Deserialize, Debug)]
pub struct XmlPart {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@nrSessions")]
    pub nr_session: Option<i32>,

    pub classes: XmlClasses,
}

#[derive(Deserialize, Debug)]
pub struct XmlClasses {
    #[serde(rename = "@maxHeadCount")]
    pub max_head_count: Option<i32>,

    #[serde(default)]
    pub class: Vec<XmlClass>,
}

#[derive(Deserialize, Debug)]
pub struct XmlClass {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@parent")]
    pub parent: Option<String>,

    #[serde(rename = "@label")]
    pub label: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct XmlSolutionGroup {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@headCount")]
    pub head_count: Option<i32>,

    pub students: Option<XmlStudents>,
}

#[derive(Deserialize, Debug)]
pub struct XmlStudents {
    #[serde(rename = "student", default)]
    pub students: Vec<XmlRefIdElement<String>>,
}

#[derive(Deserialize, Debug)]
pub struct XmlRefIdElement<T> {
    #[serde(rename = "@refId")]
    pub ref_id: T,
}

#[derive(Deserialize, Debug)]
pub struct XmlSolutionClass {
    #[serde(rename = "@refId")]
    pub ref_id: String,
    pub groups: Option<XmlSolutionClassGroups>,
}

#[derive(Deserialize, Debug)]
pub struct XmlSolutionClassGroups {
    #[serde(rename = "group", default)]
    pub groups_id: Vec<XmlRefIdElement<String>>,
}

#[derive(Deserialize, Debug)]
pub struct XmlSession {
    #[serde(rename = "@rank")]
    pub rank: String,

    #[serde(rename = "@class")]
    pub class: String,

    #[serde(rename = "startingSlot")]
    pub starting_slot: Option<XmlSessionStartingSlot>,

    pub rooms: Option<XmlSessionRooms>,
    pub teachers: Option<XmlSessionTeachers>,
}

#[derive(Deserialize, Debug)]
pub struct XmlSessionStartingSlot {
    #[serde(rename = "@dailySlot")]
    pub daily_slot: u32,

    #[serde(rename = "@day")]
    pub day: u32,

    #[serde(rename = "@week")]
    pub week: u32,
}

#[derive(Deserialize, Debug)]
pub struct XmlSessionRooms {
    #[serde(rename = "room", default)]
    pub rooms_id: Vec<XmlRefIdElement<String>>,
}

#[derive(Deserialize, Debug)]
pub struct XmlSessionTeachers {
    #[serde(rename = "teacher", default)]
    pub teachers_id: Vec<XmlRefIdElement<String>>,
}
