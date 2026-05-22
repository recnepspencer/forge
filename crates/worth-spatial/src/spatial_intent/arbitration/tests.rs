use super::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    declare_spatial_arbitration_runtime, resolve_spatial_intent_conflict_by_choice,
    resolve_spatial_intent_conflict_by_policy, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialChosenIntentAuthority, SpatialIntentCandidate, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialIntentResolutionError,
    SpatialObservedRelationFact,
};

#[test]
fn common_path_preserves_move_only_as_single_clear_intent() {
    let analysis = analyze_spatial_intent_conflict(SpatialAuthoredActKind::Move, &[]);

    assert_eq!(
        analysis.conflict_class(),
        SpatialIntentConflictClass::SingleClearIntent
    );
    assert_eq!(
        analysis.escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::MoveOnly)
    );
}

#[test]
fn grazing_contact_is_unsafe_to_assume() {
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
}

#[test]
fn overlap_preserves_blocked_future_candidates() {
    let analysis = analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialIntentConflictClass::BlockedCandidateSet
    );
    assert_eq!(
        analysis.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::MergeBoolean)
    );
}

#[test]
fn advanced_path_can_enable_higher_scope_candidates_without_hiding_conflict() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults()
            .with_merge_boolean()
            .with_subtract_boolean(),
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialIntentConflictClass::UnsafeToAssume
    );
    assert_eq!(
        analysis.escalation(),
        SpatialIntentEscalation::AskForClarification
    );
}

#[test]
fn policy_resolution_preserves_auto_resolved_choice() {
    let resolution = resolve_spatial_intent_conflict_by_policy(analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[],
    ))
    .expect("policy resolution");

    assert_eq!(
        resolution.chosen_candidate(),
        SpatialIntentCandidate::MoveOnly
    );
    assert_eq!(
        resolution.authority(),
        SpatialChosenIntentAuthority::PolicyAutoResolve
    );
}

#[test]
fn explicit_choice_can_resolve_unsafe_to_assume_conflict() {
    let resolution = resolve_spatial_intent_conflict_by_choice(
        analyze_spatial_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::GrazingContact],
        ),
        SpatialIntentCandidate::SnapFlush,
    )
    .expect("explicit choice");

    assert_eq!(
        resolution.chosen_candidate(),
        SpatialIntentCandidate::SnapFlush
    );
    assert_eq!(
        resolution.authority(),
        SpatialChosenIntentAuthority::ExplicitChoice
    );
}

#[test]
fn blocked_candidate_cannot_be_resolved_by_choice() {
    let error = resolve_spatial_intent_conflict_by_choice(
        analyze_spatial_intent_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::Overlap],
        ),
        SpatialIntentCandidate::MergeCandidate,
    )
    .expect_err("blocked candidate must fail");

    assert_eq!(
        error,
        SpatialIntentResolutionError::CandidateBlocked(
            SpatialIntentCandidate::MergeCandidate,
            SpatialBlockedCapability::MergeBoolean
        )
    );
}

#[test]
fn runtime_declaration_preserves_query_seam_and_invariant_violations() {
    let analysis = analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
    );
    let runtime = declare_spatial_arbitration_runtime(analysis.clone());
    let eligibility = runtime
        .to_query_eligibility()
        .expect("runtime request should be valid");

    assert_eq!(
        eligibility
            .request()
            .runtime_declaration()
            .expect("runtime declaration")
            .name(),
        "worth.spatial.arbitration"
    );
    assert!(!runtime.graph_composition_invariant_violations().is_empty());
    assert!(!runtime
        .graph_composition_capability_support_rows()
        .is_empty());
    assert_eq!(runtime.declaration(), &analysis);
}
