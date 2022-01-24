use serde::{Deserialize, Serialize};
use std::{fs, io};

use crate::GlobalPaths;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub minecraft_creds: MinecraftCredentials,
    pub setup_complete: bool,
}

impl Config {
    pub fn write(config: Self) -> Result<(), io::Error> {
        fs::write(
            &GlobalPaths::get().config_file,
            serde_json::to_string_pretty(&config).unwrap(),
        )?;

        Ok(())
    }

    pub fn read() -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = fs::read_to_string(&GlobalPaths::get().config_file)?;
        let config: Self = serde_json::from_str(&config_file)?;

        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MinecraftCredentials {
    pub username: String,
    pub password: String,
}
