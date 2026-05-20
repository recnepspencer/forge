use worth_kernel::facade::{
    analyze_primitive_intent_conflict, analyze_primitive_intent_conflict_with_capabilities,
    prepare_primitive_chosen_intent_resolution_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_clarification_request,
    prepare_primitive_intent_conflict_dx_surface_report,
    resolve_primitive_intent_conflict_by_choice, resolve_primitive_intent_conflict_by_policy,
    PrimitiveConstructionChosenIntentResolutionAuthority,
    PrimitiveConstructionChosenIntentResolutionCase,
    PrimitiveConstructionIntentArbitrationDxSurface,
    PrimitiveConstructionIntentArbitrationPolicyCase, PrimitiveIntentClarificationRequestError,
    SpatialAuthoredActKind, SpatialIntentCandidate, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialObservedRelationFact,
};

#[test]
fn kernel_public_facade_exports_intent_arbitration_analysis_surface() {
    let direct = analyze_primitive_intent_conflict(SpatialAuthoredActKind::Move, &[]);
    let advanced = analyze_primitive_intent_conflict_with_capabilities(
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
fn kernel_public_facade_exports_intent_resolution_surface() {
    let policy = resolve_primitive_intent_conflict_by_policy(analyze_primitive_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[],
    ))
    .expect("policy resolution");
    let explicit = resolve_primitive_intent_conflict_by_choice(
        analyze_primitive_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::GrazingContact],
        ),
        SpatialIntentCandidate::SnapFlush,
    )
    .expect("explicit resolution");

    assert_eq!(policy.chosen_candidate(), SpatialIntentCandidate::MoveOnly);
    assert_eq!(
        explicit.authority(),
        worth_kernel::facade::SpatialChosenIntentAuthority::ExplicitChoice
    );
}

#[test]
fn kernel_public_facade_exports_intent_clarification_boundary_surface() {
    let unresolved =
        prepare_primitive_intent_clarification_request(analyze_primitive_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::GrazingContact],
        ))
        .expect("clarification request");
    let blocked =
        prepare_primitive_intent_clarification_request(analyze_primitive_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::HostPenetration],
        ))
        .expect("blocked clarification");
    let direct = prepare_primitive_intent_clarification_request(analyze_primitive_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[],
    ));

    assert_eq!(
        unresolved.escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert!(!unresolved.candidates().is_empty());
    assert!(blocked.blocked_capability().is_some());
    assert_eq!(
        direct,
        Err(
            PrimitiveIntentClarificationRequestError::NoClarificationBoundary(
                SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::MoveOnly)
            )
        )
    );
}
