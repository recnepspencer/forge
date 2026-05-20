use super::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionIntentArbitrationConflictClass,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationPolicyCase,
};
use worth_spatial::facade::{SpatialBlockedCapability, SpatialIntentEscalation};

#[test]
fn arbitration_policy_report_preserves_blocked_and_conflict_truth() {
    let report = prepare_primitive_intent_arbitration_policy_report().expect("policy report");
    let overlap = report
        .row(PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates)
        .expect("overlap row");

    assert_eq!(
        overlap.conflict_class(),
        PrimitiveConstructionIntentArbitrationConflictClass::BlockedCandidateSet
    );
    assert_eq!(
        overlap.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::MergeBoolean)
    );
    assert!(!overlap.blocked_candidates().is_empty());
}

#[test]
fn arbitration_dx_surface_report_distinguishes_common_and_human_boundaries() {
    let report = prepare_primitive_intent_conflict_dx_surface_report().expect("dx report");
    let direct = report
        .row(PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly)
        .expect("direct row");
    let grazing = report
        .row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
        .expect("grazing row");

    assert_eq!(
        direct.dx_surface(),
        PrimitiveConstructionIntentArbitrationDxSurface::CommonPath
    );
    assert_eq!(
        grazing.dx_surface(),
        PrimitiveConstructionIntentArbitrationDxSurface::HumanEscalation
    );
}

#[test]
fn chosen_intent_resolution_report_preserves_policy_and_explicit_choice_truth() {
    let report = prepare_primitive_chosen_intent_resolution_report().expect("chosen report");
    let policy = report
        .row(PrimitiveConstructionChosenIntentResolutionCase::PolicyMoveOnly)
        .expect("policy row");
    let explicit = report
        .row(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
        .expect("explicit row");

    assert_eq!(
        policy.authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::PolicyAutoResolve
    );
    assert_eq!(
        explicit.authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice
    );
}
