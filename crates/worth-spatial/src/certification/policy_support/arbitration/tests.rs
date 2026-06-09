use super::{
    analyze_spatial_arbitration_conflict, analyze_spatial_arbitration_conflict_with_capabilities,
    analyze_spatial_arbitration_conflict_with_capabilities_and_profile,
    resolve_spatial_arbitration_conflict_by_choice, resolve_spatial_arbitration_conflict_by_policy,
    SpatialArbitrationCandidate, SpatialArbitrationCapabilitySet, SpatialArbitrationConflictClass,
    SpatialArbitrationEscalation, SpatialArbitrationPreviewCommitDisposition,
    SpatialArbitrationPreviewWarning, SpatialArbitrationResolutionError, SpatialAuthoredActKind,
    SpatialBlockedCapability, SpatialChosenArbitrationAuthority, SpatialIdentityContinuityClass,
    SpatialObservedRelationFact,
};
use crate::certification::policy_support::{
    SpatialArbitrationPolicyProfile, SpatialPreviewRichness,
};

#[test]
fn common_path_preserves_move_only_as_single_clear_intent() {
    let analysis = analyze_spatial_arbitration_conflict(SpatialAuthoredActKind::Move, &[]);

    assert_eq!(
        analysis.conflict_class(),
        SpatialArbitrationConflictClass::SingleClearIntent
    );
    assert_eq!(
        analysis.escalation(),
        SpatialArbitrationEscalation::AutoResolve(SpatialArbitrationCandidate::MoveOnly)
    );
}

#[test]
fn grazing_contact_is_unsafe_to_assume() {
    let analysis = analyze_spatial_arbitration_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialArbitrationConflictClass::UnsafeToAssume
    );
    assert_eq!(
        analysis.escalation(),
        SpatialArbitrationEscalation::AskForClarification
    );
}

#[test]
fn overlap_preserves_blocked_future_candidates() {
    let analysis = analyze_spatial_arbitration_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialArbitrationConflictClass::BlockedCandidateSet
    );
    assert_eq!(
        analysis.escalation(),
        SpatialArbitrationEscalation::BlockedByMissingCapability(
            SpatialBlockedCapability::MergeBoolean
        )
    );
}

#[test]
fn advanced_path_can_enable_higher_scope_candidates_without_hiding_conflict() {
    let analysis = analyze_spatial_arbitration_conflict_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialArbitrationCapabilitySet::blocked_defaults()
            .with_merge_boolean()
            .with_subtract_boolean(),
    );

    assert_eq!(
        analysis.conflict_class(),
        SpatialArbitrationConflictClass::UnsafeToAssume
    );
    assert_eq!(
        analysis.escalation(),
        SpatialArbitrationEscalation::AskForClarification
    );
}

#[test]
fn policy_resolution_preserves_auto_resolved_choice() {
    let resolution = resolve_spatial_arbitration_conflict_by_policy(
        analyze_spatial_arbitration_conflict(SpatialAuthoredActKind::Move, &[]),
    )
    .expect("policy resolution");

    assert_eq!(
        resolution.chosen_candidate(),
        SpatialArbitrationCandidate::MoveOnly
    );
    assert_eq!(
        resolution.authority(),
        SpatialChosenArbitrationAuthority::PolicyAutoResolve
    );
}

#[test]
fn explicit_choice_can_resolve_unsafe_to_assume_conflict() {
    let resolution = resolve_spatial_arbitration_conflict_by_choice(
        analyze_spatial_arbitration_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::GrazingContact],
        ),
        SpatialArbitrationCandidate::SnapFlush,
    )
    .expect("explicit choice");

    assert_eq!(
        resolution.chosen_candidate(),
        SpatialArbitrationCandidate::SnapFlush
    );
    assert_eq!(
        resolution.authority(),
        SpatialChosenArbitrationAuthority::ExplicitChoice
    );
}

#[test]
fn blocked_candidate_cannot_be_resolved_by_choice() {
    let error = resolve_spatial_arbitration_conflict_by_choice(
        analyze_spatial_arbitration_conflict(
            SpatialAuthoredActKind::Move,
            &[SpatialObservedRelationFact::Overlap],
        ),
        SpatialArbitrationCandidate::MergeCandidate,
    )
    .expect_err("blocked candidate must fail");

    assert_eq!(
        error,
        SpatialArbitrationResolutionError::CandidateBlocked(
            SpatialArbitrationCandidate::MergeCandidate,
            SpatialBlockedCapability::MergeBoolean
        )
    );
}

#[test]
fn arbitration_declaration_exposes_support_and_invariant_truth_without_local_runtime_handoff() {
    let analysis = analyze_spatial_arbitration_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
    );
    assert!(analysis.query_support_matrix().rows().len() > 0);
    assert!(analysis.query_support_traceability_report().rows().len() > 0);
    assert!(!analysis.graph_composition_invariant_violations().is_empty());
    assert!(!analysis
        .graph_composition_capability_support_rows()
        .is_empty());
}

#[test]
fn preview_semantics_live_on_arbitration_declaration() {
    let analysis = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialArbitrationCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialArbitrationPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(
        analysis.preview_commit_disposition(),
        SpatialArbitrationPreviewCommitDisposition::WouldAutoResolve(
            SpatialArbitrationCandidate::AttachRelationally
        )
    );
    assert_eq!(
        analysis.preview_richness(),
        SpatialPreviewRichness::Standard
    );
    assert!(analysis.preview_warnings().contains(
        &SpatialArbitrationPreviewWarning::ProfileDrivenAutoResolve(
            SpatialArbitrationCandidate::AttachRelationally
        )
    ));
}

#[test]
fn high_fidelity_preview_warning_lives_on_arbitration_declaration() {
    let analysis = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Align,
        &[SpatialObservedRelationFact::FrameAligned],
        SpatialArbitrationCapabilitySet::blocked_defaults(),
        SpatialArbitrationPolicyProfile::high_fidelity_preview(),
    );

    assert_eq!(
        analysis.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(analysis
        .preview_warnings()
        .contains(&SpatialArbitrationPreviewWarning::HighFidelityPreview));
}

#[test]
fn continuity_semantics_live_on_analysis_and_resolution() {
    let policy_analysis = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialArbitrationCapabilitySet::blocked_defaults(),
        SpatialArbitrationPolicyProfile::aggressive_snap(),
    );
    let merged_analysis = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialArbitrationCapabilitySet::blocked_defaults().with_merge_boolean(),
        SpatialArbitrationPolicyProfile::conservative_exact_modeling(),
    );
    let merged_resolution = resolve_spatial_arbitration_conflict_by_choice(
        merged_analysis,
        SpatialArbitrationCandidate::MergeCandidate,
    )
    .expect("merge choice");
    let blocked_analysis = analyze_spatial_arbitration_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialArbitrationCapabilitySet::blocked_defaults(),
        SpatialArbitrationPolicyProfile::conservative_exact_modeling(),
    );

    assert_eq!(
        policy_analysis
            .identity_continuity_assessment()
            .continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        merged_resolution
            .identity_continuity_assessment()
            .continuity_class(),
        SpatialIdentityContinuityClass::IdentityMerged
    );
    assert_eq!(
        blocked_analysis
            .identity_continuity_assessment()
            .continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
}
