use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExistingTargetMergePolicy {
    PreserveEquivalentOtherwiseAdoptSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceOnlyMergePolicy {
    IntroduceAdoptableSkipNonAdoptable,
    RejectIntroduction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeletionMergePolicy {
    PreserveTargetOnly,
    RejectTargetOnlyConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictMergePolicy {
    RejectSharedStateConflict,
    ResolveSourceStateWhenStructureMatches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectMergePolicy {
    RequireConflict,
    PreferSource,
    PreferTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictIsolationGranularity {
    PerNode,
    PerAspect,
    HostDeclaredRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeReconciliationPolicy {
    pub existing_target: ExistingTargetMergePolicy,
    pub source_only: SourceOnlyMergePolicy,
    pub deletion: DeletionMergePolicy,
    pub conflict: ConflictMergePolicy,
}

impl BranchMergeReconciliationPolicy {
    pub fn built_in_default() -> Self {
        Self {
            existing_target: ExistingTargetMergePolicy::PreserveEquivalentOtherwiseAdoptSource,
            source_only: SourceOnlyMergePolicy::IntroduceAdoptableSkipNonAdoptable,
            deletion: DeletionMergePolicy::PreserveTargetOnly,
            conflict: ConflictMergePolicy::ResolveSourceStateWhenStructureMatches,
        }
    }
}
