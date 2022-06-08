use actix_web::{web, HttpResponse};
use chrono::Utc;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::domain::{NewUser, UserEmail, UserName};
use crate::startup::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryInto<NewUser> for FormData {
    type Error = String;

    fn try_into(self) -> Result<NewUser, Self::Error> {
        let email = UserEmail::parse(self.email)?;
        let name = UserName::parse(self.name)?;
        Ok(NewUser { email, name })
    }
}

pub async fn create_user(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = form.0.try_into().expect("Failed to parse form data.");
    insert_user(&new_user, &pool).expect("Failed to execute query.");

    HttpResponse::Ok().finish()
}

fn insert_user(new_user: &NewUser, pool: &PgPool) -> diesel::QueryResult<usize> {
    use crate::models::NewUser;
    use crate::schema::users;

    let connection = pool
        .get()
        .expect("Failed to acquire a Postgres connection from the pool.");
    diesel::insert_into(users::table)
        .values(NewUser {
            id: &Uuid::new_v4(),
            email: new_user.email.as_ref(),
            name: new_user.name.as_ref(),
            created_at: &Utc::now(),
        })
        .execute(&connection)
}
