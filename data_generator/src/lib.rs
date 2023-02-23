use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub value: f64,
    pub time: SystemTime,
}

impl Data {
    pub fn get_new_value(&self) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            value: rng.gen_range(-10.0..10.0),
            time: SystemTime::now(),
        }
    }

    pub fn new() -> Self {
        Self {
            value: 15.3,
            time: SystemTime::now(),
        }
    }
}
