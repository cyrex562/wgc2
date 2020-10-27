use std::{fs::File, io::Write, path::Path};

use crate::{
    iproute2_support::ip_addr_add,
    iproute2_support::ip_link_add,
    iproute2_support::ip_link_set_up,
    multi_error::MultiError,
    utils::{ret_multi_err, run_command},
};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

const WG_DFLT_LISTEN_PORT: u16 = 51820;
const WG_LNK_TYPE: &str = "wireguard";

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
                        ifc.listen_port = port;
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
pub struct WgKey {
    pub key: String,
}

pub fn parse_wg_keylike(output: &str) -> Result<WgKey, MultiError> {
    let mut out: WgKey = Default::default();
    out.key.insert_str(0, output.trim());
    Ok(out)
}

pub fn create_wg_private_key() -> Result<WgKey, MultiError> {
    log::debug!("creating wg private key");
    let out = match run_command("wg", &vec!["genkey"], None) {
        Ok(x) => x,
        Err(e) => {
            let msg = format!("failed to run wg genkey: {}", e.to_string());
            return Err(ret_multi_err(msg));
        }
    };

    let result = match parse_wg_keylike(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            let msg = format!("failed to parse genkey output: {}", e.to_string());
            return Err(ret_multi_err(msg));
        }
    };

    Ok(result)
}

pub fn gen_wg_public_key(private_key: &String) -> Result<WgKey, MultiError> {
    log::debug!("generating wg public key");
    let out = match run_command("wg", &vec!["pubkey"], Some(private_key.clone())) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed to run wg pubkey: {}", e.to_string());
            return Err(ret_multi_err(err_msg));
        }
    };

    let result = match parse_wg_keylike(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed to parse genkey output: {}", e.to_string());
            return Err(ret_multi_err(err_msg));
        }
    };

    Ok(result)
}

pub fn ip_link_add_wg(dev_name: &String) -> Result<(), MultiError> {
    log::debug!("adding interface for device={}", dev_name);
    ip_link_add(dev_name, &WG_LNK_TYPE.to_string())?;
    Ok(())
}

pub fn wg_set_private_key(ifc_name: &String, private_key: &String) -> Result<(), MultiError> {
    log::debug!("setting private key for interface={}", ifc_name);
    let key_file_path = wg_create_pvt_key_file(None, Some(private_key.clone()))?;

    let _out = run_command(
        "wg",
        &vec!["set", ifc_name, "private-key", key_file_path.as_str()],
        None,
    )?;
    Ok(())
}

pub fn wg_set_listen_port(ifc_name: &String, listen_port: &String) -> Result<(), MultiError> {
    log::debug!("setting listen port={} for device={}", listen_port, ifc_name);
    let _out = run_command(
        "wg",
        &vec!["set", ifc_name, "listen-port", listen_port],
        None,
    )?;
    Ok(())
}

pub fn wg_showconf(ifc_name: &String) -> Result<String, MultiError> {
    
    let out = match run_command("wg", &vec!["showconf", ifc_name], None) {
        Ok(x) => x,
        Err(e) => {
            let msg = format!("failed to run wg showconf: {}", e.to_string());
            return Err(ret_multi_err(msg));
        }
    };

    Ok(out)
}

pub fn wg_create_pvt_key_file(
    dev_name: Option<String>,
    key: Option<String>,
) -> Result<String, MultiError> {
    log::debug!("creating private key file");
    let pvt_key: String;

    if key.is_some() {
        pvt_key = key.unwrap().clone();
    } else {
        pvt_key = create_wg_private_key()?.key;
    }

    let out: String;
    let path: String;
    let mut file: File;
    if dev_name.is_some() {
        let file_name = format!("/etc/wireguard/{}.private.key", dev_name.unwrap());
        path = file_name.clone();
        file = File::open(file_name)?;
    } else {
        let tmp_file = NamedTempFile::new()?;
        path = String::from(tmp_file.path().to_str().unwrap());
        file = tmp_file.into_file();
    }
    log::debug!("key file path={}", &path);
    file.write_all(pvt_key.as_bytes())?;
    out = path;
    Ok(out)
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WgCreateInterfaceRequest {
    pub ifc_name: String,
    pub address: String,
    pub listen_port: u16,
    pub set_link_up: bool,
    pub persist: bool,
}

///
///
///
pub fn create_wg_interface(
    ifc_name: &String,
    address: &String,
    listen_port: Option<u16>,
    set_link_up: bool,
    persist: bool,
) -> Result<WgInterface, MultiError> {
    log::debug!("creating wireguard interface: name={}, address={}, set_link_up={:?}, persist={:?}", ifc_name, address, set_link_up, persist);
    // todo: check if interface already exists
    // todo: check if address already exists
    // todo: check if something is already listening on that port
    // todo: validate that the given address conforms to A.B.C.D/E
    let mut out: WgInterface = Default::default();
    // set interface name
    out.name = ifc_name.clone();
    // create a private key
    out.private_key = create_wg_private_key()?.key;
    // create a public key
    out.public_key = gen_wg_public_key(&out.private_key)?.key;
    // set listen port
    if listen_port.is_some() {
        out.listen_port = listen_port.unwrap();
    } else {
        out.listen_port = WG_DFLT_LISTEN_PORT;
    }
    // set address
    out.address = address.clone();
    // create new interface with ip link
    ip_link_add_wg(&out.name)?;
    // add address to interface
    ip_addr_add(&out.name, &out.address)?;
    // set private key
    wg_set_private_key(&out.name, &out.private_key)?;
    // set listen port
    let listen_port_string: String = format!("{}", &out.listen_port);
    wg_set_listen_port(&out.name, &listen_port_string)?;
    // set link up
    if set_link_up {
        ip_link_set_up(&out.name)?
    }
    // persist by saving config and setting up service with wg quick
    if persist {
        let conf = wg_showconf(&out.name)?;
        let dev_file_path_str = format!("/etc/wireguard/{}.conf", &out.name);
        let dev_file_path = Path::new(dev_file_path_str.as_str());
        let mut def_file = std::fs::File::create(dev_file_path)?;
        let _bytes_written = def_file.write_all(conf.as_bytes())?;
        let svc_name = format!("wg_quick@{}.service", &out.name);
        let _svc_enable_out = run_command("systemctl", &vec!["enable", svc_name.as_str()], None)?;
    }
    Ok(out)
}
