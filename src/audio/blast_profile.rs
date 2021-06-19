use super::blast_wave::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

/// A helpder struct for storing blast profiles
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlastProfile {
    pub delay: Duration,
    pub peak: f32,
    pub positive_phase_duration: Duration,
    pub curve: f32,
    pub bands: Vec<(BlastHarmonicBand, HarmonicGeneration)>,
    pub transient_clipping: u16,
}

impl BlastProfile {
    pub fn from_file(name: &str) -> Self {
        let path_string = format!("{}.json", name);
        let mut file = match File::open(&path_string) {
            Err(why) => panic!("couldn't open {}: {}", path_string, why),
            Ok(file) => file,
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", path_string, why),
            Ok(_) => (),
        }
        serde_json::from_str(&data).unwrap()
    }

    pub fn serialize(&self, name: String) -> Result<()> {
        let profile = serde_json::to_string(&self)?;

        let path_string = format!("{}.json", name);
        let path = Path::new(&path_string);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(profile.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        Ok(())
    }
}
