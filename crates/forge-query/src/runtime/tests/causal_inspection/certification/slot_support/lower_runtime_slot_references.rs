use forge_runtime_bridge::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner,
    BridgeCausalEvidenceReferenceIdentity, BridgeCausalInspectionAdmissionSummary,
    BridgeIdentityEvidence, BridgeRoute, TruthCommitIdentity,
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
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.admitted_inspection_digest(),
        ),
        forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
            admitted.subject().anchor_for_reporting(),
        ),
    )
    .expect("query admission summary should be valid");
    let commit_identity = commit_identity.evidence_identity();
    BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        summary,
        vec![
            query_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        admitted
                            .subject()
                            .query_observation_bridge_evidence_identity(),
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.route_identity().evidence_identity(),
                )
                .expect("route evidence reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.historical_evaluation_record_identity),
                    ),
                )
                .expect("historical evaluation reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewExecution,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.preview_execution_record_identity),
                    ),
                )
                .expect("preview execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgePreviewDiscard,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.preview_discard_record_identity),
                    ),
                )
                .expect("preview discard reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeSourceMaterialization,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.source_materialization_record_identity),
                    ),
                )
                .expect("source materialization reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStructuralRemap,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.structural_remap_record_identity),
                    ),
                )
                .expect("structural remap reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeStreamReplay,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.stream_replay_record_identity),
                    ),
                )
                .expect("stream replay reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackAdmission,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_admission_record_identity),
                    ),
                )
                .expect("writeback admission reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_mapper_envelope_identity),
                    ),
                )
                .expect("writeback mapper envelope reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_mapped_family_input_identity),
                    ),
                )
                .expect("writeback mapped input reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackMapper,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_mapper_record_identity),
                    ),
                )
                .expect("writeback mapper reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackExecution,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_execution_record_identity),
                    ),
                )
                .expect("writeback execution reference identity should be valid"),
            ),
            bridge_reference(
                BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
                    BridgeCausalEvidenceFamily::BridgeWritebackReplay,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(&retained_evidence.writeback_replay_record_identity),
                    ),
                )
                .expect("writeback replay reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Relational,
                BridgeCausalEvidenceReferenceIdentity::relational_authority(
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!(
                            "relational-authority:{}",
                            commit_identity.as_str()
                        )),
                    ),
                )
                .expect("relational authority reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!(
                            "signal-invalidation:{}",
                            commit_identity.as_str()
                        )),
                    ),
                )
                .expect("signal invalidation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalEvaluation,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!("signal-evaluation:{}", commit_identity.as_str())),
                    ),
                )
                .expect("signal evaluation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalForensicAvailability,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!("signal-forensic:{}", commit_identity.as_str())),
                    ),
                )
                .expect("signal forensic reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalReplayCursor,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!(
                            "signal-replay-cursor:{}",
                            commit_identity.as_str()
                        )),
                    ),
                )
                .expect("signal replay cursor reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalLineage,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!("signal-lineage:{}", commit_identity.as_str())),
                    ),
                )
                .expect("signal lineage reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalProvenance,
                    forge_runtime_bridge::facade::BridgeIdentityEvidence::from_external_authority(
                        bridge_evidence(format!("signal-provenance:{}", commit_identity.as_str())),
                    ),
                )
                .expect("signal provenance reference identity should be valid"),
            ),
        ],
    )
    .expect("bridge request should be valid")
}

fn bridge_evidence(value: impl AsRef<str>) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(value)
}
