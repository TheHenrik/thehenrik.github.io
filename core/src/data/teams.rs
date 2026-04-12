use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "University")]
    pub uni: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Flag")]
    pub flag: String,
}
