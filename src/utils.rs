use std::{io::Write, process::{Command, Output, Stdio}};

use actix_web::{error, Error};

use crate::multi_error::MultiError;

///
///
///
pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}: {}: {}: {}",
                chrono::Local::now().format("%Y%m%d.%H%M%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

///
///
///
pub fn ret_internal_server_error(msg: String) -> Error {
    log::error!("{}", msg);
    error::ErrorInternalServerError(msg)
}

///
///
///
pub fn run_command(command: &str, args: &Vec<&str>, stdin: Option<String>) -> Result<String, MultiError> {

    let output: Output;
    if stdin.is_some() {
        let mut child = Command::new(command)
            .args(args.as_slice())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok().unwrap();
        child.stdin.as_mut().unwrap().write_all(stdin.unwrap().as_bytes())?;
        output = child.wait_with_output()?;
    } else {
        output = Command::new(command)
        .args(args.as_slice())
        .output()?;
    }

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
