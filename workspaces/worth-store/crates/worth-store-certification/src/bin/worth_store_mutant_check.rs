use std::num::NonZeroU64;
use std::path::PathBuf;

use worth_store_certification::courtroom::protocol_models::mutants::run_controlled_mutant_program;
use worth_store_formal_models::runner::ProtocolCheckBounds;

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    let [_, java, tool_jar, state_root] = arguments.as_slice() else {
        eprintln!("usage: worth-store-mutant-check <java> <tla2tools.jar> <state-root>");
        std::process::exit(2);
    };
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(1_000_000).unwrap(),
        NonZeroU64::new(100).unwrap(),
    );
    let state_root = PathBuf::from(state_root);
    match run_controlled_mutant_program(java, tool_jar, state_root, bounds) {
        Ok(report) => println!(
            "rejected and localized {} controlled protocol defects",
            report.rejections().len()
        ),
        Err(failure) => {
            eprintln!("controlled protocol mutation program failed: {failure:?}");
            std::process::exit(1);
        }
    }
}
