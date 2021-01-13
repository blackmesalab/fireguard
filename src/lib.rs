extern crate async_trait;
extern crate bollard;
extern crate chrono;
extern crate clap;
extern crate color_eyre;
extern crate futures_util;
extern crate ipnet;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate parking_lot;
extern crate pretty_env_logger;
extern crate rand;
extern crate read_input;
extern crate serde;
extern crate signal_hook;
extern crate tera;
extern crate toml;
extern crate whoami;

mod cmd;
mod config;
mod docker;
mod ip;
mod shell;
mod utils;
mod wg;

use std::env;

use clap::Clap;
use color_eyre::eyre::Result;

use cmd::Fireguard;
use utils::setup_logging;

pub async fn run() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let cmd = Fireguard::parse();
    setup_logging(cmd.debug);
    info!("Running Fireguard {}", version);
    debug!("{:#?}", cmd);
    Ok(cmd.exec().await?)
}
