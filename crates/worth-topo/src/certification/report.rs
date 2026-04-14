use forge_relational::facade::identity::{EntityId, RelationId};
use serde::{Deserialize, Serialize};
use worth_schema::facade::{
    CertifiedTopologyInterpretation, WorthMutationOrigin, WorthTopologyReadArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDeterministicDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationEntityRow {
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationRelationRow {
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyLocalizationReport {
    pub topology_entities: Vec<WorthTopologyLocalizationEntityRow>,
    pub topology_relations: Vec<WorthTopologyLocalizationRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentRow {
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamingAttachmentReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<WorthNamingAttachmentRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveFamilyCoverageEntry {
    pub family: String,
    pub observed: bool,
    pub observed_member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthPrimitiveFamilyCoverageMatrix {
    pub entries: Vec<WorthPrimitiveFamilyCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBranchLocalTopologyReport {
    pub mutation_origin: WorthMutationOrigin,
    pub branch_local: bool,
    pub snapshot_id: u64,
    pub touched_aspect_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthReplayParityReport {
    pub mutation_origin: WorthMutationOrigin,
    pub replay_origin: bool,
    pub parity_status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorthMilestoneOneCertificationReport {
    pub named_truth_validated: bool,
    pub topology_validated: bool,
    pub topology_truth_digest: WorthDeterministicDigest,
    pub naming_truth_digest: WorthDeterministicDigest,
    pub topology_validation_digest: WorthDeterministicDigest,
    pub topology_localization_report: WorthTopologyLocalizationReport,
    pub naming_attachment_report: WorthNamingAttachmentReport,
    pub primitive_family_coverage_matrix: WorthPrimitiveFamilyCoverageMatrix,
    pub branch_local_topology_report: WorthBranchLocalTopologyReport,
    pub milestone_1_replay_parity_report: WorthReplayParityReport,
    pub read_artifact: WorthTopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}
