use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalScore {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "Round 1")]
    pub round_1: f32,
    #[serde(rename = "Round 2")]
    pub round_2: f32,
    #[serde(rename = "Round 3")]
    pub round_3: f32,
    #[serde(rename = "Round 4")]
    pub round_4: f32,
    #[serde(rename = "Round 5")]
    pub round_5: f32,
    #[serde(rename = "Round Total")]
    pub round_total: f32,
    #[serde(rename = "Presentation")]
    pub presentation: f32,
    #[serde(rename = "Report")]
    pub report: f32,
    #[serde(rename = "Penalties")]
    pub penalties: f32,
    #[serde(rename = "Total")]
    pub total: f32,
}

impl GlobalScore {
    pub fn rounds(&self) -> Vec<f32> {
        vec![
            self.round_1,
            self.round_2,
            self.round_3,
            self.round_4,
            self.round_5,
        ]
    }

    pub fn set_rounds(&mut self, rounds: &[f32]) {
        if let Some(&r) = rounds.get(0) {
            self.round_1 = r;
        }
        if let Some(&r) = rounds.get(1) {
            self.round_2 = r;
        }
        if let Some(&r) = rounds.get(2) {
            self.round_3 = r;
        }
        if let Some(&r) = rounds.get(3) {
            self.round_4 = r;
        }
        if let Some(&r) = rounds.get(4) {
            self.round_5 = r;
        }
    }
}
