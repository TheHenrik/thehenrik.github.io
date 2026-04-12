use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundScore {
    pub team_id: u8,
    pub raw: f32,
    pub normalized: f32,
    pub loading_bonus: f32,
    pub prediction_bonus: f32,
    pub total: f32,
}
