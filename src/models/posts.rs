use crate::{errors::AppError, schema::posts, DbPool};
use actix_web::{get, post, web, HttpResponse, ResponseError};
use diesel::{associations::HasTable, prelude::*};
use serde_derive::{Deserialize, Serialize};

use super::users::User;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg), belongs_to(users), table_name = posts)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub user_id: i64,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i64,
}
#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub user: User,
}

#[derive(Deserialize)]
pub struct PostData {
    pub title: String,
    pub body: String,
}

#[get("/")]
pub async fn get_posts(conn: web::Data<DbPool>) -> HttpResponse {
    use crate::schema::users::dsl::*;
    let posts_with_users = web::block(move || {
        let connection = &mut conn.get().expect("Couldn't get db connection from pool");
        let all_posts = posts::table
            .inner_join(users::table())
            .limit(5)
            .select((posts::id, posts::title, posts::body, users::all_columns()))
            .load::<(i64, String, String, User)>(connection)
            .expect("msg");

        let result = all_posts
            .into_iter()
            .map(|(p_id, title, body, user)| PostResponse {
                id: p_id,
                title,
                body,
                user,
            })
            .collect::<Vec<PostResponse>>();

        result
    })
    .await;

    match posts_with_users {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => AppError::from(e).error_response(),
    }
}

#[post("/create_post")]
pub async fn create_post(
    conn: web::Data<DbPool>,
    post_data: web::Json<Vec<PostData>>,
) -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let connection = &mut conn.get().expect("Couldn't get db connection from pool");

    let new_posts = post_data
        .into_inner()
        .iter()
        .map(|post| NewPost {
            title: post.title.clone(),
            body: post.body.clone(),
            user_id: 1,
        })
        .collect::<Vec<NewPost>>();

    let result = diesel::insert_into(posts)
        .values(new_posts)
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
pub async fn get_post_by_id(conn: web::Data<DbPool>, post_id: web::Path<i64>) -> HttpResponse {
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
    conn: web::Data<DbPool>,
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
