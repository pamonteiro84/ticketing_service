use serde::{Deserialize, Serialize};
use super::station::Station;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStop {
    pub station: Station,
    pub distance_km: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub stops: Vec<RouteStop>,
}

impl Route {
    pub fn stop_index(&self, station_name: &str) -> Option<usize> {
        self.stops.iter().position(|s| s.station.name == station_name)
    }

    pub fn legs_overlap(o1: usize, d1: usize, o2: usize, d2: usize) -> bool {
        o1 < d2 && o2 < d1
    }
}