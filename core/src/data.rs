use anyhow::Result;

use std::path::Path;
pub mod flight;
pub mod global_penalties;
pub mod global_scores;
mod io;
pub mod plane;
pub mod round_scores;
#[cfg(test)]
mod scoring_tests;
pub mod teams;

use std::fs;

use flight::Flight;
use global_penalties::Penalty;
use global_scores::GlobalScore;
use io::{load_csv, load_ron, save_csv, save_ron};
use plane::Plane;
use round_scores::RoundScore;
use teams::Team;

pub struct Round {
    pub path: std::path::PathBuf,
    pub scores: Vec<RoundScore>,
    pub flights: Vec<Flight>,
}

impl Round {
    pub fn new(path: &Path) -> Option<Result<Self>> {
        if !path.exists() {
            return None;
        }
        let mut round = Round {
            path: path.to_path_buf(),
            scores: Vec::new(),
            flights: Vec::new(),
        };
        Some(round.load_round().map(|_| round))
    }
    pub fn populate_flights(&mut self) -> Result<()> {
        let mut flights = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                if path.file_stem().and_then(|s| s.to_str()) == Some("scores") {
                    continue;
                }
                let mut flight: Flight = load_ron(&path)?;
                let id_str = path.file_stem().unwrap().to_str().unwrap();
                flight.team_id = id_str.parse()?;

                let csv_path = path.with_extension("csv");
                if csv_path.exists() {
                    flight.telemetry = load_csv(&csv_path)?;
                }
                flights.push(flight);
            }
        }
        self.flights = flights;
        Ok(())
    }
    pub fn update_score(&self, n_announced: &[u8]) -> Vec<f32> {
        todo!()
    }
    pub fn load_round(&mut self) -> Result<()> {
        let scores_path = self.path.join("scores.csv");
        if scores_path.exists() {
            self.scores = load_csv(&scores_path)?;
        }
        self.populate_flights()?;
        Ok(())
    }
    pub fn save_round(&self) -> Result<()> {
        let scores_path = self.path.join("scores.csv");
        save_csv(&scores_path, &self.scores)?;
        for flight in &self.flights {
            let ron_path = self.path.join(format!("{:02}.ron", flight.team_id));
            save_ron(&ron_path, flight)?;
            let csv_path = self.path.join(format!("{:02}.csv", flight.team_id));
            save_csv(&csv_path, &flight.telemetry)?;
        }
        Ok(())
    }
}

pub struct Competition {
    pub teams: Vec<Team>,
    pub planes: Vec<Plane>,
    pub rounds: Vec<Round>,
    pub scores: Vec<GlobalScore>,
    pub penalties: Vec<Penalty>,
}

impl Competition {
    pub fn new() -> Result<Self> {
        let mut comp = Competition {
            teams: Vec::new(),
            planes: Vec::new(),
            rounds: Vec::new(),
            scores: Vec::new(),
            penalties: Vec::new(),
        };
        comp.load_state()?;
        Ok(comp)
    }
    pub fn update_scores(&mut self) {
        let n_announced: Vec<u8> = self.planes.iter().map(|plane| plane.n_announced).collect();
        let round_scores: Vec<Vec<f32>> = self
            .rounds
            .iter()
            .map(|round| round.update_score(&n_announced))
            .collect();

        for (id, global) in self.scores.iter_mut().enumerate() {
            let mut team_round_scores = Vec::new();
            for r_scores in &round_scores {
                if let Some(s) = r_scores.get(id) {
                    team_round_scores.push(*s);
                }
            }
            global.set_rounds(&team_round_scores);
            global.round_total = two_largest(&team_round_scores);
            global.penalties = self.penalties[id].sum();
            global.total =
                global.round_total + global.presentation + global.report - global.penalties;
        }
    }
    pub fn load_state(&mut self) -> Result<()> {
        self.penalties = load_csv(Path::new("data/global_penalties.csv"))?;
        self.scores = load_csv(Path::new("data/scores.csv"))?;
        self.teams = load_csv(Path::new("data/teams.csv"))?;
        self.planes = self
            .teams
            .iter()
            .map(|team| {
                let id = team.id;
                load_ron(Path::new(&format!("data/planes/{id}.ron")))
            })
            .collect::<Result<Vec<Plane>>>()?;
        self.rounds = (1..)
            .map_while(|i| Round::new(Path::new(&format!("data/round_{i}/"))))
            .collect::<Result<Vec<Round>>>()?;
        Ok(())
    }
    pub fn save_state(&self) -> Result<()> {
        save_csv(Path::new("data/global_penalties.csv"), &self.penalties)?;
        save_csv(Path::new("data/scores.csv"), &self.scores)?;
        save_csv(Path::new("data/teams.csv"), &self.teams)?;
        for round in self.rounds.iter() {
            round.save_round()?
        }
        for (id, plane) in self.planes.iter().enumerate() {
            let p = format!("data/planes/{id}.ron");
            let pth = Path::new(&p);
            save_ron(&pth, plane)?;
        }
        Ok(())
    }
}

fn two_largest(v: &[f32]) -> f32 {
    match v {
        [] => 0.0,
        [x] => *x,
        _ => {
            let mut max1 = &v[0];
            let mut max2 = &v[1];

            for x in &v[1..] {
                if x > max1 {
                    max2 = max1;
                    max1 = x;
                } else if x > max2 {
                    max2 = x;
                }
            }
            (max1 + max2) / 2.0
        }
    }
}
