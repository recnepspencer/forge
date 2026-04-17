mod constants;
mod proofs;

pub use constants::{
    CHUNK_SHAPE_VERSION, EQUIVALENCE_CONTRACT_VERSION,
    FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ,
    FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH,
    FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY,
    FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH, LAYOUT_FAMILY_VERSION,
    STRUCTURAL_BLOCK_FAMILY_VERSION,
};
pub use proofs::{
    AdmittedAspectLayoutReadPlan, AspectLayoutFallbackClass, AspectLayoutPerformanceEnvelope,
    AspectLayoutControlTruth, AspectLayoutReadExecutionDecision,
    AspectLayoutReadExecutionResult,
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutSliceId,
    AspectLayoutTarget, AspectProjectionSet, AspectReadRegime, AspectScopeClass,
    CdcTouchedAspectScope, ChunkDeterminismWitness, ChunkModelFrozenPhysicalLayout,
    ChunkShapeVersion, DedupAdmittedBlockReuse, EntitySetUniformAspectScope,
    EquivalenceContractVersion, ExplicitBroadFallbackPlan, MaxAdmittedAspectSlicesPerRead,
    MaxAdmittedBlockDecodeBreadth, MaxAdmittedControlReplayBreadthForParity,
    MaxDeterministicChunkWidth, Milestone6ChunkModelExport,
    Milestone6DerivedArtifactRebuildReport,
    Milestone7IndependentLayoutReference,
    Milestone6LayoutMaterialization, Milestone9PhysicalChunkReference, PhysicalChunkId,
    RejectedAspectLayoutReadPlan, StructuralBlockLookup, StructuralBlockLookupResult,
    DedupBackedReadResult,
    SingleEntityAspectScope, StructuralBlockId,
};
pub(crate) use proofs::{
    admit_milestone_7_reference_from_plan, admit_milestone_9_reference_from_frozen,
    chunk_membership_artifact_id, classify_layout_request, freeze_chunk_model_from_plan,
    layout_materialization_artifact_id, layout_scope_membership_artifact_id,
    published_layout_request_artifact_id, stable_layout_digest, stable_layout_truth_digest,
    structural_block_artifact_id,
};
