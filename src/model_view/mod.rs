use rocket::serde::Serialize;
use rocket::{
    http::Status,
    serde::json::{json, Value},
};

use crate::repositories::Repository;

pub type Response = Result<Value, Status>;

pub trait ModelViewSet<S: Serialize>: Repository<S> {
    fn create(&self) -> Response {
        if self.save() {
            return Ok(json!("add success"));
        }
        Err(Status::BadRequest)
    }

    fn list() -> Response {
        Ok(json!(Self::all()))
    }

    fn retrive(id: i32) -> Response {
        if let Some(result) = Self::get(id).get(0) {
            return Ok(json!(result));
        }
        Err(Status::NotFound)
    }

    fn destory(id: i32) -> Response {
        if Self::delete(id) {
            return Ok(json!("deleted success"));
        }
        Err(Status::BadRequest)
    }

    fn put(&self) -> Response {
        if self.update() {
            return Ok(json!("update success"));
        }
        Err(Status::BadRequest)
    }
}
