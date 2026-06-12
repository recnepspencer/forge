use crate::diagnostics::merge::BridgeMergeRecordIdentity;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::error::BridgeDeliveryErrorKind;
use crate::identity::BridgeIdentityEvidence;
use crate::routing::BridgeRouteIdentity;
use crate::source::SourceFailureClass;

use super::super::digest_basis::{
    compose_retained_causal_mapping_evidence_identity, retained_mapping_identity_digest_part,
    retained_mapping_shape_part, RetainedCausalMappingDigestArtifact,
};

pub(crate) fn source_materialization_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .source_materialization_record_for_identity(reference_identity)
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::SourceMaterializationRecord,
                &[
                    retained_mapping_identity_digest_part(record.record_identity().as_str()),
                    retained_mapping_identity_digest_part(record.source_contract_identity()),
                    retained_mapping_identity_digest_part(record.source_declaration_identity()),
                    retained_mapping_identity_digest_part(record.source_capability_digest()),
                    retained_mapping_identity_digest_part(record.adapter_capability_digest()),
                    retained_mapping_identity_digest_part(record.planned_packet_set_digest()),
                    retained_mapping_identity_digest_part(record.materialized_packet_set_digest()),
                    retained_mapping_identity_digest_part(record.digest()),
                ],
            )
        })
}

pub(crate) fn source_failure_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .source_failure_record_for_identity(reference_identity)
        .map(|record| source_failure_digest(&record))
}

pub(crate) fn source_failure_digest(
    record: &crate::source::SourceFailureRecord,
) -> BridgeIdentityEvidence {
    compose_retained_causal_mapping_evidence_identity(
        RetainedCausalMappingDigestArtifact::SourceFailureRecord,
        &[
            retained_mapping_identity_digest_part(record.failure_identity().as_str()),
            retained_mapping_identity_digest_part(record.declaration_identity().as_str()),
            retained_mapping_identity_digest_part(record.selector_identity()),
            retained_mapping_identity_digest_part(record.source_capability_digest()),
            retained_mapping_shape_part(source_failure_class_label(record.failure_class())),
            retained_mapping_shape_part(delivery_error_kind_label(record.delivery_error_kind())),
            retained_mapping_identity_digest_part(record.digest()),
        ],
    )
}

fn source_failure_class_label(value: SourceFailureClass) -> &'static str {
    match value {
        SourceFailureClass::UnsupportedSourceCapability => "unsupported-source-capability",
        SourceFailureClass::SourceContractMismatch => "source-contract-mismatch",
        SourceFailureClass::SourceContractVersionMismatch => "source-contract-version-mismatch",
        SourceFailureClass::TruthViewSelectionMismatch => "truth-view-selection-mismatch",
        SourceFailureClass::HistoricalReadUnavailable => "historical-read-unavailable",
        SourceFailureClass::BranchReadUnavailable => "branch-read-unavailable",
        SourceFailureClass::FacetReadUnavailable => "facet-read-unavailable",
        SourceFailureClass::ReplaySourceRequestCoherenceFailure => {
            "replay-source-request-coherence-failure"
        }
        SourceFailureClass::SourceMaterializationRejected => "source-materialization-rejected",
        SourceFailureClass::AdapterCapabilityDrift => "adapter-capability-drift",
        SourceFailureClass::BuilderConfigurationConflict => "builder-configuration-conflict",
    }
}

fn delivery_error_kind_label(value: BridgeDeliveryErrorKind) -> &'static str {
    match value {
        BridgeDeliveryErrorKind::SourceContractMismatch => "source-contract-mismatch",
        BridgeDeliveryErrorKind::InvalidWideningAdmission => "invalid-widening-admission",
        BridgeDeliveryErrorKind::BulkDeliveryRejected => "bulk-delivery-rejected",
        BridgeDeliveryErrorKind::HistoricalPolicyRejected => "historical-policy-rejected",
        BridgeDeliveryErrorKind::HistoricalTruthViewUnavailable => {
            "historical-truth-view-unavailable"
        }
        BridgeDeliveryErrorKind::HistoricalBranchMismatch => "historical-branch-mismatch",
        BridgeDeliveryErrorKind::HistoricalCommitMismatch => "historical-commit-mismatch",
        BridgeDeliveryErrorKind::HistoricalSelectorMissingCommit => {
            "historical-selector-missing-commit"
        }
        BridgeDeliveryErrorKind::SnapshotAcquisitionFailure => "snapshot-acquisition-failure",
        BridgeDeliveryErrorKind::SnapshotReadFailure => "snapshot-read-failure",
        BridgeDeliveryErrorKind::SnapshotReadContractViolation => {
            "snapshot-read-contract-violation"
        }
        BridgeDeliveryErrorKind::SnapshotIdentityMismatch => "snapshot-identity-mismatch",
        BridgeDeliveryErrorKind::StructuralContractMismatch => "structural-contract-mismatch",
        BridgeDeliveryErrorKind::StructuralPlanRejected => "structural-plan-rejected",
        BridgeDeliveryErrorKind::SignalSinkRejection => "signal-sink-rejection",
    }
}

pub(crate) fn structural_remap_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .structural_remap_record_for_identity(reference_identity)
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::StructuralRemapRecord,
                &[
                    retained_mapping_identity_digest_part(record.record_identity().as_str()),
                    retained_mapping_shape_part(record.schema_version()),
                    retained_mapping_identity_digest_part(record.contract().digest()),
                    retained_mapping_identity_digest_part(record.planned_packet_set().digest()),
                    retained_mapping_identity_digest_part(record.reduced_match_set().digest()),
                    retained_mapping_identity_digest_part(record.artifact().digest()),
                ],
            )
        })
}

pub(crate) fn structural_branch_comparison_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .structural_branch_comparison_record_for_identity(reference_identity)
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::StructuralBranchComparisonRecord,
                &[
                    retained_mapping_identity_digest_part(record.record_identity().as_str()),
                    retained_mapping_shape_part(record.schema_version()),
                    retained_mapping_identity_digest_part(record.contract().digest()),
                    retained_mapping_identity_digest_part(record.planned_packet_set().digest()),
                    retained_mapping_identity_digest_part(record.reduced_match_set().digest()),
                    retained_mapping_identity_digest_part(record.artifact().digest()),
                ],
            )
        })
}

pub(crate) fn stream_replay_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .stream_replay_record_for_identity(reference_identity)
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::StreamReplayRecord,
                &[
                    retained_mapping_identity_digest_part(record.replay_record_identity().as_str()),
                    retained_mapping_identity_digest_part(
                        record.consumer_contract_identity().as_str(),
                    ),
                    retained_mapping_identity_digest_part(record.stream_window_identity().as_str()),
                    retained_mapping_identity_digest_part(record.checkpoint_token_identity()),
                    retained_mapping_identity_digest_part(record.replay_basis_digest()),
                    retained_mapping_shape_part(record.protocol_semantics_version()),
                    retained_mapping_identity_digest_part(record.digest()),
                ],
            )
        })
}

pub(crate) fn continuity_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .continuity_record_for_route_identity(&BridgeRouteIdentity::new(reference_identity))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::ContinuityRecord,
                &[
                    retained_mapping_identity_digest_part(record.route_identity().as_str()),
                    retained_mapping_shape_part(record.schema_version()),
                    retained_mapping_identity_digest_part(record.continuity_request_digest()),
                    retained_mapping_identity_digest_part(record.continuity_resolution_digest()),
                    retained_mapping_identity_digest_part(
                        record.continuity_artifact_identity().as_str(),
                    ),
                    retained_mapping_identity_digest_part(
                        record.remapped_subscription_slice_identity().as_str(),
                    ),
                ],
            )
        })
}

pub(crate) fn merge_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<BridgeIdentityEvidence> {
    facade
        .merge_record_for_identity(&BridgeMergeRecordIdentity::new(reference_identity))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::MergeRecord,
                &[
                    retained_mapping_identity_digest_part(record.record_identity().as_str()),
                    retained_mapping_shape_part(record.schema_version()),
                    retained_mapping_identity_digest_part(record.contract().digest()),
                    retained_mapping_identity_digest_part(record.bundle().digest()),
                ],
            )
        })
}
