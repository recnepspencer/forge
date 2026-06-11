use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundleReceipt, PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

fn main() {
    let _receipt = PlanarDiagnosticBundleReceipt {
        basis: panic!("private basis"),
        trigger_locality: PlanarDiagnosticTriggerLocality::PredicateAuthority,
        truth_effect: PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth,
        declaration_digest: String::new(),
        progression_digest: String::new(),
        route_plan_digest: String::new(),
        query_receipt_digest: String::new(),
        envelope_digest: String::new(),
        diagnostic_bundle_digest: String::new(),
        counters: panic!("private counters"),
    };
}
