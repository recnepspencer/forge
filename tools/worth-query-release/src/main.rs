//! Host-owned release ceremony for one externally signed Query package.

mod command;
mod denial;
mod expectations;
mod finalization;
mod input;
mod output;
mod preflight;
mod readmission;
mod report;

use clap::Parser;

use command::WorthQueryReleaseCommand;

fn main() {
    if let Err(error) = WorthQueryReleaseCommand::parse().execute() {
        eprintln!("worth-query-release: {error}");
        std::process::exit(1);
    }
}
