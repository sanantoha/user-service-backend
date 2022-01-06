pub mod schema;
pub mod models;
pub mod user_api;

#[macro_use]
extern crate diesel;

#[macro_use] 
extern crate rocket;

use rocket_sync_db_pools::database;

#[database("db")]
pub struct PgConnection(diesel::PgConnection);
