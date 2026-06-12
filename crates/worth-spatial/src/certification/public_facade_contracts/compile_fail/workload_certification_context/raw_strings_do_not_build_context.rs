use worth_spatial::facade::workload_certification_context::WorkloadCertificationContext;

fn main() {
    let _ = WorkloadCertificationContext::from_raw_strings(
        "frame:hand-authored",
        "topology:hand-authored",
        "movement:hand-authored",
    );
}
