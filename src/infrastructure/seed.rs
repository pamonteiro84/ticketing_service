use crate::domain::{Carriage, ComfortClass, Route, RouteStop, Seat, Service, Station};
use chrono::TimeZone;

pub fn make_routes() -> Vec<Route> {
    vec![
        Route {
            id: "paris-london".to_string(),
            stops: vec![
                stop("Paris", 0),
                stop("Lille", 225),
                stop("Calais", 115),
                stop("London", 130),
            ],
        },
        Route {
            id: "paris-amsterdam".to_string(),
            stops: vec![
                stop("Paris", 0),
                stop("Brussels", 265),
                stop("Antwerp", 45),
                stop("Amsterdam", 100),
            ],
        },
        Route {
            id: "amsterdam-berlin".to_string(),
            stops: vec![
                stop("Amsterdam", 0),
                stop("Cologne", 230),
                stop("Hannover", 185),
                stop("Berlin", 250),
            ],
        },
        Route {
            id: "london-paris".to_string(),
            stops: vec![
                stop("London", 0),
                stop("Calais", 130),
                stop("Lille", 115),
                stop("Paris", 225),
            ],
        },
    ]
}

pub fn make_services(routes: &[Route]) -> Vec<Service> {
    let paris_amsterdam = find(routes, "paris-amsterdam");
    let london_paris = find(routes, "london-paris");

    vec![
        Service {
            id: "5160".to_string(),
            route: paris_amsterdam.clone(),
            departure: chrono::Utc.with_ymd_and_hms(2021, 4, 1, 8, 0, 0).unwrap(),
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
            departure: chrono::Utc.with_ymd_and_hms(2021, 4, 1, 6, 0, 0).unwrap(),
            carriages: vec![
                carriage("A", first_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("H", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
                carriage("N", second_class(&["1","2","3","4","5","6","7","8","9","10"])),
            ],
        },
        Service {
            id: "9002".to_string(),
            route: paris_amsterdam,
            departure: chrono::Utc.with_ymd_and_hms(2021, 4, 1, 10, 0, 0).unwrap(),
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

fn find(routes: &[Route], id: &str) -> Route {
    routes.iter().find(|r| r.id == id).unwrap().clone()
}
