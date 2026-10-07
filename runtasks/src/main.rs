use runtasks::cli::Command;
use std::process::ExitCode;

fn main() -> ExitCode {
    // `skip(1)` drops the program name.
    let result = Command::parse(std::env::args().skip(1)).and_then(runtasks::run);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE // non-zero exit code so CI/scripts notice
        }
    }
}
