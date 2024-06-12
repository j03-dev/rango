mod user_controler;

pub use user_controler::user_controler;

pub mod custome_response {
    pub use rocket::http::Status;
    pub use rocket::{
        response::status::Custom,
        serde::json::{json, Json, Value},
    };

    pub type V = Custom<Value>;
    pub type Response = Result<V, V>;
}
