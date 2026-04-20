use forge_relational::facade::history::{BranchId, CommitId};
use forge_store::{
    ContinuationBatchBudget, ContinuationRetentionStatus, ContinuationStrategy,
    CursorContinuationPlan, CursorContinuationRequest, FetchWidth, MaxBatchItems,
    MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch,
    StableBasisLayoutPosture, StableBasisReadRequest, StableBasisReadScope,
};

fn main() {
    let basis = StableBasisReadRequest::new(
        BranchId("main".to_string()),
        CommitId(1),
        StableBasisReadScope::SingleEntity(forge_store::SingleEntityAspectScope::new("entity-a")),
        "support:ctx:v1",
        "schema-support:v1",
        StableBasisLayoutPosture::ProofOnly,
        "authority:basis:v1",
        ContinuationRetentionStatus::Retained,
    );
    let budget = ContinuationBatchBudget::new(
        FetchWidth::new(16),
        MaxBatchItems::new(32),
        MaxCoveredCommits::new(4),
        MaxMaterializedBytes::new(4096),
        MaxSupportRowsPerBatch::new(16),
    );
    let request = CursorContinuationRequest::new(
        "cursor-main",
        "subscriber-a",
        BranchId("main".to_string()),
        "demo-feed",
        "schema:v1",
        1,
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        budget,
    );
    let _ = CursorContinuationPlan::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        ContinuationStrategy::AdmittedLayoutNarrow,
    );
    let _ = (basis, request);
}
