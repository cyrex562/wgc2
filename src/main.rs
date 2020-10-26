mod multi_error;
mod utils;
mod wg_support;

use actix_files::NamedFile;
use actix_web::{error, get, middleware, web, App, Error, HttpResponse, HttpServer, Responder};

use crate::multi_error::MultiError;
use crate::utils::{setup_logger, ret_internal_server_error};
use crate::wg_support::{
    parse_wg_show_allowed_ips, parse_wg_show_endpoints, parse_wg_show_fwmark,
    parse_wg_show_interfaces, parse_wg_show_latest_handshakes, parse_wg_show_listen_port,
    parse_wg_show_output, parse_wg_show_peers, parse_wg_show_persistent_keepalive,
    parse_wg_show_preshared_keys, parse_wg_show_pub_key, parse_wg_show_pvt_key,
    parse_wg_show_transfer, WgShowAll, WgShowInterfaces,
};
use clap::{Arg, ArgMatches};
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::{Write};

#[derive(Debug, Clone, Default)]
pub struct AppContext {
    pub bind_address: String,
    pub port: u16,
}

fn run_command(command: &str, args: &Vec<&str>) -> Result<String, MultiError> {
    let output = Command::new(command)
        .args(args.as_slice())
        .output()
        .expect("failed to execute command");

    let stdout_string = String::from_utf8(output.stdout.clone())?;
    let stderr_string = String::from_utf8(output.stderr.clone())?;

    log::debug!(
        "output={:?}, stdout={}, stderr={}",
        output,
        stdout_string,
        stderr_string
    );

    if !stderr_string.is_empty() {
        log::error!("failed to execute command: {}", stderr_string);
        return Err(MultiError {
            kind: "CommandError".to_string(),
            message: "failed to execute to command".to_string(),
        });
    }

    Ok(stdout_string)
}

#[get("/")]
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/show")]
async fn wg_show() -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["show"]) {
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

#[get("/show/interfaces")]
async fn wg_show_interfaces() -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["show", "interfaces"]) {
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

#[get("/show/{interface}")]
async fn wg_show_interface(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["show", path.as_str()]) {
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

#[get("/show/{interface}/{element}")]
async fn wg_show_ifc_element(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let path = path.into_inner();
    let out = match run_command("wg", &vec!["show", &path.0, &path.1]) {
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
#[get("/showconf/{interface}")]
async fn wg_showconf_ifc(path: web::Path<String>) -> Result<NamedFile, Error> {
    let path = path.into_inner();
    let out = match run_command("wg", &vec!["showconf", &path]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg showconf {}: {}", path, e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg showconf"));
        }
    };

    let mut file = match NamedTempFile::new() {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to create temp file for download: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to create temp file for download"));
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

#[get("/genkey")]
async fn wg_genkey() -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["genkey"]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg genkey: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg genkey"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
}

#[get("/genpsk")]
async fn wg_genpsk() -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["genpsk"]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg genpsk: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg genpsk"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
}

fn setup_arg_matches<'a>() -> ArgMatches<'a> {
    clap::App::new("wgc2")
        .version("")
        .author("xyz")
        .about("")
        // ip address to bind to
        .arg(
            Arg::with_name("bind_address")
                .long("address")
                .short("a")
                .help("address to bind to")
                .takes_value(true)
                .value_name("BIND ADDRESS"),
        )
        .arg(
            Arg::with_name("listen_port")
                .long("port")
                .short("p")
                .help("port to listen on")
                .takes_value(true)
                .value_name("PORT NUMBER"),
        )
        .get_matches()
}

pub fn parse_cmd_line() -> Result<AppContext, MultiError> {
    let cmd_line = setup_arg_matches();
    let mut app_ctx: AppContext = Default::default();
    let mut bind_address: String = String::from("127.0.0.1");
    if cmd_line.is_present("bind_address") {
        let bind_addr_str = cmd_line.value_of("bind_address").unwrap();
        bind_address = bind_addr_str.to_string();
    }
    app_ctx.bind_address = bind_address;
    let mut port: u16 = 8080;
    if cmd_line.is_present("listen_port") {
        let port_str = cmd_line.value_of("listen_port").unwrap();
        port = port_str.parse::<u16>()?;
    }
    app_ctx.port = port;

    Ok(app_ctx)
}

#[actix_web::main]
async fn main() -> std::result::Result<(), MultiError> {
    let app_ctx = parse_cmd_line()?;
    let listen_addr = format!("{}:{}", app_ctx.bind_address, app_ctx.port);

    setup_logger().unwrap();

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api/v1/wg")
                .service(wg_show)
                .service(wg_show_interfaces)
                .service(wg_show_interface)
                .service(wg_show_ifc_element)
                .service(wg_showconf_ifc)
                .service(wg_genkey)
                .service(wg_genpsk),
        )
    })
    .bind(listen_addr.as_str())?
    .run()
    .await?;

    Ok(())
}
