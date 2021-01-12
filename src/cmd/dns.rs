use std::path::Path;

use clap::Clap;
use color_eyre::eyre::{bail, Result};
use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::cmd::{Command, Fireguard};
use crate::config::{Config, Peer as ConfigPeer};
use crate::shell::Shell;

const DNSMASQ_LIST_TMPL: &str = r#"# {{ host.repository }} - {{ host.name }} dnsmasq dynamic list
{% for peer in peers -%}
peer.address peer.name
{% endfor -%}"#;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct DnsEntry {
    address: String,
    name: String,
    record: String,
}

impl DnsEntry {
    pub fn new(address: &str, name: &str, record: &str) -> Self {
        Self { address: address.to_string(), name: name.to_string(), record: record.to_string() }
    }
}

/// Dns - DNS service discovery management
#[derive(Clap, Debug)]
pub struct Dns {
    /// Peer subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// List the available DNS names
    List(List),
    /// Remove an existing peer
    Render(Render),
    // /// Print peer info
    // Up(Up),
}

impl Command for Dns {}
impl Dns {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        let config = self.load_config(&self.repository, &fg.config_dir, &fg.config_file).await?;
        match self.action {
            Action::List(ref action) => action.exec(fg, config, &self.repository).await?,
            Action::Render(ref action) => action.exec(fg, config, &self.repository).await?,
        }
        Ok(())
    }
}

/// List the available peers in this trust repository
#[derive(Clap, Debug)]
pub struct List {}

impl Command for List {}
impl List {
    pub async fn exec(&self, fg: &Fireguard, config: Config, repository: &str) -> Result<()> {
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        info!("Available DNS entries for repository {}: {}", repository, config.peers.len());
        for peer in config.peers.values() {
            let address = peer.address.parse::<Ipv4Net>()?;
            println!("\t{}.{}.{}\t{}", peer.peername, peer.peername, config.domain, address.addr());
        }
        Ok(())
    }
}

/// Render the dnsmasq dynamic DNS list
#[derive(Clap, Debug)]
pub struct Render {
    /// Config file path
    #[clap(short = 'c', long = "config-dir", default_value = "/etc/dnsmasq.d")]
    pub config_dir: String,
}

impl Command for Render {}
impl Render {
    async fn pre_checks(&self, _fg: &Fireguard) -> Result<()> {
        if !Shell::runnable("dnsmasq") {
            bail!("Missing command dependency")
        }
        let config = Path::new(&self.config_dir);
        if config.is_dir() {
            Ok(())
        } else {
            bail!(
                "Please create DNSMasq config directory {} as root: mkdir -p {} && chown {} {}",
                self.config_dir,
                self.config_dir,
                whoami::username(),
                self.config_dir
            );
        }
    }

    pub async fn exec(&self, fg: &Fireguard, config: Config, repository: &str) -> Result<()> {
        self.pre_checks(fg).await?;
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        info!("Rendering {} DNS entries for repository {}", config.peers.len(), repository);
        let dns_config_path = Path::new(&self.config_dir).join("99-ddns.conf");
        let mut dns_tera = Tera::default();
        dns_tera.add_raw_template("99-ddns.conf", DNSMASQ_LIST_TMPL)?;
        let dns_config = dns_tera.render(
            "99-ddns.conf",
            &Context::from_serialize(
                config
                    .peers
                    .values()
                    .map(|x| {
                        let ip = x.address.parse::<Ipv4Net>().unwrap_or("127.0.0.1/8".parse::<Ipv4Net>().unwrap());
                        DnsEntry::new(
                            &ip.addr().to_string(),
                            &format!("{}.{}.{}", x.peername, x.username, config.domain),
                            "A",
                        )
                    })
                    .collect::<Vec<DnsEntry>>(),
            )?,
        )?;
        let mut file = File::create(&dns_config_path).await?;
        file.write_all(&dns_config.as_bytes()).await?;
        info!("DNS configuration written to {}", dns_config_path.display());
        Ok(())
    }
}
