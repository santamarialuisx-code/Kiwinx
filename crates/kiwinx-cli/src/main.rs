use std::{env, process::ExitCode};

use kiwinx_common::GuestAddress;
use kiwinx_core::System;
use kiwinx_testkit::exit_after_add;

const USAGE: &str = "Usage: kiwinx-cli demo\n       kiwinx-cli --help";

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);

    match arguments.next().as_deref() {
        Some("demo") if arguments.next().is_none() => run_demo(),
        Some("--help") | Some("-h") if arguments.next().is_none() => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run_demo() -> ExitCode {
    let mut system = System::new(4096);
    let program = exit_after_add(40, 2);

    if let Err(error) = system.load(GuestAddress::new(0), &program) {
        eprintln!("failed to load the demo: {error}");
        return ExitCode::FAILURE;
    }

    match system.run(GuestAddress::new(0), 16) {
        Ok(exit_code) => {
            println!("Guest exited with code {exit_code}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("demo execution failed: {error}");
            ExitCode::FAILURE
        }
    }
}
