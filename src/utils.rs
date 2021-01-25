use std::env;
use std::net::IpAddr;
use std::process;
use std::time::Duration;

use color_eyre::eyre::{bail, Result};
use log::LevelFilter;
use reqwest::Client;

use crate::shell::Shell;

pub const APT_PACKAGES_HOST: &str = "bc wireguard wireguard-dkms wireguard-tools git";
pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn setup_logging(debug: bool) {
    let level = if debug { LevelFilter::Debug } else { LevelFilter::Info };

    if process::id() != 1 {
        // Not in docker
        let mut builder = pretty_env_logger::formatted_timed_builder();
        builder.format_timestamp_secs();
        builder.filter_level(level);
        builder.init()
    } else {
        // In docker
        let mut builder = env_logger::builder();
        builder.format_timestamp(None);
        builder.filter_level(level);
        builder.format_level(false);
        builder.format_indent(None);
        builder.init()
    }
}

pub async fn install_wireguard_kernel_module() -> Result<()> {
    let kver_cmd = Shell::exec("uname", "-r", None, false).await;
    let kver = kver_cmd.stdout().trim();
    let modprobe_cmd = Shell::exec("modprobe", "wireguard", None, false).await;
    if modprobe_cmd.success() {
        info!("Wireguard module already installed for kernel version {}", kver);
        Ok(())
    } else {
        let armv7 = Shell::exec("uname", "-r |grep -q 'v7+'", None, true).await;
        let armv7l = Shell::exec("uname", "-r |grep -q 'v7l+'", None, true).await;
        let armv8 = Shell::exec("uname", "-r |grep -q 'v8+'", None, true).await;
        let package: String;
        if armv7.success() || armv7l.success() || armv8.success() {
            package = "raspberrypi-kernel-headers".to_string();
            info!("Detected Rasbian installation, installing kernel headers {} and Wireguard module", package);
        } else {
            package = format!("linux-headers-{}", kver);
            info!(
                "Detected generic debian based installation, installing kernel headers {} and Wireguard module",
                package
            );
        }
        info!("Wireguard module not found, building it for kernel version {}", kver);
        Shell::exec("apt-get", "update", None, false).await;
        let apt_cmd =
            Shell::exec("apt-get", &format!("-y install {} {}", package, APT_PACKAGES_HOST), None, false).await;
        if apt_cmd.success() {
            warn!("Wireguard kernel module installed successfully for version {}", kver);
            Ok(())
        } else {
            bail!("Unable to find kernel header for kernel version {}, Wireguard module not installed", kver);
        }
    }
}

pub async fn enforce_host_config() -> Result<()> {
    let uname_s = Shell::exec("uname", "-s", None, false).await;
    let os = uname_s.stdout();
    if os == "Linux" {
        info!("The detected OS is {}, which is supported", os);
    } else {
        bail!("Unfortunately {} is not yet supported", os)
    };
    let sysctl_status = Shell::exec("sysctl", "-n net.ipv4.ip_forward", None, false).await;
    let forward_status = sysctl_status.stdout();
    if forward_status == "1" {
        info!("ipv4 forwarding already enabled");
        Ok(())
    } else {
        info!("ipv4 forwarding disabled, trying to enable it");
        let ipv4_fwd_enable = Shell::exec("sysctl", "-n net.ipv4.ip_forward=1", None, false).await;
        if ipv4_fwd_enable.success() {
            info!("Succesfully enabled ipv4 forwarding");
            Ok(())
        } else {
            bail!("Unable to activate ipv4 forward: ");
        }
    }
}

pub fn build_reqwest_client(connect_timeout: Option<Duration>, request_timeout: Option<Duration>) -> Result<Client> {
    Ok(Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(connect_timeout.unwrap_or_else(|| Duration::from_millis(1500)))
        .timeout(request_timeout.unwrap_or_else(|| Duration::from_millis(20000)))
        .local_address(IpAddr::from([127, 0, 0, 1]))
        .no_proxy()
        .build()?)
}
