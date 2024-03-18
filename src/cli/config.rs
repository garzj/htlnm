use std::{
    fs::{create_dir_all, File},
    io::ErrorKind,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::api::login::LoginData;

use super::io_helpers::exit_err_msg;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub user_name: String,
    pub login_cache: Option<LoginData>,
}

impl Config {
    pub fn new(user_name: String) -> Self {
        Self {
            user_name,
            login_cache: None,
        }
    }

    pub fn load<F: FnOnce() -> Self>(config_path: &PathBuf, new_fn: F) -> Config {
        let file = File::open(&config_path);
        let config_data: Config = file.map_or_else(
            |e| match e.kind() {
                ErrorKind::NotFound => new_fn(),
                _ => exit_err_msg("Failed to open config file", e),
            },
            |f| {
                serde_json::from_reader(f)
                    .unwrap_or_else(|e| exit_err_msg("Failed to parse config file", e))
            },
        );
        config_data
    }

    pub fn write_file(&self, config_path: &PathBuf) {
        if let Some(p) = config_path.parent() {
            create_dir_all(p)
                .unwrap_or_else(|e| exit_err_msg("Failed to create config file parent dirs", e));
        }

        let file = File::create(config_path)
            .unwrap_or_else(|e| exit_err_msg("Failed to open config file", e));
        serde_json::to_writer_pretty(file, self)
            .unwrap_or_else(|e| exit_err_msg("Failed to write config file", e));
    }
}
