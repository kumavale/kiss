//! `kiss` is a `kill` clone.  
//! Sends a unix signal to the specified process id.  

mod signal;
mod message;

use std::process::ExitCode;
use signal::*;
use message::*;
use nix::unistd::Pid;
use nix::sys::signal::{kill as kiss, Signal};

#[allow(non_camel_case_types)]
type pid_t = i32;

fn main() -> ExitCode {
    // signal
    let mut signal: Option<Signal> = None;
    // pid lists
    let mut pids: Vec<pid_t> = vec![];

    // parse command line arguments
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match &*arg {
            "-h" | "--help" => {
                help();
                return ExitCode::SUCCESS;
            }
            "-v" | "--version" => {
                version();
                return ExitCode::SUCCESS;
            }
            "-l" | "--list" => {
                list();
                return ExitCode::SUCCESS;
            }
            "-s" | "--signal" => {
                // set signal
                if let Some(arg) = args.next() {
                    match from_str(&arg) {
                        Ok(sig) => signal = Some(sig),
                        Err(e) => {
                            eprintln!("{e}");
                            return ExitCode::FAILURE;
                        }
                    }
                } else {
                    not_enough_arg(&arg);
                    return ExitCode::FAILURE;
                }
            }
            pid => {
                // set pid
                if let Ok(pid) = pid.parse::<pid_t>() {
                    pids.push(pid);
                } else {
                    invalid_option(pid);
                    return ExitCode::FAILURE;
                }
            }
        }
    }

    if pids.is_empty() {
        do_nothing();
        ExitCode::SUCCESS
    } else if let Some(sig) = signal {
        run(sig, &pids);
        ExitCode::SUCCESS
    } else {
        expect_signal();
        ExitCode::FAILURE
    }
}

fn run(sig: Signal, pids: &[pid_t]) {
    for pid in pids {
        match kiss(Pid::from_raw(*pid), sig) {
            Ok(_) =>   println!("{sig} to {pid}"),
            Err(e) => eprintln!("failed: `{e}`"),
        }
    }
}
