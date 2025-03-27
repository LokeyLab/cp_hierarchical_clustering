use std::{error::Error, fs::File, io::Write};

use super::ClusterHierarchy;

impl ClusterHierarchy {
    pub fn simple_save(&self, filename: &str) -> std::io::Result<()> {
        let json_str =
            serde_json::to_string_pretty(&self.merges).expect("Can't serialize hierarchy!");

        let mut file = File::create(filename)?;
        file.write_all(json_str.as_bytes())?;

        return Ok(());
    }

    pub fn to_string(&self) -> Result<String, Box<dyn Error>> {
        return Ok(serde_json::to_string_pretty(&self.merges).expect("Can't serialize hierarchy!"));
    }
}
