use serde::Deserialize;
use std::{fs::read_to_string, process::exit};

/*
 * Below is the structure of the Configuration.
 * If the TOML file is updated, the ConfigurationStruct should be
 * updated accordingly, along with the appropriate nesting.
 */

#[derive(Deserialize)]
pub struct ConfigurationStruct {
    pub guild: GuildConfig,
}

#[derive(Deserialize)]
pub struct GuildConfig {
    pub id: u64,
    pub roles: RoleConfig,
}

#[derive(Deserialize)]
pub struct RoleConfig {
    pub visitor: u64,
    pub member: u64,
}

pub fn load() -> ConfigurationStruct {
    // Get string of configuration data from TOML.
    let content: String = match read_to_string("./config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to read configuration file: {:?}", e);
            exit(1);
        }
    };

    // Convert string to ConfigurationStruct.
    let configuration: ConfigurationStruct = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            println!("Unable to assign configuration data: {:?}", e);
            exit(1);
        }
    };

    configuration
}
