use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}
