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
