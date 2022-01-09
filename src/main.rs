use user_service_backend::PgConnection;
use user_service_backend::user_api::{index, get_users, get_user_by_id, create_user, update_user, delete_user};
use rocket::{launch, routes};

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(PgConnection::fairing())
    .attach(CORS)
    .mount("/ping", routes![index])
    .mount("/api/v1/users", routes![get_users, get_user_by_id, create_user, update_user, delete_user])
}
