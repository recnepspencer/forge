use crate::authority::AuthoritativeExportBundle;
use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const BRANCH_DELTA_FAMILY_VERSION: u32 = 1;
pub const MAX_DIRECT_LAYER_READ_DEPTH: usize = 4;
pub const MAX_DIRECT_LAYER_READ_RECORDS: usize = 32;
pub const MAX_REWRITE_LAYER_WIDTH: usize = 3;
pub const RECOMMENDED_REWRITE_LAYER_WIDTH: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BranchDeltaLayerId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationRequest {
    pub new_branch_id: BranchId,
    pub source_branch_id: BranchId,
}

impl SharedBaseBranchCreationRequest {
    pub fn new(new_branch_id: BranchId, source_branch_id: BranchId) -> Self {
        Self {
            new_branch_id,
            source_branch_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationReceipt {
    pub branch_id: BranchId,
    pub source_branch_id: BranchId,
    pub source_frontier_commit_id: Option<CommitId>,
    pub delta_family_version: u32,
    pub authority_basis_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedBaseBranchCreationWitness {
    request: SharedBaseBranchCreationRequest,
    source_frontier_commit_id: Option<CommitId>,
    authority_basis_digest: String,
}

impl SharedBaseBranchCreationWitness {
    pub(crate) fn new(
        request: SharedBaseBranchCreationRequest,
        source_frontier_commit_id: Option<CommitId>,
        authority_basis_digest: String,
    ) -> Self {
        Self {
            request,
            source_frontier_commit_id,
            authority_basis_digest,
        }
    }

    pub fn request(&self) -> &SharedBaseBranchCreationRequest {
        &self.request
    }

    pub fn source_frontier_commit_id(&self) -> Option<CommitId> {
        self.source_frontier_commit_id
    }

    pub fn authority_basis_digest(&self) -> &str {
        &self.authority_basis_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadRequest {
    pub branch_id: BranchId,
    pub target_commit_id: CommitId,
}

impl BranchDeltaReadRequest {
    pub fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaReadStrategy {
    EmptyBranchReuse,
    DirectLayerRead,
    AuthorityReplayControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaReadRegime {
    Sparse,
    Dense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplexityStatus {
    Verified,
    Debt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaFallbackClass {
    None,
    RequiresAuthorityReplayControlLane,
    RequiresMergeAwareWidening,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaLocality {
    pub branch_id: BranchId,
    pub base_frontier_commit_id: Option<CommitId>,
    pub target_commit_id: CommitId,
    pub commit_span: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaPerformanceEnvelope {
    pub layers_traversed: usize,
    pub records_decoded: usize,
    pub replay_commit_count: usize,
    pub fallback_class: BranchDeltaFallbackClass,
    pub complexity_status: ComplexityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadPlan {
    pub strategy: BranchDeltaReadStrategy,
    pub regime: BranchDeltaReadRegime,
    pub locality: BranchDeltaLocality,
    pub used_layer_ids: Vec<BranchDeltaLayerId>,
    pub commit_ids: Vec<CommitId>,
    pub performance: BranchDeltaPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SameBranchDescendantWitness {
    branch_id: BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_commit_id: CommitId,
    commit_ids: Vec<CommitId>,
}

impl SameBranchDescendantWitness {
    pub(crate) fn new(
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_commit_id: CommitId,
        commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            base_frontier_commit_id,
            target_commit_id,
            commit_ids,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn base_frontier_commit_id(&self) -> Option<CommitId> {
        self.base_frontier_commit_id
    }

    pub fn target_commit_id(&self) -> CommitId {
        self.target_commit_id
    }

    pub fn commit_ids(&self) -> &[CommitId] {
        &self.commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone7IndependentReference {
    branch_id: BranchId,
    target_commit_id: CommitId,
}

impl Milestone7IndependentReference {
    pub(crate) fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn target_commit_id(&self) -> CommitId {
        self.target_commit_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteRequest {
    pub branch_id: BranchId,
    pub target_commit_id: CommitId,
}

impl BranchDeltaRewriteRequest {
    pub fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaRewriteStrategy {
    NotNeeded,
    ReplaceContiguousSegment,
    RejectAsTooBroad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaRewritePolicyDecision {
    NoAction,
    Defer,
    CompactNow,
    RejectAsTooBroad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteEligibleDeltaSegment {
    branch_id: BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_frontier_commit_id: CommitId,
    layer_ids: Vec<BranchDeltaLayerId>,
    commit_ids: Vec<CommitId>,
}

impl RewriteEligibleDeltaSegment {
    pub(crate) fn new(
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_frontier_commit_id: CommitId,
        layer_ids: Vec<BranchDeltaLayerId>,
        commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            base_frontier_commit_id,
            target_frontier_commit_id,
            layer_ids,
            commit_ids,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn base_frontier_commit_id(&self) -> Option<CommitId> {
        self.base_frontier_commit_id
    }

    pub fn target_frontier_commit_id(&self) -> CommitId {
        self.target_frontier_commit_id
    }

    pub fn layer_ids(&self) -> &[BranchDeltaLayerId] {
        &self.layer_ids
    }

    pub fn commit_ids(&self) -> &[CommitId] {
        &self.commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewritePlan {
    strategy: BranchDeltaRewriteStrategy,
    segment: Option<RewriteEligibleDeltaSegment>,
    rewrite_breadth: usize,
}

impl BranchDeltaRewritePlan {
    pub(crate) fn new(
        strategy: BranchDeltaRewriteStrategy,
        segment: Option<RewriteEligibleDeltaSegment>,
        rewrite_breadth: usize,
    ) -> Self {
        Self {
            strategy,
            segment,
            rewrite_breadth,
        }
    }

    pub fn strategy(&self) -> BranchDeltaRewriteStrategy {
        self.strategy
    }

    pub fn segment(&self) -> Option<&RewriteEligibleDeltaSegment> {
        self.segment.as_ref()
    }

    pub fn rewrite_breadth(&self) -> usize {
        self.rewrite_breadth
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteRecommendation {
    pub decision: BranchDeltaRewritePolicyDecision,
    pub plan: BranchDeltaRewritePlan,
    pub recommended_layer_width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaAutoCompactDisposition {
    NoAction,
    Deferred,
    Compacted,
    RejectedAsTooBroad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaAutoCompactOutcome {
    pub disposition: BranchDeltaAutoCompactDisposition,
    pub recommendation: BranchDeltaRewriteRecommendation,
    pub rewrite_receipt: Option<BranchDeltaRewriteReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteReceipt {
    pub branch_id: BranchId,
    pub target_frontier_commit_id: CommitId,
    pub replacement_layer_id: Option<BranchDeltaLayerId>,
    pub replaced_layer_ids: Vec<BranchDeltaLayerId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRebuildReceipt {
    pub branch_id: BranchId,
    pub rebuilt_layer_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaReadResult {
    pub plan: BranchDeltaReadPlan,
    authoritative_export: AuthoritativeExportBundle,
}

impl BranchDeltaReadResult {
    pub fn new(plan: BranchDeltaReadPlan, authoritative_export: AuthoritativeExportBundle) -> Self {
        Self {
            plan,
            authoritative_export: authoritative_export.into_canonicalized(),
        }
    }

    pub fn authoritative_export(&self) -> &AuthoritativeExportBundle {
        &self.authoritative_export
    }
}

pub fn stable_branch_delta_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("branch delta digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn stable_shared_base_authority_digest(
    source_branch_id: &BranchId,
    source_frontier_commit_id: Option<CommitId>,
    canonicalization_version: u32,
) -> String {
    stable_branch_delta_digest(&(
        source_branch_id,
        source_frontier_commit_id,
        canonicalization_version,
    ))
}

pub fn stable_branch_delta_layer_authority_digest(
    branch_id: &BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_frontier_commit_id: CommitId,
    commit_ids: &[CommitId],
    canonicalization_version: u32,
) -> String {
    stable_branch_delta_digest(&(
        branch_id,
        base_frontier_commit_id,
        target_frontier_commit_id,
        commit_ids,
        canonicalization_version,
    ))
}
