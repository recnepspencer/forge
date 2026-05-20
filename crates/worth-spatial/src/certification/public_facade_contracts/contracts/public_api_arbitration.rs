use worth_spatial::facade::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialChosenIntentAuthority,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialIntentConflictClass,
    SpatialIntentEscalation, SpatialIntentResolutionError, SpatialObservedRelationFact,
};

#[test]
fn spatial_public_facade_exports_common_path_arbitration_surface() {
    let analysis = analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialIntentConflictClass::UnsafeToAssume
    );
    assert_eq!(
        analysis.escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert!(analysis
        .candidates()
        .iter()
        .any(|candidate| candidate.candidate() == SpatialIntentCandidate::MoveOnly));
    assert!(analysis
        .candidates()
        .iter()
        .any(|candidate| candidate.candidate() == SpatialIntentCandidate::SnapFlush));
}

#[test]
fn spatial_public_facade_exports_advanced_arbitration_surface() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[
            SpatialObservedRelationFact::Overlap,
            SpatialObservedRelationFact::InsideTarget,
        ],
        SpatialIntentCapabilitySet::blocked_defaults(),
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialIntentConflictClass::BlockedCandidateSet
    );
    assert_eq!(
        analysis.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::MergeBoolean)
    );
    assert!(analysis
        .candidates()
        .iter()
        .any(|candidate| candidate.candidate() == SpatialIntentCandidate::NestInside));
}

#[test]
fn spatial_public_facade_exports_chosen_intent_resolution_surface() {
    let policy = resolve_spatial_intent_conflict_by_policy(analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[],
    ))
    .expect("policy resolution");
    let explicit = resolve_spatial_intent_conflict_by_choice(
        analyze_spatial_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::GrazingContact],
        ),
        SpatialIntentCandidate::SnapFlush,
    )
    .expect("explicit resolution");
    let blocked = resolve_spatial_intent_conflict_by_choice(
        analyze_spatial_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::Overlap],
        ),
        SpatialIntentCandidate::MergeCandidate,
    )
    .expect_err("blocked merge");

    assert_eq!(policy.chosen_candidate(), SpatialIntentCandidate::MoveOnly);
    assert_eq!(
        policy.authority(),
        SpatialChosenIntentAuthority::PolicyAutoResolve
    );
    assert_eq!(
        explicit.chosen_candidate(),
        SpatialIntentCandidate::SnapFlush
    );
    assert_eq!(
        explicit.authority(),
        SpatialChosenIntentAuthority::ExplicitChoice
    );
    assert_eq!(
        blocked,
        SpatialIntentResolutionError::CandidateBlocked(
            SpatialIntentCandidate::MergeCandidate,
            SpatialBlockedCapability::MergeBoolean
        )
    );
}
