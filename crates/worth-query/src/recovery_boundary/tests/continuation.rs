use crate::application::WorthQueryDeclarationFamilyMarker;
use worth_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::binding_pipeline::{
    WorthQueryBindingLinkedArtifacts, WorthQueryBindingRequestDescriptor,
    WorthQueryBindingWitnessCheck,
};
use crate::continuation_pipeline::{
    WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome,
    WorthQueryContinuationExecutionTranscript, WorthQueryPreparedContinuationChecked,
    WorthQueryPreparedContinuationOutcome, WorthQueryPreparedContinuationTranscript,
};
use crate::recovery_boundary::{
    worth_query_recovery_brief_from_prepared_continuation_checked,
    worth_query_recovery_brief_from_prepared_continuation_proof, WorthQueryRecoveryAction,
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryBasisPosture,
    WorthQueryRecoveryEvidenceStrength, WorthQueryRecoverySourceFamily, WorthQueryRecoveryStopKind,
};

use super::support::{
    standard_aspect_contract, RecoveryDomain, RecoveryInput, RequiredIntentRouteFamily,
};

fn prepared_stale_checked(
) -> WorthQueryPreparedContinuationChecked<RecoveryDomain, RecoveryInput<RequiredIntentRouteFamily>>
{
    WorthQueryPreparedContinuationChecked::new(
        WorthQueryPreparedContinuationOutcome::Stale("retained continuation is stale".to_string()),
        "prepared-stale".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-stale"),
    )
}

fn prepared_stale_proof() -> WorthQueryPreparedContinuationTranscript<
    RecoveryDomain,
    RecoveryInput<RequiredIntentRouteFamily>,
> {
    WorthQueryPreparedContinuationTranscript::new(
        WorthQueryBindingRequestDescriptor::new(
            RequiredIntentRouteFamily::semantic_family_key(),
            "prepare_continuation",
            standard_aspect_contract(),
        ),
        WorthQueryPreparedContinuationOutcome::Stale("retained continuation is stale".to_string()),
        vec![WorthQueryBindingWitnessCheck::failed(
            "freshness",
            "basis is stale",
        )],
        Vec::new(),
        "prepared-stale".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-stale"),
    )
}

fn execution_drift_proof(
    outcome: WorthQueryContinuationExecutionOutcome<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    >,
    execution_digest: &str,
) -> WorthQueryContinuationExecutionTranscript<
    RecoveryDomain,
    RecoveryInput<RequiredIntentRouteFamily>,
> {
    WorthQueryContinuationExecutionTranscript::new(
        WorthQueryBindingRequestDescriptor::new(
            RequiredIntentRouteFamily::semantic_family_key(),
            "execute_prepared_continuation",
            standard_aspect_contract(),
        ),
        outcome,
        vec![WorthQueryBindingWitnessCheck::failed(
            "typed_drift",
            "typed continuation drift stopped execution",
        )],
        execution_digest.to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-execution-drift"),
    )
}

#[test]
fn prepared_continuation_checked_and_proof_preserve_stale_aspect_native_recovery() {
    let checked =
        worth_query_recovery_brief_from_prepared_continuation_checked(prepared_stale_checked())
            .expect("checked stale continuation should recover");
    let proof = worth_query_recovery_brief_from_prepared_continuation_proof(prepared_stale_proof())
        .expect("proof stale continuation should recover");

    assert_eq!(
        checked.source_family(),
        WorthQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        proof.source_family(),
        WorthQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        checked.basis_posture(),
        WorthQueryRecoveryBasisPosture::StaleBasis
    );
    assert_eq!(
        proof.basis_posture(),
        WorthQueryRecoveryBasisPosture::StaleBasis
    );
    assert_eq!(
        checked.aspect_posture(),
        WorthQueryRecoveryAspectPosture::AspectSensitiveReadmission
    );
    assert_eq!(
        proof.aspect_posture(),
        WorthQueryRecoveryAspectPosture::AspectSensitiveReadmission
    );
    assert_eq!(
        checked.recommended_action(),
        WorthQueryRecoveryAction::RefreshBasis
    );
    assert_eq!(
        proof.recommended_action(),
        WorthQueryRecoveryAction::RefreshBasis
    );
    assert_eq!(
        checked.evidence_strength(),
        WorthQueryRecoveryEvidenceStrength::CheckedRetained
    );
    assert_eq!(
        proof.evidence_strength(),
        WorthQueryRecoveryEvidenceStrength::ProofRetained
    );
    assert_eq!(
        proof.explanation().support_truth_kind(),
        Some(FoundationalBoundaryEvidenceSupportTruthKind::StaleBasisDisclosure)
    );
    assert_eq!(
        proof.explanation().basis_disclosure(),
        Some(FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis)
    );
    assert_eq!(
        proof.explanation().degraded_recovery_posture(),
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::ReplayReconstructed)
    );
}

#[test]
fn execution_async_request_drift_maps_to_rebind_recovery() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::AsyncRequestDrift("async request drifted".to_string()),
        "execution-async-drift".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-async"),
    );

    let brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("async request drift should yield a recovery brief");

    assert_eq!(
        brief.stop_kind(),
        WorthQueryRecoveryStopKind::AsyncRequestDrift
    );
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::RebindContext
    );
}

#[test]
fn execution_remask_drift_maps_to_support_recovery() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::RemaskDrift("remask drifted".to_string()),
        "execution-remask-drift".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-remask"),
    );

    let brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("remask drift should yield a recovery brief");

    assert_eq!(brief.stop_kind(), WorthQueryRecoveryStopKind::RemaskDrift);
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::CheckSupport
    );
}

#[test]
fn execution_replay_drift_maps_to_refresh_basis_recovery() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::ReplayDrift("replay drifted".to_string()),
        "execution-replay-drift".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-replay"),
    );

    let brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("replay drift should yield a recovery brief");

    assert_eq!(brief.stop_kind(), WorthQueryRecoveryStopKind::ReplayDrift);
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::RefreshBasis
    );
}

#[test]
fn execution_preview_crossed_residue_maps_to_explicit_handoff_recovery() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::PreviewCrossedResidue("preview residue crossed".to_string()),
        "execution-preview-residue".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-preview"),
    );

    let brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("preview-crossed residue should yield a recovery brief");

    assert_eq!(
        brief.stop_kind(),
        WorthQueryRecoveryStopKind::PreviewCrossedResidue
    );
    assert_eq!(
        brief.recommended_action(),
        WorthQueryRecoveryAction::UseExplicitHandoff
    );
}

#[test]
fn execution_replay_drift_checked_and_proof_recovery_match() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::ReplayDrift("replay drifted".to_string()),
        "execution-replay-drift".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-replay"),
    );
    let proof = execution_drift_proof(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::ReplayDrift("replay drifted".to_string()),
        "execution-replay-drift",
    );

    let checked_brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("checked replay drift should yield a recovery brief");
    let proof_brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_proof(
            proof,
        )
        .expect("proof replay drift should yield a recovery brief");

    assert_eq!(checked_brief.stop_kind(), proof_brief.stop_kind());
    assert_eq!(
        checked_brief.recommended_action(),
        proof_brief.recommended_action()
    );
    assert_eq!(
        checked_brief.stop_kind(),
        WorthQueryRecoveryStopKind::ReplayDrift
    );
}

#[test]
fn execution_preview_crossed_residue_checked_and_proof_recovery_match() {
    let checked: WorthQueryContinuationExecutionChecked<
        RecoveryDomain,
        RecoveryInput<RequiredIntentRouteFamily>,
    > = WorthQueryContinuationExecutionChecked::new(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::PreviewCrossedResidue("preview residue crossed".to_string()),
        "execution-preview-residue".to_string(),
        WorthQueryBindingLinkedArtifacts::new().with_envelope_digest("env-preview"),
    );
    let proof = execution_drift_proof(
        WorthQueryContinuationExecutionOutcome::<
            RecoveryDomain,
            RecoveryInput<RequiredIntentRouteFamily>,
        >::PreviewCrossedResidue("preview residue crossed".to_string()),
        "execution-preview-residue",
    );

    let checked_brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_checked(
            checked,
        )
        .expect("checked preview residue should yield a recovery brief");
    let proof_brief =
        crate::recovery_boundary::worth_query_recovery_brief_from_continuation_execution_proof(
            proof,
        )
        .expect("proof preview residue should yield a recovery brief");

    assert_eq!(checked_brief.stop_kind(), proof_brief.stop_kind());
    assert_eq!(
        checked_brief.recommended_action(),
        proof_brief.recommended_action()
    );
    assert_eq!(
        checked_brief.stop_kind(),
        WorthQueryRecoveryStopKind::PreviewCrossedResidue
    );
}
