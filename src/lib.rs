extern crate askama;
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
extern crate serde_yaml;
extern crate shiplift;
extern crate toml;
extern crate whoami;

mod cmd;
mod config;
mod docker;
mod ip;
mod shell;
mod wg;

use clap::Clap;
use color_eyre::eyre::Result;

use docker::Docker;
use cmd::Fireguard;

pub async fn run() -> Result<()> {
    pretty_env_logger::init();
    let cmd = Fireguard::parse();
    let _ = Docker::new().await;
    Ok(cmd.exec().await?)
}
