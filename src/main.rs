pub mod common;
pub mod iproute2_support;
pub mod multi_error;
pub mod utils;
pub mod wg_endpoints;
pub mod wg_support;

use crate::multi_error::MultiError;
use crate::utils::setup_logger;
use actix_web::{
    get, middleware,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use clap::{Arg, ArgMatches};
use wg_endpoints::{
    handle_create_wg_interface, handle_delete_wg_interface, handle_wg_add_peer,
    handle_wg_remove_peer, handle_wg_set, handle_wg_show, handle_wg_show_interface,
    handle_wg_show_interfaces, wg_genkey, wg_genpsk, wg_pubkey, wg_show_ifc_element,
    wg_showconf_ifc,
};

#[derive(Debug, Clone, Default)]
pub struct AppContext {
    pub bind_address: String,
    pub port: u16,
}

#[get("/")]
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

///
///
///
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
                .route("/show", web::get().to(handle_wg_show))
                .route("/show/interfaces", web::get().to(handle_wg_show_interfaces))
                .route("/show/{interface}", web::get().to(handle_wg_show_interface))
                .route(
                    "/show/{interface}/{element}",
                    web::get().to(wg_show_ifc_element),
                )
                .route("/showconf/{interface}", web::get().to(wg_showconf_ifc))
                .route("/genkey", web::get().to(wg_genkey))
                .route("/genpsk", web::get().to(wg_genpsk))
                .route("/pubkey", web::post().to(wg_pubkey))
                .route("/interface", web::post().to(handle_create_wg_interface))
                .route(
                    "/interface/{interface}",
                    web::delete().to(handle_delete_wg_interface),
                )
                .route("/set/{interface}", web::put().to(handle_wg_set))
                .route("/{interface}/peer", web::post().to(handle_wg_add_peer))
                .route(
                    "/{interface}/peer",
                    web::delete().to(handle_wg_remove_peer),
                ),
        )
        // .service(
        //     web::scope("/api/v1/")
        // )
    })
    .bind(listen_addr.as_str())?
    .run()
    .await?;

    Ok(())
}
