use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::{
    CompositeCommitIdentity, ProductBranchIdentity, ProductBranchLifecycleIncarnation,
    ProductBranchReferenceGeneration, RuntimeWorldOwnerIdentity,
};
use crate::retention::ObservationRetentionObligation;

use super::reference_cell::ProductBranchReferenceSnapshot;

/// Complete product-head observation used by every compare-and-publish
/// operation. A single reference snapshot supplies the commit and basis;
/// retention is an owner-issued operational obligation, not a caller tuple.
#[derive(Debug)]
pub struct ProductBranchObservation {
    snapshot: Arc<ProductBranchReferenceSnapshot>,
    obligation: Arc<ProductBranchObservationObligation>,
}

impl Clone for ProductBranchObservation {
    fn clone(&self) -> Self {
        Self {
            snapshot: Arc::clone(&self.snapshot),
            obligation: Arc::clone(&self.obligation),
        }
    }
}

impl PartialEq for ProductBranchObservation {
    fn eq(&self, other: &Self) -> bool {
        self.owner_identity() == other.owner_identity()
            && self.branch_identity() == other.branch_identity()
            && self.lifecycle_incarnation() == other.lifecycle_incarnation()
            && self.reference_generation() == other.reference_generation()
            && self.selected_commit() == other.selected_commit()
            && self.basis() == other.basis()
    }
}

impl Eq for ProductBranchObservation {}

/// Cloned observations share this retention-issued binding. Its two
/// independent component obligations release through RAII when the final
/// observation clone disappears.
#[derive(Debug)]
pub(crate) struct ProductBranchObservationObligation {
    retention: ObservationRetentionObligation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchObservationConstructionDenial {
    RetentionBasisMismatch,
}

impl ProductBranchObservationObligation {
    pub(crate) fn owner_issued(retention: ObservationRetentionObligation) -> Self {
        Self { retention }
    }

    pub(crate) const fn dependency_class(&self) -> crate::retention::ComponentBasisDependencyClass {
        crate::retention::ComponentBasisDependencyClass::AdmittedObservation
    }

    pub(crate) fn retention(&self) -> &ObservationRetentionObligation {
        &self.retention
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductBranchObservationMismatchAxis {
    OwnerIdentity,
    BranchIdentity,
    LifecycleIncarnation,
    ReferenceGeneration,
    SelectedCompositeCommit,
    CompositeBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductBranchObservationMismatch {
    axes: Vec<ProductBranchObservationMismatchAxis>,
}

impl ProductBranchObservationMismatch {
    pub fn axes(&self) -> &[ProductBranchObservationMismatchAxis] {
        &self.axes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeWorldBranchAdmissionDenial {
    OwnerUnavailable,
    ForeignOwner,
    RetiredBranch,
    CapacityExhausted,
    IdentityExhausted,
}

impl ProductBranchObservation {
    pub(crate) fn owner_issued(
        snapshot: ProductBranchReferenceSnapshot,
        retention: ObservationRetentionObligation,
    ) -> Result<Self, ProductBranchObservationConstructionDenial> {
        if !retention.matches_basis(snapshot.commit().basis()) {
            return Err(ProductBranchObservationConstructionDenial::RetentionBasisMismatch);
        }
        Ok(Self {
            snapshot: Arc::new(snapshot),
            obligation: Arc::new(ProductBranchObservationObligation::owner_issued(retention)),
        })
    }

    pub fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.snapshot.owner()
    }

    pub fn branch_identity(&self) -> &ProductBranchIdentity {
        self.snapshot.branch()
    }

    pub fn lifecycle_incarnation(&self) -> ProductBranchLifecycleIncarnation {
        self.snapshot.lifecycle()
    }

    pub fn reference_generation(&self) -> ProductBranchReferenceGeneration {
        self.snapshot.generation()
    }

    pub fn selected_commit(&self) -> &CompositeCommitIdentity {
        self.snapshot.commit().identity()
    }

    pub fn basis(&self) -> &AdmittedCompositeRuntimeWorldBasis {
        self.snapshot.commit().basis()
    }

    pub(crate) fn retention_obligation(&self) -> &ProductBranchObservationObligation {
        &self.obligation
    }

    pub fn compare(&self, observed: &Self) -> Result<(), ProductBranchObservationMismatch> {
        let mut axes = Vec::new();
        if self.owner_identity() != observed.owner_identity() {
            axes.push(ProductBranchObservationMismatchAxis::OwnerIdentity);
        }
        if self.branch_identity() != observed.branch_identity() {
            axes.push(ProductBranchObservationMismatchAxis::BranchIdentity);
        }
        if self.lifecycle_incarnation() != observed.lifecycle_incarnation() {
            axes.push(ProductBranchObservationMismatchAxis::LifecycleIncarnation);
        }
        if self.reference_generation() != observed.reference_generation() {
            axes.push(ProductBranchObservationMismatchAxis::ReferenceGeneration);
        }
        if self.selected_commit() != observed.selected_commit() {
            axes.push(ProductBranchObservationMismatchAxis::SelectedCompositeCommit);
        }
        if crate::basis::compare_exact(self.basis(), observed.basis()).is_err() {
            axes.push(ProductBranchObservationMismatchAxis::CompositeBasis);
        }
        if axes.is_empty() {
            Ok(())
        } else {
            Err(ProductBranchObservationMismatch { axes })
        }
    }
}
