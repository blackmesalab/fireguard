#[macro_use]
extern crate log;
extern crate clap;
extern crate eyre;
extern crate pretty_env_logger;

mod cmd;
mod shell;

use clap::Clap;
use eyre::Result;

use cmd::CommandLine;

pub fn run() -> Result<()> {
    pretty_env_logger::init();
    Ok(CommandLine::parse().exec()?)
}
