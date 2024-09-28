use crate::bgp::types::as_number::AsNumber;

use crate::bgp::types::simple_types::ConnectionEstablishmentMode;
use std::net::SocketAddr;

/// This trait models the configuration of a remote BGP peer
pub trait BgpPeer<'a> {
    fn peer_address(&'a self) -> &'a SocketAddr;
    fn local_address(&'a self) -> &'a SocketAddr;

    fn remote_as_number(&'a self) -> &'a AsNumber;

    fn local_as_number(&'a self) -> &'a AsNumber;

    fn connection_mode(&'a self) -> &'a ConnectionEstablishmentMode;
    fn name(&'a self) -> &'a String;
}

/// Default implementation of BGP peer configuration model
#[derive(Debug,Clone)]
pub struct DefaultBgpPeer {
    peer_address: SocketAddr,
    local_address: SocketAddr,
    remote_as_number: AsNumber,
    local_as_number: AsNumber,
    connection_mode: ConnectionEstablishmentMode,
    name: String,
}

impl<'a> BgpPeer<'a> for DefaultBgpPeer {
    fn peer_address(&'a self) -> &'a SocketAddr {
        &self.peer_address
    }

    fn local_address(&'a self) -> &'a SocketAddr {
        &self.local_address
    }

    fn remote_as_number(&'a self) -> &'a AsNumber {
        &self.remote_as_number
    }

    fn local_as_number(&'a self) -> &'a AsNumber {
        &self.local_as_number
    }

    fn connection_mode(&'a self) -> &'a ConnectionEstablishmentMode {
        &self.connection_mode
    }

    fn name(&'a self) -> &'a String { &self.name }
}

impl DefaultBgpPeer {
    pub fn new(peer_address: SocketAddr,
               local_address: SocketAddr,
               remote_as_number: AsNumber,
               local_as_number: AsNumber,
               connection_mode: ConnectionEstablishmentMode,
               name: &str) -> DefaultBgpPeer {
        DefaultBgpPeer { peer_address,
            local_address,
            remote_as_number,
            local_as_number,
            connection_mode,
            name: String::from(name) }
    }
}