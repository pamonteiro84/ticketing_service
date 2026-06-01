use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::carriage::Carriage;
use super::route::Route;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub route: Route,
    pub departure: DateTime<Utc>,
    pub carriages: Vec<Carriage>,
}

