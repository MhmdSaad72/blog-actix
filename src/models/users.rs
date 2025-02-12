use actix_web::{get, post, web, HttpResponse, ResponseError};
use diesel::{associations::HasTable, prelude::*};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::{errors::AppError, schema::users, DbPool};

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

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_name: String,
    pub email: String,
}

#[post("/create-user")]
pub async fn create_user(conn: web::Data<DbPool>, new_user: web::Json<NewUser>) -> HttpResponse {
    use self::users::dsl::*;
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");
    let new_user = new_user.into_inner();
    let result = diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(connection);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => AppError::from(e).error_response(),
    }
}

#[get("/")]
pub async fn get_all_users(conn: web::Data<DbPool>) -> HttpResponse {
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");
    let result = User::table().load::<User>(connection);
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => AppError::from(e).error_response(),
    }
}

#[get("/test-json")]
pub async fn handle_json(body: web::Bytes) -> HttpResponse {
    let json_payload = std::str::from_utf8(&body).expect("msg");
    let json_value: Value = serde_json::from_str(json_payload).expect("gfgf");
    let order_id = search_key(&json_value, "order_id");
    // You can now process the JSON value as needed
    println!("Received JSON: {:?}", json_value);
    println!("Order id: {:?}", order_id);

    HttpResponse::Ok().json(json_value)
}

fn search_key<'a>(json: &'a Value, key: &str) -> Option<&'a Value> {
    match json {
        Value::Object(map) => map
            .get(key)
            .or_else(|| map.values().find_map(|v| search_key(v, key))),
        Value::Array(arr) => arr.iter().find_map(|v| search_key(v, key)),
        _ => None,
    }
}
