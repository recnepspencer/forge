#[path = "proofs/core.rs"]
mod core;
#[path = "proofs/digests.rs"]
mod digests;
#[path = "proofs/execution.rs"]
mod execution;
#[path = "proofs/lookup.rs"]
mod lookup;
#[path = "proofs/physical.rs"]
mod physical;
#[path = "proofs/planning.rs"]
mod planning;
#[path = "proofs/scopes.rs"]
mod scopes;

pub(crate) use digests::{
    chunk_membership_artifact_id, layout_materialization_artifact_id,
    layout_scope_membership_artifact_id, published_layout_request_artifact_id,
    stable_layout_digest, stable_layout_truth_digest, structural_block_artifact_id,
};
pub(crate) use physical::{
    admit_milestone_7_reference_from_plan, admit_milestone_9_reference_from_frozen,
    freeze_chunk_model_from_plan,
};
pub(crate) use planning::classify_layout_request;

pub use core::{
    AspectLayoutSliceId, ChunkShapeVersion, EquivalenceContractVersion, MaxAdmittedAspectSlicesPerRead,
    MaxAdmittedBlockDecodeBreadth, MaxAdmittedControlReplayBreadthForParity,
    MaxDeterministicChunkWidth, Milestone6LayoutSupportLane, Milestone6LayoutSupportPolicy,
    Milestone6LayoutSupportPublicationDisposition, Milestone6PreparedLayoutSupport,
    Milestone6ResolvedLayoutSupportLane, PhysicalChunkId, StructuralBlockId,
};
pub use execution::{
    AspectLayoutControlTruth, AspectLayoutReadExecutionDecision,
    AspectLayoutReadExecutionResult, DedupBackedReadResult, Milestone6ChunkModelExport,
    Milestone6DerivedArtifactRebuildReport, Milestone6LayoutMaterialization,
};
pub use lookup::{StructuralBlockLookup, StructuralBlockLookupResult};
pub use physical::{
    ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout,
    Milestone7IndependentLayoutReference, Milestone9PhysicalChunkReference,
};
pub use planning::{
    AdmittedAspectLayoutReadPlan, AspectLayoutFallbackClass, AspectLayoutPerformanceEnvelope,
    AspectLayoutReadPlanDecision, DedupAdmittedBlockReuse, ExplicitBroadFallbackPlan,
    RejectedAspectLayoutReadPlan,
};
pub use scopes::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectReadRegime,
    AspectScopeClass, CdcTouchedAspectScope,
    EntitySetUniformAspectScope, SingleEntityAspectScope,
};
