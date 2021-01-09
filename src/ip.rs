use std::net::Ipv4Addr;

use eyre::{bail, Result};
use ipnet::Ipv4Net;
use rand::seq::SliceRandom;

#[derive(Debug, Default)]
pub struct IpPool {
    subnet: Ipv4Net,
    subnet_lenght: usize,
    free_list: Vec<Ipv4Addr>,
    used_list: Vec<Ipv4Addr>,
}

impl IpPool {
    pub fn new(network: &str, peers: Vec<String>) -> Result<Self> {
        let subnet = network.parse::<Ipv4Net>()?;
        info!(
            "Creating new IPv4 pool for subnet {}, capacity {} out of {}",
            network,
            peers.len(),
            subnet.prefix_len()
        );
        let used_list: Vec<Ipv4Addr> = peers.iter().map(|x| {
            match x.parse::<Ipv4Net>() {
                Ok(ip) => ip.addr(),
                Err(e) => {
                    error!("Unable to parse peer IPv4 address {:?}: {}", x, e);
                    "127.0.0.1/8".parse::<Ipv4Net>().unwrap().addr()
                }
            }
        }).collect();
        debug!("IPv4 pool used list: {:?}", used_list);
        let mut free_list: Vec<Ipv4Addr> = subnet.hosts().collect::<Vec<Ipv4Addr>>();
        free_list.retain(|x| !used_list.contains(x));
        debug!("IPv4 pool free list: {:?}", free_list);
        Ok(IpPool { subnet, subnet_lenght: subnet.prefix_len() as usize, free_list, used_list })
    }

    pub fn ip(&mut self) -> Result<Ipv4Addr> {
        if self.used_list.len() >= self.subnet_lenght {
            bail!("The pool is full and cannot take more hosts, you need to scale up you subnet size");
        }
        let free_ip = match self.free_list.choose(&mut rand::thread_rng()) {
            Some(free_ip) => free_ip,
            None => bail!("Unable to find a free IPv4 in the pool"),
        };
        let free_ip = free_ip.clone();
        self.free_list.retain(|&x| x != free_ip);
        self.used_list.push(free_ip);
        info!(
            "Allocating new free IPv4 {} from the IP pool, capacity {} out of {}",
            free_ip,
            self.used_list.len(),
            self.subnet.prefix_len()
        );
        Ok(free_ip)
    }
}
