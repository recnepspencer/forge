use std::process::ExitCode;

fn main() -> ExitCode {
    match store_proof_control::cli::run(std::env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("store proof control denied: {error}");
            ExitCode::FAILURE
        }
    }
}
