use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgPeer {
    pub public_key: String,
    pub allowed_ips: String,
    pub persistent_keepalive: String,
    pub endpoint: String,
    pub preshared_key: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u16,
    pub address: String,
    pub peers: Vec<WgPeer>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowAll {
    pub interfaces: Vec<WgInterface>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowInterfaces {
    pub interfaces: Vec<String>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPublicKey {
    pub public_key: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPrivateKey {
    pub private_key: String,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowListenPort {
    pub listen_port: u16,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowFwMark {
    pub fwmark: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPeers {
    pub peers: Vec<String>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgPresharedKey {
    pub peer: String,
    pub preshared_key: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPresharedKeys {
    pub preshared_keys: Vec<WgPresharedKey>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgEndpoint {
    pub peer: String,
    pub endpoint: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowEndpoints {
    pub endpoints: Vec<WgEndpoint>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgAllowedIps {
    pub peer: String,
    pub allowed_ips: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowAllowedIps {
    pub allowed_ips: Vec<WgAllowedIps>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgHandshake {
    pub peer: String,
    pub handshake: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowLatestHandshakes {
    pub latest_handshakes: Vec<WgHandshake>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgPersistentKeepalive {
    pub peer: String,
    pub persistent_keepalive: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPersistentKeepalive {
    pub persistent_keepalives: Vec<WgPersistentKeepalive>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgTransfer {
    pub peer: String,
    pub transmitted: u64,
    pub received: u64,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowTransfer {
    pub transfers: Vec<WgTransfer>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgKey {
    pub key: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WgCreateInterfaceRequest {
    pub ifc_name: String,
    pub address: String,
    pub listen_port: u16,
    pub set_link_up: bool,
    pub persist: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WgPeerParameters {
    pub public_key: String,
    pub remove: Option<bool>,
    pub preshared_key: Option<String>,
    pub endpoint: Option<String>,
    pub persistent_keepalive: Option<u32>,
    pub allowed_ips: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WgInterfaceParameters {
    pub listen_port: Option<u16>,
    pub private_key: Option<String>,
    pub fwmark: Option<String>,
    pub peer: Option<WgPeerParameters>,
}

impl fmt::Display for WgPeerParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut psk: String = String::from("");
        if self.preshared_key.is_some() {
            psk = self.preshared_key.as_ref().unwrap().to_string();
        }

        let mut ep: String = String::from("");
        if self.endpoint.is_some() {
            ep = self.endpoint.as_ref().unwrap().to_string();
        }

        let mut ap: String = String::from("");
        if self.allowed_ips.is_some() {
            ap = self.allowed_ips.as_ref().unwrap().to_string();
        }

        write!(f, "public_key={}, remove={}, preshared_key={}, endpoint={}, persistent_keepalive={}, allowed_ips={}",
        &self.public_key,
        &self.remove.unwrap_or_else(|| { false }),
        &psk,
        &ep,
        &self.persistent_keepalive.unwrap_or_else(|| {0}),
        &ap)
    }
}

impl fmt::Display for WgInterfaceParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pk: String = String::from("");
        if self.private_key.is_some() {
            pk = self.private_key.as_ref().unwrap().to_string();
        }

        let mut fm: String = String::from("");
        if self.fwmark.is_some() {
            fm = self.fwmark.as_ref().unwrap().to_string();
        }

        let mut p: WgPeerParameters = Default::default();
        if self.peer.is_some() {
            p = self.peer.as_ref().unwrap().clone();
        }

        write!(
            f,
            "listen_port={}, private_key={}, fwmark={}, peer={}",
            self.listen_port.unwrap_or(0),
            &pk,
            &fm,
            &p
        )
    }
}
