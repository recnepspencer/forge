use crate::diagnostics::history::BridgeHistoricalEvaluationRecordIdentity;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::routing::BridgeRouteIdentity;
use crate::speculation::{
    BridgePreviewDiscardRecordIdentity, BridgePreviewPromotionRecordIdentity,
    PreviewExecutionRecordIdentity,
};

use super::super::digest_basis::{retained_mapping_digest, RetainedCausalMappingDigestArtifact};

pub(crate) fn route_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .route_record_for_route_identity(&BridgeRouteIdentity::new(reference_identity))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::RouteRecord,
                &[
                    record.route_identity().as_str(),
                    record.invalidation_identity().as_str(),
                    record.source_commit().as_str(),
                    record.planning_summary_digest(),
                    record.lowering_summary_digest(),
                ],
            )
        })
}

pub(crate) fn historical_evaluation_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .historical_record_for_record_identity(&BridgeHistoricalEvaluationRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::HistoricalEvaluationRecord,
                &[
                    record.record_identity().as_str(),
                    record.decision_log().decision_log_identity().as_str(),
                    record.decision_log().snapshot_identity().as_str(),
                ],
            )
        })
}

pub(crate) fn preview_execution_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .preview_execution_record_for_identity(&PreviewExecutionRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::PreviewExecutionRecord,
                &[
                    record.record_identity().as_str(),
                    record.preview_session_identity(),
                    record.preview_declaration_digest(),
                    record.branch_binding_digest(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn preview_discard_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .preview_discard_record_for_identity(&BridgePreviewDiscardRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::PreviewDiscardRecord,
                &[
                    record.record_identity().as_str(),
                    record.preview_session_identity(),
                    record.preview_execution_record_identity().as_str(),
                    record.residue_report().digest(),
                    record.digest(),
                ],
            )
        })
}

pub(crate) fn preview_promotion_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &str,
) -> Option<String> {
    facade
        .preview_promotion_record_for_identity(&BridgePreviewPromotionRecordIdentity::new(
            reference_identity,
        ))
        .map(|record| {
            retained_mapping_digest(
                RetainedCausalMappingDigestArtifact::PreviewPromotionRecord,
                &[
                    record.record_identity().as_str(),
                    record.preview_session_identity(),
                    record.preview_execution_record_identity().as_str(),
                    record.promotion_proof_digest(),
                    record.authoritative_commit_boundary_digest(),
                    record.authoritative_artifact_digest(),
                    record.digest(),
                ],
            )
        })
}
