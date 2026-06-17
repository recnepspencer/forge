use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReferenceIdentity, BridgeIdentityEvidence, BridgeRoute,
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
    let summary = crate::runtime::tests::causal_inspection::bridge_admitted_summary(admitted);
    let commit_id = commit_identity
        .relational_commit_id()
        .expect("causal lower-runtime references must carry relational commit authority");
    BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    admitted
                        .subject()
                        .query_observation_bridge_evidence_identity(),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().bridge_admission_evidence(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
                    bridge_evidence(&retained_evidence.historical_evaluation_record_identity),
                )
                .expect("historical evaluation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewExecution,
                    bridge_evidence(&retained_evidence.preview_execution_record_identity),
                )
                .expect("preview execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewDiscard,
                    bridge_evidence(&retained_evidence.preview_discard_record_identity),
                )
                .expect("preview discard reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeSourceMaterialization,
                    bridge_evidence(&retained_evidence.source_materialization_record_identity),
                )
                .expect("source materialization reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStructuralRemap,
                    bridge_evidence(&retained_evidence.structural_remap_record_identity),
                )
                .expect("structural remap reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStreamReplay,
                    bridge_evidence(&retained_evidence.stream_replay_record_identity),
                )
                .expect("stream replay reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackAdmission,
                    bridge_evidence(&retained_evidence.writeback_admission_record_identity),
                )
                .expect("writeback admission reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope,
                    bridge_evidence(&retained_evidence.writeback_mapper_envelope_identity),
                )
                .expect("writeback mapper envelope reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput,
                    bridge_evidence(&retained_evidence.writeback_mapped_family_input_identity),
                )
                .expect("writeback mapped input reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapper,
                    bridge_evidence(&retained_evidence.writeback_mapper_record_identity),
                )
                .expect("writeback mapper reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackExecution,
                    bridge_evidence(&retained_evidence.writeback_execution_record_identity),
                )
                .expect("writeback execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackReplay,
                    bridge_evidence(&retained_evidence.writeback_replay_record_identity),
                )
                .expect("writeback replay reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Relational,
                BridgeCausalEvidenceReferenceIdentity::relational_authority(bridge_evidence(
                    format!("relational-authority:commit-{commit_id}"),
                ))
                .expect("relational authority reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    bridge_evidence(format!("signal-invalidation:commit-{commit_id}")),
                )
                .expect("signal invalidation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalEvaluation,
                    bridge_evidence(format!("signal-evaluation:commit-{commit_id}")),
                )
                .expect("signal evaluation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalForensicAvailability,
                    bridge_evidence(format!("signal-forensic:commit-{commit_id}")),
                )
                .expect("signal forensic reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalReplayCursor,
                    bridge_evidence(format!("signal-replay-cursor:commit-{commit_id}")),
                )
                .expect("signal replay cursor reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalLineage,
                    bridge_evidence(format!("signal-lineage:commit-{commit_id}")),
                )
                .expect("signal lineage reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalProvenance,
                    bridge_evidence(format!("signal-provenance:commit-{commit_id}")),
                )
                .expect("signal provenance reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid")
}

fn bridge_evidence(value: impl AsRef<str>) -> BridgeIdentityEvidence {
    crate::runtime::tests::causal_inspection::bridge_external_evidence(value)
}
