use serde_json::{json, Value};
use uuid::Uuid;
use crate::app::BookingService;
use crate::error::AppError;
use crate::infrastructure::ReservationSystem;
use super::request::parse_passengers;
use super::response::{Response, SimpleResponse};

pub fn create_booking(store: &mut ReservationSystem, body: Value) -> Box<dyn Response> {
    let passengers = match parse_passengers(&body) {
        Ok(p) => p,
        Err(e) => return SimpleResponse::unprocessable(&e),
    };

    let id = {
        let mut service = BookingService::new(store);
        match service.create_booking(passengers) {
            Ok(id) => id,
            Err(AppError::SeatAlreadyTaken { service_id, seat_ref }) => return SimpleResponse::conflict(
                &format!("seat '{}' already taken on service '{}'", seat_ref, service_id),
            ),
            Err(e) => return SimpleResponse::unprocessable(&e.to_string()),
        }
    };

    match store.get_booking(id) {
        Ok(booking) => SimpleResponse::created(json!({
            "id": booking.id,
            "passengers": booking.passengers.iter().map(|p| json!({
                "name": p.name,
                "tickets": p.tickets.iter().map(|t| json!({
                    "service_id": t.service_id,
                    "seat": format!("{}{}", t.seat_ref.carriage_id, t.seat_ref.seat_id),
                    "origin": t.origin,
                    "destination": t.destination,
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })),
        Err(_) => SimpleResponse::internal_error("booking vanished after creation"),
    }
}

pub fn get_booking(store: &ReservationSystem, id_str: &str) -> Box<dyn Response> {
    let id = match Uuid::parse_str(id_str) {
        Ok(id) => id,
        Err(_) => return SimpleResponse::unprocessable("invalid booking id"),
    };

    match store.get_booking(id) {
        Ok(booking) => SimpleResponse::ok(json!({
            "id": booking.id,
            "passengers": booking.passengers.iter().map(|p| json!({
                "name": p.name,
                "tickets": p.tickets.iter().map(|t| json!({
                    "service_id": t.service_id,
                    "seat": format!("{}{}", t.seat_ref.carriage_id, t.seat_ref.seat_id),
                    "origin": t.origin,
                    "destination": t.destination,
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })),
        Err(AppError::BookingNotFound(_)) => SimpleResponse::not_found("booking not found"),
        Err(e) => SimpleResponse::unprocessable(&e.to_string()),
    }
}
