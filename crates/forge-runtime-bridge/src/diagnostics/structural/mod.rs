mod counters;
mod explanations;
mod records;
mod replay;

pub use counters::BridgeStructuralCounters;
pub use explanations::{
    BridgeStructuralBranchComparisonExplanation, BridgeStructuralRemapExplanation,
};
#[allow(unused_imports)]
pub use records::{
    BridgeCanonicalStructuralBranchComparisonRecord, BridgeCanonicalStructuralRemapRecord,
    BridgeStructuralBranchComparisonRecord, BridgeStructuralBranchComparisonRecordIdentity,
    BridgeStructuralBranchComparisonReplaySummary, BridgeStructuralRemapRecord,
    BridgeStructuralRemapRecordIdentity, BridgeStructuralRemapReplaySummary,
    BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1,
};
pub(crate) use replay::{
    validate_structural_replay_contract, validate_structural_replay_outcome,
};
