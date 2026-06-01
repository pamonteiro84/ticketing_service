use serde_json::json;
use ticketing_service::api::{HttpClient, Router};
use ticketing_service::infrastructure::{seed, ReservationSystem};

fn make_router() -> Router {
    let routes = seed::make_routes();
    let services = seed::make_services(&routes);
    Router::new(ReservationSystem::new(services))
}

#[test]
fn book_two_first_class_seats_returns_201() {
    let mut router = make_router();

    let body = json!({
        "passengers": [
            {
                "name": "Alice",
                "tickets": [{ "service_id": "5160", "seat": "A11", "origin": "Paris", "destination": "Amsterdam" }]
            },
            {
                "name": "Bob",
                "tickets": [{ "service_id": "5160", "seat": "A12", "origin": "Paris", "destination": "Amsterdam" }]
            }
        ]
    });

    let response = router.post("/bookings", body);
    assert_eq!(response.status_code(), 201);
}

#[test]
fn booking_same_seats_twice_returns_409() {
    let mut router = make_router();

    let body = json!({
        "passengers": [
            {
                "name": "Alice",
                "tickets": [{ "service_id": "5160", "seat": "A11", "origin": "Paris", "destination": "Amsterdam" }]
            },
            {
                "name": "Bob",
                "tickets": [{ "service_id": "5160", "seat": "A12", "origin": "Paris", "destination": "Amsterdam" }]
            }
        ]
    });

    router.post("/bookings", body.clone());
    let response = router.post("/bookings", body);
    assert_eq!(response.status_code(), 409);
}

#[test]
fn book_multi_service_trip_returns_201() {
    let mut router = make_router();

    let body = json!({
        "passengers": [
            {
                "name": "Carol",
                "tickets": [
                    { "service_id": "9001", "seat": "H1", "origin": "London", "destination": "Paris" },
                    { "service_id": "9002", "seat": "A1", "origin": "Paris", "destination": "Amsterdam" }
                ]
            },
            {
                "name": "Dave",
                "tickets": [
                    { "service_id": "9001", "seat": "N5", "origin": "London", "destination": "Paris" },
                    { "service_id": "9002", "seat": "T7", "origin": "Paris", "destination": "Amsterdam" }
                ]
            }
        ]
    });

    let response = router.post("/bookings", body);
    assert_eq!(response.status_code(), 201);
}

#[test]
fn booking_same_multi_service_trip_twice_returns_409() {
    let mut router = make_router();

    let body = json!({
        "passengers": [
            {
                "name": "Carol",
                "tickets": [
                    { "service_id": "9001", "seat": "H1", "origin": "London", "destination": "Paris" },
                    { "service_id": "9002", "seat": "A1", "origin": "Paris", "destination": "Amsterdam" }
                ]
            },
            {
                "name": "Dave",
                "tickets": [
                    { "service_id": "9001", "seat": "N5", "origin": "London", "destination": "Paris" },
                    { "service_id": "9002", "seat": "T7", "origin": "Paris", "destination": "Amsterdam" }
                ]
            }
        ]
    });

    router.post("/bookings", body.clone());
    let response = router.post("/bookings", body);
    assert_eq!(response.status_code(), 409);
}
