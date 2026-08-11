use serde::{Deserialize, Serialize};

use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::OutputIdentity;

use super::super::aspect_policy_registry::{
    AspectMergePolicyName, AspectMergePolicySelectionBasis,
};
use super::super::conflict_isolation_registry::{
    ConflictIsolationPolicyName, ConflictIsolationSelectionBasis,
};
use super::super::core::BranchMergeBase;
use super::super::merge_base_registry::{MergeBaseSelectionBasis, MergeBaseStrategyName};
use super::super::policy::ConflictIsolationGranularity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofMinimalOverlapBasis {
    pub shared_nodes: Vec<NodeId>,
}

impl ProofMinimalOverlapBasis {
    pub fn breadth(&self) -> u64 {
        self.shared_nodes.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConservativeOverlapExpansion {
    pub expanded_nodes: Vec<NodeId>,
    pub support_nodes: Vec<NodeId>,
}

impl ConservativeOverlapExpansion {
    pub fn breadth(&self) -> u64 {
        self.expanded_nodes.len() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedMergeCandidateSet {
    pub nodes: Vec<NodeId>,
}

impl PlannedMergeCandidateSet {
    pub fn breadth(&self) -> u64 {
        self.nodes.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityCorrespondenceBasis {
    ExactNodeId,
    OutputIdentityTargetJournal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityCorrespondenceStatus {
    Matched,
    UnmatchedNoCandidate,
    UnmatchedRejectedAdmissibility,
    AmbiguousCandidates,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityCorrespondenceRecord {
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub basis: Option<IdentityCorrespondenceBasis>,
    pub status: IdentityCorrespondenceStatus,
    pub source_output_identity: Option<OutputIdentity>,
    pub target_output_identity: Option<OutputIdentity>,
    pub candidate_count: u32,
    #[serde(default)]
    pub candidate_target_nodes: Vec<NodeId>,
    pub admissibility_rejection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredIdentityCorrespondencePlan {
    pub target_candidate_count: u64,
    pub source_lookup_count: u64,
    pub ambiguous_match_count: u64,
    pub rejected_admissibility_count: u64,
    pub records: Vec<IdentityCorrespondenceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredDeletionPolicyPlan {
    pub target_only_nodes: Vec<NodeId>,
    pub target_only_count: u64,
    pub rejected_target_only_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredConflictIsolationRecord {
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub granularity: ConflictIsolationGranularity,
    pub isolated_aspects: Vec<Aspect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictIsolationWitness {
    pub granularity: ConflictIsolationGranularity,
    pub conflict_record_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RegionIsolationSummary {
    pub isolated_region_count: u64,
    pub host_declared_region_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConservativeIsolationExpansion {
    pub expanded_node_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredConflictIsolationPlan {
    pub selected_policy_name: Option<ConflictIsolationPolicyName>,
    pub selected_policy_digest: Option<String>,
    pub selected_policy_basis: Option<ConflictIsolationSelectionBasis>,
    pub expansion_breadth: u64,
    pub witness: Option<ConflictIsolationWitness>,
    pub region_summary: RegionIsolationSummary,
    pub conservative_expansion: ConservativeIsolationExpansion,
    pub records: Vec<LoweredConflictIsolationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredMergeBasePlan {
    pub resolved_base: BranchMergeBase,
    pub selected_merge_base_name: MergeBaseStrategyName,
    pub selected_merge_base_digest: String,
    pub selected_merge_base_basis: MergeBaseSelectionBasis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredAspectMergePolicyRecord {
    pub aspect: Aspect,
    pub selected_policy_name: AspectMergePolicyName,
    pub selected_policy_digest: String,
    pub selected_policy_basis: AspectMergePolicySelectionBasis,
    pub affected_source_nodes: Vec<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredAspectMergePolicyPlan {
    pub records: Vec<LoweredAspectMergePolicyRecord>,
}

impl LoweredAspectMergePolicyPlan {
    pub fn aspect_count(&self) -> u64 {
        self.records.len() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectMergeDecisionOutcome {
    SourceAuthorityAdopted,
    SourceIntroducedIntoTarget,
    EquivalentUnchanged,
    TargetPreserved,
    SourceSkippedNonAdoptable,
    ConflictRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoweredAspectMergeDecisionRecord {
    pub aspect: Aspect,
    pub source_node: NodeId,
    pub target_node: Option<NodeId>,
    pub selected_policy_name: AspectMergePolicyName,
    pub selected_policy_digest: String,
    pub selected_policy_basis: AspectMergePolicySelectionBasis,
    pub outcome: AspectMergeDecisionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoweredAspectMergeDecisionPlan {
    pub records: Vec<LoweredAspectMergeDecisionRecord>,
}
