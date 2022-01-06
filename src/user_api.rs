use rocket::serde::json::Json;
use rocket::response::status::{NotFound, Created, NoContent};
use diesel::prelude::*;

use crate::PgConnection;
use crate::models::{User, NewUser, UpdateUser, ApiError};
use crate::schema::users;


#[get("/")]
pub fn index() -> &'static str {
    "pong"
}

#[get("/")]
pub async fn get_users(conn: PgConnection) -> Json<Vec<User>> {
    conn
        .run(|c| users::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch users")
}

#[get("/<id>")]
pub async fn get_user_by_id(conn: PgConnection, id: i32) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    conn
        .run(move |c| users::table.filter(users::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(
                Json(ApiError {
                    details: e.to_string()
                })
            )
        })
            
}

#[post("/", data = "<user>")]
pub async fn create_user(conn: PgConnection, user: Json<NewUser>) -> Result<Created<Json<User>>, Json<ApiError>> {
    conn
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&user.into_inner())
                .get_result(c)
        })
        .await
        .map(|x| Created::new("/").body(Json(x)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string()
            })
        })
}

#[put("/", data = "<update_user>")]
pub async fn update_user(conn: PgConnection, update_user: Json<UpdateUser>) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    let user = update_user.into_inner();

    conn
        .run(move |c| {            
            diesel::update(users::table.find(user.id))
                .set(&user)
                .get_result(c)
        })
        .await
        .map(Json)
        .map_err(|e| NotFound(Json(
            ApiError {
                details: e.to_string()
            }))
        )
}

#[delete("/<id>")]
pub async fn delete_user(conn: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    conn
        .run(move |c| {
            let affected = diesel::delete(users::table.filter(users::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
                match affected {
                    1 => Ok(()),
                    0 => Err("NotFound".to_string()),
                    x => Err(format!("error code: {}", x))
                }                
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| NotFound(Json(
            ApiError {
                details: e.to_string()
            }))
        )
}
