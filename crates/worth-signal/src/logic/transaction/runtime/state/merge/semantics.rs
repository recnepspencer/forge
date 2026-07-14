use serde::{Deserialize, Serialize};

use super::conflict_isolation_registry::{
    ConflictIsolationPolicyName, ConflictIsolationSelectionBasis,
};
use super::conflict_policy_registry::{ConflictPolicyName, ConflictPolicySelectionBasis};
use super::deletion_policy_registry::{DeletionPolicyName, DeletionPolicySelectionBasis};
use super::identity_matcher_registry::{IdentityMatcherName, IdentityMatcherSelectionBasis};
use super::merge_base_registry::{MergeBaseSelectionBasis, MergeBaseStrategyName};
use super::source_only_policy_registry::{SourceOnlyPolicyName, SourceOnlyPolicySelectionBasis};
use super::strategy_registry::{MergeStrategyName, MergeStrategySelectionBasis};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedMergeSemanticsBundle {
    pub strategy_name: MergeStrategyName,
    pub strategy_digest: String,
    pub strategy_basis: MergeStrategySelectionBasis,
    pub merge_base_name: MergeBaseStrategyName,
    pub merge_base_digest: String,
    pub merge_base_basis: MergeBaseSelectionBasis,
    pub conflict_policy_name: ConflictPolicyName,
    pub conflict_policy_digest: String,
    pub conflict_policy_basis: ConflictPolicySelectionBasis,
    pub conflict_isolation_name: ConflictIsolationPolicyName,
    pub conflict_isolation_digest: String,
    pub conflict_isolation_basis: ConflictIsolationSelectionBasis,
    pub identity_matcher_name: IdentityMatcherName,
    pub identity_matcher_digest: String,
    pub identity_matcher_basis: IdentityMatcherSelectionBasis,
    pub source_only_policy_name: SourceOnlyPolicyName,
    pub source_only_policy_digest: String,
    pub source_only_policy_basis: SourceOnlyPolicySelectionBasis,
    pub deletion_policy_name: DeletionPolicyName,
    pub deletion_policy_digest: String,
    pub deletion_policy_basis: DeletionPolicySelectionBasis,
}

impl SelectedMergeSemanticsBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        strategy_name: MergeStrategyName,
        strategy_digest: String,
        strategy_basis: MergeStrategySelectionBasis,
        merge_base_name: MergeBaseStrategyName,
        merge_base_digest: String,
        merge_base_basis: MergeBaseSelectionBasis,
        conflict_policy_name: ConflictPolicyName,
        conflict_policy_digest: String,
        conflict_policy_basis: ConflictPolicySelectionBasis,
        conflict_isolation_name: ConflictIsolationPolicyName,
        conflict_isolation_digest: String,
        conflict_isolation_basis: ConflictIsolationSelectionBasis,
        identity_matcher_name: IdentityMatcherName,
        identity_matcher_digest: String,
        identity_matcher_basis: IdentityMatcherSelectionBasis,
        source_only_policy_name: SourceOnlyPolicyName,
        source_only_policy_digest: String,
        source_only_policy_basis: SourceOnlyPolicySelectionBasis,
        deletion_policy_name: DeletionPolicyName,
        deletion_policy_digest: String,
        deletion_policy_basis: DeletionPolicySelectionBasis,
    ) -> Self {
        Self {
            strategy_name,
            strategy_digest,
            strategy_basis,
            merge_base_name,
            merge_base_digest,
            merge_base_basis,
            conflict_policy_name,
            conflict_policy_digest,
            conflict_policy_basis,
            conflict_isolation_name,
            conflict_isolation_digest,
            conflict_isolation_basis,
            identity_matcher_name,
            identity_matcher_digest,
            identity_matcher_basis,
            source_only_policy_name,
            source_only_policy_digest,
            source_only_policy_basis,
            deletion_policy_name,
            deletion_policy_digest,
            deletion_policy_basis,
        }
    }
}
