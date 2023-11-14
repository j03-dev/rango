use rocket::{
    http::Status,
    serde::json::{json, Value},
};
use rocket::serde::Serialize;

use crate::repositories::Repository;

pub type Response = Result<Value, Status>;

pub trait ModelViewSet<S: Serialize>: Repository<S> {
    fn create(&self) -> Response {
        if self.save() {
            Ok(json!("add success"))
        } else {
            Err(Status::BadRequest)
        }
    }

    fn list() -> Response {
        Ok(json!(Self::all()))
    }

    fn retrieve(id: i32) -> Response {
        if let Some(result) = Self::get(id).get(0) {
            Ok(json!(result))
        } else {
            Err(Status::NotFound)
        }
    }

    fn destroy(id: i32) -> Response {
        if Self::delete(id) {
            Ok(json!("deleted success"))
        } else {
            Err(Status::BadRequest)
        }
    }

    fn put(&self) -> Response {
        if self.update() {
            Ok(json!("update success"))
        } else {
            Err(Status::BadRequest)
        }
    }
}
