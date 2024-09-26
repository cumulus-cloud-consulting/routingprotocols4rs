use std::collections::HashMap;
use std::iter::Map;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use anyhow::{anyhow, bail, Result};
use bgp_shared::bgp::config::bgp_config::{BgpConfig, DefaultBgpConfig};
use bgp_shared::bgp::config::bgp_peer::{BgpPeer, DefaultBgpPeer};
use bgp_shared::bgp::types::simple_types::ConnectionEstablishmentMode;
use serde::{Deserialize, Serialize};
use bgp_shared::bgp::types::as_number::AsNumber;
use bgp_shared::bgp::types::bgp_constants::BGP_PORT_NUMBER;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    peers: Vec<BgpPeerConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpPeerConfig {
    peer_name: String,
    local_address: String,
    local_port: Some(u16),
    peer_address: String,
    peer_port: Some(u16),
    local_as_number: u32,
    remote_as_number: u32,
    connection_establishment_mode: ConnectionEstablishmentMode,
}

impl ConfigFile {
    pub fn parse_config_file(config: &ConfigFile) -> Result<Box<dyn BgpConfig>> {
        let mut bgp_config = DefaultBgpConfig::new();

        config.peers.iter().for_each(|bgp_peer_config: BgpPeerConfig| {
            match Self::parse_bgp_peer_config(&bgp_peer_config) {
                Ok(bgp_peer) => {
                    bgp_config.add_peer(bgp_peer);
                }
                Err(err) => {}
            }
        });

        let mut name_count: HashMap<String, u16> = HashMap::new();

        &bgp_config.peers().iter().for_each(|peer| {
            match name_count.get_key_value(peer.name()) {
                Some((key, value)) => {
                    name_count.insert(key.clone(), *value + 1);
                }
                None => {
                    name_count.insert(peer.name().clone(), 1);
                }
            }
        });

        match name_count.iter().find(|item| item.1 > 1) {
            Some(item) => Err(anyhow!("Duplicate peer definition {}", item.0)),
            None => Ok(Box::new(bgp_config))
        }
    }

    fn parse_bgp_peer_config(config: &BgpPeerConfig) -> Result<Box<dyn BgpPeer>> {
        if (config.peer_name.is_empty()) {
            return Err(anyhow!("Empty peer name found"));
        }

        let local_port = match config.local_port {
            Some(port) => port,
            None => BGP_PORT_NUMBER
        };
        let local_addr = match Self::parse_sock_addr(&config.local_address, local_port) {
            Ok(sock_addr) => sock_addr,
            Err(err) => return Err(err)
        };
        let peer_port = match config.peer_port {
            Some(port) => port,
            None => BGP_PORT_NUMBER
        };
        let peer_addr = match Self::parse_sock_addr(&config.peer_address, peer_port) {
            Ok(sock_addr) => sock_addr,
            Err(err) => return Err(err)
        };

        Ok(Box::new(DefaultBgpPeer::new(peer_addr,
                                        local_addr,
                                        AsNumber::from(config.remote_as_number),
                                        AsNumber::from(config.local_as_number),
                                        config.connection_establishment_mode,
                                        &config.peer_name.clone())))
    }

    fn parse_sock_addr(addr: &String, port: u16) -> Result<SocketAddr> {
        match Ipv4Addr::from_str(addr) {
            Ok(ipv4_addr) => Ok(SocketAddr::new(IpAddr::V4(ipv4_addr), port)),
            Err(ipv4_err) => match Ipv6Addr::from_str(addr) {
                Ok(ipv6_addr) => Ok(SocketAddr::new(IpAddr::V6(ipv6_addr), port)),
                Err(err) => Err(anyhow!("Cannot parse IP address {}", addr)),
            }
        }
    }
}