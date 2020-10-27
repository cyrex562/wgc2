use std::io::Write;

use actix_files::NamedFile;
use actix_web::{error, web, HttpResponse, Responder};
use tempfile::NamedTempFile;

use crate::{
    utils::{ret_internal_server_error, run_command},
    wg_support::{
        create_wg_interface, create_wg_private_key, delete_wg_interface, gen_wg_public_key,
        parse_wg_keylike, parse_wg_show_allowed_ips, parse_wg_show_endpoints, parse_wg_show_fwmark,
        parse_wg_show_interfaces, parse_wg_show_latest_handshakes, parse_wg_show_listen_port,
        parse_wg_show_output, parse_wg_show_peers, parse_wg_show_persistent_keepalive,
        parse_wg_show_preshared_keys, parse_wg_show_pub_key, parse_wg_show_pvt_key,
        parse_wg_show_transfer, wg_showconf, WgCreateInterfaceRequest, WgKey, WgShowAll,
        WgShowInterfaces,
    },
};

pub async fn wg_show() -> impl Responder {
    let out = match run_command("wg", &vec!["show"], None) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg show"));
        }
    };

    let interfaces = match parse_wg_show_output(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to parse output: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to parse output"));
        }
    };
    let result: WgShowAll = WgShowAll { interfaces };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn wg_show_interfaces() -> impl Responder {
    let out = match run_command("wg", &vec!["show", "interfaces"], None) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show interfaces: {}", e.to_string());
            return Err(error::ErrorInternalServerError(
                "failed to run wg show interfaces",
            ));
        }
    };

    let interfaces = match parse_wg_show_interfaces(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to parse output: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to parse output"));
        }
    };

    let result = WgShowInterfaces { interfaces };

    Ok(HttpResponse::Ok().json(result))
}

pub async fn wg_show_interface(path: web::Path<String>) -> impl Responder {
    let out = match run_command("wg", &vec!["show", path.as_str()], None) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show {}: {}", path, e.to_string());
            return Err(error::ErrorInternalServerError(
                "failed to run wg show interface",
            ));
        }
    };

    let interfaces = match parse_wg_show_output(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to aprse output: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to parse output"));
        }
    };
    let result = interfaces[0].clone();

    Ok(HttpResponse::Ok().json(result))
}

pub async fn wg_show_ifc_element(path: web::Path<(String, String)>) -> impl Responder {
    let path = path.into_inner();
    let out = match run_command("wg", &vec!["show", &path.0, &path.1], None) {
        Ok(x) => x,
        Err(e) => {
            log::error!(
                "failed to run wg show {} {}: {}",
                path.0,
                path.1,
                e.to_string()
            );
            return Err(error::ErrorInternalServerError(
                "failed to run wg show interface",
            ));
        }
    };

    // wg show {ifc} public-key | private-key | listen-port | fwmark | peers | preshared-keys | endpoints | allowed-ips | latest-handshakes | persistent-keepalive | transfer
    // public-key
    if &path.1 == "public-key" {
        let result = match parse_wg_show_pub_key(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // private-key
    else if &path.1 == "private-key" {
        let result = match parse_wg_show_pvt_key(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // listen-port
    else if &path.1 == "listen-port" {
        let result = match parse_wg_show_listen_port(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // fwmark
    else if &path.1 == "fwmark" {
        let result = match parse_wg_show_fwmark(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // peers
    else if &path.1 == "peers" {
        let result = match parse_wg_show_peers(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // preshared-keys
    else if &path.1 == "preshared-keys" {
        let result = match parse_wg_show_preshared_keys(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // endpoints
    else if &path.1 == "endpoints" {
        let result = match parse_wg_show_endpoints(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // allowed-ips
    else if &path.1 == "allowed-ips" {
        let result = match parse_wg_show_allowed_ips(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // latest-handshakes
    else if &path.1 == "latest-handshakes" {
        let result = match parse_wg_show_latest_handshakes(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // persistent-keepalive
    else if &path.1 == "persistent-keepalive" {
        let result = match parse_wg_show_persistent_keepalive(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    }
    // transfer
    else if &path.1 == "transfer" {
        let result = match parse_wg_show_transfer(out.as_str()) {
            Ok(x) => x,
            Err(e) => {
                log::error!("failed to parse output: {}", e.to_string());
                return Err(error::ErrorInternalServerError("failed to parse output"));
            }
        };
        Ok(HttpResponse::Ok().json(result))
    } else {
        Err(error::ErrorBadRequest("invalid wg show request"))
    }
}

///
///
///
pub async fn wg_showconf_ifc(path: web::Path<String>) -> impl Responder {
    let path = path.into_inner();
    let out = match wg_showconf(&path) {
        Ok(x) => x,
        Err(e) => {
            let msg = format!("failed to run wg showconf: {}", e.to_string());
            return Err(ret_internal_server_error(msg));
        }
    };

    let mut file = match NamedTempFile::new() {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to create temp file for download: {}", e.to_string());
            return Err(error::ErrorInternalServerError(
                "failed to create temp file for download",
            ));
        }
    };

    match file.write(out.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            let err_msg = format!("failed to write to temp file: {}", e.to_string());
            return Err(ret_internal_server_error(err_msg));
        }
    };

    let mut fpath: String = String::new();
    fpath.push_str(&file.path().to_str().unwrap());
    let out_file = file.into_file();

    let named_file = match NamedFile::from_file(out_file, fpath) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed to get named file: {}", e.to_string());
            return Err(ret_internal_server_error(err_msg));
        }
    };

    Ok(named_file)
}

///
///
///
pub async fn wg_genkey() -> impl Responder {
    let result = match create_wg_private_key() {
        Ok(x) => x,
        Err(e) => {
            let msg = format!("failed to create private key: {}", e.to_string());
            return Err(ret_internal_server_error(msg));
        }
    };

    Ok(HttpResponse::Ok().json(result))
}

///
///
///
pub async fn wg_genpsk() -> impl Responder {
    let out = match run_command("wg", &vec!["genpsk"], None) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed run wg genpsk: {}", e.to_string());
            return Err(ret_internal_server_error(err_msg));
        }
    };

    let result = match parse_wg_keylike(out.as_str()) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed to parse genkey output: {}", e.to_string());
            return Err(ret_internal_server_error(err_msg));
        }
    };

    Ok(HttpResponse::Ok().json(result))
}

///
///
///
pub async fn wg_pubkey(req: web::Json<WgKey>) -> impl Responder {
    let in_key = req.key.clone();
    let result = match gen_wg_public_key(&in_key) {
        Ok(x) => x,
        Err(e) => {
            let err_msg = format!("failed to get public key: {}", e.to_string());
            return Err(ret_internal_server_error(err_msg));
        }
    };
    Ok(HttpResponse::Ok().json(result))
}

///
///
///
pub async fn handle_create_wg_interface(
    req: web::Json<WgCreateInterfaceRequest>,
) -> impl Responder {
    let ifc_name = req.ifc_name.clone();
    let address = req.address.clone();
    let listen_port = req.listen_port.clone();
    let set_link_up = req.set_link_up;
    let persist = req.persist;
    let out =
        match create_wg_interface(&ifc_name, &address, Some(listen_port), set_link_up, persist) {
            Ok(x) => x,
            Err(e) => {
                let err_msg = format!("failed to create interface: {}", e.to_string());
                return Err(ret_internal_server_error(err_msg));
            }
        };
    Ok(HttpResponse::Ok().json(out))
}

///
///
///
pub async fn handle_delete_wg_interface(path: web::Path<String>) -> impl Responder {
    match delete_wg_interface(path.as_str()) {
        Ok(_) => Ok(HttpResponse::Ok()),
        Err(e) => {
            let msg = format!("failed to delete interface: {}", e.to_string());
            Err(ret_internal_server_error(msg))
        }
    }
}

///
/// 
/// 
pub async fn handle_wg_set(req)