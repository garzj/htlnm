use std::path::PathBuf;

use base_cmd::BaseCommand;
use clap::Parser;
use std::borrow::Cow;

use crate::api::Api;

use self::{
    config::Config,
    io_helpers::{exit_err_msg, question, question_password},
};

mod base_cmd;
mod config;
mod io_helpers;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    /// Forces a new session
    #[arg(short, long)]
    login: bool,

    /// User for login
    #[arg(short, long)]
    user: Option<String>,

    /// Password for login
    #[arg(short, long)]
    password: Option<String>,

    /// Sets a custom config file to save the token
    #[arg(short, long, value_name = "FILE", default_value = Cli::default_config_path() )]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<BaseCommand>,
}

impl Cli {
    fn default_config_path() -> String {
        dirs::config_local_dir()
            .unwrap_or_else(|| PathBuf::new())
            .join("htlnm")
            .join("config.json")
            .to_str()
            .expect("Failed to construct path for config file.")
            .to_owned()
    }

    pub fn run(&self) {
        let mut app_config = Config::load(&self.config, || {
            Config::new(
                self.user
                    .clone()
                    .unwrap_or_else(|| question("No username cached. Please enter: ")),
            )
        });
        if let Some(username) = self.user.clone() {
            app_config.user_name = username
        }

        let mut api = Api::new();
        let login_data = api
            .login(
                &app_config.login_cache,
                self.login,
                &app_config.user_name,
                || {
                    self.password.as_ref().map_or_else(
                        || Cow::from(question_password("Please enter your password: ")),
                        |v| Cow::from(v),
                    )
                },
            )
            .unwrap_or_else(|e| exit_err_msg("Login failed", e));
        let is_new_session = app_config
            .login_cache
            .map_or(true, |c| !c.access_token.eq(&login_data.access_token));
        app_config.login_cache = Some(login_data.clone());
        app_config.write_file(&self.config);

        if let Some(ref cmd) = self.command {
            cmd.run(&api);
        } else if is_new_session {
            println!("Logged in.");
        } else {
            println!("Nothing to do.");
        }
    }
}
