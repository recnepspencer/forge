use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

use super::super::constants::FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH;
use super::{
    core::{AspectLayoutSliceId, ChunkShapeVersion, PhysicalChunkId},
    digests::{aspect_projection_digest, stable_layout_digest},
    planning::AdmittedAspectLayoutReadPlan,
    scopes::AspectLayoutReadRequest,
};
use crate::layout::CHUNK_SHAPE_VERSION;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChunkDeterminismWitness {
    physical_chunk_id: PhysicalChunkId,
    chunk_shape_version: ChunkShapeVersion,
    determinism_digest: String,
    ordered_slice_ids: Vec<AspectLayoutSliceId>,
}
impl ChunkDeterminismWitness {
    pub(crate) fn new(
        physical_chunk_id: PhysicalChunkId,
        chunk_shape_version: ChunkShapeVersion,
        determinism_digest: String,
        ordered_slice_ids: Vec<AspectLayoutSliceId>,
    ) -> Self {
        Self {
            physical_chunk_id,
            chunk_shape_version,
            determinism_digest,
            ordered_slice_ids,
        }
    }
    pub fn physical_chunk_id(&self) -> &PhysicalChunkId {
        &self.physical_chunk_id
    }
    pub fn chunk_shape_version(&self) -> ChunkShapeVersion {
        self.chunk_shape_version
    }
    pub fn determinism_digest(&self) -> &str {
        &self.determinism_digest
    }
    pub fn ordered_slice_ids(&self) -> &[AspectLayoutSliceId] {
        &self.ordered_slice_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChunkModelFrozenPhysicalLayout {
    request: AspectLayoutReadRequest,
    chunk_width: u64,
    witness: ChunkDeterminismWitness,
}
impl ChunkModelFrozenPhysicalLayout {
    pub(crate) fn new(
        request: AspectLayoutReadRequest,
        chunk_width: u64,
        witness: ChunkDeterminismWitness,
    ) -> Self {
        Self {
            request,
            chunk_width,
            witness,
        }
    }
    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }
    pub fn chunk_width(&self) -> u64 {
        self.chunk_width
    }
    pub fn witness(&self) -> &ChunkDeterminismWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7IndependentLayoutReference {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    projection_digest: String,
}
impl Milestone7IndependentLayoutReference {
    pub(crate) fn new(
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        scope_class: String,
        projection_digest: String,
    ) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
            scope_class,
            projection_digest,
        }
    }
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
    pub fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }
    pub fn scope_class(&self) -> &str {
        &self.scope_class
    }
    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone9PhysicalChunkReference {
    physical_chunk_id: PhysicalChunkId,
    chunk_shape_version: ChunkShapeVersion,
    determinism_digest: String,
    chunk_member_count: usize,
}
impl Milestone9PhysicalChunkReference {
    pub(crate) fn new(
        physical_chunk_id: PhysicalChunkId,
        chunk_shape_version: ChunkShapeVersion,
        determinism_digest: String,
        chunk_member_count: usize,
    ) -> Self {
        Self {
            physical_chunk_id,
            chunk_shape_version,
            determinism_digest,
            chunk_member_count,
        }
    }
    pub fn physical_chunk_id(&self) -> &PhysicalChunkId {
        &self.physical_chunk_id
    }
    pub fn chunk_shape_version(&self) -> ChunkShapeVersion {
        self.chunk_shape_version
    }
    pub fn determinism_digest(&self) -> &str {
        &self.determinism_digest
    }
    pub fn chunk_member_count(&self) -> usize {
        self.chunk_member_count
    }
}

pub(crate) fn freeze_chunk_model_from_plan(
    plan: &AdmittedAspectLayoutReadPlan,
) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
    let chunk_width = plan.slice_ids().len() as u64;
    if chunk_width == 0 || chunk_width > FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH.value() {
        return Err(StoreError::new(
            StoreErrorKind::PhysicalChunkDeterminismViolation,
            format!(
                "layout chunk width {} is outside the first-ship deterministic chunk budget {}",
                chunk_width,
                FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH.value()
            ),
        ));
    }
    let ordered_slice_ids = plan.slice_ids().to_vec();
    let determinism_digest = stable_layout_digest(&(
        plan.request().target().branch_id().clone(),
        plan.request().target().frontier_commit_id(),
        plan.request().scope_class().label(),
        aspect_projection_digest(plan.request().projection_set())?,
        CHUNK_SHAPE_VERSION.value(),
        ordered_slice_ids
            .iter()
            .map(AspectLayoutSliceId::as_str)
            .collect::<Vec<_>>(),
    ));
    let physical_chunk_id = PhysicalChunkId::new(stable_layout_digest(&(
        determinism_digest.clone(),
        CHUNK_SHAPE_VERSION.value(),
    )));
    Ok(ChunkModelFrozenPhysicalLayout::new(
        plan.request().clone(),
        chunk_width,
        ChunkDeterminismWitness::new(
            physical_chunk_id,
            CHUNK_SHAPE_VERSION,
            determinism_digest,
            ordered_slice_ids,
        ),
    ))
}

pub(crate) fn admit_milestone_7_reference_from_plan(
    plan: &AdmittedAspectLayoutReadPlan,
) -> Result<Milestone7IndependentLayoutReference, StoreError> {
    Ok(Milestone7IndependentLayoutReference::new(
        plan.request().target().branch_id().clone(),
        plan.request().target().frontier_commit_id(),
        plan.request().scope_class().label().to_string(),
        aspect_projection_digest(plan.request().projection_set())?,
    ))
}

pub(crate) fn admit_milestone_9_reference_from_frozen(
    frozen: &ChunkModelFrozenPhysicalLayout,
) -> Milestone9PhysicalChunkReference {
    Milestone9PhysicalChunkReference::new(
        frozen.witness().physical_chunk_id().clone(),
        frozen.witness().chunk_shape_version(),
        frozen.witness().determinism_digest().to_string(),
        frozen.witness().ordered_slice_ids().len(),
    )
}
