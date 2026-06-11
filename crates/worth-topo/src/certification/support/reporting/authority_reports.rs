use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationEntityRow {
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationRelationRow {
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationReport {
    pub topology_entities: Vec<TopologyLocalizationEntityRow>,
    pub topology_relations: Vec<TopologyLocalizationRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateEntityRow {
    pub source: String,
    pub entity_id: EntityId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateRelationRow {
    pub source: String,
    pub relation_id: RelationId,
    pub kind_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyLocalizationAggregateReport {
    pub topology_entities: Vec<TopologyLocalizationAggregateEntityRow>,
    pub topology_relations: Vec<TopologyLocalizationAggregateRelationRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentAggregateRow {
    pub source: String,
    pub topology_entity_id: EntityId,
    pub topology_kind_name: String,
    pub attached_persistent_name_ids: Vec<EntityId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingAttachmentAggregateReport {
    pub fully_named: bool,
    pub orphan_persistent_name_ids: Vec<EntityId>,
    pub attachments: Vec<NamingAttachmentAggregateRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveFamilyCoverageEntry {
    pub family: String,
    pub observed: bool,
    pub observed_member_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimitiveFamilyCoverageMatrix {
    pub entries: Vec<PrimitiveFamilyCoverageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyBranchAuthoringBoundary {
    SchemaTopologyAuthoring,
}

impl TopologyBranchAuthoringBoundary {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaTopologyAuthoring => "schema_topology_authoring",
        }
    }

    pub const fn from_mutation_origin(mutation_origin: MutationOrigin) -> Option<Self> {
        match mutation_origin {
            MutationOrigin::BranchLocalApplication => Some(Self::SchemaTopologyAuthoring),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchLocalTopologyReport {
    pub mutation_origin: MutationOrigin,
    pub branch_local: bool,
    pub branch_authoring_boundary: Option<TopologyBranchAuthoringBoundary>,
    pub branch_id: BranchId,
    pub snapshot_id: u64,
    pub touched_aspect_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayParityStatus {
    NotChecked,
    Match,
    Mismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayParityReport {
    pub mutation_origin: MutationOrigin,
    pub replay_origin: bool,
    pub branch_id: BranchId,
    pub parity_status: ReplayParityStatus,
    pub equivalence_contract: DerivedEquivalenceContractReport,
    pub replay_equivalence_contract: Option<DerivedEquivalenceContractReport>,
    pub relational_replay_checked: bool,
    pub relational_replay_verified: bool,
    pub replayed_commit_id: Option<String>,
    pub compared_surfaces: Vec<ReplayObservableSurface>,
    pub mismatch_count: usize,
    pub replay_failure: Option<ReplayFailureClass>,
    pub interpretation_digest_match: bool,
    pub truth_digest_match: bool,
    pub validation_digest_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneOneCounters {
    pub topology_entity_upsert_count: usize,
    pub topology_relation_upsert_count: usize,
    pub topology_relation_remove_count: usize,
    pub commit_boundary_validator_count: usize,
    pub commit_boundary_rejection_count: usize,
    pub derived_topology_interpretation_count: usize,
    pub derived_topology_full_fallback_count: usize,
    pub naming_target_lookup_count: usize,
    pub primitive_family_member_count: usize,
    pub replay_history_length: usize,
    pub replay_interpretation_rerun_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneOneCertificationReport {
    pub named_truth_validated: bool,
    pub topology_validated: bool,
    pub topology_truth_digest: DeterministicDigest,
    pub naming_truth_digest: DeterministicDigest,
    pub topology_validation_digest: DeterministicDigest,
    pub topology_validation_report: TopologyValidationReport,
    pub topology_localization_report: TopologyLocalizationReport,
    pub naming_attachment_report: NamingAttachmentReport,
    pub primitive_family_coverage_matrix: PrimitiveFamilyCoverageMatrix,
    pub branch_local_topology_report: BranchLocalTopologyReport,
    pub milestone_1_replay_parity_report: ReplayParityReport,
    pub derived_invalidation_report: DerivedInvalidationReport,
    pub derived_rebuild_report: DerivedRebuildReport,
    pub derived_fallback_report: DerivedFallbackReport,
    pub derived_equivalence_contract_report: DerivedEquivalenceContractReport,
    pub derived_read_diagnostics: DerivedReadDiagnostics,
    pub counters: MilestoneOneCounters,
    pub read_artifact: TopologyReadArtifact,
    pub certified_interpretation: CertifiedTopologyInterpretation,
}
