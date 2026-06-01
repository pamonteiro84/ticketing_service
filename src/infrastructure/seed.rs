use crate::domain::{Carriage, ComfortClass, Route, RouteStop, Seat, Service, Station};
use chrono::DateTime;

pub fn make_services() -> Vec<Service> {
    let paris_amsterdam = Route {
        id: "paris-amsterdam".to_string(),
        stops: vec![
            stop("Paris", 0),
            stop("Brussels", 265),
            stop("Antwerp", 45),
            stop("Amsterdam", 100),
        ],
    };
    let london_paris = Route {
        id: "london-paris".to_string(),
        stops: vec![
            stop("London", 0),
            stop("Calais", 130),
            stop("Lille", 115),
            stop("Paris", 225),
        ],
    };

    vec![
        Service {
            id: "5160".to_string(),
            route: paris_amsterdam.clone(),
            departure: DateTime::from_timestamp(1_617_264_000, 0).unwrap_or_default(),
            carriages: vec![
                carriage("A", first_class(&["1","2","3","4","5","6","7","8","9","10","11","12","13","14","15"])),
                carriage("H", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("N", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("T", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
            ],
        },
        Service {
            id: "9001".to_string(),
            route: london_paris,
            departure: DateTime::from_timestamp(1_617_256_800, 0).unwrap_or_default(),
            carriages: vec![
                carriage("A", first_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("H", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("N", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
            ],
        },
        Service {
            id: "9002".to_string(),
            route: paris_amsterdam,
            departure: DateTime::from_timestamp(1_617_271_200, 0).unwrap_or_default(),
            carriages: vec![
                carriage("A", first_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("T", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
            ],
        },
    ]
}

fn stop(name: &str, distance_km: u32) -> RouteStop {
    RouteStop { station: Station::new(name), distance_km }
}

fn carriage(id: &str, seats: Vec<Seat>) -> Carriage {
    Carriage { id: id.to_string(), seats }
}

fn first_class(ids: &[&str]) -> Vec<Seat> {
    ids.iter().map(|id| Seat { id: id.to_string(), comfort: ComfortClass::FirstClass }).collect()
}

fn second_class(ids: &[&str]) -> Vec<Seat> {
    ids.iter().map(|id| Seat { id: id.to_string(), comfort: ComfortClass::SecondClass }).collect()
}


