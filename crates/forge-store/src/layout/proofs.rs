use crate::{
    failure::{StoreError, StoreErrorKind},
    ComplexityStatus,
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::constants::{
    CHUNK_SHAPE_VERSION, EQUIVALENCE_CONTRACT_VERSION,
    FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ,
    FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH,
    FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY,
    FIRST_SHIP_MAX_DETERMINISTIC_CHUNK_WIDTH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedAspectSlicesPerRead(u64);

impl MaxAdmittedAspectSlicesPerRead {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedBlockDecodeBreadth(u64);

impl MaxAdmittedBlockDecodeBreadth {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedControlReplayBreadthForParity(u64);

impl MaxAdmittedControlReplayBreadthForParity {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxDeterministicChunkWidth(u64);

impl MaxDeterministicChunkWidth {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChunkShapeVersion(u32);

impl ChunkShapeVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EquivalenceContractVersion(u32);

impl EquivalenceContractVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AspectLayoutSliceId(String);

impl AspectLayoutSliceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StructuralBlockId(String);

impl StructuralBlockId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PhysicalChunkId(String);

impl PhysicalChunkId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SingleEntityAspectScope {
    entity_id: String,
}

impl SingleEntityAspectScope {
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
        }
    }

    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySetUniformAspectScope {
    entity_ids: Vec<String>,
}

impl EntitySetUniformAspectScope {
    pub fn new(entity_ids: Vec<String>) -> Self {
        Self { entity_ids }
    }

    pub fn entity_ids(&self) -> &[String] {
        &self.entity_ids
    }

    pub(crate) fn canonical_entity_ids(&self) -> Vec<String> {
        canonicalize_strings(&self.entity_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CdcTouchedAspectScope {
    cdc_token: String,
    touched_entity_ids: Vec<String>,
}

impl CdcTouchedAspectScope {
    pub fn new(cdc_token: impl Into<String>, touched_entity_ids: Vec<String>) -> Self {
        Self {
            cdc_token: cdc_token.into(),
            touched_entity_ids,
        }
    }

    pub fn cdc_token(&self) -> &str {
        &self.cdc_token
    }

    pub fn touched_entity_ids(&self) -> &[String] {
        &self.touched_entity_ids
    }

    pub(crate) fn canonical_touched_entity_ids(&self) -> Vec<String> {
        canonicalize_strings(&self.touched_entity_ids)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectScopeClass {
    SingleEntity(SingleEntityAspectScope),
    EntitySetUniform(EntitySetUniformAspectScope),
    CdcTouched(CdcTouchedAspectScope),
    Generalized { descriptor: String },
}

impl AspectScopeClass {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SingleEntity(_) => "single_entity",
            Self::EntitySetUniform(_) => "entity_set_uniform",
            Self::CdcTouched(_) => "cdc_touched",
            Self::Generalized { .. } => "generalized",
        }
    }

    pub(crate) fn canonical_scope_key(&self) -> Result<CanonicalScopeKey, StoreError> {
        match self {
            Self::SingleEntity(scope) => {
                let entity_id = scope.entity_id.trim();
                if entity_id.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "single-entity aspect scope requires a non-empty entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members: vec![entity_id.to_string()],
                    cdc_token: None,
                })
            }
            Self::EntitySetUniform(scope) => {
                let members = scope.canonical_entity_ids();
                if members.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "entity-set uniform aspect scope requires at least one entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members,
                    cdc_token: None,
                })
            }
            Self::CdcTouched(scope) => {
                let cdc_token = scope.cdc_token.trim();
                if cdc_token.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "cdc-touched aspect scope requires a non-empty CDC token",
                    ));
                }
                let members = scope.canonical_touched_entity_ids();
                if members.is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "cdc-touched aspect scope requires at least one touched entity id",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members,
                    cdc_token: Some(cdc_token.to_string()),
                })
            }
            Self::Generalized { descriptor } => {
                if descriptor.trim().is_empty() {
                    return Err(StoreError::new(
                        StoreErrorKind::AspectScopeAmbiguous,
                        "generalized aspect scope requires a non-empty descriptor",
                    ));
                }
                Ok(CanonicalScopeKey {
                    scope_label: self.label().to_string(),
                    members: vec![descriptor.trim().to_string()],
                    cdc_token: None,
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutTarget {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
}

impl AspectLayoutTarget {
    pub fn new(branch_id: BranchId, frontier_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectProjectionSet {
    aspect_names: Vec<String>,
}

impl AspectProjectionSet {
    pub fn new(aspect_names: Vec<String>) -> Self {
        Self { aspect_names }
    }

    pub fn aspect_names(&self) -> &[String] {
        &self.aspect_names
    }

    pub(crate) fn canonical_aspects(&self) -> Result<Vec<String>, StoreError> {
        let aspects = canonicalize_strings(&self.aspect_names);
        if aspects.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::AspectScopeAmbiguous,
                "aspect projection set requires at least one aspect name",
            ));
        }
        Ok(aspects)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutReadRequest {
    target: AspectLayoutTarget,
    scope_class: AspectScopeClass,
    projection_set: AspectProjectionSet,
}

impl AspectLayoutReadRequest {
    pub fn new(
        target: AspectLayoutTarget,
        scope_class: AspectScopeClass,
        projection_set: AspectProjectionSet,
    ) -> Self {
        Self {
            target,
            scope_class,
            projection_set,
        }
    }

    pub fn target(&self) -> &AspectLayoutTarget {
        &self.target
    }

    pub fn scope_class(&self) -> &AspectScopeClass {
        &self.scope_class
    }

    pub fn projection_set(&self) -> &AspectProjectionSet {
        &self.projection_set
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectReadRegime {
    DirectLayoutSlice,
    BlockReuseBacked,
    ControlReplay,
    ExplicitBroadFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectLayoutFallbackClass {
    None,
    UnsupportedScopeClass,
    BudgetExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectLayoutPerformanceEnvelope {
    pub strategy: AspectReadRegime,
    pub scope_class: String,
    pub complexity_status: ComplexityStatus,
    pub fallback_class: AspectLayoutFallbackClass,
    pub layout_slices_read: usize,
    pub blocks_decoded: usize,
    pub control_replay_breadth: usize,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedAspectLayoutReadPlan {
    request: AspectLayoutReadRequest,
    slice_ids: Vec<AspectLayoutSliceId>,
    structural_block_id: StructuralBlockId,
    performance: AspectLayoutPerformanceEnvelope,
}

impl AdmittedAspectLayoutReadPlan {
    pub(crate) fn new(
        request: AspectLayoutReadRequest,
        slice_ids: Vec<AspectLayoutSliceId>,
        structural_block_id: StructuralBlockId,
        performance: AspectLayoutPerformanceEnvelope,
    ) -> Self {
        Self {
            request,
            slice_ids,
            structural_block_id,
            performance,
        }
    }

    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }

    pub fn slice_ids(&self) -> &[AspectLayoutSliceId] {
        &self.slice_ids
    }

    pub fn structural_block_id(&self) -> &StructuralBlockId {
        &self.structural_block_id
    }

    pub fn performance(&self) -> &AspectLayoutPerformanceEnvelope {
        &self.performance
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExplicitBroadFallbackPlan {
    request: AspectLayoutReadRequest,
    performance: AspectLayoutPerformanceEnvelope,
    reason: String,
}

impl ExplicitBroadFallbackPlan {
    pub(crate) fn new(
        request: AspectLayoutReadRequest,
        performance: AspectLayoutPerformanceEnvelope,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            request,
            performance,
            reason: reason.into(),
        }
    }

    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }

    pub fn performance(&self) -> &AspectLayoutPerformanceEnvelope {
        &self.performance
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RejectedAspectLayoutReadPlan {
    request: AspectLayoutReadRequest,
    reason: String,
}

impl RejectedAspectLayoutReadPlan {
    pub(crate) fn new(request: AspectLayoutReadRequest, reason: impl Into<String>) -> Self {
        Self {
            request,
            reason: reason.into(),
        }
    }

    pub fn request(&self) -> &AspectLayoutReadRequest {
        &self.request
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum AspectLayoutReadPlanDecision {
    Admitted(AdmittedAspectLayoutReadPlan),
    Fallback(ExplicitBroadFallbackPlan),
    Rejected(RejectedAspectLayoutReadPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DedupAdmittedBlockReuse {
    branch_id: BranchId,
    frontier_commit_id: CommitId,
    scope_class: String,
    structural_block_id: StructuralBlockId,
    equivalence_contract_version: EquivalenceContractVersion,
    slice_ids: Vec<AspectLayoutSliceId>,
}

impl DedupAdmittedBlockReuse {
    pub(crate) fn new(
        plan: &AdmittedAspectLayoutReadPlan,
        equivalence_contract_version: EquivalenceContractVersion,
    ) -> Self {
        Self {
            branch_id: plan.request.target.branch_id.clone(),
            frontier_commit_id: plan.request.target.frontier_commit_id,
            scope_class: plan.request.scope_class.label().to_string(),
            structural_block_id: plan.structural_block_id.clone(),
            equivalence_contract_version,
            slice_ids: plan.slice_ids.clone(),
        }
    }

    pub(crate) fn from_parts(
        branch_id: BranchId,
        frontier_commit_id: CommitId,
        scope_class: String,
        structural_block_id: StructuralBlockId,
        equivalence_contract_version: EquivalenceContractVersion,
        slice_ids: Vec<AspectLayoutSliceId>,
    ) -> Self {
        Self {
            branch_id,
            frontier_commit_id,
            scope_class,
            structural_block_id,
            equivalence_contract_version,
            slice_ids,
        }
    }

    pub fn structural_block_id(&self) -> &StructuralBlockId {
        &self.structural_block_id
    }

    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn frontier_commit_id(&self) -> CommitId {
        self.frontier_commit_id
    }

    pub(crate) fn scope_class(&self) -> &str {
        &self.scope_class
    }

    pub fn equivalence_contract_version(&self) -> EquivalenceContractVersion {
        self.equivalence_contract_version
    }

    pub fn slice_ids(&self) -> &[AspectLayoutSliceId] {
        &self.slice_ids
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6LayoutMaterialization {
    artifact_id: String,
    admitted_plan: AdmittedAspectLayoutReadPlan,
    block_reuse: DedupAdmittedBlockReuse,
    frozen_layout: ChunkModelFrozenPhysicalLayout,
    milestone_7_reference: Milestone7IndependentLayoutReference,
    milestone_9_reference: Milestone9PhysicalChunkReference,
}

impl Milestone6LayoutMaterialization {
    pub(crate) fn new(
        artifact_id: String,
        admitted_plan: AdmittedAspectLayoutReadPlan,
        block_reuse: DedupAdmittedBlockReuse,
        frozen_layout: ChunkModelFrozenPhysicalLayout,
        milestone_7_reference: Milestone7IndependentLayoutReference,
        milestone_9_reference: Milestone9PhysicalChunkReference,
    ) -> Self {
        Self {
            artifact_id,
            admitted_plan,
            block_reuse,
            frozen_layout,
            milestone_7_reference,
            milestone_9_reference,
        }
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    pub fn admitted_plan(&self) -> &AdmittedAspectLayoutReadPlan {
        &self.admitted_plan
    }

    pub fn block_reuse(&self) -> &DedupAdmittedBlockReuse {
        &self.block_reuse
    }

    pub fn frozen_layout(&self) -> &ChunkModelFrozenPhysicalLayout {
        &self.frozen_layout
    }

    pub fn milestone_7_reference(&self) -> &Milestone7IndependentLayoutReference {
        &self.milestone_7_reference
    }

    pub fn milestone_9_reference(&self) -> &Milestone9PhysicalChunkReference {
        &self.milestone_9_reference
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct CanonicalScopeKey {
    scope_label: String,
    members: Vec<String>,
    cdc_token: Option<String>,
}

pub fn stable_layout_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let canonical = serde_json::to_vec(value)
        .expect("serializing deterministic layout digest input should succeed");
    let mut hasher = Sha256::new();
    hasher.update(canonical);
    format!("{:x}", hasher.finalize())
}

fn canonicalize_strings(values: &[String]) -> Vec<String> {
    let mut canonical = values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    canonical.sort();
    canonical.dedup();
    canonical
}

pub(crate) fn aspect_projection_digest(
    projection_set: &AspectProjectionSet,
) -> Result<String, StoreError> {
    Ok(stable_layout_digest(&projection_set.canonical_aspects()?))
}

pub(crate) fn canonical_slice_ids(
    request: &AspectLayoutReadRequest,
) -> Result<Vec<AspectLayoutSliceId>, StoreError> {
    let projection_digest = aspect_projection_digest(&request.projection_set)?;
    let scope_key = request.scope_class.canonical_scope_key()?;
    Ok(scope_key
        .members
        .into_iter()
        .map(|member| {
            AspectLayoutSliceId::new(stable_layout_digest(&(
                request.scope_class.label(),
                &request.target.branch_id,
                request.target.frontier_commit_id,
                &projection_digest,
                &member,
                CHUNK_SHAPE_VERSION.value(),
                EQUIVALENCE_CONTRACT_VERSION.value(),
            )))
        })
        .collect())
}

pub(crate) fn structural_block_id_for_plan(
    request: &AspectLayoutReadRequest,
    slice_ids: &[AspectLayoutSliceId],
) -> Result<StructuralBlockId, StoreError> {
    let projection_digest = aspect_projection_digest(&request.projection_set)?;
    let scope_key = request.scope_class.canonical_scope_key()?;
    Ok(StructuralBlockId::new(stable_layout_digest(&(
        request.scope_class.label(),
        &request.target.branch_id,
        request.target.frontier_commit_id,
        &projection_digest,
        CHUNK_SHAPE_VERSION.value(),
        EQUIVALENCE_CONTRACT_VERSION.value(),
        scope_key.members,
        slice_ids.iter().map(AspectLayoutSliceId::as_str).collect::<Vec<_>>(),
    ))))
}

pub(crate) fn classify_layout_request(
    request: AspectLayoutReadRequest,
) -> Result<AspectLayoutReadPlanDecision, StoreError> {
    let slice_ids = canonical_slice_ids(&request)?;
    let scope_class = request.scope_class().clone();
    match scope_class {
        AspectScopeClass::Generalized { descriptor } => Ok(AspectLayoutReadPlanDecision::Fallback(
            ExplicitBroadFallbackPlan::new(
                request,
                AspectLayoutPerformanceEnvelope {
                    strategy: AspectReadRegime::ExplicitBroadFallback,
                    scope_class: "generalized".to_string(),
                    complexity_status: ComplexityStatus::Debt,
                    fallback_class: AspectLayoutFallbackClass::UnsupportedScopeClass,
                    layout_slices_read: 0,
                    blocks_decoded: 0,
                    control_replay_breadth: slice_ids.len(),
                    chunk_count: 0,
                },
                format!(
                    "generalized aspect scope `{}` is not admitted for Milestone 6 Phase 1",
                    descriptor
                ),
            ),
        )),
        AspectScopeClass::SingleEntity(_)
        | AspectScopeClass::EntitySetUniform(_)
        | AspectScopeClass::CdcTouched(_) => {
            if slice_ids.is_empty() {
                return Ok(AspectLayoutReadPlanDecision::Rejected(
                    RejectedAspectLayoutReadPlan::new(
                        request,
                        "aspect layout request does not identify any canonical slices",
                    ),
                ));
            }
            if slice_ids.len() as u64 > FIRST_SHIP_MAX_ADMITTED_ASPECT_SLICES_PER_READ.value()
                || slice_ids.len() as u64 > FIRST_SHIP_MAX_ADMITTED_BLOCK_DECODE_BREADTH.value()
            {
                return Ok(AspectLayoutReadPlanDecision::Fallback(
                    ExplicitBroadFallbackPlan::new(
                        request.clone(),
                        AspectLayoutPerformanceEnvelope {
                            strategy: AspectReadRegime::ExplicitBroadFallback,
                            scope_class: request.scope_class.label().to_string(),
                            complexity_status: ComplexityStatus::Debt,
                            fallback_class: AspectLayoutFallbackClass::BudgetExceeded,
                            layout_slices_read: slice_ids.len(),
                            blocks_decoded: slice_ids.len(),
                            control_replay_breadth: slice_ids.len(),
                            chunk_count: 0,
                        },
                        "aspect layout request exceeded the first-ship admitted local budget",
                    ),
                ));
            }

            let structural_block_id = structural_block_id_for_plan(&request, &slice_ids)?;
            let regime = if slice_ids.len() == 1 {
                AspectReadRegime::DirectLayoutSlice
            } else {
                AspectReadRegime::BlockReuseBacked
            };
            Ok(AspectLayoutReadPlanDecision::Admitted(
                AdmittedAspectLayoutReadPlan::new(
                    request.clone(),
                    slice_ids.clone(),
                    structural_block_id,
                    AspectLayoutPerformanceEnvelope {
                        strategy: regime,
                        scope_class: request.scope_class.label().to_string(),
                        complexity_status: ComplexityStatus::Verified,
                        fallback_class: AspectLayoutFallbackClass::None,
                        layout_slices_read: slice_ids.len(),
                        blocks_decoded: slice_ids.len(),
                        control_replay_breadth: slice_ids
                            .len()
                            .min(FIRST_SHIP_MAX_ADMITTED_CONTROL_REPLAY_BREADTH_FOR_PARITY.value()
                                as usize),
                        chunk_count: 0,
                    },
                ),
            ))
        }
    }
}

pub(crate) fn freeze_chunk_model_from_plan(
    plan: &AdmittedAspectLayoutReadPlan,
) -> Result<ChunkModelFrozenPhysicalLayout, StoreError> {
    let chunk_width = plan.slice_ids.len() as u64;
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
    let ordered_slice_ids = plan.slice_ids.clone();
    let determinism_digest = stable_layout_digest(&(
        plan.request.target.branch_id.clone(),
        plan.request.target.frontier_commit_id,
        plan.request.scope_class.label(),
        aspect_projection_digest(&plan.request.projection_set)?,
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
        plan.request.clone(),
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
        plan.request.target.branch_id.clone(),
        plan.request.target.frontier_commit_id,
        plan.request.scope_class.label().to_string(),
        aspect_projection_digest(&plan.request.projection_set)?,
    ))
}

pub(crate) fn admit_milestone_9_reference_from_frozen(
    frozen: &ChunkModelFrozenPhysicalLayout,
) -> Milestone9PhysicalChunkReference {
    Milestone9PhysicalChunkReference::new(
        frozen.witness.physical_chunk_id.clone(),
        frozen.witness.chunk_shape_version,
        frozen.witness.determinism_digest.clone(),
        frozen.witness.ordered_slice_ids.len(),
    )
}

pub(crate) fn layout_materialization_artifact_id(
    plan: &AdmittedAspectLayoutReadPlan,
) -> String {
    let basis = (
        plan.request().target().branch_id().clone(),
        plan.request().target().frontier_commit_id(),
        plan.request().scope_class().label(),
        plan.slice_ids()
            .iter()
            .map(AspectLayoutSliceId::as_str)
            .collect::<Vec<_>>(),
        plan.structural_block_id().as_str(),
    );
    format!("layout-materialization:{}", stable_layout_digest(&basis))
}

pub(crate) fn layout_scope_membership_artifact_id(
    request: &AspectLayoutReadRequest,
) -> Result<String, StoreError> {
    Ok(format!(
        "layout-scope-membership:{}",
        stable_layout_digest(&(
            request.target().branch_id().clone(),
            request.target().frontier_commit_id(),
            request.scope_class().label(),
            aspect_projection_digest(request.projection_set())?,
        ))
    ))
}

pub(crate) fn chunk_membership_artifact_id(
    frozen: &ChunkModelFrozenPhysicalLayout,
) -> String {
    format!(
        "layout-chunk-membership:{}",
        frozen.witness().physical_chunk_id().as_str()
    )
}
