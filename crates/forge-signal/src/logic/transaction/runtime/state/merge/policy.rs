use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExistingTargetMergePolicy {
    PreserveEquivalentOtherwiseAdoptSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceOnlyMergePolicy {
    IntroduceAdoptableSkipNonAdoptable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictMergePolicy {
    RejectSharedStateConflict,
    ResolveSourceStateWhenStructureMatches,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchMergeReconciliationPolicy {
    pub existing_target: ExistingTargetMergePolicy,
    pub source_only: SourceOnlyMergePolicy,
    pub conflict: ConflictMergePolicy,
}
