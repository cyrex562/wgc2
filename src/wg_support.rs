use crate::multi_error::MultiError;

pub struct WgPeer {
    pub public_key: String,
    pub allowed_ips: String,
    pub persistent_keepalive: String,
    pub endpoint: String,
}

#[derive(Default, Clone, Debug)]
pub struct WgInterface {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
    pub listening_port: u16,
    pub peers: Vec<WgPeer>
}

pub fn parse_wg_show_output(output: &str) -> Result<Vec<WgInterface>, MultiError> {
    let mut out: Vec<WgInterface> = Vec::new();

    for ifc_raw in output.split("interface: ") {
        let mut ifc: WgInterface = Default::default();
        for ele in ifc_raw.split("peer: ") {
            let is_ifc: bool = ele.find("listening_port").is_some();
            if !is_ifc {
                let mut peer: WgPeer = Default::default();
                let mut line_count = 0;
                for line in ele.strip("\n") {
                    if line_count == 0 {
                        let mut pub_key = line.trim();
                        peer.public_key = pub_key.clone().to_string();
                    } else if line.find("  endpoint: ").is_some() {
                        let mut endpoint = line.strip_prefix("  endpoint: ").unwrap();
                        endpoint = endpoint.trim();
                        peer.endpoint = endpoint.clone().to_string();
                    } else if line.find("  allowed ips: ").is_some() {
                        let mut allowed_ips = line.strip_prefix("  allowed ips: ").unwrap();
                        allowed_ips = allowed_ips.trim();
                        peer.allowed_ips = allowed_ips.clone().to_string();
                    } else if line.find("  latest handshake: ").is_some() {
                        // do nothing
                    } else if line.find("  tranfer: ").is_some() {
                        // do nothing
                    } else if line.find("  persistent keepalive: ").is_some() {
                        let mut keepalive = line.strip_prefix("  persistent keepalive: ").unwrap();
                        keepalive = keepalive.trim();
                        peer.persistent_keepalive = keepalive.clone().to_string();
                    } else {
                        log::warn!("unhandled peer line: \"{}\"", line);
                    }
                    line_count += 1;
                }
            } else {
                let mut line_count = 0;
                if line_count == 0 {
                    let mut ifc_name = line.strip();
                    ifc.name == ifc_name.clone().to_string();
                }
                if line.find("  public key: ").is_some() {
                    let mut pub_raw = line.strip_prefix("  public key: ");
                    pub_raw = pub_raw.strip_suffix("\n");
                    ifc.public_key = pub_raw.clone().to_string();
                } else if line.find("  private key: ") {
                    // do nothing
                } else if line.find("  listening port: ") {
                    let mut port_raw = ifc_ele_raw.strip_prefix("  listening port: ");
                    port_raw = port_raw.strip_suffix("\n");
                    ifc.listening_port = u16::from(port_raw.strip());
                } else if line.find("  ") {

                } else {
                    log::warn!("")
                }
            }
            let mut line_count = 0;
            for line in ele.split("\n") {
                if is_ifc {

                } else {
                    let mut peer
                    if line_count == 0 {
                        let mut pub_key = line.strip();

                    }
                }

                line_count += 1;
            }
        }



        for ifc_ele_raw in ifc_raw.split("\n") {
            if ifc_ele_raw.find("interface: ") {
                let mut name_raw = ifc_ele_raw.strip_prefix("interface: ").unwrap();
                name_raw = name_raw.strip_suffix("\n").unwrap();
                ifc.name = name_raw.clone().to_string();
            } else if ifc_ele_raw.find("  public key: ") {

            } else if ifc_ele_raw.find("  private key: ") {
                // skip private key because its hidden in wg show
            } else if ifc_ele_raw.find("  listening port: ") {

            }
        }
    }

    Ok(out)
}