use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject,
};

use super::runtime_handles::diagnostic_handle;

pub(crate) fn diagnostic_receipt_for(
    world: &'static str,
    subject: PlanarDiagnosticSubject,
) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(subject)
        .inspect_failure_locality()
        .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
            world,
        )))
        .expect("local rebuild diagnostic plan")
        .certify()
        .expect("local rebuild diagnostic receipt")
}
