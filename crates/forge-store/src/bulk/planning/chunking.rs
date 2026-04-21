use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use super::{
    core::{BulkPlanKind, BulkSourceMember},
    freeze::{FrozenBulkSourceManifest, FrozenTransformBasis, FrozenTransformTargetPartition},
    utils::{serialization_error, stable_digest, BULK_FAMILY_VERSION},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkWidthBudget {
    max_width_units: u64,
}

impl ChunkWidthBudget {
    pub fn new(max_width_units: u64) -> Self {
        Self { max_width_units }
    }
    pub fn max_width_units(&self) -> u64 {
        self.max_width_units
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChunkOrdinal(u64);

impl ChunkOrdinal {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedBulkChunk {
    ordinal: ChunkOrdinal,
    member_ids: Vec<String>,
    width_units: u64,
}

impl PlannedBulkChunk {
    pub fn ordinal(&self) -> ChunkOrdinal {
        self.ordinal
    }
    pub fn member_ids(&self) -> &[String] {
        &self.member_ids
    }
    pub fn width_units(&self) -> u64 {
        self.width_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicChunkPlan {
    family_version: u32,
    kind: BulkPlanKind,
    program_id: String,
    plan_id: String,
    source_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: Option<CommitId>,
    input_digest: String,
    chunk_width_budget: ChunkWidthBudget,
    chunks: Vec<PlannedBulkChunk>,
}

pub type CanonicalChunkPlan = DeterministicChunkPlan;

impl DeterministicChunkPlan {
    pub fn for_ingest(
        manifest: &FrozenBulkSourceManifest,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<Self, StoreError> {
        let chunks = plan_chunks(manifest.ordered_members(), chunk_width_budget)?;
        let plan_id = compute_plan_id(
            BulkPlanKind::Ingest,
            manifest.program_id(),
            manifest.source_identity(),
            manifest.target_branch_scope(),
            None,
            manifest.manifest_digest(),
            chunk_width_budget,
            &chunks,
        )?;
        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            kind: BulkPlanKind::Ingest,
            program_id: manifest.program_id().to_string(),
            plan_id,
            source_identity: manifest.source_identity().to_string(),
            target_branch_scope: manifest.target_branch_scope().clone(),
            basis_commit_id: None,
            input_digest: manifest.manifest_digest().to_string(),
            chunk_width_budget,
            chunks,
        })
    }

    pub fn for_transform(
        basis: &FrozenTransformBasis,
        partition: &FrozenTransformTargetPartition,
        chunk_width_budget: ChunkWidthBudget,
    ) -> Result<Self, StoreError> {
        if basis.program_id() != partition.program_id()
            || basis.transform_identity() != partition.transform_identity()
            || basis.target_branch_scope() != partition.target_branch_scope()
            || basis.basis_commit_id() != partition.basis_commit_id()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkTransformBasisDrift,
                "bulk transform target partition drifted from the frozen transform basis",
            ));
        }

        let chunks = plan_chunks(partition.ordered_members(), chunk_width_budget)?;
        let plan_id = compute_plan_id(
            BulkPlanKind::Transform,
            basis.program_id(),
            basis.transform_identity(),
            basis.target_branch_scope(),
            Some(basis.basis_commit_id()),
            partition.partition_digest(),
            chunk_width_budget,
            &chunks,
        )?;
        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            kind: BulkPlanKind::Transform,
            program_id: basis.program_id().to_string(),
            plan_id,
            source_identity: basis.transform_identity().to_string(),
            target_branch_scope: basis.target_branch_scope().clone(),
            basis_commit_id: Some(basis.basis_commit_id()),
            input_digest: partition.partition_digest().to_string(),
            chunk_width_budget,
            chunks,
        })
    }

    pub fn family_version(&self) -> u32 {
        self.family_version
    }
    pub fn kind(&self) -> BulkPlanKind {
        self.kind
    }
    pub fn program_id(&self) -> &str {
        &self.program_id
    }
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }
    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }
    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.basis_commit_id
    }
    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
    pub fn chunk_width_budget(&self) -> ChunkWidthBudget {
        self.chunk_width_budget
    }
    pub fn chunks(&self) -> &[PlannedBulkChunk] {
        &self.chunks
    }
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
    pub fn chunk_by_ordinal(&self, ordinal: ChunkOrdinal) -> Option<&PlannedBulkChunk> {
        self.chunks.iter().find(|chunk| chunk.ordinal() == ordinal)
    }

    pub(crate) fn has_valid_plan_id(&self) -> Result<bool, StoreError> {
        let expected = compute_plan_id(
            self.kind,
            &self.program_id,
            &self.source_identity,
            &self.target_branch_scope,
            self.basis_commit_id,
            &self.input_digest,
            self.chunk_width_budget,
            &self.chunks,
        )?;
        Ok(self.plan_id == expected)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetAdmittedChunkPlan {
    kind: BulkPlanKind,
    program_id: String,
    plan_id: String,
    target_branch_scope: BranchId,
    basis_commit_id: Option<CommitId>,
    chunk: PlannedBulkChunk,
    admitted_memory_units: u64,
}

impl BudgetAdmittedChunkPlan {
    pub fn admit(
        plan: &DeterministicChunkPlan,
        ordinal: ChunkOrdinal,
        admitted_memory_units: u64,
    ) -> Result<Self, StoreError> {
        if admitted_memory_units == 0 {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkWidthBudgetExceeded,
                "bulk chunk admission requires positive memory units",
            ));
        }
        let Some(chunk) = plan
            .chunks()
            .iter()
            .find(|chunk| chunk.ordinal() == ordinal)
        else {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkContractUnsupported,
                format!(
                    "bulk chunk ordinal {} does not exist in deterministic plan `{}`",
                    ordinal.value(),
                    plan.plan_id()
                ),
            ));
        };
        if chunk.width_units() > admitted_memory_units {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkWidthBudgetExceeded,
                format!(
                    "bulk chunk ordinal {} width {} exceeds admitted memory units {}",
                    ordinal.value(),
                    chunk.width_units(),
                    admitted_memory_units
                ),
            ));
        }

        Ok(Self {
            kind: plan.kind(),
            program_id: plan.program_id().to_string(),
            plan_id: plan.plan_id().to_string(),
            target_branch_scope: plan.target_branch_scope().clone(),
            basis_commit_id: plan.basis_commit_id(),
            chunk: chunk.clone(),
            admitted_memory_units,
        })
    }
    pub fn kind(&self) -> BulkPlanKind {
        self.kind
    }
    pub fn program_id(&self) -> &str {
        &self.program_id
    }
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }
    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }
    pub fn basis_commit_id(&self) -> Option<CommitId> {
        self.basis_commit_id
    }
    pub fn chunk(&self) -> &PlannedBulkChunk {
        &self.chunk
    }
    pub fn admitted_memory_units(&self) -> u64 {
        self.admitted_memory_units
    }
}

fn plan_chunks(
    members: &[BulkSourceMember],
    chunk_width_budget: ChunkWidthBudget,
) -> Result<Vec<PlannedBulkChunk>, StoreError> {
    if chunk_width_budget.max_width_units == 0 {
        return Err(StoreError::new(
            StoreErrorKind::BulkChunkWidthBudgetExceeded,
            "bulk chunk width budget must be positive before planning",
        ));
    }

    let mut chunks = Vec::new();
    let mut current_member_ids = Vec::new();
    let mut current_width = 0u64;
    let mut next_ordinal = 0u64;

    for member in members {
        if member.width_units() > chunk_width_budget.max_width_units {
            return Err(StoreError::new(
                StoreErrorKind::BulkChunkWidthBudgetExceeded,
                format!(
                    "bulk member `{}` width {} exceeds chunk budget {}",
                    member.member_id(),
                    member.width_units(),
                    chunk_width_budget.max_width_units
                ),
            ));
        }

        if current_width + member.width_units() > chunk_width_budget.max_width_units
            && !current_member_ids.is_empty()
        {
            chunks.push(PlannedBulkChunk {
                ordinal: ChunkOrdinal::new(next_ordinal),
                member_ids: current_member_ids,
                width_units: current_width,
            });
            next_ordinal += 1;
            current_member_ids = Vec::new();
            current_width = 0;
        }

        current_member_ids.push(member.member_id().to_string());
        current_width += member.width_units();
    }

    if !current_member_ids.is_empty() {
        chunks.push(PlannedBulkChunk {
            ordinal: ChunkOrdinal::new(next_ordinal),
            member_ids: current_member_ids,
            width_units: current_width,
        });
    }

    Ok(chunks)
}

fn compute_plan_id(
    kind: BulkPlanKind,
    program_id: &str,
    source_identity: &str,
    target_branch_scope: &BranchId,
    basis_commit_id: Option<CommitId>,
    input_digest: &str,
    chunk_width_budget: ChunkWidthBudget,
    chunks: &[PlannedBulkChunk],
) -> Result<String, StoreError> {
    #[derive(Serialize)]
    struct PlanDigestInput<'a> {
        family_version: u32,
        kind: BulkPlanKind,
        program_id: &'a str,
        source_identity: &'a str,
        target_branch_scope: &'a BranchId,
        basis_commit_id: Option<CommitId>,
        input_digest: &'a str,
        chunk_width_budget: ChunkWidthBudget,
        chunks: &'a [PlannedBulkChunk],
    }

    let digest_basis = serde_json::to_string(&PlanDigestInput {
        family_version: BULK_FAMILY_VERSION,
        kind,
        program_id,
        source_identity,
        target_branch_scope,
        basis_commit_id,
        input_digest,
        chunk_width_budget,
        chunks,
    })
    .map_err(serialization_error("deterministic bulk chunk plan"))?;

    Ok(stable_digest(&digest_basis))
}
