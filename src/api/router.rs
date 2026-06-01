use serde_json::Value;
use crate::infrastructure::ReservationSystem;
use super::handlers;
use super::response::{HttpClient, Response, SimpleResponse};

pub struct Router {
    store: ReservationSystem,
}

impl Router {
    pub fn new(store: ReservationSystem) -> Self {
        Self { store }
    }
}

impl HttpClient for Router {
    fn post(&mut self, url: &str, body: Value) -> Box<dyn Response> {
        match url {
            "/bookings" => handlers::create_booking(&mut self.store, body),
            _ => SimpleResponse::not_found("unknown route"),
        }
    }

    fn get(&self, url: &str) -> Box<dyn Response> {
        let segments: Vec<&str> = url.trim_start_matches('/').split('/').collect();

        match segments.as_slice() {
            ["bookings", id] => handlers::get_booking(&self.store, id),
            _ => SimpleResponse::not_found("unknown route"),
        }
    }
}
