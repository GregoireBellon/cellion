use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::solutions)]
pub struct Solutions {
    pub id: i32,
    pub filename: String,
    pub created_at: chrono::NaiveDateTime,
}
