use worth_spatial::facade::open_planar_posture::{
    OpenPlanarPostureCase, OpenPlanarPostureCounters, OpenPlanarPostureReceipt,
};
use worth_spatial::facade::planar_clean_fail_boundary::PlanarOpenInputKind;
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubjectKind;

fn main() {
    let counters: OpenPlanarPostureCounters = unsafe { std::mem::zeroed() };
    let _receipt = OpenPlanarPostureReceipt {
        posture_digest: "digest".to_string(),
        workload_identity: "workload".to_string(),
        topology_receipt_identity: "topology".to_string(),
        unsupported_surface_identity: "support".to_string(),
        clean_fail_boundary_identity: "clean-fail".to_string(),
        diagnostic_receipt_identity: "diagnostic".to_string(),
        open_input_kind: Some(PlanarOpenInputKind::OpenPlanarDomain),
        diagnostic_subject_kind: PlanarDiagnosticSubjectKind::UnsupportedPlanarClass,
        posture_case: OpenPlanarPostureCase::UnsupportedOpenSheet,
        counters,
        bounded_surrogate_was_not_used: true,
    };
}
