use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::solutions)]
pub struct Solution {
    pub id: i32,
    pub filename: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(belongs_to(Solution))]
#[diesel(table_name = crate::schema::rooms)]
pub struct Room {
    pub id: String,
    pub solution_id: i32,
    pub capacity: i32,
    pub name: Option<String>,
}
