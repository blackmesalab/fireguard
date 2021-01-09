extern crate clap;
extern crate eyre;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_yaml;
extern crate toml;
extern crate parking_lot;

mod cmd;
mod config;
mod shell;
mod wg;

use clap::Clap;
use eyre::Result;

use cmd::CommandLine;

pub fn run() -> Result<()> {
    pretty_env_logger::init();
    let cmd = CommandLine::parse();
    Ok(cmd.exec()?)
}
