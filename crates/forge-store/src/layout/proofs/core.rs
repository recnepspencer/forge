use serde::{Deserialize, Serialize};

use super::scopes::AspectLayoutReadRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedAspectSlicesPerRead(u64);
impl MaxAdmittedAspectSlicesPerRead {
    pub const fn new(value: u64) -> Self { Self(value) }
    pub const fn value(self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedBlockDecodeBreadth(u64);
impl MaxAdmittedBlockDecodeBreadth {
    pub const fn new(value: u64) -> Self { Self(value) }
    pub const fn value(self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxAdmittedControlReplayBreadthForParity(u64);
impl MaxAdmittedControlReplayBreadthForParity {
    pub const fn new(value: u64) -> Self { Self(value) }
    pub const fn value(self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MaxDeterministicChunkWidth(u64);
impl MaxDeterministicChunkWidth {
    pub const fn new(value: u64) -> Self { Self(value) }
    pub const fn value(self) -> u64 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChunkShapeVersion(u32);
impl ChunkShapeVersion {
    pub const fn new(value: u32) -> Self { Self(value) }
    pub const fn value(self) -> u32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EquivalenceContractVersion(u32);
impl EquivalenceContractVersion {
    pub const fn new(value: u32) -> Self { Self(value) }
    pub const fn value(self) -> u32 { self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AspectLayoutSliceId(String);
impl AspectLayoutSliceId {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct StructuralBlockId(String);
impl StructuralBlockId {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PhysicalChunkId(String);
impl PhysicalChunkId {
    pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Milestone6LayoutSupportLane {
    ProofOnly,
    OnDemandMaterialized,
    PolicyEagerMaterialized,
}
impl Milestone6LayoutSupportLane {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProofOnly => "proof_only",
            Self::OnDemandMaterialized => "on_demand_materialized",
            Self::PolicyEagerMaterialized => "policy_eager_materialized",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Milestone6ResolvedLayoutSupportLane {
    ProofOnly,
    OnDemandMaterialized,
    PolicyEagerMaterializedPublished,
    PolicyEagerMaterializedReuseExisting,
}
impl Milestone6ResolvedLayoutSupportLane {
    pub fn label(self) -> &'static str {
        match self {
            Self::ProofOnly => "proof_only",
            Self::OnDemandMaterialized => "on_demand_materialized",
            Self::PolicyEagerMaterializedPublished => "policy_eager_materialized_published",
            Self::PolicyEagerMaterializedReuseExisting => "policy_eager_materialized_reuse_existing",
        }
    }
    pub fn uses_materialized_support(self) -> bool { !matches!(self, Self::ProofOnly) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Milestone6LayoutSupportPublicationDisposition {
    None,
    PublishedThisOperation,
    ReusedExisting,
}
impl Milestone6LayoutSupportPublicationDisposition {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PublishedThisOperation => "published_this_operation",
            Self::ReusedExisting => "reused_existing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Milestone6LayoutSupportPolicy {
    materialize_hot_branch_reads: bool,
    materialize_repeated_scope_reads: bool,
    repeated_scope_threshold: u64,
}
impl Milestone6LayoutSupportPolicy {
    pub const fn new(
        materialize_hot_branch_reads: bool,
        materialize_repeated_scope_reads: bool,
        repeated_scope_threshold: u64,
    ) -> Self {
        Self { materialize_hot_branch_reads, materialize_repeated_scope_reads, repeated_scope_threshold }
    }
    pub const fn materialize_hot_branch_reads(self) -> bool { self.materialize_hot_branch_reads }
    pub const fn materialize_repeated_scope_reads(self) -> bool { self.materialize_repeated_scope_reads }
    pub const fn repeated_scope_threshold(self) -> u64 { self.repeated_scope_threshold }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6PreparedLayoutSupport {
    requested_lane: Milestone6LayoutSupportLane,
    resolved_lane: Milestone6ResolvedLayoutSupportLane,
    publication_disposition: Milestone6LayoutSupportPublicationDisposition,
    request: AspectLayoutReadRequest,
    layout_materialization_artifact_id: Option<String>,
}
impl Milestone6PreparedLayoutSupport {
    pub(crate) fn proof_only(request: AspectLayoutReadRequest) -> Self {
        Self {
            requested_lane: Milestone6LayoutSupportLane::ProofOnly,
            resolved_lane: Milestone6ResolvedLayoutSupportLane::ProofOnly,
            publication_disposition: Milestone6LayoutSupportPublicationDisposition::None,
            request,
            layout_materialization_artifact_id: None,
        }
    }

    pub(crate) fn resolved(
        requested_lane: Milestone6LayoutSupportLane,
        resolved_lane: Milestone6ResolvedLayoutSupportLane,
        publication_disposition: Milestone6LayoutSupportPublicationDisposition,
        request: AspectLayoutReadRequest,
        layout_materialization_artifact_id: Option<String>,
    ) -> Self {
        Self {
            requested_lane,
            resolved_lane,
            publication_disposition,
            request,
            layout_materialization_artifact_id,
        }
    }

    pub fn requested_lane(&self) -> Milestone6LayoutSupportLane { self.requested_lane }
    pub fn resolved_lane(&self) -> Milestone6ResolvedLayoutSupportLane { self.resolved_lane }
    pub fn publication_disposition(&self) -> Milestone6LayoutSupportPublicationDisposition { self.publication_disposition }
    pub fn request(&self) -> &AspectLayoutReadRequest { &self.request }
    pub fn layout_materialization_artifact_id(&self) -> Option<&str> { self.layout_materialization_artifact_id.as_deref() }
}
