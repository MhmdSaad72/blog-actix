use diesel::{associations::HasTable, prelude::*};
use serde_derive::Serialize;

use crate::{errors::AppError, schema::users};

type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub user_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub user_name: &'a str,
    pub email: &'a str,
}

pub fn create_user(conn: &mut PgConnection, username: &str, user_email: &str) -> Result<User> {
    use self::users::dsl::*;

    let new_user = NewUser {
        user_name: username,
        email: user_email,
    };
    conn.transaction::<User, AppError, _>(|conn| {
        diesel::insert_into(users).values(&new_user).execute(conn)?;

        User::table()
            .order(id.desc())
            .select(User::as_select())
            .first(conn)
            .map_err(Into::into)
    })
}
