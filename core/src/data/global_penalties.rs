use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "Delay_Prelim")]
    pub delay_prelim: f32,
    #[serde(rename = "Delay_Report")]
    pub delay_report: f32,
    #[serde(rename = "Poster")]
    pub poster_missing: f32,
    #[serde(rename = "Proof_of_flight")]
    pub delay_pof: f32,
    #[serde(rename = "Tech_inspection")]
    pub tech_inspection: f32,
    #[serde(rename = "Disregard_Instructions")]
    pub disregard_instructions: f32,
    #[serde(rename = "Protest")]
    pub protest: f32,
    #[serde(rename = "Drawings")]
    pub drawings: f32,
    #[serde(rename = "Aircraft")]
    pub aircraft_penalty: f32,
    #[serde(rename = "DSQ")]
    pub dsq: f32,
}

impl Penalty {
    pub fn sum(&self) -> f32 {
        self.delay_prelim
            + self.delay_report
            + self.poster_missing
            + self.delay_pof
            + self.tech_inspection
            + self.disregard_instructions
            + self.protest
            + self.drawings
            + self.aircraft_penalty
            + self.dsq
    }
}
