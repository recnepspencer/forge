use worth_kernel::facade::{
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationPolicyCase, PrimitiveIntentClarificationRequestError,
    PrimitiveIntentConflict, SpatialAuthoredActKind, SpatialIntentCandidate,
    SpatialIntentCapabilitySet, SpatialIntentConflictClass, SpatialIntentEscalation,
    SpatialObservedRelationFact,
};
use worth_spatial::facade::SpatialBlockedCapability;

#[test]
fn kernel_public_facade_exports_artifact_style_intent_arbitration_analysis_surface() {
    let direct = PrimitiveIntentConflict::analyze(SpatialAuthoredActKind::Move, &[]);
    let advanced = PrimitiveIntentConflict::analyze_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults()
            .with_merge_boolean()
            .with_subtract_boolean(),
    );

    assert_eq!(
        direct.conflict_class(),
        SpatialIntentConflictClass::SingleClearIntent
    );
    assert_eq!(
        advanced.conflict_class(),
        SpatialIntentConflictClass::UnsafeToAssume
    );
}

#[test]
fn kernel_public_facade_exports_intent_arbitration_reports() {
    let policy = prepare_primitive_intent_arbitration_policy_report().expect("policy");
    let dx = prepare_primitive_intent_conflict_dx_surface_report().expect("dx");
    let chosen = prepare_primitive_chosen_intent_resolution_report().expect("chosen");

    assert_eq!(
        policy
            .row(PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly)
            .expect("direct row")
            .chosen_candidate(),
        Some(SpatialIntentCandidate::MoveOnly)
    );
    assert_eq!(
        dx.row(PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict)
            .expect("grazing row")
            .dx_surface(),
        PrimitiveConstructionIntentArbitrationDxSurface::HumanEscalation
    );
    assert_eq!(
        chosen
            .row(PrimitiveConstructionChosenIntentResolutionCase::ExplicitSnapFlush)
            .expect("chosen row")
            .authority(),
        PrimitiveConstructionChosenIntentResolutionAuthority::ExplicitChoice
    );
}

#[test]
fn kernel_public_facade_exports_artifact_style_intent_resolution_surface() {
    let policy = PrimitiveIntentConflict::analyze(SpatialAuthoredActKind::Move, &[])
        .resolve_by_policy()
        .expect("policy resolution");
    let explicit = PrimitiveIntentConflict::analyze(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
    )
    .resolve_by_choice(SpatialIntentCandidate::SnapFlush)
    .expect("explicit resolution");

    assert_eq!(policy.chosen_candidate(), SpatialIntentCandidate::MoveOnly);
    assert_eq!(
        explicit.authority(),
        worth_kernel::facade::SpatialChosenIntentAuthority::ExplicitChoice
    );
}

#[test]
fn kernel_public_facade_exports_artifact_style_intent_clarification_boundary_surface() {
    let unresolved = PrimitiveIntentConflict::analyze(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
    )
    .clarification_request()
    .expect("clarification request");
    let blocked = PrimitiveIntentConflict::analyze(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostPenetration],
    )
    .clarification_request()
    .expect("blocked clarification");
    let direct =
        PrimitiveIntentConflict::analyze(SpatialAuthoredActKind::Move, &[]).clarification_request();

    assert_eq!(
        unresolved.escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert_eq!(
        unresolved
            .candidates()
            .iter()
            .map(|candidate| candidate.candidate())
            .collect::<Vec<_>>(),
        vec![
            SpatialIntentCandidate::SnapFlush,
            SpatialIntentCandidate::MoveOnly,
        ]
    );
    assert_eq!(
        blocked.blocked_capability(),
        Some(SpatialBlockedCapability::CutOpening)
    );
    assert_eq!(
        direct,
        Err(
            PrimitiveIntentClarificationRequestError::NoClarificationBoundary(
                SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::MoveOnly)
            )
        )
    );
}
