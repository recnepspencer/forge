use crate::diagnostics::merge::BridgeMergeRecordIdentity;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::routing::BridgeRouteIdentity;

use super::super::digest_basis::{retained_mapping_digest, RetainedCausalMappingDigestArtifact};

pub(crate) fn source_materialization_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .source_materialization_record_for_identity(reference_identity)
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::SourceMaterializationRecord,
                &[
                    record.record_identity().as_str(),
                    record.source_contract_identity(),
                    record.source_declaration_identity(),
                    record.source_capability_digest(),
                    record.adapter_capability_digest(),
                    record.planned_packet_set_digest(),
                    record.materialized_packet_set_digest(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn source_failure_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .source_failure_record_for_identity(reference_identity)
        .map(|record| {
            let failure_class = format!("{:?}", record.failure_class());
            let delivery_error_kind = format!("{:?}", record.delivery_error_kind());
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::SourceFailureRecord,
                &[
                    record.failure_identity().as_str(),
                    record.declaration_identity().as_str(),
                    record.selector_identity(),
                    record.source_capability_digest(),
                    failure_class.as_str(),
                    delivery_error_kind.as_str(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn structural_remap_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .structural_remap_record_for_identity(reference_identity)
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::StructuralRemapRecord,
                &[
                    record.record_identity().as_str(),
                    record.schema_version(),
                    record.contract().digest(),
                    record.planned_packet_set().digest(),
                    record.reduced_match_set().digest(),
                    record.artifact().digest(),
                ],
            )
        })
}

pub(crate) fn structural_branch_comparison_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .structural_branch_comparison_record_for_identity(reference_identity)
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::StructuralBranchComparisonRecord,
                &[
                    record.record_identity().as_str(),
                    record.schema_version(),
                    record.contract().digest(),
                    record.planned_packet_set().digest(),
                    record.reduced_match_set().digest(),
                    record.artifact().digest(),
                ],
            )
        })
}

pub(crate) fn stream_replay_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .stream_replay_record_for_identity(reference_identity)
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::StreamReplayRecord,
                &[
                    record.replay_record_identity().as_str(),
                    record.consumer_contract_identity().as_str(),
                    record.stream_window_identity().as_str(),
                    record.checkpoint_token_identity(),
                    record.replay_basis_digest(),
                    record.protocol_semantics_version(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn continuity_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .continuity_record_for_route_identity(&BridgeRouteIdentity::new(reference_identity))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::ContinuityRecord,
                &[
                    record.route_identity().as_str(),
                    record.schema_version(),
                    record.continuity_request_digest(),
                    record.continuity_resolution_digest(),
                    record.continuity_artifact_identity().as_str(),
                    record.remapped_subscription_slice_identity().as_str(),
                ],
            )
        })
}

pub(crate) fn merge_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .merge_record_for_identity(&BridgeMergeRecordIdentity::new(reference_identity))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::MergeRecord,
                &[
                    record.record_identity().as_str(),
                    record.schema_version(),
                    record.contract().digest(),
                    record.bundle().digest(),
                ],
            )
        })
}
