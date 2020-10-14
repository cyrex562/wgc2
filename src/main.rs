mod utils;
mod multi_error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, error, middleware, HttpRequest, Error};

use std::process::Command;
use crate::utils::setup_logger;
use crate::multi_error::MultiError;


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
async fn wg_show_interface(path: web::Path<(String)>) -> Result<HttpResponse, Error> {
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



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1/wg")
                    .service(wg_show)
                    .service(wg_show_interfaces)
                    .service(wg_show_interface)
                    .service(wg_show_ifc_element))
                // web::resource("api/v1/wg/show").route(web::get().to(wg_show)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}