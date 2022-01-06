use rocket::serde::{Deserialize, Serialize};
use diesel::{Insertable, Queryable, AsChangeset};

use crate::schema::users;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}