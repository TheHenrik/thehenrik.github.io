#[cfg(test)]
mod tests {
    use crate::data::Round;
    use crate::data::flight::{Flight, Telemetry};
    use std::path::PathBuf;

    #[test]
    fn test_scoring_basic() {
        let n_announced = vec![10u8];
        let round = Round {
            path: PathBuf::from("fake_path"),
            scores: Vec::new(),
            flights: vec![Flight {
                team_id: 1,
                payload: 10,
                take_off: 40.0,
                loading: 30.0,
                unloading: 30.0,
                telemetry: vec![
                    Telemetry {
                        time: 0.0,
                        current: 0.0,
                        speed: 0.0,
                        altitude: 0.0,
                        position: 0.0,
                        power: 0.0,
                        voltage: 12.0,
                    },
                    Telemetry {
                        time: 10.0,
                        current: 40.0,
                        speed: 10.0,
                        altitude: 10.0,
                        position: 100.0,
                        power: 480.0,
                        voltage: 12.0,
                    },
                ],
            }],
        };

        let scores = round.update_score(&n_announced);
        assert!(scores[0] > 1000.0); // 1000 (normalized) + 30 (loading) + 30 (prediction)
    }

    #[test]
    fn test_zero_score_voltage() {
        let n_announced = vec![10u8];
        let round = Round {
            path: PathBuf::from("fake_path"),
            scores: Vec::new(),
            flights: vec![Flight {
                team_id: 1,
                payload: 10,
                take_off: 40.0,
                loading: 30.0,
                unloading: 30.0,
                telemetry: vec![Telemetry {
                    time: 0.0,
                    current: 0.0,
                    speed: 0.0,
                    altitude: 0.0,
                    position: 0.0,
                    power: 0.0,
                    voltage: 13.0,
                }],
            }],
        };

        let scores = round.update_score(&n_announced);
        assert_eq!(scores[0], 0.0);
    }
}
