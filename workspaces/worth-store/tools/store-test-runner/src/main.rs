use std::process::ExitCode;

fn main() -> ExitCode {
    match store_test_runner::run_from_environment() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("store-test-runner: {error}");
            ExitCode::FAILURE
        }
    }
}
