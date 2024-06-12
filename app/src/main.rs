#[macro_use]
extern crate rocket;

#[main]
async fn main() {
    rocket::build()
        .attach(controllers::controller())
        .launch()
        .await
        .expect("failed to launch rocket instance");
}
