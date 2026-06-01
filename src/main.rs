use ticketing_service::infrastructure::{seed, ReservationSystem};
use ticketing_service::api::Router;

// Running an HTTP server is out of scope for this assignment.
// The REST interface is implemented via the HttpClient trait and exercised directly in integration tests.
fn main() {
    let store = ReservationSystem::new(seed::make_services());
    let _router = Router::new(store);
}

