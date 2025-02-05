use crate::{errors::AppError, schema::posts};
use actix_web::{get, post, web, HttpResponse, ResponseError};
use diesel::{
    associations::HasTable,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use serde_derive::{Deserialize, Serialize};

#[allow(dead_code)]
type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[get("/")]
pub async fn get_posts(conn: web::Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");
    let result = Post::table()
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    HttpResponse::Ok().json(result)
}

#[post("/create_post")]
pub async fn create_post(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    new_posts: web::Json<Vec<NewPost>>,
) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");

    let result = diesel::insert_into(posts)
        .values(new_posts.into_inner())
        .returning(Post::as_returning())
        .get_results(connection);

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            println!("{:?}", e);
            AppError::from(e).error_response()
        }
    }
}

#[get("/{id}")]
pub async fn get_post_by_id(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    post_id: web::Path<i64>,
) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");

    let result = posts
        .find(post_id.into_inner())
        .select(Post::as_select())
        .first::<Post>(connection);
    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => AppError::from(e).error_response(),
    }
}

#[post("/update_post/{id}")]
pub async fn update_post(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    post_id: web::Path<i64>,
    new_post: web::Json<NewPost>,
) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");

    let result = diesel::update(Post::table().find(post_id.into_inner()))
        .set((title.eq(&new_post.title), body.eq(&new_post.body)))
        .returning(Post::as_returning())
        .get_result(connection);

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => AppError::from(e).error_response(),
    }
}
