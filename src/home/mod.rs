// use reqwest;
// use reqwest::header::AUTHORIZATION;
// use reqwest::header::CONTENT_TYPE;
// use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};

pub mod fan;

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    entity_id: String,
}

