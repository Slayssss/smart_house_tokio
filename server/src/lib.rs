use chrono::{DateTime, Utc};
use data_generator::Data;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
// pub struct Data {
//     pub value: f64,
//     pub time: SystemTime,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Thermometer {
    time: SystemTime,
    temperature: f64,
    voltage: f64,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            time: SystemTime::now(),
            temperature: 0.0,
            voltage: 0.0,
        }
    }

    pub fn update(&mut self, data: &Data) -> &Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(40.0..80.0);

        self.time = data.time.clone();
        self.temperature = data.value;
        self.voltage = value;

        self
    }

    pub fn get_data(&self) -> String {
        let dt: DateTime<Utc> = self.time.clone().into();

        format!(
            "{:?},{},{}",
            dt.format("%d/%m/%Y %H:%M"),
            self.temperature,
            self.voltage
        )
    }
}
