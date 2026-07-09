use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceCertificationFamily, ResourceCertificationRecord,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn main() {
    let _WORTHd = ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncResourceLifecycleParity,
        "fake-digest",
        WORTHd_performance(),
    );
}
