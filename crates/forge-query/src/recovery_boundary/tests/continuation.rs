use crate::application::ForgeQueryDeclarationFamilyMarker;
use forge_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::binding_pipeline::{
    ForgeQueryBindingLinkedArtifacts, ForgeQueryBindingRequestDescriptor,
    ForgeQueryBindingWitnessCheck,
};
use crate::continuation_pipeline::{
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationOutcome,
    ForgeQueryPreparedContinuationTranscript,
};
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_prepared_continuation_checked,
    forge_query_recovery_brief_from_prepared_continuation_proof, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture,
    ForgeQueryRecoveryEvidenceStrength, ForgeQueryRecoverySourceFamily,
};

use super::support::{
    standard_aspect_contract, RecoveryDomain, RecoveryInput, RequiredIntentRouteFamily,
};

fn prepared_stale_checked(
) -> ForgeQueryPreparedContinuationChecked<RecoveryDomain, RecoveryInput<RequiredIntentRouteFamily>>
{
    ForgeQueryPreparedContinuationChecked::new(
        ForgeQueryPreparedContinuationOutcome::Stale("retained continuation is stale".to_string()),
        "prepared-stale".to_string(),
        ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-stale"),
    )
}

fn prepared_stale_proof() -> ForgeQueryPreparedContinuationTranscript<
    RecoveryDomain,
    RecoveryInput<RequiredIntentRouteFamily>,
> {
    ForgeQueryPreparedContinuationTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(
            RequiredIntentRouteFamily::semantic_family_key(),
            "prepare_continuation",
            standard_aspect_contract(),
        ),
        ForgeQueryPreparedContinuationOutcome::Stale("retained continuation is stale".to_string()),
        vec![ForgeQueryBindingWitnessCheck::failed(
            "freshness",
            "basis is stale",
        )],
        Vec::new(),
        "prepared-stale".to_string(),
        ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-stale"),
    )
}

#[test]
fn prepared_continuation_checked_and_proof_preserve_stale_aspect_native_recovery() {
    let checked =
        forge_query_recovery_brief_from_prepared_continuation_checked(prepared_stale_checked())
            .expect("checked stale continuation should recover");
    let proof = forge_query_recovery_brief_from_prepared_continuation_proof(prepared_stale_proof())
        .expect("proof stale continuation should recover");

    assert_eq!(
        checked.source_family(),
        ForgeQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        proof.source_family(),
        ForgeQueryRecoverySourceFamily::Continuation
    );
    assert_eq!(
        checked.basis_posture(),
        ForgeQueryRecoveryBasisPosture::StaleBasis
    );
    assert_eq!(
        proof.basis_posture(),
        ForgeQueryRecoveryBasisPosture::StaleBasis
    );
    assert_eq!(
        checked.aspect_posture(),
        ForgeQueryRecoveryAspectPosture::AspectSensitiveReadmission
    );
    assert_eq!(
        proof.aspect_posture(),
        ForgeQueryRecoveryAspectPosture::AspectSensitiveReadmission
    );
    assert_eq!(
        checked.recommended_action(),
        ForgeQueryRecoveryAction::RefreshBasis
    );
    assert_eq!(
        proof.recommended_action(),
        ForgeQueryRecoveryAction::RefreshBasis
    );
    assert_eq!(
        checked.evidence_strength(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained
    );
    assert_eq!(
        proof.evidence_strength(),
        ForgeQueryRecoveryEvidenceStrength::ProofRetained
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
