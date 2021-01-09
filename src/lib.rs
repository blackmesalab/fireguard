extern crate clap;
extern crate eyre;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate ipnet;
extern crate parking_lot;
extern crate pretty_env_logger;
extern crate read_input;
extern crate serde;
extern crate serde_yaml;
extern crate toml;
extern crate rand;

mod cmd;
mod config;
mod ip;
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
