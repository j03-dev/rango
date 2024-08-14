#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[main]
async fn main() {
    rocket::build()
        .attach(controllers::controller())
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .launch()
        .await
        .expect("failed to launch rocket instance");
}
