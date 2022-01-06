use user_service_backend::PgConnection;
use user_service_backend::user_api::{index, get_users, get_user_by_id, create_user, update_user, delete_user};
use rocket::{launch, routes};


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(PgConnection::fairing())
    .mount("/ping", routes![index])
    .mount("/api/v1/users", routes![get_users, get_user_by_id, create_user, update_user, delete_user])
}
