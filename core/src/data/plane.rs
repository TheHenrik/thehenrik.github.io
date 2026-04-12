use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    pub team_id: u8,
    pub name: String,
    pub empty_weight: f32,
    #[serde(rename = "n_announced")]
    pub n_announced: u8,
    pub max_payload: u8,
    pub span: f32,
    pub n_motors: u8,
}
