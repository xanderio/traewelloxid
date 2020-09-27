use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub base_url: String,
}

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("api responsed with bad request (400)")]
    BadRequest400,
    #[error("api responsed with unauthorized (401)")]
    Unauthorized401,
    #[error("api responsed with non acceptable (406)")]
    NotAcceptable406,
}

pub struct StatusPaginate {
    current_page: u64,
    data: Vec<Status>,
    first_page_url: String,
    from_page_url: String,
    from: u64,
    next_page_url: String,
    path: String,
    per_page: u64,
    prev_page_url: String,
    to: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Status {
    id: u64,
    created_at: String,
    updated_at: String,
    body: String,
    #[serde(rename = "type")]
    status_type: String,
    link_count: String,
    liked: bool,
    user: User,
    train_checkin: TrainCheckin,
    event: Option<Event>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    id: u64,
    name: String,
    username: String,
    train_distance: f64,
    train_duration: u64,
    points: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TrainCheckin {
    id: u64,
    status_id: u64,
    trip_id: String,
    origin: Station,
    destintion: Station,
    distance: u64,
    departure: String,
    arrival: String,
    points: u64,
    delay: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Station {
    id: u64,
    ibnr: String,
    name: String,
    latiude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HafasTrip {
    id: u64,
    trip_id: String,
    category: String,
    number: String,
    linename: String,
    origin: String,
    destination: String,
    stopovers: String,
    polyline: String,
    departure: String,
    arrival: String,
    delay: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: u64,
    name: String,
    slug: String,
    hashtag: String,
    host: String,
    url: String,
    trainstation: String,
    begin: String,
    end: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum View {
    Global,
    Personal,
    User(u32),
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                View::Global => "global",
                View::Personal => "personal",
                View::User(_) => "user",
            }
        )
    }
}
