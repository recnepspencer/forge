use crate::facade::{
    BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner, BridgeCausalEvidenceReference,
    BridgeCausalEvidenceReferenceIdentity, BridgePreviewDiscardRecord,
    BridgePreviewExecutionRecord, BridgePreviewPromotionRecord,
    BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity, BridgeRequestKind,
    BridgeRouteResultSummary, BridgeSignalBranchIdentity, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeTruthViewSelector, TruthBranchIdentity,
    TruthSnapshotIdentity,
};

pub(super) fn preview_declaration(
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    binding_identity: BridgeSpeculativeBranchBindingIdentity,
    truth_branch_identity: TruthBranchIdentity,
    signal_branch_identity: BridgeSignalBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        declaration_identity,
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            binding_identity,
            truth_branch_identity.clone(),
            signal_branch_identity,
        ),
        BridgePreviewSessionBasis::new(
            BridgeTruthViewSelector::branch_snapshot(truth_branch_identity, snapshot_identity),
            BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        ),
    )
}

pub(super) fn bridge_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    let family = identity.family();
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

pub(super) fn query_observation_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
    )
    .expect("query observation reference should be valid")
}

pub(super) fn missing_bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    bridge_reference(
        BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
            family,
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(identity),
        )
        .expect("bridge reference identity should be valid"),
    )
}

pub(super) fn bridge_route_reference(
    route_summary: &BridgeRouteResultSummary,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgeRoute,
        route_summary.route_identity().as_str(),
    )
}

pub(super) fn bridge_preview_execution_reference(
    record: &BridgePreviewExecutionRecord,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgePreviewExecution,
        record.record_identity().as_str(),
    )
}

pub(super) fn bridge_preview_discard_reference(
    record: &BridgePreviewDiscardRecord,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgePreviewDiscard,
        record.record_identity().as_str(),
    )
}

pub(super) fn bridge_preview_promotion_reference(
    record: &BridgePreviewPromotionRecord,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgePreviewPromotion,
        record.record_identity().as_str(),
    )
}
