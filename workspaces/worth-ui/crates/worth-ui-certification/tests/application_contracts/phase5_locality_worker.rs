use std::path::PathBuf;

use worth_ui_certification::scenario::phase5_locality_matrix;

const EVIDENCE_PREFIX: &str = "WORTH_UI_PHASE5_PRODUCTION_LOCALITY=";
const WORKER_FILTER: &str = "phase5_locality_worker::phase5_locality_matrix_worker";

pub(super) fn invocation() -> (PathBuf, [&'static str; 4]) {
    (
        std::env::current_exe().expect("application-contract executable identity"),
        ["--exact", WORKER_FILTER, "--ignored", "--nocapture"],
    )
}

#[test]
#[ignore = "filtered subprocess worker for the Phase 5 locality matrix"]
fn phase5_locality_matrix_worker() {
    let worker = std::thread::Builder::new()
        .name("worth-ui-phase5-locality-worker".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(|| {
            let (executable, arguments) = invocation();
            phase5_locality_matrix::execute_worker(&executable, &arguments)
        })
        .expect("the locality worker thread starts");
    match worker
        .join()
        .expect("the locality worker thread remains live")
    {
        Ok(rows) => println!(
            "{EVIDENCE_PREFIX}{}",
            serde_json::to_string(&rows).expect("matrix evidence serializes")
        ),
        Err(denial) => panic!("WORTH UI Phase 5 locality matrix denied: {denial}"),
    }
}
