pub mod error;
mod domain;
mod infrastructure;
mod app;
mod api;

use ticketing_service::infrastructure::{seed, ReservationSystem};
use ticketing_service::api::Router;

fn main() {
    println!("Hello, world!");
}
