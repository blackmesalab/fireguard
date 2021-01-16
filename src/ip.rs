use std::net::Ipv4Addr;

use color_eyre::eyre::{bail, Result};
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
        info!("Creating new IPv4 pool for subnet {}, capacity {} out of {}", network, peers.len(), subnet.prefix_len());
        let used_list: Vec<Ipv4Addr> = peers
            .iter()
            .map(|x| match x.parse::<Ipv4Net>() {
                Ok(ip) => ip.addr(),
                Err(e) => {
                    error!("Unable to parse peer IPv4 address {:?}: {}", x, e);
                    "127.0.0.1/8".parse::<Ipv4Net>().unwrap().addr()
                }
            })
            .collect();
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_creation_with_expected_values() {
        let mut data = Vec::new();
        data.push((
            "192.168.1.0/24",
            vec!["192.168.1.1/32".to_string(), "192.168.1.234/32".to_string(), "192.168.1.2/32".to_string()],
            251,
            3,
        ));
        data.push(("10.0.0.0/16", vec![], 65534, 0));

        for (subnet, peers, free_len, used_len) in data {
            let pool = IpPool::new(subnet, peers.to_vec()).unwrap();
            assert_eq!(pool.free_list.len(), free_len);
            assert_eq!(pool.used_list.len(), used_len);
            for peer in peers {
                let ipaddr_peer = &Ipv4Addr::from_str(&(peer.splitn(2, "/").collect::<Vec<&str>>())[0]).unwrap();
                assert!(!pool.free_list.contains(ipaddr_peer));
            }
        }
    }

    #[test]
    fn test_generate_ip_is_in_subnet() {
        let subnets = vec!["10.0.0.0/8", "10.10.0.0/16", "192.168.1.0/24", "192.168.1.0/31"];
        for subnet in subnets {
            let mut pool = IpPool::new(subnet, vec![]).unwrap();
            let ip = pool.ip().unwrap();
            let this: Ipv4Net = subnet.parse().unwrap();
            assert!(this.contains(&ip));
        }
    }

    #[test]
    fn test_full_pool_bails() {
        let mut pool =
            IpPool::new("192.168.1.0/31", vec!["192.168.1.0/32".to_string(), "192.168.1.1/32".to_string()]).unwrap();
        let err = pool.ip();
        assert!(err.is_err());
        assert_eq!(err.err().unwrap().to_string(), "Unable to find a free IPv4 in the pool");
    }
}
