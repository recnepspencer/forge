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
    retained_mapping_external_authority_part, RetainedCausalMappingDigestArtifact,
};

pub(crate) fn route_record_digest(
    facade: &BridgeDiagnosticsFacade,
    reference_identity: &BridgeIdentityEvidence,
) -> Option<BridgeIdentityEvidence> {
    facade
        .route_record_for_route_identity(&BridgeRouteIdentity::new(reference_identity.as_str()))
        .map(|record| {
            compose_retained_causal_mapping_evidence_identity(
                RetainedCausalMappingDigestArtifact::RouteRecord,
                &[
                    retained_mapping_bridge_identity_part(record.route_identity()),
                    retained_mapping_bridge_identity_part(record.invalidation_identity()),
                    retained_mapping_bridge_identity_part(record.source_commit()),
                    retained_mapping_external_authority_part(record.planning_summary_digest()),
                    retained_mapping_external_authority_part(record.lowering_summary_digest()),
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
                &[
                    retained_mapping_bridge_identity_part(record.record_identity()),
                    retained_mapping_external_authority_part(record.preview_session_identity()),
                    retained_mapping_external_authority_part(record.preview_declaration_digest()),
                    retained_mapping_external_authority_part(record.branch_binding_digest()),
                    retained_mapping_external_authority_part(record.digest()),
                ],
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
                    retained_mapping_external_authority_part(record.preview_session_identity()),
                    retained_mapping_bridge_identity_part(record.preview_execution_record_identity()),
                    retained_mapping_external_authority_part(record.residue_report().digest()),
                    retained_mapping_external_authority_part(record.digest()),
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
                    retained_mapping_external_authority_part(record.preview_session_identity()),
                    retained_mapping_bridge_identity_part(record.preview_execution_record_identity()),
                    retained_mapping_external_authority_part(record.promotion_proof_digest()),
                    retained_mapping_external_authority_part(
                        record.authoritative_commit_boundary_digest(),
                    ),
                    retained_mapping_external_authority_part(record.authoritative_artifact_digest()),
                    retained_mapping_external_authority_part(record.digest()),
                ],
            )
        })
}
