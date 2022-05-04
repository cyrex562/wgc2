use clap::App;
use subprocess::Exec;
use subprocess::Redirection;
use std::error;
use std::fmt;
use subprocess::{ExitStatus, Popen, PopenConfig};

#[derive(Debug, Clone)]
struct RunCommandError {
    msg: String,
}

impl From<std::io::Error> for RunCommandError {
    fn from(err: std::io::Error) -> RunCommandError {
        RunCommandError {
            msg: format!("I/O error: {:?}", err),
        }
    }
}

impl From<subprocess::PopenError> for RunCommandError {
    fn from(err: subprocess::PopenError) -> RunCommandError {
        RunCommandError {
            msg: format!("subprocess Popen error: {:?}", err),
        }
    }
}

fn run_command(command_line: &[String]) -> Result<String, RunCommandError> {
    let mut result = String::from("");

    let mut p = Popen::create(
        command_line,
        PopenConfig {
            stdout: subprocess::Redirection::Pipe,
            ..Default::default()
        },
    )?;

    let (out, err) = p.communicate(None)?;
    let exit_status_opt = p.poll();
    if exit_status_opt.is_some() {
        let exit_status = exit_status_opt.unwrap();
        match exit_status {
            ExitStatus::Exited(code) => {
                println!("exit code: {}", code);
                if code != 0 {
                    return Err(RunCommandError {
                        msg: format!(
                            "non-zero exit code {}; stderr: {}; stdout: {}",
                            code,
                            err.unwrap(),
                            out.unwrap()
                        ),
                    });
                } else {
                    println!("zero exit code; stdout: {}", out.clone().unwrap());
                    result = out.unwrap();
                }
            }
            ExitStatus::Signaled(signal) => {
                println!("exited due to signal {}", signal);
            }
            ExitStatus::Other(other) => {
                return Err(RunCommandError {
                    msg: String::from("process exited in non-standard state"),
                })
            }
            ExitStatus::Undetermined => {
                return Err(RunCommandError {
                    msg: String::from("process in indeterminate state"),
                })
            }
        }
    } else {
        p.terminate()?;
    }

    Ok(result)
}

fn gen_key() -> Result<(String, String), RunCommandError> {
    let private_key = Exec::shell("wg genkey").stdout(Redirection::Pipe).stderr(Redirection::Merge).capture()?.stdout_str();
    let public_key = Exec::shell("wg pubkey").stdin(private_key.as_str()).stdout(Redirection::Pipe).stderr(Redirection::Merge).capture()?.stdout_str();
    Ok((private_key, public_key))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let app = App::new("wgmu")
        .about("WireGuard Management Utility")
        .version("v0.1.0")
        .author("Cyrex cyrex562@gmail.com")
        .subcommand(
            App::new("show")
                .about("gets information about the state and properties of WG interfaces"),
        )
        .subcommand(App::new("genkey").about(
            "generates a wireguard private key and displays both the private and public keys",
        ))
        .get_matches();

    // wg show
    if let Some(matches) = app.subcommand_matches("show") {
        match run_command(&[String::from("wg"), String::from("show")]) {
            Ok(result_string) => {
                print!("wg show result: {}", result_string);
            }
            Err(err) => {
                panic!("wg show call failed: {:?}", err);
            }
        }
    }
    // wg genkey
    if let Some(matches) = app.subcommand_matches("genkey") {
        match gen_key() {
            Ok((private_key, public_key)) => {
                print!("private key: {}", private_key);
                print!("public key: {}\n", public_key);
            },
            Err(err) => {
                panic!("wg genkey call failed: {:?}", err);
            }
        }
    }


    Ok(())
}
