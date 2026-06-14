use crate::diagnostics::history::BridgeHistoricalEvaluationRecordIdentity;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::identity::BridgeIdentityEvidence;
use crate::routing::BridgeRouteIdentity;
use crate::speculation::{
    BridgePreviewDiscardRecordIdentity, BridgePreviewPromotionRecordIdentity,
    PreviewExecutionRecordIdentity,
};

use super::super::digest_basis::{
    compose_retained_causal_mapping_evidence_identity, retained_mapping_bridge_identity_part,
    RetainedCausalMappingDigestArtifact,
};

pub(crate) fn route_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .route_record_for_route_identity(&BridgeRouteIdentity::from_reference_evidence(
            reference_identity,
        ))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::RouteRecord,
                &[
                    retained_mapping_bridge_identity_part(record.route_identity()),
                    retained_mapping_bridge_identity_part(record.invalidation_identity()),
                    retained_mapping_bridge_identity_part(record.source_commit()),
                ],
            )
        })
}

pub(crate) fn historical_evaluation_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .historical_record_for_record_identity(&BridgeHistoricalEvaluationRecordIdentity::new(
            reference_identity.as_str(),
        ))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::HistoricalEvaluationRecord,
                &[
                    retained_mapping_bridge_identity_part(record.record_identity()),
                    retained_mapping_bridge_identity_part(record.decision_log().decision_log_identity()),
                    retained_mapping_bridge_identity_part(record.decision_log().snapshot_identity()),
                ],
            )
        })
}

pub(crate) fn preview_execution_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .preview_execution_record_for_identity(&PreviewExecutionRecordIdentity::new(
            reference_identity.as_str(),
        ))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::PreviewExecutionRecord,
                &[retained_mapping_bridge_identity_part(record.record_identity())],
            )
        })
}

pub(crate) fn preview_discard_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .preview_discard_record_for_identity(&BridgePreviewDiscardRecordIdentity::new(
            reference_identity.as_str(),
        ))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::PreviewDiscardRecord,
                &[
                    retained_mapping_bridge_identity_part(record.record_identity()),
                    retained_mapping_bridge_identity_part(record.preview_execution_record_identity()),
                ],
            )
        })
}

pub(crate) fn preview_promotion_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .preview_promotion_record_for_identity(&BridgePreviewPromotionRecordIdentity::new(
            reference_identity.as_str(),
        ))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::PreviewPromotionRecord,
                &[
                    retained_mapping_bridge_identity_part(record.record_identity()),
                    retained_mapping_bridge_identity_part(record.preview_execution_record_identity()),
                ],
            )
        })
}
