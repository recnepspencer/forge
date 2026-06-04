use crate::facade::{
    BridgeAggregateMutationEvidenceDigest, BridgeAuthorityEvidenceDeferredBoundary,
    BridgeAuthorityEvidenceReadyCapability, BridgeAuthorityEvidenceVerificationGate,
    BridgeMutationEvidenceCarryForwardSection, BridgeMutationEvidenceContinuityFamily,
    BridgeMutationEvidenceExistingTruthBindingFamily, BridgeMutationEvidenceNamingFamily,
    BridgeMutationEvidenceSymbolicTargetReferenceFamily, RuntimeBridge,
};

#[test]
fn bridge_public_authoritative_mutation_evidence_support_freezes_admitted_families() {
    let support = RuntimeBridge::public_authoritative_mutation_evidence_support();

    assert!(support
        .carry_forward_sections()
        .iter()
        .any(|item| item == &BridgeMutationEvidenceCarryForwardSection::ExistingTruthBinding));
    assert_eq!(
        support.existing_truth_binding_families(),
        &[
            BridgeMutationEvidenceExistingTruthBindingFamily::DirectEntityIdentity,
            BridgeMutationEvidenceExistingTruthBindingFamily::DirectRelationIdentity,
        ]
    );
    assert_eq!(
        support.symbolic_target_reference_families(),
        &[BridgeMutationEvidenceSymbolicTargetReferenceFamily::SameBatchDeclaredTarget]
    );
    assert!(support
        .naming_mutation_families()
        .iter()
        .any(|item| item == &BridgeMutationEvidenceNamingFamily::RebindTarget));
    assert!(support
        .continuity_mutation_families()
        .iter()
        .any(|item| item == &BridgeMutationEvidenceContinuityFamily::SplitExistingTarget));
    assert!(support
        .aggregate_evidence_digests()
        .iter()
        .any(|item| item == &BridgeAggregateMutationEvidenceDigest::ContinuityMutation));
    assert!(!support.support_digest().is_empty());
}

#[test]
fn bridge_public_authoritative_mutation_evidence_closeout_answers_carry_forward_contract() {
    let support = RuntimeBridge::public_authoritative_mutation_evidence_support();
    let closeout = RuntimeBridge::public_authoritative_mutation_evidence_closeout();

    assert_eq!(closeout.support_digest(), support.support_digest());
    assert_eq!(
        closeout.ready_capabilities(),
        &[
            BridgeAuthorityEvidenceReadyCapability::QueryFacingContractCarriesTargetCausalityProvenanceNamingContinuity,
            BridgeAuthorityEvidenceReadyCapability::BatchSessionBundlesPreserveAggregateEvidenceDigests,
            BridgeAuthorityEvidenceReadyCapability::ReplaySafeRequestReceiptDigestsCarriedForward,
        ]
    );
    assert_eq!(
        closeout.deferred_boundaries(),
        &[
            BridgeAuthorityEvidenceDeferredBoundary::DurableRestartTemporalAsyncAuthorityMutationSemantics,
            BridgeAuthorityEvidenceDeferredBoundary::UnsupportedMutationFamiliesRemainFailClosed,
            BridgeAuthorityEvidenceDeferredBoundary::DownstreamDomainsCannotReconstructDroppedCausalityProvenance,
        ]
    );
    assert!(closeout
        .verification_gates()
        .iter()
        .any(|gate| gate
            == &BridgeAuthorityEvidenceVerificationGate::FocusedRuntimeBridgeWritebackTests));
    assert!(!closeout.closeout_digest().is_empty());
}
