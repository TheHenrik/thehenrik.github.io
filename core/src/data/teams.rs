use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u8,
    pub name: String,
    pub uni: String,
    pub country: String,
    pub flag: String,
}
