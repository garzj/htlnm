use clap::Subcommand;

use crate::api::Api;

use self::get_cmd::GetCommand;

use super::io_helpers::exit_err_msg;

mod get_cmd;

#[derive(Subcommand)]
pub enum BaseCommand {
    /// Fetch data from the api
    Get(GetCommand),
}

impl BaseCommand {
    pub fn run(&self, api: &Api) {
        match self {
            BaseCommand::Get(get_cmd) => get_cmd
                .run(&api)
                .unwrap_or_else(|e| exit_err_msg("Api request failed", e)),
        };
    }
}
