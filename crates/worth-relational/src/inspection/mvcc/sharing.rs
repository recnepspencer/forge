use crate::branch::RelationalBranchCoordinationCellId;
use crate::identity::data::PartitionId;

#[path = "sharing_accounting.rs"]
mod accounting;
#[path = "sharing_inspection.rs"]
mod inspection;
#[path = "sharing_inventory.rs"]
mod inventory;
#[path = "sharing_scope.rs"]
mod scope;

use accounting::RelationalAuthoritativeAllocationAccounting;

pub const RELATIONAL_SHARING_INSPECTION_VERSION: u16 = 4;

/// Explicit scope of byte totals in the Phase 5 sharing observation.
///
/// Version 4 reports every owner-defined authoritative allocation reachable
/// from the selected branches and the complete-root visibility commitments.
/// Branch metadata remains a separate lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalSharingByteMetricScope {
    CompleteAuthoritativeOwnerAllocations,
    #[deprecated(note = "inspection version 3 reports complete owner allocations")]
    AuthoritativePartitionPayloadsOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RelationalAuthoritativeAllocationKind {
    PartitionPayload,
    PartitionStateObject,
    RootRegionObject,
    RootMetadata,
    RootSchemaAuthority,
    RootReachabilitySetObject,
    RootReachabilityStructure,
    RootReplacementStorage,
    RootRemovalStorage,
    CanonicalCommitArtifact,
    CanonicalCommitPayload,
    CanonicalCommitEnvelope,
    CanonicalCommitEnvelopeNested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationalAuthoritativeAllocationLocator {
    runtime_instance_id: u64,
    kind: RelationalAuthoritativeAllocationKind,
    owner_id: u64,
    creation_owner_id: u64,
    partition_id: Option<PartitionId>,
}

impl RelationalAuthoritativeAllocationLocator {
    pub(super) const fn new(
        runtime_instance_id: u64,
        kind: RelationalAuthoritativeAllocationKind,
        owner_id: u64,
        creation_owner_id: u64,
        partition_id: Option<PartitionId>,
    ) -> Self {
        Self {
            runtime_instance_id,
            kind,
            owner_id,
            creation_owner_id,
            partition_id,
        }
    }
    pub const fn runtime_instance_id(self) -> u64 {
        self.runtime_instance_id
    }
    pub const fn kind(self) -> RelationalAuthoritativeAllocationKind {
        self.kind
    }
    pub const fn owner_id(self) -> u64 {
        self.owner_id
    }
    pub const fn creation_owner_id(self) -> u64 {
        self.creation_owner_id
    }
    pub const fn partition_id(self) -> Option<PartitionId> {
        self.partition_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationalAuthoritativeAllocationObservation {
    locator: RelationalAuthoritativeAllocationLocator,
    authoritative_bytes: u64,
}

/// Read-only evidence that one selected root commits to one complete visible
/// truth/schema/index/canonical-commit tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationalVisibilityCommitmentObservation {
    root_id: u64,
    digest: [u8; 32],
}

impl RelationalVisibilityCommitmentObservation {
    pub(super) const fn new(root_id: u64, digest: [u8; 32]) -> Self {
        Self { root_id, digest }
    }

    pub const fn root_id(self) -> u64 {
        self.root_id
    }

    pub const fn digest(self) -> [u8; 32] {
        self.digest
    }
}

impl RelationalAuthoritativeAllocationObservation {
    pub(super) const fn new(
        locator: RelationalAuthoritativeAllocationLocator,
        authoritative_bytes: u64,
    ) -> Self {
        Self {
            locator,
            authoritative_bytes,
        }
    }
    pub const fn locator(self) -> RelationalAuthoritativeAllocationLocator {
        self.locator
    }
    pub const fn authoritative_bytes(self) -> u64 {
        self.authoritative_bytes
    }
}

/// Correctness-index posture exposed by read-only MVCC inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalCorrectnessIndexPosture {
    AuthoritativeFallback,
}

/// Runtime-affine, owner-issued identity for one immutable storage region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationalStorageRegionLocator {
    runtime_instance_id: u64,
    creation_root_id: u64,
    region_id: u64,
    partition_id: PartitionId,
}

impl RelationalStorageRegionLocator {
    pub const fn runtime_instance_id(self) -> u64 {
        self.runtime_instance_id
    }

    pub const fn root_id(self) -> u64 {
        self.creation_root_id
    }

    pub const fn region_id(self) -> u64 {
        self.region_id
    }

    pub const fn partition_id(self) -> PartitionId {
        self.partition_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalBranchSharingInspectionDenial {
    ForeignRuntime,
    UnknownBranch,
    RootUnavailable,
    DuplicateBranch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalBranchSharingObservation {
    inspection_version: u16,
    byte_metric_scope: RelationalSharingByteMetricScope,
    branch_count: u64,
    unique_root_count: u64,
    root_ids: Vec<u64>,
    unique_canonical_commit_artifacts: u64,
    logical_branch_partition_payload_bytes: u64,
    unique_physical_partition_payload_bytes: u64,
    logical_branch_root_metadata_bytes: u64,
    unique_physical_root_metadata_bytes: u64,
    logical_branch_root_reachability_bytes: u64,
    unique_physical_root_reachability_bytes: u64,
    logical_branch_canonical_commit_bytes: u64,
    unique_physical_canonical_commit_bytes: u64,
    logical_branch_authoritative_bytes: u64,
    unique_physical_authoritative_bytes: u64,
    unique_diagnostic_bytes: u64,
    unique_retention_metadata_bytes: u64,
    unique_allocator_bookkeeping_bytes: u64,
    unique_optional_cache_bytes: u64,
    branch_metadata_bytes: u64,
    copied_truth_bytes: u64,
    copied_commit_envelopes: u64,
    fork_materialized_entity_count: u64,
    fork_materialized_relation_count: u64,
    fork_materialized_authoritative_bytes: u64,
    shared_root_acquisitions: u64,
    publication_touched_region_count: u64,
    publication_reused_region_count: u64,
    publication_new_authoritative_bytes: u64,
    reclaimable_unique_bytes: u64,
    coordination_contacts: u64,
    coordination_waits: u64,
    correctness_index_posture: RelationalCorrectnessIndexPosture,
    coordination_cells: Vec<RelationalBranchCoordinationCellId>,
    region_locators: Vec<RelationalStorageRegionLocator>,
    authoritative_allocations: Vec<RelationalAuthoritativeAllocationObservation>,
    visibility_commitments: Vec<RelationalVisibilityCommitmentObservation>,
    inspection_reconstructed_region_count: u64,
}

impl RelationalBranchSharingObservation {
    pub const fn inspection_version(&self) -> u16 {
        self.inspection_version
    }
    pub const fn byte_metric_scope(&self) -> RelationalSharingByteMetricScope {
        self.byte_metric_scope
    }
    pub const fn branch_count(&self) -> u64 {
        self.branch_count
    }
    pub const fn unique_root_count(&self) -> u64 {
        self.unique_root_count
    }
    pub fn root_ids(&self) -> &[u64] {
        &self.root_ids
    }
    pub const fn unique_canonical_commit_artifacts(&self) -> u64 {
        self.unique_canonical_commit_artifacts
    }
    pub const fn logical_branch_partition_payload_bytes(&self) -> u64 {
        self.logical_branch_partition_payload_bytes
    }
    pub const fn unique_physical_partition_payload_bytes(&self) -> u64 {
        self.unique_physical_partition_payload_bytes
    }
    pub const fn logical_branch_root_metadata_bytes(&self) -> u64 {
        self.logical_branch_root_metadata_bytes
    }
    pub const fn unique_physical_root_metadata_bytes(&self) -> u64 {
        self.unique_physical_root_metadata_bytes
    }
    pub const fn logical_branch_root_reachability_bytes(&self) -> u64 {
        self.logical_branch_root_reachability_bytes
    }
    pub const fn unique_physical_root_reachability_bytes(&self) -> u64 {
        self.unique_physical_root_reachability_bytes
    }
    pub const fn logical_branch_canonical_commit_bytes(&self) -> u64 {
        self.logical_branch_canonical_commit_bytes
    }
    pub const fn unique_physical_canonical_commit_bytes(&self) -> u64 {
        self.unique_physical_canonical_commit_bytes
    }
    pub const fn logical_branch_authoritative_bytes(&self) -> u64 {
        self.logical_branch_authoritative_bytes
    }
    pub const fn unique_physical_authoritative_bytes(&self) -> u64 {
        self.unique_physical_authoritative_bytes
    }
    pub const fn unique_diagnostic_bytes(&self) -> u64 {
        self.unique_diagnostic_bytes
    }
    pub const fn unique_retention_metadata_bytes(&self) -> u64 {
        self.unique_retention_metadata_bytes
    }
    pub const fn unique_allocator_bookkeeping_bytes(&self) -> u64 {
        self.unique_allocator_bookkeeping_bytes
    }
    pub const fn unique_optional_cache_bytes(&self) -> u64 {
        self.unique_optional_cache_bytes
    }
    pub const fn branch_metadata_bytes(&self) -> u64 {
        self.branch_metadata_bytes
    }
    pub const fn copied_truth_bytes(&self) -> u64 {
        self.copied_truth_bytes
    }
    pub const fn copied_commit_envelopes(&self) -> u64 {
        self.copied_commit_envelopes
    }
    pub const fn fork_materialized_entity_count(&self) -> u64 {
        self.fork_materialized_entity_count
    }
    pub const fn fork_materialized_relation_count(&self) -> u64 {
        self.fork_materialized_relation_count
    }
    pub const fn fork_materialized_authoritative_bytes(&self) -> u64 {
        self.fork_materialized_authoritative_bytes
    }
    pub const fn shared_root_acquisitions(&self) -> u64 {
        self.shared_root_acquisitions
    }
    pub const fn publication_touched_region_count(&self) -> u64 {
        self.publication_touched_region_count
    }
    pub const fn publication_reused_region_count(&self) -> u64 {
        self.publication_reused_region_count
    }
    pub const fn publication_new_authoritative_bytes(&self) -> u64 {
        self.publication_new_authoritative_bytes
    }
    pub const fn reclaimable_unique_bytes(&self) -> u64 {
        self.reclaimable_unique_bytes
    }
    pub const fn coordination_contacts(&self) -> u64 {
        self.coordination_contacts
    }
    pub const fn coordination_waits(&self) -> u64 {
        self.coordination_waits
    }
    pub const fn correctness_index_posture(&self) -> RelationalCorrectnessIndexPosture {
        self.correctness_index_posture
    }
    pub fn coordination_cells(&self) -> &[RelationalBranchCoordinationCellId] {
        &self.coordination_cells
    }
    pub fn region_locators(&self) -> &[RelationalStorageRegionLocator] {
        &self.region_locators
    }
    pub fn authoritative_allocations(&self) -> &[RelationalAuthoritativeAllocationObservation] {
        &self.authoritative_allocations
    }
    pub fn visibility_commitments(&self) -> &[RelationalVisibilityCommitmentObservation] {
        &self.visibility_commitments
    }
    pub const fn inspection_reconstructed_region_count(&self) -> u64 {
        self.inspection_reconstructed_region_count
    }
}
