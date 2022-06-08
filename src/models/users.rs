use chrono::{DateTime, Utc};
use diesel::types::FromSql;

use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a uuid::Uuid,
    pub email: &'a str,
    pub name: &'a str,
    pub created_at: &'a DateTime<Utc>,
}
