#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(controllers::controller().await?)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .launch()
        .await?;
    Ok(())
}
