use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

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
pub fn ret_multi_err(msg: String) -> MultiError {
    log::error!("{}", msg);
    MultiError {
        kind: "Error".to_string(),
        message: msg,
    }
}

///
///
///
pub fn run_command(
    command: &str,
    args: &Vec<&str>,
    stdin: Option<String>,
) -> Result<String, MultiError> {
    let output: Output;
    log::debug!("command={}, args=[{:?}]", command, args);
    if stdin.is_some() {
        let mut child = Command::new(command)
            .args(args.as_slice())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok()
            .unwrap();
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(stdin.unwrap().as_bytes())?;
        output = child.wait_with_output()?;
    } else {
        output = Command::new(command).args(args.as_slice()).output()?;
    }

    let stdout_string = String::from_utf8(output.stdout.clone())?;
    let stderr_string = String::from_utf8(output.stderr.clone())?;

    // log::debug!(
    //     "output={:?}, stdout=\"{}\", stderr=\"{}\"",
    //     output,
    //     stdout_string,
    //     stderr_string
    // );

    if !stderr_string.is_empty() {
        log::error!(
            "failed to execute command={}, args=[{:?}], stderr=\"{}\"",
            command,
            args,
            stderr_string
        );
        return Err(MultiError {
            kind: "CommandError".to_string(),
            message: "failed to execute to command".to_string(),
        });
    }

    Ok(stdout_string)
}

pub fn stop_systemd_service(unit: &str) -> Result<(), MultiError> {
    let _out = run_command("systemctl", &vec!["stop", unit], None)?;
    Ok(())
}

pub fn disable_systemd_service(unit: &str) -> Result<(), MultiError> {
    let _out = run_command("systemctl", &vec!["disable", unit], None)?;
    Ok(())
}

pub fn daemon_reload_systemd() -> Result<(), MultiError> {
    let _out = run_command("systemctl", &vec!["daemon-reload"], None)?;
    Ok(())
}

pub fn rest_failed_systemd() -> Result<(), MultiError> {
    let _out = run_command("systemctl", &vec!["reset-failed"], None)?;
    Ok(())
}
