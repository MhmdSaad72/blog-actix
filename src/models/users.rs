use diesel::prelude::*;
use serde_derive::Serialize;

use crate::{errors::AppError, schema::users};

#[allow(dead_code)]
type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub email: &'a str,
}

#[allow(dead_code)]
pub fn create_user(conn: &mut PgConnection, username: &str, user_email: &str) -> Result<User> {
    use self::users::dsl::*;

    let new_user = NewUser {
        user_name: username,
        email: user_email,
    };

    let result = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn);

    Ok(result.unwrap())
}
