#[macro_use]
extern crate rocket;

use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

use services::auth::obtain_auth_token;
use services::user::{destroy_user, register_user, retrieve_user, update_user};

mod auth;
mod model_view;
mod repositories;
mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: AllowedMethods::new(),
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
                obtain_auth_token,
                register_user,
                destroy_user,
                retrieve_user,
                update_user
            ],
        )
        .attach(cors)
}
