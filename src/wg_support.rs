use crate::multi_error::MultiError;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgPeer {
    pub public_key: String,
    pub allowed_ips: String,
    pub persistent_keepalive: String,
    pub endpoint: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listening_port: u16,
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

pub fn parse_wg_show_interfaces(output: &str) -> Result<Vec<String>, MultiError> {
    let mut out: Vec<String> = Vec::new();
    for item in output.split_whitespace() {
        out.push(String::from(item));
    }
    Ok(out)
}

pub fn parse_wg_show_output(output: &str) -> Result<Vec<WgInterface>, MultiError> {
    let mut out: Vec<WgInterface> = Vec::new();

    for ifc_raw in output.trim().split("interface: ") {
        if ifc_raw.is_empty() {
            continue;
        }
        let mut ifc: WgInterface = Default::default();
        for ele in ifc_raw.split("peer: ") {
            if ele.find("listening port").is_some() {
                let mut line_num = 0;
                for line in ele.lines() {
                    // interface name
                    if line_num == 0 {
                        ifc.name = line.trim().to_string();
                    }
                    // interface public key
                    else if line.find("public key: ").is_some() {
                        let mut pub_key = line.trim().strip_prefix("public key: ").unwrap();
                        pub_key = pub_key.trim();
                        ifc.public_key = pub_key.to_string();
                    }
                    // listening port
                    else if line.find("listening port: ").is_some() {
                        let mut port_str = line.trim().strip_prefix("listening port: ").unwrap();
                        port_str = port_str.trim();
                        let port: u16 = port_str.parse::<u16>().unwrap();
                        ifc.listening_port = port;
                    }
                    line_num += 1
                }
            } else {
                let mut peer: WgPeer = Default::default();
                let mut line_num = 0;
                for line in ele.lines() {
                    // peer public key / ID
                    if line_num == 0 {
                        peer.public_key = line.trim().to_string();
                    }
                    // endpoint
                    else if line.find("endpoint").is_some() {
                        let endpoint_str = line.trim().strip_prefix("endpoint: ").unwrap();
                        peer.endpoint = endpoint_str.trim().to_string();
                    }
                    // allowed ips
                    else if line.find("allowed ips").is_some() {
                        let allowed_str = line.trim().strip_prefix("allowed ips: ").unwrap();
                        peer.allowed_ips = allowed_str.trim().to_string();
                    }
                    // persistent keepalive
                    else if line.find("persistent keepalive").is_some() {
                        let keepalive_str =
                            line.trim().strip_prefix("persistent keepalive: ").unwrap();
                        peer.persistent_keepalive = keepalive_str.trim().to_string();
                    }

                    line_num += 1;
                }

                ifc.peers.push(peer);
            }
        }

        out.push(ifc);
    }

    Ok(out)
}

//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 public-key
// ChItTztrI97Gtl98pgGXWv2GcWq+Tvmt8/2WK/ZeyQg=
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 private-key
// +N+D390T54HDuAs0kdNFxpq7p0I0k9QVEkT2N5QpVV4=
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 listen-port
// 37997
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 fwmark
// off
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 peers
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 preshared-keys
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    (none)
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 endpoints
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    167.172.139.112:54323
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 allowed-ips
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    10.255.0.0/24
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 latest-handshakes
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    1603308037
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 persistent-keepalive
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    25
//   vpn-gw-1.aplabs1.net  box-admin  ~  sudo wg show wg0 transfer
// vJSIFglo+1FhDLRt8j5aYwj0EB4/UatiGBgT2H7qQVo=    910456764       17587575272
//   vpn-gw-1.aplabs1.net  box-admin  ~ 

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPublicKey {
    pub public_key: String,
}

pub fn parse_wg_show_pub_key(output: &str) -> Result<WgShowPublicKey, MultiError> {
    let mut out: WgShowPublicKey = Default::default();
    out.public_key = output.trim().to_string();
    Ok(out)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPrivateKey {
    pub private_key: String,
}

pub fn parse_wg_show_pvt_key(output: &str) -> Result<WgShowPrivateKey, MultiError> {
    let mut out: WgShowPrivateKey = Default::default();
    out.private_key = output.trim().to_string();
    Ok(out)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowListenPort {
    pub listen_port: u16,
}

pub fn parse_wg_show_listen_port(output: &str) -> Result<WgShowListenPort, MultiError> {
    let mut out: WgShowListenPort = Default::default();
    out.listen_port = output.trim().parse::<u16>().unwrap();
    Ok(out)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowFwMark {
    pub fwmark: String,
}

pub fn parse_wg_show_fwmark(output: &str) -> Result<WgShowFwMark, MultiError> {
    let mut out: WgShowFwMark = Default::default();
    out.fwmark = output.trim().to_string();
    Ok(out)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgShowPeers {
    pub peers: Vec<String>,
}

pub fn parse_wg_show_peers(output: &str) -> Result<WgShowPeers, MultiError> {
    let mut out: WgShowPeers = Default::default();
    for line in output.lines() {
        out.peers.push(line.trim().to_string())
    }
    Ok(out)
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

pub fn parse_wg_show_preshared_keys(output: &str) -> Result<WgShowPresharedKeys, MultiError> {
    let mut out: WgShowPresharedKeys = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgPresharedKey {
            peer: parts[0].to_string(),
            preshared_key: parts[1].to_string(),
        };
        out.preshared_keys.push(psk);
    }
    Ok(out)
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

pub fn parse_wg_show_endpoints(output: &str) -> Result<WgShowEndpoints, MultiError> {
    let mut out: WgShowEndpoints = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgEndpoint {
            peer: parts[0].to_string(),
            endpoint: parts[1].to_string(),
        };
        out.endpoints.push(psk);
    }
    Ok(out)
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

pub fn parse_wg_show_allowed_ips(output: &str) -> Result<WgShowAllowedIps, MultiError> {
    let mut out: WgShowAllowedIps = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgAllowedIps {
            peer: parts[0].to_string(),
            allowed_ips: parts[1].to_string(),
        };
        out.allowed_ips.push(psk);
    }
    Ok(out)
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

pub fn parse_wg_show_latest_handshakes(output: &str) -> Result<WgShowLatestHandshakes, MultiError> {
    let mut out: WgShowLatestHandshakes = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgHandshake {
            peer: parts[0].to_string(),
            handshake: parts[1].to_string(),
        };
        out.latest_handshakes.push(psk);
    }
    Ok(out)
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

pub fn parse_wg_show_persistent_keepalive(
    output: &str,
) -> Result<WgShowPersistentKeepalive, MultiError> {
    let mut out: WgShowPersistentKeepalive = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgPersistentKeepalive {
            peer: parts[0].to_string(),
            persistent_keepalive: parts[1].to_string(),
        };
        out.persistent_keepalives.push(psk);
    }
    Ok(out)
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

pub fn parse_wg_show_transfer(output: &str) -> Result<WgShowTransfer, MultiError> {
    let mut out: WgShowTransfer = Default::default();
    for line in output.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let psk = WgTransfer {
            peer: parts[0].to_string(),
            transmitted: parts[1].to_string().parse::<u64>()?,
            received: parts[2].to_string().parse::<u64>()?,
        };
        out.transfers.push(psk);
    }
    Ok(out)
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct WgGenkey {
    pub key: String,
}

pub fn parse_wg_genkey(output: &str) -> Result<WgGenkey, MultiError> {
    let mut out: WgGenkey = Default::default();
    out.key.insert_str(0, output.trim());
    Ok(out)
}
