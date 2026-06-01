use ticketing_service::infrastructure::{seed, ReservationSystem};
use ticketing_service::api::Router;

// Running an HTTP server is out of scope for this assignment.
// The REST interface is implemented via the HttpClient trait and exercised directly in integration tests.
fn main() {
    let routes = seed::make_routes();
    let services = seed::make_services(&routes);
    let store = ReservationSystem::new(services);
    let _router = Router::new(store);
}

