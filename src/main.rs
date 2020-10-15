mod utils;
mod multi_error;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, error, middleware, Error};

use std::process::Command;
use crate::utils::setup_logger;
use crate::multi_error::MultiError;
use clap::{ArgMatches, Arg};


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

    log::debug!("output={:?}, stdout={}, stderr={}", output, stdout_string, stderr_string);

    if !stderr_string.is_empty() {
        log::error!("failed to execute command: {}", stderr_string);
        return Err(MultiError {kind: "CommandError".to_string(), message: "failed to execute to command".to_string()});
    }

    Ok(stdout_string)
}


#[get("/")]
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/show")]
async fn wg_show() -> Result<HttpResponse, Error> {
    let wg_show_out = match run_command("wg", &vec!["show"]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg show"));
        }
    };

    Ok(HttpResponse::Ok().json(wg_show_out))
}

#[get("/show/interfaces")]
async fn wg_show_interfaces() -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["show", "interfaces"]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show interfaces: {}", e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg show interfaces"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
}

#[get("/show/{interface}")]
async fn wg_show_interface(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let out = match run_command("wg", &vec!["show", path.as_str()]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show {}: {}", path, e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg show interface"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
}

#[get("/show/{interface}/{element}")]
async fn wg_show_ifc_element(path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let path = path.into_inner();
    let out = match run_command("wg", &vec!["show", &path.0, &path.1]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg show {} {}: {}", path.0, path.1, e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg show interface"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
}

#[get("/showconf/{interface}")]
async fn wg_showconf_ifc(path: web::Path<String>) -> Result<HttpResponse, Error> {
    let path = path.into_inner();
    let out = match run_command("wg", &vec!["showconf", &path]) {
        Ok(x) => x,
        Err(e) => {
            log::error!("failed to run wg showconf {}: {}", path, e.to_string());
            return Err(error::ErrorInternalServerError("failed to run wg showconf"));
        }
    };

    Ok(HttpResponse::Ok().json(out))
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
    .arg(Arg::with_name("bind_address")
        .long("address")
        .short("a")
        .help("address to bind to")
        .takes_value(true)
        .value_name("BIND ADDRESS"))
    .arg(Arg::with_name("listen_port")
        .long("port")
        .short("p")
        .help("port to listen on")
        .takes_value(true)
        .value_name("PORT NUMBER"))
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
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1/wg")
                    .service(wg_show)
                    .service(wg_show_interfaces)
                    .service(wg_show_interface)
                    .service(wg_show_ifc_element)
                    .service(wg_showconf_ifc)
                    .service(wg_genkey)
                    .service(wg_genpsk))
    })
        .bind(listen_addr.as_str())?
        .run()
        .await?;

    Ok(())
}