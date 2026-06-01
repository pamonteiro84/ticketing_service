use serde::Serialize;
use serde_json::{json, Value};

pub trait Response {
    fn status_code(&self) -> u16;
    fn body(&self) -> Value;
}

pub trait HttpClient {
    fn post(&mut self, url: &str, body: Value) -> Box<dyn Response>;
    fn get(&self, url: &str) -> Box<dyn Response>;
}

pub struct SimpleResponse {
    status: u16,
    body: Value,
}

impl SimpleResponse {
    fn build(status: u16, body: Value) -> Box<dyn Response> {
        Box::new(Self { status, body })
    }

    pub fn ok(data: impl Serialize) -> Box<dyn Response> {
        Self::build(200, json!(data))
    }

    pub fn created(data: impl Serialize) -> Box<dyn Response> {
        Self::build(201, json!(data))
    }

    pub fn not_found(message: &str) -> Box<dyn Response> {
        Self::build(404, json!({ "error": message }))
    }

    pub fn conflict(message: &str) -> Box<dyn Response> {
        Self::build(409, json!({ "error": message }))
    }

    pub fn unprocessable(message: &str) -> Box<dyn Response> {
        Self::build(422, json!({ "error": message }))
    }

    pub fn internal_error(message: &str) -> Box<dyn Response> {
        Self::build(500, json!({ "error": message }))
    }
}

impl Response for SimpleResponse {
    fn status_code(&self) -> u16 {
        self.status
    }

    fn body(&self) -> Value {
        self.body.clone()
    }
}
