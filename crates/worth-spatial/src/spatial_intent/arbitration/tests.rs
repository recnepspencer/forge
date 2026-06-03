use super::{
    analyze_spatial_intent_conflict, analyze_spatial_intent_conflict_with_capabilities,
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    resolve_spatial_intent_conflict_by_choice, resolve_spatial_intent_conflict_by_policy,
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialChosenIntentAuthority,
    SpatialIdentityContinuityClass, SpatialIntentCandidate, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentEscalation, SpatialIntentPreviewCommitDisposition,
    SpatialIntentPreviewWarning, SpatialIntentResolutionError, SpatialObservedRelationFact,
};
use crate::spatial_intent::policy::{SpatialIntentPolicyProfile, SpatialPreviewRichness};

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
fn arbitration_declaration_exposes_query_seam_and_invariant_violations() {
    let analysis = analyze_spatial_intent_conflict(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
    );
    let eligibility = analysis
        .to_query_eligibility()
        .expect("declaration request should be valid");

    assert_eq!(
        eligibility
            .request()
            .runtime_declaration()
            .expect("runtime declaration")
            .name(),
        "worth.spatial.arbitration"
    );
    assert!(!analysis.graph_composition_invariant_violations().is_empty());
    assert!(!analysis
        .graph_composition_capability_support_rows()
        .is_empty());
    assert!(analysis.admit_query_intent().is_ok());
}

#[test]
fn preview_semantics_live_on_arbitration_declaration() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialIntentPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(
        analysis.preview_commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        analysis.preview_richness(),
        SpatialPreviewRichness::Standard
    );
    assert!(analysis.preview_warnings().contains(
        &SpatialIntentPreviewWarning::ProfileDrivenAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    ));
}

#[test]
fn high_fidelity_preview_warning_lives_on_arbitration_declaration() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Align,
        &[SpatialObservedRelationFact::FrameAligned],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::high_fidelity_preview(),
    );

    assert_eq!(
        analysis.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(analysis
        .preview_warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}

#[test]
fn continuity_semantics_live_on_analysis_and_resolution() {
    let policy_analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::aggressive_snap(),
    );
    let merged_analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults().with_merge_boolean(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );
    let merged_resolution = resolve_spatial_intent_conflict_by_choice(
        merged_analysis,
        SpatialIntentCandidate::MergeCandidate,
    )
    .expect("merge choice");
    let blocked_analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
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
