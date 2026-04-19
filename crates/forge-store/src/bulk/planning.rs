use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const BULK_FAMILY_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkPlanKind {
    Ingest,
    Transform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkSourceMember {
    member_id: String,
    width_units: u64,
}

impl BulkSourceMember {
    pub fn new(member_id: impl Into<String>, width_units: u64) -> Self {
        Self {
            member_id: member_id.into(),
            width_units,
        }
    }

    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    pub fn width_units(&self) -> u64 {
        self.width_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkIngestSourceRequest {
    program_id: String,
    source_identity: String,
    target_branch_scope: BranchId,
    source_members: Vec<BulkSourceMember>,
}

impl BulkIngestSourceRequest {
    pub fn new(
        program_id: impl Into<String>,
        source_identity: impl Into<String>,
        target_branch_scope: BranchId,
        source_members: Vec<BulkSourceMember>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            source_identity: source_identity.into(),
            target_branch_scope,
            source_members,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn source_members(&self) -> &[BulkSourceMember] {
        &self.source_members
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkTransformRequest {
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    target_members: Vec<BulkSourceMember>,
}

impl BulkTransformRequest {
    pub fn new(
        program_id: impl Into<String>,
        transform_identity: impl Into<String>,
        target_branch_scope: BranchId,
        basis_commit_id: CommitId,
        target_members: Vec<BulkSourceMember>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            transform_identity: transform_identity.into(),
            target_branch_scope,
            basis_commit_id,
            target_members,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn transform_identity(&self) -> &str {
        &self.transform_identity
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }

    pub fn target_members(&self) -> &[BulkSourceMember] {
        &self.target_members
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenBulkSourceManifest {
    family_version: u32,
    program_id: String,
    source_identity: String,
    target_branch_scope: BranchId,
    ordered_members: Vec<BulkSourceMember>,
    manifest_digest: String,
}

impl FrozenBulkSourceManifest {
    pub fn freeze(request: BulkIngestSourceRequest) -> Result<Self, StoreError> {
        ensure_program_identity(request.program_id(), request.source_identity())?;
        let ordered_members = canonicalize_members(request.source_members().to_vec())?;

        #[derive(Serialize)]
        struct ManifestDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            source_identity: &'a str,
            target_branch_scope: &'a BranchId,
            ordered_members: &'a [BulkSourceMember],
        }

        let digest_basis = serde_json::to_string(&ManifestDigestInput {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id(),
            source_identity: request.source_identity(),
            target_branch_scope: request.target_branch_scope(),
            ordered_members: &ordered_members,
        })
        .map_err(serialization_error("bulk ingest source manifest"))?;

        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            source_identity: request.source_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            ordered_members,
            manifest_digest: stable_digest(&digest_basis),
        })
    }

    pub fn family_version(&self) -> u32 {
        self.family_version
    }

    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Ingest
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn ordered_members(&self) -> &[BulkSourceMember] {
        &self.ordered_members
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct ManifestDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            source_identity: &'a str,
            target_branch_scope: &'a BranchId,
            ordered_members: &'a [BulkSourceMember],
        }

        let digest_basis = serde_json::to_string(&ManifestDigestInput {
            family_version: self.family_version,
            program_id: &self.program_id,
            source_identity: &self.source_identity,
            target_branch_scope: &self.target_branch_scope,
            ordered_members: &self.ordered_members,
        })
        .map_err(serialization_error("bulk ingest source manifest"))?;
        Ok(self.manifest_digest == stable_digest(&digest_basis))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformBasis {
    family_version: u32,
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    basis_digest: String,
}

impl FrozenTransformBasis {
    pub fn freeze(request: &BulkTransformRequest) -> Result<Self, StoreError> {
        ensure_program_identity(request.program_id(), request.transform_identity())?;

        #[derive(Serialize)]
        struct BasisDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
        }

        let digest_basis = serde_json::to_string(&BasisDigestInput {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id(),
            transform_identity: request.transform_identity(),
            target_branch_scope: request.target_branch_scope(),
            basis_commit_id: request.basis_commit_id(),
        })
        .map_err(serialization_error("bulk transform basis"))?;

        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            transform_identity: request.transform_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            basis_commit_id: request.basis_commit_id(),
            basis_digest: stable_digest(&digest_basis),
        })
    }

    pub fn family_version(&self) -> u32 {
        self.family_version
    }

    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Transform
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn transform_identity(&self) -> &str {
        &self.transform_identity
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct BasisDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
        }

        let digest_basis = serde_json::to_string(&BasisDigestInput {
            family_version: self.family_version,
            program_id: &self.program_id,
            transform_identity: &self.transform_identity,
            target_branch_scope: &self.target_branch_scope,
            basis_commit_id: self.basis_commit_id,
        })
        .map_err(serialization_error("bulk transform basis"))?;
        Ok(self.basis_digest == stable_digest(&digest_basis))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformTargetPartition {
    family_version: u32,
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    ordered_members: Vec<BulkSourceMember>,
    partition_digest: String,
}

impl FrozenTransformTargetPartition {
    pub fn freeze(
        request: &BulkTransformRequest,
        basis: &FrozenTransformBasis,
    ) -> Result<Self, StoreError> {
        if basis.program_id() != request.program_id()
            || basis.transform_identity() != request.transform_identity()
            || basis.target_branch_scope() != request.target_branch_scope()
            || basis.basis_commit_id() != request.basis_commit_id()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkTransformBasisDrift,
                "bulk transform target partition request drifted from the frozen transform basis",
            ));
        }

        let ordered_members = canonicalize_members(request.target_members().to_vec())?;

        #[derive(Serialize)]
        struct PartitionDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
            ordered_members: &'a [BulkSourceMember],
        }

        let digest_basis = serde_json::to_string(&PartitionDigestInput {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id(),
            transform_identity: request.transform_identity(),
            target_branch_scope: request.target_branch_scope(),
            basis_commit_id: request.basis_commit_id(),
            ordered_members: &ordered_members,
        })
        .map_err(serialization_error("bulk transform target partition"))?;

        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            transform_identity: request.transform_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            basis_commit_id: request.basis_commit_id(),
            ordered_members,
            partition_digest: stable_digest(&digest_basis),
        })
    }

    pub fn family_version(&self) -> u32 {
        self.family_version
    }

    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Transform
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn transform_identity(&self) -> &str {
        &self.transform_identity
    }

    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }

    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }

    pub fn ordered_members(&self) -> &[BulkSourceMember] {
        &self.ordered_members
    }

    pub fn partition_digest(&self) -> &str {
        &self.partition_digest
    }

    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct PartitionDigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
            ordered_members: &'a [BulkSourceMember],
        }

        let digest_basis = serde_json::to_string(&PartitionDigestInput {
            family_version: self.family_version,
            program_id: &self.program_id,
            transform_identity: &self.transform_identity,
            target_branch_scope: &self.target_branch_scope,
            basis_commit_id: self.basis_commit_id,
            ordered_members: &self.ordered_members,
        })
        .map_err(serialization_error("bulk transform target partition"))?;
        Ok(self.partition_digest == stable_digest(&digest_basis))
    }
}

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

fn ensure_program_identity(program_id: &str, source_identity: &str) -> Result<(), StoreError> {
    if program_id.trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkPlanDeterminismViolation,
            "bulk program id must be non-empty",
        ));
    }
    if source_identity.trim().is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkSourceIdentityUnavailable,
            "bulk source or transform identity must be non-empty",
        ));
    }
    Ok(())
}

fn canonicalize_members(
    mut members: Vec<BulkSourceMember>,
) -> Result<Vec<BulkSourceMember>, StoreError> {
    if members.is_empty() {
        return Err(StoreError::new(
            StoreErrorKind::BulkPlanDeterminismViolation,
            "bulk planning requires at least one source or target member",
        ));
    }
    members.sort_by(|left, right| left.member_id.cmp(&right.member_id));
    let mut previous_member: Option<&str> = None;
    for member in &members {
        if member.member_id.trim().is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                "bulk members must declare non-empty identities",
            ));
        }
        if member.width_units == 0 {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                format!(
                    "bulk member `{}` must declare positive width units",
                    member.member_id
                ),
            ));
        }
        if previous_member == Some(member.member_id()) {
            return Err(StoreError::new(
                StoreErrorKind::BulkPlanDeterminismViolation,
                format!(
                    "bulk member `{}` appears more than once in canonical ordering",
                    member.member_id
                ),
            ));
        }
        previous_member = Some(member.member_id());
    }
    Ok(members)
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

fn stable_digest(value: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(value.as_bytes());
    format!("{:x}", digest.finalize())
}

fn serialization_error(label: &'static str) -> impl FnOnce(serde_json::Error) -> StoreError {
    move |error| {
        StoreError::new(
            StoreErrorKind::Serialization,
            format!("failed to serialize {label}: {error}"),
        )
    }
}
