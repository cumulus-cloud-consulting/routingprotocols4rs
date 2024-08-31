use crate::bgp::types::as_number::AsNum;
use std::net::{IpAddr, SocketAddr};

/// This trait models the configuration of a remote BGP peer
pub trait BgpPeer<'a> {
    fn peer_address(&'a self) -> &'a IpAddr;
    fn local_address(&'a self) -> &'a SocketAddr;

    fn remote_as_number(&'a self) -> &'a AsNum;

    fn local_as_number(&'a self) -> &'a AsNum;
}

/// Default implementation of BGP peer configuration model
pub struct DefaultBgpPeer {
    peer_address: IpAddr,
    local_address: SocketAddr,
    remote_as_number: AsNum,
    local_as_number: AsNum,
}

impl <'a> BgpPeer<'a> for DefaultBgpPeer {

    fn peer_address(&'a self) -> &'a IpAddr {
        &self.peer_address
    }

    fn local_address(&'a self) -> &'a SocketAddr {
        &self.local_address
    }

    fn remote_as_number(&'a self) -> &'a AsNum {
        &self.remote_as_number
    }

    fn local_as_number(&'a self) -> &'a AsNum {
        &self.local_as_number
    }
}