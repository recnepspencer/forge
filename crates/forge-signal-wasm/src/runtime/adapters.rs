use serde::{Deserialize, Serialize};

use crate::recipe::model::{KeyedRecipeFamilySpec, KeyedSourceFamilySpec, RecipeSpec, SourceSpec};
use crate::runtime::compute_callbacks::CapturedHostCapabilityRead;
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;
use forge_signal::facade::adapters::{
    ArtifactMergeComparable, BranchMergeBase, BranchMergeCounters, BranchMergePlan,
    BranchMergeResult, LoweredMergeBasePlan, MergeNodeMap, MergePlanProofReport,
    MergeResultProofReport, MergedArtifactRecord, SelectedMergeSemanticsBundle,
};
use forge_signal::facade::NodeId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostCapabilityTransportArtifact {
    pub family: String,
    pub registration_id: String,
    pub compatibility: String,
    pub exact_restore_outcome: String,
    pub portable_import_outcome: String,
    pub portable_import_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnavailableCallbackArtifact {
    pub id: String,
    pub signal_kind: String,
    pub reason: String,
    pub current_reads: Vec<String>,
    #[serde(default)]
    pub host_capability_reads: Vec<CapturedHostCapabilityRead>,
    #[serde(default)]
    pub host_capability_transports: Vec<HostCapabilityTransportArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDefinitionEnvelope {
    pub policy: RuntimePolicySpec,
    pub sources: Vec<SourceSpec>,
    pub recipes: Vec<RecipeSpec>,
    pub source_families: Vec<KeyedSourceFamilySpec>,
    pub recipe_families: Vec<KeyedRecipeFamilySpec>,
    #[serde(default)]
    pub unavailable_callbacks: Vec<UnavailableCallbackArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(not(test), allow(dead_code))]
pub struct RuntimeEnvelope {
    pub definitions: RuntimeDefinitionEnvelope,
    pub snapshot: RuntimeSnapshotEnvelope,
}

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

impl From<&MergedArtifactRecord> for MergeRecordSummary {
    fn from(value: &MergedArtifactRecord) -> Self {
        Self {
            source_node: node_id_key(value.source_node),
            target_node: value.target_node.map(node_id_key),
            source_artifact_id: value.source_artifact_id.map(|id| id.0),
            target_artifact_id_before: value.target_artifact_id_before.map(|id| id.0),
            target_artifact_id_after: value.target_artifact_id_after.map(|id| id.0),
            action: format!("{:?}", value.action),
            basis: format!("{:?}", value.basis),
            source_comparable: value
                .source_comparable
                .as_ref()
                .map(MergeComparableSummary::from),
            target_comparable: value
                .target_comparable
                .as_ref()
                .map(MergeComparableSummary::from),
            identity_basis: value.identity_basis.map(|basis| format!("{basis:?}")),
            identity_status: value.identity_status.map(|status| format!("{status:?}")),
            identity_candidate_count: value.identity_candidate_count,
            resolved_conflict_kinds: value
                .resolved_conflict_kinds
                .iter()
                .map(|kind| format!("{kind:?}"))
                .collect(),
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

impl From<BranchMergePlan> for MergePlanArtifactSummary {
    fn from(value: BranchMergePlan) -> Self {
        Self {
            source_branch_id: value.source_branch_id().0,
            target_branch_id: value.target_branch_id().0,
            schema_registry_digest: value.schema_registry_digest().to_owned(),
            registry_bundle_digest: value.registry_bundle_digest().to_owned(),
            lowered_strategy_bundle_digest: value.lowered_strategy_bundle_digest().to_owned(),
            merge_kind: format!("{:?}", value.merge_kind()),
            selected_semantics: MergeSemanticsSummary::from(value.selected_semantics()),
            source_snapshot_id: value.source_snapshot_id().map(|id| id.0),
            target_snapshot_id_before: value.target_snapshot_id_before().map(|id| id.0),
            merge_base: value.merge_base().map(MergeBaseSummary::from),
            lowered_merge_base: value
                .lowered_merge_base()
                .map(LoweredMergeBaseSummary::from),
            resolution_plan: value
                .resolution_plan()
                .map(|plan| ConflictResolutionPlanSummary {
                    source_branch_id: plan.source_branch_id.0,
                    target_branch_id: plan.target_branch_id.0,
                    divergence: format!("{:?}", plan.divergence),
                    records: plan
                        .records
                        .iter()
                        .map(|record| ConflictResolutionRecordSummary {
                            source_node: node_id_key(record.source_node),
                            target_node: node_id_key(record.target_node),
                            required_resolution: record
                                .required_resolution
                                .iter()
                                .map(|entry| format!("{entry:?}"))
                                .collect(),
                            supported_strategies: record
                                .supported_strategies
                                .iter()
                                .map(|entry| format!("{entry:?}"))
                                .collect(),
                        })
                        .collect(),
                }),
            node_map: merge_node_map_entries(value.node_map()),
            node_plan: value
                .node_plan()
                .iter()
                .map(|entry| NodeMergePlanSummary {
                    source_node: node_id_key(entry.source_node()),
                    shape_kind: match entry.shape() {
                        forge_signal::facade::adapters::NodeReconciliationShape::ExistingTargetNode {
                            ..
                        } => "ExistingTargetNode".to_owned(),
                        forge_signal::facade::adapters::NodeReconciliationShape::SourceOnlyIntroduction => {
                            "SourceOnlyIntroduction".to_owned()
                        }
                    },
                    target_node: match entry.shape() {
                        forge_signal::facade::adapters::NodeReconciliationShape::ExistingTargetNode {
                            target_node,
                        } => Some(node_id_key(target_node)),
                        forge_signal::facade::adapters::NodeReconciliationShape::SourceOnlyIntroduction => {
                            None
                        }
                    },
                    source_state: NodeMergeInputStateSummary {
                        current_artifact_id: entry.source_state().current_artifact_id().map(|id| id.0),
                        comparable: entry
                            .source_state()
                            .comparable()
                            .map(MergeComparableSummary::from),
                        authority: entry.source_state().authority().map(|authority| {
                            MergeArtifactAuthoritySummary {
                                authority_class: format!("{:?}", authority.authority_class),
                                adoptability: format!("{:?}", authority.adoptability),
                            }
                        }),
                        exists_in_branch: entry.source_state().exists_in_branch(),
                    },
                    target_state: NodeMergeInputStateSummary {
                        current_artifact_id: entry.target_state().current_artifact_id().map(|id| id.0),
                        comparable: entry
                            .target_state()
                            .comparable()
                            .map(MergeComparableSummary::from),
                        authority: entry.target_state().authority().map(|authority| {
                            MergeArtifactAuthoritySummary {
                                authority_class: format!("{:?}", authority.authority_class),
                                adoptability: format!("{:?}", authority.adoptability),
                            }
                        }),
                        exists_in_branch: entry.target_state().exists_in_branch(),
                    },
                    decision: format!("{:?}", entry.decision()),
                    resolved_conflict_kinds: entry
                        .resolved_conflict_kinds()
                        .iter()
                        .map(|kind| format!("{kind:?}"))
                        .collect(),
                })
                .collect(),
            adoption_core: value
                .adoption_core()
                .iter()
                .map(|entry| AdoptionPlanCoreSummary {
                    source_node: node_id_key(entry.source_node),
                    target_identity: {
                        let target_identity_json =
                            serde_json::to_value(&entry.target_identity).unwrap_or_default();
                        let kind = target_identity_json
                            .as_object()
                            .and_then(|object| object.keys().next().cloned())
                            .unwrap_or_else(|| format!("{:?}", entry.target_identity));
                        let mapped_target_node = target_identity_json
                            .get("ExistingMapping")
                            .and_then(|value| value.get("mapped_target_node"))
                            .and_then(|value| value.as_str())
                            .map(ToOwned::to_owned);
                        AdoptionTargetIdentitySummary {
                            kind,
                            mapped_target_node,
                        }
                    },
                    authority: MergeArtifactAuthoritySummary {
                        authority_class: format!("{:?}", entry.authority.authority_class),
                        adoptability: format!("{:?}", entry.authority.adoptability),
                    },
                    entry_contract: AdoptedNodeContractSummary {
                        merge_strategy_name: entry
                            .entry_contract
                            .eval_config
                            .merge_strategy_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        conflict_policy_name: entry
                            .entry_contract
                            .eval_config
                            .conflict_policy_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        identity_matcher_name: entry
                            .entry_contract
                            .eval_config
                            .identity_matcher_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        source_only_policy_name: entry
                            .entry_contract
                            .eval_config
                            .source_only_policy_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        deletion_policy_name: entry
                            .entry_contract
                            .eval_config
                            .deletion_policy_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        conflict_isolation_policy_name: entry
                            .entry_contract
                            .eval_config
                            .conflict_isolation_policy_name
                            .as_ref()
                            .map(|name| name.as_str().to_owned()),
                        aspect_merge_policy_binding_count: entry
                            .entry_contract
                            .eval_config
                            .aspect_merge_policy_bindings
                            .len(),
                        condition: format!("{:?}", entry.entry_contract.eval_config.condition),
                        comparator: entry
                            .entry_contract
                            .eval_config
                            .comparator
                            .as_ref()
                            .map(|comparator| format!("{comparator:?}")),
                        partitioned_output: entry.entry_contract.eval_config.partitioned_output,
                    },
                    dependency_count: entry.dependency_topology.dependencies.len(),
                    dependency_snapshot_edge_count: entry
                        .dependency_snapshot_ref
                        .snapshot
                        .entries()
                        .len(),
                })
                .collect(),
            adoption_policy: value
                .adoption_policy()
                .iter()
                .map(|policy| AdoptionCarryPolicySummary {
                    runtime_artifact: format!("{:?}", policy.runtime_artifact),
                    retained_artifact: format!("{:?}", policy.retained_artifact),
                    causality: format!("{:?}", policy.causality),
                })
                .collect(),
        }
    }
}

impl From<BranchMergeResult> for MergeResultArtifactSummary {
    fn from(value: BranchMergeResult) -> Self {
        Self {
            source_branch: value.source_branch.0,
            target_branch: value.target_branch.0,
            schema_registry_digest: value.schema_registry_digest,
            registry_bundle_digest: value.registry_bundle_digest,
            lowered_strategy_bundle_digest: value.lowered_strategy_bundle_digest,
            merge_kind: format!("{:?}", value.merge_kind),
            selected_semantics: MergeSemanticsSummary::from(&value.selected_semantics),
            merged_snapshot_id: value.merged_snapshot_id.map(|id| id.0),
            source_snapshot_id: value.source_snapshot_id.map(|id| id.0),
            target_snapshot_id_before: value.target_snapshot_id_before.map(|id| id.0),
            target_snapshot_id_after: value.target_snapshot_id_after.map(|id| id.0),
            lowered_merge_base: value
                .lowered_merge_base
                .as_ref()
                .map(LoweredMergeBaseSummary::from),
            resolution_plan: value.resolution_plan.as_ref().map(|plan| {
                ConflictResolutionPlanSummary {
                    source_branch_id: plan.source_branch_id.0,
                    target_branch_id: plan.target_branch_id.0,
                    divergence: format!("{:?}", plan.divergence),
                    records: plan
                        .records
                        .iter()
                        .map(|record| ConflictResolutionRecordSummary {
                            source_node: node_id_key(record.source_node),
                            target_node: node_id_key(record.target_node),
                            required_resolution: record
                                .required_resolution
                                .iter()
                                .map(|entry| format!("{entry:?}"))
                                .collect(),
                            supported_strategies: record
                                .supported_strategies
                                .iter()
                                .map(|entry| format!("{entry:?}"))
                                .collect(),
                        })
                        .collect(),
                }
            }),
            records: value.records.iter().map(MergeRecordSummary::from).collect(),
            counters: MergeCountersSummary::from(&value.counters),
        }
    }
}

fn merge_node_map_entries(value: &MergeNodeMap) -> Vec<MergeNodeMapEntrySummary> {
    value
        .source_to_target
        .iter()
        .map(|(source_node, target_node)| MergeNodeMapEntrySummary {
            source_node: node_id_key(*source_node),
            target_node: node_id_key(*target_node),
        })
        .collect()
}

fn node_id_key(node: NodeId) -> String {
    format!("{}:{}", node.index(), node.generation())
}
