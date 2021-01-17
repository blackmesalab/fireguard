use std::process;

use color_eyre::eyre::{bail, Result};
use log::LevelFilter;

use crate::shell::Shell;

pub const APT_PACKAGES_HOST: &str = "bc wireguard wireguard-dkms wireguard-tools git";
pub const APT_PACKAGES_DOCKER: &str = "bc ca-certificates dnsmasq iptables wireguard-tools iproute2";

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

pub async fn install_packages_in_docker() -> Result<()> {
    Shell::exec("apt-get", "update", None, false).await;
    use crate::utils::install_packages_in_docker;
    let apt_cmd = Shell::exec("apt-get", &format!("-y install {}", APT_PACKAGES_DOCKER), None, false).await;
    if apt_cmd.success() {
        info!("Packages {} installed inside Fireguard docker container:\n{}", APT_PACKAGES_DOCKER, apt_cmd.stdout());
        Ok(())
    } else {
        bail!(
            "Error installing packages {} inside Fireguard docker container:\n{}",
            APT_PACKAGES_DOCKER,
            apt_cmd.stderr()
        );
    }
}

pub async fn enforce_host_config() -> Result<()> {
    let uname_s= Shell::exec("uname", "-s", None, false).await;
    let os = uname_s.stdout();
    if os == "Linux" {
        info!("The detected OS is {}, which is supported", os);
        Ok(())
    } else {
        bail!("Unfortunately {} is not yet supported", os)
    }
}