use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flight {
    pub team_id: u8,
    pub payload: u8,
    pub take_off: f32,
    pub loading: f32,
    pub unloading: f32,
    pub telemetry: Vec<Telemetry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub time: f32,
    pub current: f32,
    pub speed: f32,
    pub altitude: f32,
    pub position: f32,
    pub power: f32,
    pub voltage: f32,
}
