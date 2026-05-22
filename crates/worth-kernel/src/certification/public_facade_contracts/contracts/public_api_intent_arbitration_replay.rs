use worth_kernel::facade::diagnostics::arbitration::*;
use worth_spatial::facade::{
    SpatialBlockedCapability, SpatialIntentCandidate, SpatialIntentEscalation,
};

#[test]
fn kernel_public_facade_exports_intent_arbitration_replay_parity_surface() {
    let explicit = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        PrimitiveConstructionPreservedIntentResolutionCase::ExplicitSnapFlush,
    )
    .expect("explicit replay report");
    let blocked = prepare_primitive_construction_intent_arbitration_replay_parity_report(
        PrimitiveConstructionPreservedIntentResolutionCase::HostPenetrationBlockedCut,
    )
    .expect("blocked replay report");

    assert!(explicit.parity_verified());
    assert_eq!(
        explicit.direct_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Resolved {
            candidate: SpatialIntentCandidate::SnapFlush,
            authority: PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice,
        }
    );
    assert!(blocked.parity_verified());
    assert_eq!(
        blocked.replay_row().preserved_truth(),
        PrimitiveConstructionPreservedIntentTruth::Unresolved {
            escalation: SpatialIntentEscalation::BlockedByMissingCapability(
                SpatialBlockedCapability::CutOpening
            ),
            blocked_capability: Some(SpatialBlockedCapability::CutOpening),
        }
    );
}
