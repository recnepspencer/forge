use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryAction, PlanarRecoveryBlockerKind, PlanarRecoveryPostureCounters,
    PlanarRecoveryPostureReceipt, PlanarRecoverySourcePosture, PlanarRecoveryTargetScope,
    PlanarRecoveryTruthEffect,
};

fn main() {
    let _receipt = PlanarRecoveryPostureReceipt {
        basis: panic!("private basis"),
        blocker_kind: PlanarRecoveryBlockerKind::ProjectionBasis,
        source_posture: PlanarRecoverySourcePosture::Denied,
        recovery_action: PlanarRecoveryAction::InspectProjectionBasis,
        target_scope: PlanarRecoveryTargetScope::ProjectionBasisInspection,
        truth_effect: PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth,
        declaration_digest: String::new(),
        progression_digest: String::new(),
        route_plan_digest: String::new(),
        query_receipt_digest: String::new(),
        envelope_digest: String::new(),
        recovery_posture_digest: String::new(),
        counters: PlanarRecoveryPostureCounters::default(),
    };
}
