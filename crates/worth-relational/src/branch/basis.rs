use std::sync::{Arc, OnceLock};

use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;

use super::{
    RelationalBranchIdentity, RelationalBranchReferenceObservation, RelationalBranchRoot,
    RelationalBranchVersion,
};

pub const RELATIONAL_BRANCH_BASIS_DESCRIPTOR_VERSION: u16 = 2;

/// Lifecycle posture carried by a descriptive branch basis.
///
/// Phase 6 admits only `Live`. Archive/delete transitions remain owned by the
/// later lifecycle phase, but a transported descriptor cannot erase posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalBranchBasisPosture {
    Live,
    Archived,
}

/// Serializable description of one exact Relational branch basis.
///
/// This value is deliberately non-operational. Serialization, copying, or
/// restoration weakens freshness; only the owning runtime may resolve and
/// readmit it into an operational basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalBranchBasisDescriptor {
    descriptor_version: u16,
    runtime_instance_id: u64,
    branch_id: BranchId,
    reference: RelationalBranchReferenceObservation,
    truth_version: RelationalBranchVersion,
    root_identity: u64,
    #[serde(default)]
    schema_commitment: [u8; 32],
    visibility_commitment: [u8; 32],
    posture: RelationalBranchBasisPosture,
}

pub(crate) struct RelationalLiveBranchBasisDescriptorAxes {
    pub(crate) runtime_instance_id: u64,
    pub(crate) branch_id: BranchId,
    pub(crate) reference: RelationalBranchReferenceObservation,
    pub(crate) truth_version: RelationalBranchVersion,
    pub(crate) root_identity: u64,
    pub(crate) schema_commitment: [u8; 32],
    pub(crate) visibility_commitment: [u8; 32],
}

impl RelationalBranchBasisDescriptor {
    pub(crate) fn live(axes: RelationalLiveBranchBasisDescriptorAxes) -> Self {
        Self {
            descriptor_version: RELATIONAL_BRANCH_BASIS_DESCRIPTOR_VERSION,
            runtime_instance_id: axes.runtime_instance_id,
            branch_id: axes.branch_id,
            reference: axes.reference,
            truth_version: axes.truth_version,
            root_identity: axes.root_identity,
            schema_commitment: axes.schema_commitment,
            visibility_commitment: axes.visibility_commitment,
            posture: RelationalBranchBasisPosture::Live,
        }
    }

    pub const fn descriptor_version(&self) -> u16 {
        self.descriptor_version
    }

    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn reference(&self) -> &RelationalBranchReferenceObservation {
        &self.reference
    }

    pub const fn truth_version(&self) -> RelationalBranchVersion {
        self.truth_version
    }

    pub const fn root_identity(&self) -> u64 {
        self.root_identity
    }

    pub const fn schema_commitment(&self) -> [u8; 32] {
        self.schema_commitment
    }

    pub const fn visibility_commitment(&self) -> [u8; 32] {
        self.visibility_commitment
    }

    pub const fn posture(&self) -> RelationalBranchBasisPosture {
        self.posture
    }
}

/// Structurally valid but still non-operational descriptor.
///
/// Resolution proves protocol shape and cross-field consistency only. It does
/// not prove owner affinity, currentness, retained-root availability, or live
/// lifecycle posture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRelationalBasisDescriptor {
    descriptor: RelationalBranchBasisDescriptor,
}

impl ResolvedRelationalBasisDescriptor {
    pub(crate) fn new(descriptor: RelationalBranchBasisDescriptor) -> Self {
        Self { descriptor }
    }

    pub fn descriptor(&self) -> &RelationalBranchBasisDescriptor {
        &self.descriptor
    }

    pub(crate) fn into_descriptor(self) -> RelationalBranchBasisDescriptor {
        self.descriptor
    }
}

/// Owner-admitted immutable read basis.
///
/// Clones share one owner-issued lease and one immutable root. Cloning never
/// revalidates, reacquires retention, copies truth, or contacts live branch
/// state.
#[derive(Clone, Debug)]
pub struct AdmittedRelationalBranchBasis {
    pub(crate) inner: Arc<AdmittedRelationalBranchBasisInner>,
}

#[derive(Debug)]
pub(crate) struct AdmittedRelationalBranchBasisInner {
    pub(crate) descriptor: RelationalBranchBasisDescriptor,
    pub(crate) identity: RelationalBranchIdentity,
    pub(crate) root: Arc<RelationalBranchRoot>,
    pub(crate) _authority: super::RelationalBranchObservationAuthority,
    pub(crate) retention: crate::history::retention::RelationalObservationRetentionObligation,
    pub(super) registry_lease: OnceLock<super::basis_registry::RelationalBasisRegistryLease>,
}

impl AdmittedRelationalBranchBasis {
    pub fn descriptor(&self) -> &RelationalBranchBasisDescriptor {
        &self.inner.descriptor
    }

    pub fn identity(&self) -> &RelationalBranchIdentity {
        &self.inner.identity
    }

    pub fn retention_reason(&self) -> crate::history::retention::RelationalBasisRetentionReason {
        self.inner.retention.reason()
    }
}
