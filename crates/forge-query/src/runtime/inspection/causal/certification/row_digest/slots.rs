use crate::identity::hash_parts;

use forge_runtime_bridge::facade::BridgeCausalEvidenceFamily;

use super::super::super::inventory::CausalEvidenceFamily;
use super::super::super::materialization::{
    QueryCausalEvidenceReferenceArtifact, QueryCausalInspectionArtifact,
};

#[derive(Default)]
pub(super) struct NamedEvidenceSlots {
    pub(super) relational_authority_digest: Option<String>,
    pub(super) bridge_route_digest: Option<String>,
    pub(super) bridge_evaluation_digest: Option<String>,
    pub(super) bridge_source_materialization_digest: Option<String>,
    pub(super) bridge_structural_digest: Option<String>,
    pub(super) bridge_stream_digest: Option<String>,
    pub(super) bridge_preview_digest: Option<String>,
    pub(super) bridge_writeback_digest: Option<String>,
    pub(super) bridge_replay_digest: Option<String>,
    pub(super) signal_invalidation_digest: Option<String>,
    pub(super) signal_evaluation_digest: Option<String>,
    pub(super) signal_forensic_availability_digest: Option<String>,
    pub(super) signal_replay_cursor_digest: Option<String>,
    pub(super) signal_lineage_digest: Option<String>,
    pub(super) signal_provenance_digest: Option<String>,
    pub(super) replay_posture_digest: Option<String>,
}

pub(super) fn named_evidence_slots(artifact: &QueryCausalInspectionArtifact) -> NamedEvidenceSlots {
    let references = evidence_references(artifact);
    NamedEvidenceSlots {
        relational_authority_digest: digest_for_bridge_families(
            references,
            &[BridgeCausalEvidenceFamily::RelationalAuthority],
        ),
        bridge_route_digest: digest_for_bridge_families(
            references,
            &[BridgeCausalEvidenceFamily::BridgeRoute],
        ),
        bridge_evaluation_digest: digest_for_bridge_families(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
                BridgeCausalEvidenceFamily::BridgeHistoricalEvaluationFailure,
            ],
        ),
        bridge_source_materialization_digest: digest_for_bridge_families(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeSourceMaterialization,
                BridgeCausalEvidenceFamily::BridgeSourceFailure,
            ],
        ),
        bridge_structural_digest: digest_for_bridge_families(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeStructuralRemap,
                BridgeCausalEvidenceFamily::BridgeStructuralBranchComparison,
            ],
        ),
        bridge_stream_digest: digest_for_bridge_families(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeStreamReplay,
                BridgeCausalEvidenceFamily::BridgeStreamCheckpoint,
            ],
        ),
        bridge_preview_digest: digest_for_bridge_families(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgePreviewExecution,
                BridgeCausalEvidenceFamily::BridgePreviewDiscard,
                BridgeCausalEvidenceFamily::BridgePreviewPromotion,
            ],
        ),
        bridge_writeback_digest: digest_for_bridge_families(references, &writeback_families()),
        bridge_replay_digest: digest_for_any_family(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeStreamReplay.as_str(),
                BridgeCausalEvidenceFamily::BridgeWritebackReplay.as_str(),
                CausalEvidenceFamily::BridgeReplay.as_str(),
            ],
        ),
        signal_invalidation_digest: digest_for_any_family(
            references,
            &[
                BridgeCausalEvidenceFamily::SignalInvalidation.as_str(),
                CausalEvidenceFamily::SignalInvalidation.as_str(),
            ],
        ),
        signal_evaluation_digest: digest_for_query_families(
            references,
            &[CausalEvidenceFamily::SignalEvaluation],
        ),
        signal_forensic_availability_digest: digest_for_query_families(
            references,
            &[CausalEvidenceFamily::SignalForensicAvailability],
        ),
        signal_replay_cursor_digest: digest_for_query_families(
            references,
            &[CausalEvidenceFamily::SignalReplayCursor],
        ),
        signal_lineage_digest: digest_for_query_families(
            references,
            &[
                CausalEvidenceFamily::SignalLineage,
                CausalEvidenceFamily::Lineage,
            ],
        ),
        signal_provenance_digest: digest_for_query_families(
            references,
            &[
                CausalEvidenceFamily::SignalProvenance,
                CausalEvidenceFamily::Provenance,
            ],
        ),
        replay_posture_digest: digest_for_any_family(
            references,
            &[
                BridgeCausalEvidenceFamily::BridgeStreamReplay.as_str(),
                BridgeCausalEvidenceFamily::BridgeWritebackReplay.as_str(),
                CausalEvidenceFamily::BridgeReplay.as_str(),
                CausalEvidenceFamily::SignalReplayCursor.as_str(),
            ],
        ),
    }
}

fn writeback_families() -> [BridgeCausalEvidenceFamily; 6] {
    [
        BridgeCausalEvidenceFamily::BridgeWritebackAdmission,
        BridgeCausalEvidenceFamily::BridgeWritebackMapperEnvelope,
        BridgeCausalEvidenceFamily::BridgeWritebackMappedFamilyInput,
        BridgeCausalEvidenceFamily::BridgeWritebackMapper,
        BridgeCausalEvidenceFamily::BridgeWritebackExecution,
        BridgeCausalEvidenceFamily::BridgeWritebackReplay,
    ]
}

fn evidence_references(
    artifact: &QueryCausalInspectionArtifact,
) -> &[QueryCausalEvidenceReferenceArtifact] {
    match artifact {
        QueryCausalInspectionArtifact::Admitted(artifact) => artifact.evidence_references(),
        QueryCausalInspectionArtifact::Advisory(artifact) => artifact.evidence_references(),
        QueryCausalInspectionArtifact::Denied(_) => &[],
    }
}

fn digest_for_bridge_families(
    references: &[QueryCausalEvidenceReferenceArtifact],
    families: &[BridgeCausalEvidenceFamily],
) -> Option<String> {
    let family_names = families
        .iter()
        .map(BridgeCausalEvidenceFamily::as_str)
        .collect::<Vec<_>>();
    digest_for_any_family(references, &family_names)
}

fn digest_for_query_families(
    references: &[QueryCausalEvidenceReferenceArtifact],
    families: &[CausalEvidenceFamily],
) -> Option<String> {
    let family_names = families
        .iter()
        .map(CausalEvidenceFamily::as_str)
        .collect::<Vec<_>>();
    digest_for_any_family(references, &family_names)
}

fn digest_for_any_family(
    references: &[QueryCausalEvidenceReferenceArtifact],
    families: &[&str],
) -> Option<String> {
    let reference_digests = references
        .iter()
        .filter(|reference| families.contains(&reference.family()))
        .map(|reference| reference.reference_digest())
        .collect::<Vec<_>>();
    if reference_digests.is_empty() {
        return None;
    }
    Some(hash_parts(&[
        "causal_inspection_named_evidence_slot_digest_v1".to_string(),
        format!("families:{}", families.join("|")),
        format!("references:{}", reference_digests.join("|")),
    ]))
}
