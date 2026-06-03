use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReferenceIdentity, BridgeCausalInspectionAdmissionSummary, BridgeRoute,
    TruthCommitIdentity,
};

use super::super::super::materialization::{bridge_reference, external_reference, query_reference};
use crate::runtime::AdmittedCausalInspection;

pub(super) fn bridge_request_with_lower_runtime_slot_references(
    admitted: &AdmittedCausalInspection,
    routed: &BridgeRoute,
    retained_evidence: &super::RetainedLowerRuntimeSlotEvidence,
    commit_identity: &TruthCommitIdentity,
) -> BridgeCausalEnvelopeAssemblyRequest {
    let summary = BridgeCausalInspectionAdmissionSummary::admitted(
        admitted.admitted_inspection_digest(),
        admitted.subject().anchor_digest(),
    )
    .expect("query admission summary should be valid");
    BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    admitted.subject().query_observation_digest(),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().as_str(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
                    retained_evidence
                        .historical_evaluation_record_identity
                        .as_str(),
                )
                .expect("historical evaluation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewExecution,
                    retained_evidence.preview_execution_record_identity.as_str(),
                )
                .expect("preview execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewDiscard,
                    retained_evidence.preview_discard_record_identity.as_str(),
                )
                .expect("preview discard reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeSourceMaterialization,
                    retained_evidence
                        .source_materialization_record_identity
                        .as_str(),
                )
                .expect("source materialization reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStructuralRemap,
                    retained_evidence.structural_remap_record_identity.as_str(),
                )
                .expect("structural remap reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStreamReplay,
                    retained_evidence.stream_replay_record_identity.as_str(),
                )
                .expect("stream replay reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackAdmission,
                    retained_evidence
                        .writeback_admission_record_identity
                        .as_str(),
                )
                .expect("writeback admission reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope,
                    retained_evidence
                        .writeback_mapper_envelope_identity
                        .as_str(),
                )
                .expect("writeback mapper envelope reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput,
                    retained_evidence
                        .writeback_mapped_family_input_identity
                        .as_str(),
                )
                .expect("writeback mapped input reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapper,
                    retained_evidence.writeback_mapper_record_identity.as_str(),
                )
                .expect("writeback mapper reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackExecution,
                    retained_evidence
                        .writeback_execution_record_identity
                        .as_str(),
                )
                .expect("writeback execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackReplay,
                    retained_evidence.writeback_replay_record_identity.as_str(),
                )
                .expect("writeback replay reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Relational,
                BridgeCausalEvidenceReferenceIdentity::relational_authority(format!(
                    "relational-authority:{commit_identity}"
                ))
                .expect("relational authority reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    format!("signal-invalidation:{commit_identity}"),
                )
                .expect("signal invalidation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalEvaluation,
                    format!("signal-evaluation:{commit_identity}"),
                )
                .expect("signal evaluation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalForensicAvailability,
                    format!("signal-forensic:{commit_identity}"),
                )
                .expect("signal forensic reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalReplayCursor,
                    format!("signal-replay-cursor:{commit_identity}"),
                )
                .expect("signal replay cursor reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalLineage,
                    format!("signal-lineage:{commit_identity}"),
                )
                .expect("signal lineage reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalProvenance,
                    format!("signal-provenance:{commit_identity}"),
                )
                .expect("signal provenance reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid")
}
