use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidSeatRef(String),
    ServiceNotFound(String),
    StationNotFound { service_id: String, station: String },
    SeatNotFound { service_id: String, seat_ref: String },
    InvalidLeg { origin: String, destination: String },
    SeatAlreadyTaken { service_id: String, seat_ref: String },
    BookingNotFound(Uuid),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidSeatRef(msg) => write!(f, "Invalid seat ref: {}", msg),
            AppError::ServiceNotFound(id) => write!(f, "Service not found: {}", id),
            AppError::StationNotFound { service_id, station } => write!(f, "Station '{}' not found on service '{}'", station, service_id),
            AppError::SeatNotFound { service_id, seat_ref } => write!(f, "Seat '{}' not found on service '{}'", seat_ref, service_id),
            AppError::InvalidLeg { origin, destination } => write!(f, "Invalid leg: '{}' -> '{}'", origin, destination),
            AppError::SeatAlreadyTaken { service_id, seat_ref } => write!(f, "Seat '{}' already taken on service '{}'", seat_ref, service_id),
            AppError::BookingNotFound(id) => write!(f, "Booking not found: {}", id),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

