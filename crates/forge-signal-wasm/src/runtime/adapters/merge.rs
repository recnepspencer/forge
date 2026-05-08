use serde::{Deserialize, Serialize};

use forge_signal::facade::adapters::{
    ArtifactMergeComparable, BranchMergeBase, BranchMergeCounters, LoweredMergeBasePlan,
    MergePlanProofReport, MergeResultProofReport, SelectedMergeSemanticsBundle,
};

mod conversions;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergePlanProofEnvelope {
    pub plan: MergePlanArtifactSummary,
    pub proof: MergePlanProofReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeResultProofEnvelope {
    pub result: MergeResultArtifactSummary,
    pub proof: MergeResultProofReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeSemanticsSummary {
    pub strategy_name: String,
    pub strategy_digest: String,
    pub strategy_basis: String,
    pub merge_base_name: String,
    pub merge_base_digest: String,
    pub merge_base_basis: String,
    pub conflict_policy_name: String,
    pub conflict_policy_digest: String,
    pub conflict_policy_basis: String,
    pub conflict_isolation_name: String,
    pub conflict_isolation_digest: String,
    pub conflict_isolation_basis: String,
    pub identity_matcher_name: String,
    pub identity_matcher_digest: String,
    pub identity_matcher_basis: String,
    pub source_only_policy_name: String,
    pub source_only_policy_digest: String,
    pub source_only_policy_basis: String,
    pub deletion_policy_name: String,
    pub deletion_policy_digest: String,
    pub deletion_policy_basis: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeBaseSummary {
    pub source_branch_id: u64,
    pub target_branch_id: u64,
    pub forked_from_snapshot_id: Option<u64>,
    pub source_snapshot_id: Option<u64>,
    pub target_snapshot_id_before: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredMergeBaseSummary {
    pub resolved_base: MergeBaseSummary,
    pub selected_merge_base_name: String,
    pub selected_merge_base_digest: String,
    pub selected_merge_base_basis: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolutionRecordSummary {
    pub source_node: String,
    pub target_node: String,
    pub required_resolution: Vec<String>,
    pub supported_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolutionPlanSummary {
    pub source_branch_id: u64,
    pub target_branch_id: u64,
    pub divergence: String,
    pub records: Vec<ConflictResolutionRecordSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeNodeMapEntrySummary {
    pub source_node: String,
    pub target_node: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeDependencyFingerprintSummary {
    pub dependency_count: u32,
    pub meaningful_input_changes: u32,
    pub output_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeArtifactAuthoritySummary {
    pub authority_class: String,
    pub adoptability: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeComparableSummary {
    pub output_identity: Option<String>,
    pub continuity_token: Option<String>,
    pub dependency_fingerprint: MergeDependencyFingerprintSummary,
    pub authority: MergeArtifactAuthoritySummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMergeInputStateSummary {
    pub current_artifact_id: Option<u64>,
    pub comparable: Option<MergeComparableSummary>,
    pub authority: Option<MergeArtifactAuthoritySummary>,
    pub exists_in_branch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeMergePlanSummary {
    pub source_node: String,
    pub shape_kind: String,
    pub target_node: Option<String>,
    pub source_state: NodeMergeInputStateSummary,
    pub target_state: NodeMergeInputStateSummary,
    pub decision: String,
    pub resolved_conflict_kinds: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptionTargetIdentitySummary {
    pub kind: String,
    pub mapped_target_node: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptedNodeContractSummary {
    pub merge_strategy_name: Option<String>,
    pub conflict_policy_name: Option<String>,
    pub identity_matcher_name: Option<String>,
    pub source_only_policy_name: Option<String>,
    pub deletion_policy_name: Option<String>,
    pub conflict_isolation_policy_name: Option<String>,
    pub aspect_merge_policy_binding_count: usize,
    pub condition: String,
    pub comparator: Option<String>,
    pub partitioned_output: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptionPlanCoreSummary {
    pub source_node: String,
    pub target_identity: AdoptionTargetIdentitySummary,
    pub authority: MergeArtifactAuthoritySummary,
    pub entry_contract: AdoptedNodeContractSummary,
    pub dependency_count: usize,
    pub dependency_snapshot_edge_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdoptionCarryPolicySummary {
    pub runtime_artifact: String,
    pub retained_artifact: String,
    pub causality: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeRecordSummary {
    pub source_node: String,
    pub target_node: Option<String>,
    pub source_artifact_id: Option<u64>,
    pub target_artifact_id_before: Option<u64>,
    pub target_artifact_id_after: Option<u64>,
    pub action: String,
    pub basis: String,
    pub source_comparable: Option<MergeComparableSummary>,
    pub target_comparable: Option<MergeComparableSummary>,
    pub identity_basis: Option<String>,
    pub identity_status: Option<String>,
    pub identity_candidate_count: u32,
    pub resolved_conflict_kinds: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeCountersSummary {
    pub boundary_witness_kind: String,
    pub source_slice_breadth: u64,
    pub proof_minimal_overlap_breadth: u64,
    pub conservative_overlap_expansion_breadth: u64,
    pub final_candidate_breadth: u64,
    pub reconciliation_breadth: u64,
    pub candidate_node_count: u64,
    pub examined_node_count: u64,
    pub adopted_count: u64,
    pub introduced_node_count: u64,
    pub replaced_count: u64,
    pub preserved_target_count: u64,
    pub skipped_non_adoptable_count: u64,
    pub equivalent_unchanged_count: u64,
    pub source_only_count: u64,
    pub target_only_count: u64,
    pub dependency_remap_count: u64,
    pub identity_target_candidates_indexed: u64,
    pub identity_source_lookups: u64,
    pub identity_ambiguous_match_count: u64,
    pub identity_rejected_admissibility_count: u64,
    pub conflict_isolation_record_count: u64,
    pub conflict_isolation_expansion_breadth: u64,
    pub subscriber_repair_breadth: u64,
    pub merge_lineage_record_count: u64,
    pub replay_event_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanArtifactSummary {
    pub source_branch_id: u64,
    pub target_branch_id: u64,
    pub schema_registry_digest: String,
    pub registry_bundle_digest: String,
    pub lowered_strategy_bundle_digest: String,
    pub merge_kind: String,
    pub selected_semantics: MergeSemanticsSummary,
    pub source_snapshot_id: Option<u64>,
    pub target_snapshot_id_before: Option<u64>,
    pub merge_base: Option<MergeBaseSummary>,
    pub lowered_merge_base: Option<LoweredMergeBaseSummary>,
    pub resolution_plan: Option<ConflictResolutionPlanSummary>,
    pub node_map: Vec<MergeNodeMapEntrySummary>,
    pub node_plan: Vec<NodeMergePlanSummary>,
    pub adoption_core: Vec<AdoptionPlanCoreSummary>,
    pub adoption_policy: Vec<AdoptionCarryPolicySummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeResultArtifactSummary {
    pub source_branch: u64,
    pub target_branch: u64,
    pub schema_registry_digest: String,
    pub registry_bundle_digest: String,
    pub lowered_strategy_bundle_digest: String,
    pub merge_kind: String,
    pub selected_semantics: MergeSemanticsSummary,
    pub merged_snapshot_id: Option<u64>,
    pub source_snapshot_id: Option<u64>,
    pub target_snapshot_id_before: Option<u64>,
    pub target_snapshot_id_after: Option<u64>,
    pub lowered_merge_base: Option<LoweredMergeBaseSummary>,
    pub resolution_plan: Option<ConflictResolutionPlanSummary>,
    pub records: Vec<MergeRecordSummary>,
    pub counters: MergeCountersSummary,
}

impl From<&SelectedMergeSemanticsBundle> for MergeSemanticsSummary {
    fn from(value: &SelectedMergeSemanticsBundle) -> Self {
        Self {
            strategy_name: value.strategy_name.as_str().to_owned(),
            strategy_digest: value.strategy_digest.clone(),
            strategy_basis: format!("{:?}", value.strategy_basis),
            merge_base_name: value.merge_base_name.as_str().to_owned(),
            merge_base_digest: value.merge_base_digest.clone(),
            merge_base_basis: format!("{:?}", value.merge_base_basis),
            conflict_policy_name: value.conflict_policy_name.as_str().to_owned(),
            conflict_policy_digest: value.conflict_policy_digest.clone(),
            conflict_policy_basis: format!("{:?}", value.conflict_policy_basis),
            conflict_isolation_name: value.conflict_isolation_name.as_str().to_owned(),
            conflict_isolation_digest: value.conflict_isolation_digest.clone(),
            conflict_isolation_basis: format!("{:?}", value.conflict_isolation_basis),
            identity_matcher_name: value.identity_matcher_name.as_str().to_owned(),
            identity_matcher_digest: value.identity_matcher_digest.clone(),
            identity_matcher_basis: format!("{:?}", value.identity_matcher_basis),
            source_only_policy_name: value.source_only_policy_name.as_str().to_owned(),
            source_only_policy_digest: value.source_only_policy_digest.clone(),
            source_only_policy_basis: format!("{:?}", value.source_only_policy_basis),
            deletion_policy_name: value.deletion_policy_name.as_str().to_owned(),
            deletion_policy_digest: value.deletion_policy_digest.clone(),
            deletion_policy_basis: format!("{:?}", value.deletion_policy_basis),
        }
    }
}

impl From<&BranchMergeBase> for MergeBaseSummary {
    fn from(value: &BranchMergeBase) -> Self {
        Self {
            source_branch_id: value.source_branch_id.0,
            target_branch_id: value.target_branch_id.0,
            forked_from_snapshot_id: value.forked_from_snapshot_id.map(|id| id.0),
            source_snapshot_id: value.source_snapshot_id.map(|id| id.0),
            target_snapshot_id_before: value.target_snapshot_id_before.map(|id| id.0),
        }
    }
}

impl From<&LoweredMergeBasePlan> for LoweredMergeBaseSummary {
    fn from(value: &LoweredMergeBasePlan) -> Self {
        Self {
            resolved_base: MergeBaseSummary::from(&value.resolved_base),
            selected_merge_base_name: value.selected_merge_base_name.as_str().to_owned(),
            selected_merge_base_digest: value.selected_merge_base_digest.clone(),
            selected_merge_base_basis: format!("{:?}", value.selected_merge_base_basis),
        }
    }
}

impl From<&ArtifactMergeComparable> for MergeComparableSummary {
    fn from(value: &ArtifactMergeComparable) -> Self {
        Self {
            output_identity: value
                .output_identity
                .as_ref()
                .map(|identity| identity.as_str().to_owned()),
            continuity_token: value
                .continuity_token
                .as_ref()
                .map(|token| token.as_str().to_owned()),
            dependency_fingerprint: MergeDependencyFingerprintSummary {
                dependency_count: value.dependency_fingerprint.dependency_count,
                meaningful_input_changes: value.dependency_fingerprint.meaningful_input_changes,
                output_hash: value.dependency_fingerprint.output_hash.to_string(),
            },
            authority: MergeArtifactAuthoritySummary {
                authority_class: format!("{:?}", value.authority.authority_class),
                adoptability: format!("{:?}", value.authority.adoptability),
            },
        }
    }
}

impl From<&BranchMergeCounters> for MergeCountersSummary {
    fn from(value: &BranchMergeCounters) -> Self {
        Self {
            boundary_witness_kind: format!("{:?}", value.boundary_witness_kind),
            source_slice_breadth: value.source_slice_breadth,
            proof_minimal_overlap_breadth: value.proof_minimal_overlap_breadth,
            conservative_overlap_expansion_breadth: value.conservative_overlap_expansion_breadth,
            final_candidate_breadth: value.final_candidate_breadth,
            reconciliation_breadth: value.reconciliation_breadth,
            candidate_node_count: value.candidate_node_count,
            examined_node_count: value.examined_node_count,
            adopted_count: value.adopted_count,
            introduced_node_count: value.introduced_node_count,
            replaced_count: value.replaced_count,
            preserved_target_count: value.preserved_target_count,
            skipped_non_adoptable_count: value.skipped_non_adoptable_count,
            equivalent_unchanged_count: value.equivalent_unchanged_count,
            source_only_count: value.source_only_count,
            target_only_count: value.target_only_count,
            dependency_remap_count: value.dependency_remap_count,
            identity_target_candidates_indexed: value.identity_target_candidates_indexed,
            identity_source_lookups: value.identity_source_lookups,
            identity_ambiguous_match_count: value.identity_ambiguous_match_count,
            identity_rejected_admissibility_count: value.identity_rejected_admissibility_count,
            conflict_isolation_record_count: value.conflict_isolation_record_count,
            conflict_isolation_expansion_breadth: value.conflict_isolation_expansion_breadth,
            subscriber_repair_breadth: value.subscriber_repair_breadth,
            merge_lineage_record_count: value.merge_lineage_record_count,
            replay_event_count: value.replay_event_count,
        }
    }
}
