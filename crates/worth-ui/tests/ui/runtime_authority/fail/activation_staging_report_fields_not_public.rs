use worth_ui::facade::WorthUiActivationStagingReport;

fn main() {
    let _report = WorthUiActivationStagingReport {
        active_artifact_digest: 1,
        candidate_artifact_digest: 2,
        readiness: missing(),
        counters: missing(),
    };
}

fn missing<T>() -> T {
    loop {}
}
