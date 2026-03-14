use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Session {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOnline {
    pub current: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub session_id: String,
}