//! Schema types for sheet region merge execution.
//!
//! DOMAIN: Intent-stable selection, snapshot-scoped execution plans, results,
//! and typed output for the `execute_sheet_region_merge` compound algorithm
//! (spec §5.7–5.8).
//!
//! DEPENDENCIES: `forge-topo::bitset`, `forge-topo::handles`, `KernelState`.
//! INVARIANTS:
//!   - `MergeRegionSelection` is intent-level (face sets + optional radial selectors)
//!   - `MergePlan` is snapshot-scoped (raw indices, deterministic order, hash)
//!   - `MergeResult` is the output envelope

use forge_topo::bitset::EntityBitset;
use forge_topo::handles::FaceId;

use crate::core::KernelState;

/// Intent-level selection for a sheet region merge.
///
/// Stable, serializable, agent-facing. Determines **what** to merge.
/// Does not contain ephemeral handles — only face bitsets and optional
/// radial-use selectors for disambiguation.
#[derive(Debug)]
pub struct MergeRegionSelection {
    /// Faces to be merged together (surviving + all killed faces).
    selected_faces: EntityBitset,
    /// Faces that are NOT part of this merge but share edges with selected faces.
    /// Their radial ring entries must be preserved.
    protected_faces: EntityBitset,
    /// The face that survives after all kills. Must be in `selected_faces`.
    surviving_face: FaceId,
    /// Optional explicit radial-use selectors for edges with valence > 3.
    /// Required when face-only selection is ambiguous on a given edge.
    selected_radial_uses: Vec<RadialUseSelector>,
}

impl MergeRegionSelection {
    /// Create a new merge selection.
    pub fn new(
        selected_faces: EntityBitset,
        protected_faces: EntityBitset,
        surviving_face: FaceId,
    ) -> Self {
        Self {
            selected_faces,
            protected_faces,
            surviving_face,
            selected_radial_uses: Vec::new(),
        }
    }

    /// Create a selection with explicit radial-use disambiguation.
    pub fn with_radial_selectors(
        selected_faces: EntityBitset,
        protected_faces: EntityBitset,
        surviving_face: FaceId,
        selected_radial_uses: Vec<RadialUseSelector>,
    ) -> Self {
        Self {
            selected_faces,
            protected_faces,
            surviving_face,
            selected_radial_uses,
        }
    }

    /// The set of faces to merge.
    pub fn get_selected_faces(&self) -> &EntityBitset {
        &self.selected_faces
    }

    /// Faces that must not be modified.
    pub fn get_protected_faces(&self) -> &EntityBitset {
        &self.protected_faces
    }

    /// The face that survives all merges.
    pub fn get_surviving_face(&self) -> FaceId {
        self.surviving_face
    }

    /// Explicit radial-use selectors for ambiguous edges.
    pub fn get_radial_selectors(&self) -> &[RadialUseSelector] {
        &self.selected_radial_uses
    }
}

/// Per-edge disambiguation when face-only selection is ambiguous.
///
/// Specifies which face pair to merge on a specific edge.
/// Uses raw arena face indices (not halfedge indices) for serializability.
#[derive(Debug)]
pub struct RadialUseSelector {
    /// Arena index of the edge to disambiguate.
    edge_index: u32,
    /// Arena index of the face that survives on this edge.
    survive_face_index: u32,
    /// Arena index of the face that is killed on this edge.
    kill_face_index: u32,
}

impl RadialUseSelector {
    /// Create a new radial use selector with face indices.
    pub fn new(edge_index: u32, survive_face_index: u32, kill_face_index: u32) -> Self {
        Self { edge_index, survive_face_index, kill_face_index }
    }

    /// The edge being disambiguated.
    pub fn get_edge_index(&self) -> u32 { self.edge_index }

    /// Arena index of the surviving face.
    pub fn get_survive_face_index(&self) -> u32 { self.survive_face_index }

    /// Arena index of the killed face.
    pub fn get_kill_face_index(&self) -> u32 { self.kill_face_index }
}

/// Snapshot-scoped execution blueprint for a merge sequence.
///
/// Deterministic: steps are sorted by `edge_index`. The `plan_hash` is
/// computed over the step sequence for trace stability.
/// This is derived from a `MergeRegionSelection` + a topology snapshot.
#[derive(Debug)]
pub struct MergePlan {
    /// Ordered merge steps (sorted by edge_index for determinism).
    steps: Vec<MergeStepPlan>,
    /// Deterministic hash over the step sequence.
    plan_hash: u64,
}

impl MergePlan {
    /// Create a new merge plan.
    pub fn new(steps: Vec<MergeStepPlan>) -> Self {
        let plan_hash = Self::compute_hash(&steps);
        Self { steps, plan_hash }
    }

    /// The ordered steps.
    pub fn get_steps(&self) -> &[MergeStepPlan] {
        &self.steps
    }

    /// Deterministic hash for trace comparison.
    pub fn get_plan_hash(&self) -> u64 {
        self.plan_hash
    }

    /// Number of steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Compute FNV-1a hash over the step sequence.
    fn compute_hash(steps: &[MergeStepPlan]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for step in steps {
            h = h.wrapping_mul(0x100000001b3) ^ (step.edge_index as u64);
            h = h.wrapping_mul(0x100000001b3) ^ (step.survive_face_index as u64);
            h = h.wrapping_mul(0x100000001b3) ^ (step.kill_face_index as u64);
        }
        h
    }
}

/// One edge to merge in the execution plan.
///
/// Uses raw indices (not opaque handles) for serializability and replay.
/// Handles are re-derived from the draft arena at execution time.
#[derive(Debug, Clone, Copy)]
pub struct MergeStepPlan {
    /// Arena index of the edge to merge.
    pub edge_index: u32,
    /// Arena index of the surviving face.
    pub survive_face_index: u32,
    /// Arena index of the killed face.
    pub kill_face_index: u32,
}

/// Output of a successful merge execution.
#[derive(Debug)]
pub struct MergeResult {
    /// The face that absorbed all killed faces.
    surviving_face: FaceId,
    /// All faces that were killed during the merge.
    killed_faces: Vec<FaceId>,
    /// The execution plan that was applied.
    plan: MergePlan,
}

impl MergeResult {
    /// Create a new merge result.
    pub fn new(surviving_face: FaceId, killed_faces: Vec<FaceId>, plan: MergePlan) -> Self {
        Self { surviving_face, killed_faces, plan }
    }

    /// The surviving face.
    pub fn get_surviving_face(&self) -> FaceId {
        self.surviving_face
    }

    /// All killed faces.
    pub fn get_killed_faces(&self) -> &[FaceId] {
        &self.killed_faces
    }

    /// The plan that was executed.
    pub fn get_plan(&self) -> &MergePlan {
        &self.plan
    }
}

/// Typed output of `execute_sheet_region_merge`.
///
/// Bundles the committed `KernelState` with the `MergeResult` metadata.
/// Avoids tuple-position bugs and makes the API self-documenting.
#[derive(Debug)]
pub struct SheetRegionMergeOutput {
    /// The committed kernel state after all merge steps.
    state: KernelState,
    /// Metadata about what was merged.
    merge: MergeResult,
}

impl SheetRegionMergeOutput {
    /// Create a new merge output.
    pub fn new(state: KernelState, merge: MergeResult) -> Self {
        Self { state, merge }
    }

    /// The committed kernel state reflecting all merge mutations.
    pub fn get_state(&self) -> &KernelState {
        &self.state
    }

    /// Consume and return the committed kernel state.
    pub fn into_state(self) -> KernelState {
        self.state
    }

    /// Merge metadata (surviving face, killed faces, plan).
    pub fn get_merge(&self) -> &MergeResult {
        &self.merge
    }

    /// Consume and return both parts.
    pub fn into_parts(self) -> (KernelState, MergeResult) {
        (self.state, self.merge)
    }
}
