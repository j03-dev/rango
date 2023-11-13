#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use services::auth::obtain_authtoken;
use services::user::{destory_user, register_user, retrive_user, udpate_user};
use std::str::FromStr;

mod auth;
mod model_view;
mod repositories;
mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    let allowed_methods: AllowedMethods = ["Get", "Post"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                obtain_authtoken,
                register_user,
                destory_user,
                retrive_user,
                udpate_user
            ],
        )
        .attach(cors)
}
