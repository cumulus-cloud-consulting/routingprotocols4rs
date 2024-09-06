use crate::bgp::types::as_number::AsNumber;

use std::net::{IpAddr, SocketAddr};
use crate::bgp::types::simple_types::ConnectionEstablishmentMode;

/// This trait models the configuration of a remote BGP peer
pub trait BgpPeer<'a> {
    fn peer_address(&'a self) -> &'a IpAddr;
    fn local_address(&'a self) -> &'a SocketAddr;

    fn remote_as_number(&'a self) -> &'a AsNumber;

    fn local_as_number(&'a self) -> &'a AsNumber;

    fn connection_mode(&'a self) -> &'a ConnectionEstablishmentMode;
}

/// Default implementation of BGP peer configuration model
pub struct DefaultBgpPeer {
    peer_address: IpAddr,
    local_address: SocketAddr,
    remote_as_number: AsNumber,
    local_as_number: AsNumber,
    connection_mode: ConnectionEstablishmentMode,
}

impl<'a> BgpPeer<'a> for DefaultBgpPeer {
    fn peer_address(&'a self) -> &'a IpAddr {
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
}