use crate::bgp::config::bgp_peer;
use crate::bgp::config::bgp_peer::BgpPeer;

pub trait BgpConfig<'a> {
    fn peers(&'a self) -> Vec<Box<dyn BgpPeer<'a>>>;

    fn add_peer(&'a self, Box<dyn BgpPeer<'a>>);
}

pub struct DefaultBgpConfig {
    peers : Vec<Box<dyn BgpPeer>>,
}

impl<'a> BgpConfig<'a> for DefaultBgpConfig {
    fn peers(&'a self) -> Vec<Box<dyn BgpPeer<'a>>> {
        let _peers : Vec<Box<dyn BgpPeer<'a>>> = Vec::new();

        &self.peers.iter().for_each(|v| { v.clone(); });

        _peers
    }
    fn add_peer(&mut self, peer : Box<dyn BgpPeer>) {
        &self.peers.push(peer);
    }
}

impl DefaultBgpConfig {
    pub fn new() -> DefaultBgpConfig {
        DefaultBgpConfig {
            peers : Vec::new()
        }
    }

}