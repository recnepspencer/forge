fn main() -> std::process::ExitCode {
    match worth_ui_certification::scenario::phase5_locality_matrix::execute() {
        Ok(rows) => {
            println!(
                "WORTH_UI_PHASE5_PRODUCTION_LOCALITY={}",
                serde_json::to_string(&rows).expect("matrix evidence serializes")
            );
            std::process::ExitCode::SUCCESS
        }
        Err(denial) => {
            eprintln!("WORTH UI Phase 5 locality matrix denied: {denial}");
            std::process::ExitCode::from(3)
        }
    }
}
