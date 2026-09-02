use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::{
    CompositeCommitIdentity, ProductBranchIdentity, ProductBranchLifecycleIncarnation,
    ProductBranchReferenceGeneration, RuntimeWorldOwnerIdentity,
};

/// Complete product-head observation used by every compare-and-publish
/// operation. A partial tuple is not an observation.
#[derive(Debug)]
pub struct ProductBranchObservation {
    owner: RuntimeWorldOwnerIdentity,
    branch: ProductBranchIdentity,
    lifecycle: ProductBranchLifecycleIncarnation,
    generation: ProductBranchReferenceGeneration,
    selected_commit: CompositeCommitIdentity,
    basis: AdmittedCompositeRuntimeWorldBasis,
    obligation: Arc<ProductBranchObservationObligation>,
}

impl Clone for ProductBranchObservation {
    fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            branch: self.branch.clone(),
            lifecycle: self.lifecycle,
            generation: self.generation,
            selected_commit: self.selected_commit.clone(),
            basis: self.basis.clone(),
            obligation: Arc::clone(&self.obligation),
        }
    }
}

impl PartialEq for ProductBranchObservation {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.branch == other.branch
            && self.lifecycle == other.lifecycle
            && self.generation == other.generation
            && self.selected_commit == other.selected_commit
            && self.basis == other.basis
    }
}

impl Eq for ProductBranchObservation {}

/// Cloned observations share this one operational admission obligation. The
/// retention lane will attach its bounded registry accounting to this token;
/// a value clone never becomes a fresh observation admission.
#[derive(Debug)]
struct ProductBranchObservationObligation {
    _private: (),
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
    pub(crate) fn issued(parts: ProductBranchObservationParts) -> Self {
        Self {
            owner: parts.owner,
            branch: parts.branch,
            lifecycle: parts.lifecycle,
            generation: parts.generation,
            selected_commit: parts.selected_commit,
            basis: parts.basis,
            obligation: Arc::new(ProductBranchObservationObligation { _private: () }),
        }
    }

    pub fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub fn branch_identity(&self) -> &ProductBranchIdentity {
        &self.branch
    }

    pub fn lifecycle_incarnation(&self) -> ProductBranchLifecycleIncarnation {
        self.lifecycle
    }

    pub fn reference_generation(&self) -> ProductBranchReferenceGeneration {
        self.generation
    }

    pub fn selected_commit(&self) -> &CompositeCommitIdentity {
        &self.selected_commit
    }

    pub fn basis(&self) -> &AdmittedCompositeRuntimeWorldBasis {
        &self.basis
    }

    pub fn compare(&self, observed: &Self) -> Result<(), ProductBranchObservationMismatch> {
        let mut axes = Vec::new();
        if self.owner != observed.owner {
            axes.push(ProductBranchObservationMismatchAxis::OwnerIdentity);
        }
        if self.branch != observed.branch {
            axes.push(ProductBranchObservationMismatchAxis::BranchIdentity);
        }
        if self.lifecycle != observed.lifecycle {
            axes.push(ProductBranchObservationMismatchAxis::LifecycleIncarnation);
        }
        if self.generation != observed.generation {
            axes.push(ProductBranchObservationMismatchAxis::ReferenceGeneration);
        }
        if self.selected_commit != observed.selected_commit {
            axes.push(ProductBranchObservationMismatchAxis::SelectedCompositeCommit);
        }
        if self.basis != observed.basis {
            axes.push(ProductBranchObservationMismatchAxis::CompositeBasis);
        }
        if axes.is_empty() {
            Ok(())
        } else {
            Err(ProductBranchObservationMismatch { axes })
        }
    }
}

pub(crate) struct ProductBranchObservationParts {
    pub(crate) owner: RuntimeWorldOwnerIdentity,
    pub(crate) branch: ProductBranchIdentity,
    pub(crate) lifecycle: ProductBranchLifecycleIncarnation,
    pub(crate) generation: ProductBranchReferenceGeneration,
    pub(crate) selected_commit: CompositeCommitIdentity,
    pub(crate) basis: AdmittedCompositeRuntimeWorldBasis,
}
