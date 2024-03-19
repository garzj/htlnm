use clap::Subcommand;
use handlebars::Handlebars;
use serde::Serialize;

use crate::api::Api;

use self::{dump_cmd::DumpCommand, get_cmd::GetCommand};

use super::io_helpers::exit_err_msg;

mod dump_cmd;
mod get_cmd;

#[derive(Subcommand)]
pub enum BaseCommand {
    /// Fetch data from the api
    Get(GetCommand),
    /// Dump all data from the api
    Dump(DumpCommand),
}

impl BaseCommand {
    pub fn run(&self, api: &Api) {
        match self {
            BaseCommand::Get(get_cmd) => get_cmd
                .run(&api)
                .unwrap_or_else(|e| exit_err_msg("Api request failed", e)),
            BaseCommand::Dump(dump_cmd) => dump_cmd
                .run(&api)
                .unwrap_or_else(|e| exit_err_msg("Dump failed", e)),
        };
    }
}

fn print_data<T>(format: &Option<String>, data: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    println!(
        "{}",
        format.as_ref().map_or_else(
            || anyhow::Ok(serde_json::to_string_pretty(data)?),
            |format| anyhow::Ok(Handlebars::new().render_template(&format, data)?)
        )?
    );
    Ok(())
}
