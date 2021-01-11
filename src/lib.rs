extern crate async_trait;
extern crate bollard;
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
extern crate tera;
extern crate toml;
extern crate whoami;

mod cmd;
mod config;
mod docker;
mod ip;
mod shell;
mod wg;

use std::env;

use clap::Clap;
use color_eyre::eyre::Result;

use cmd::Fireguard;
use docker::Docker;

pub async fn run() -> Result<()> {
    pretty_env_logger::init();
    let version = env!("CARGO_PKG_VERSION");
    info!("Running Fireguard {}", version);
    Docker::new().await?;
    let cmd = Fireguard::parse();
    debug!("{:#?}", cmd);
    Ok(cmd.exec().await?)
}
