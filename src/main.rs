extern crate rfc822_sanitizer;

use clap::Parser;
use cli::Cli;

mod api;
mod cli;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
