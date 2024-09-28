use std::rc::Rc;
use crate::bgp::config::bgp_peer::BgpPeer;

pub trait BgpConfig<'a> {
    fn peers(&'a self) -> Vec<Rc<dyn BgpPeer<'a>>>;

    fn add_peer(&'a mut self, peer : Box<dyn BgpPeer<'a>> );
}

pub struct DefaultBgpConfig<'a> {
    peers : Vec<Rc<dyn BgpPeer<'a>>>,
}

impl<'a> BgpConfig<'a> for DefaultBgpConfig<'a> {
    fn peers(&'a self) -> Vec<Rc<dyn BgpPeer<'a>>> {
        let mut _peers : Vec<Rc<dyn BgpPeer<'a>>> = Vec::new();

        self.peers.iter().for_each(|v| { _peers.push(v.clone()); });

        _peers
    }
    fn add_peer(&'a mut self, peer : Box<dyn BgpPeer<'a>>) {
        self.peers.push(Rc::from(peer));
    }
}

impl <'a> DefaultBgpConfig<'a> {
    pub fn new() -> DefaultBgpConfig<'a> {
        DefaultBgpConfig {
            peers : Vec::new()
        }
    }

}