use forge_query::facade::{
    forge_query_lower_runtime_closeout_report, forge_query_lower_runtime_closure_test,
    forge_query_lower_runtime_phase_manifest,
};

fn main() {
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(run)
        .expect("golden transcript thread should spawn")
        .join()
        .expect("golden transcript should run");
}

fn run() {
    let report = forge_query_lower_runtime_closeout_report();
    let manifest = forge_query_lower_runtime_phase_manifest();
    let closure = forge_query_lower_runtime_closure_test();

    let _ = report.report_digest();
    let _ = report.stabilization_target_digest();
    let _ = report
        .phase_manifest()
        .rows()
        .last()
        .expect("phase manifest should end in the stabilization closeout artifact")
        .next_consumer();
    let _ = manifest.manifest_digest();
    let _ = manifest.typestate_transition_digest();
    let _ = closure.suite_digest();
    let _ = closure.certification_bundle().certification_bundle_digest();
}
