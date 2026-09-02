use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::history::ExplicitCommitHistoryProtectionObligation;
use crate::identity::{
    CompositeCommitIdentity, ProductBranchIdentity, ProductBranchLifecycleIncarnation,
    ProductBranchReferenceGeneration, RuntimeWorldOwnerIdentity,
};
use crate::retention::ObservationRetentionObligation;

use super::reference_cell::ProductBranchReferenceSnapshot;

/// Complete product-head observation used by every compare-and-publish
/// operation. A single reference snapshot supplies the commit and basis;
/// retention is an opaque handoff owned operationally by Phase 2.
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

/// Cloned observations share one coherent pair of exact component and History
/// obligations. Cloning this wrapper acquires neither authority again.
#[derive(Debug)]
pub(crate) struct ProductBranchObservationObligation {
    _components: ObservationRetentionObligation,
    _history: ExplicitCommitHistoryProtectionObligation,
}

impl ProductBranchObservationObligation {
    fn owner_issued(
        components: ObservationRetentionObligation,
        history: ExplicitCommitHistoryProtectionObligation,
    ) -> Self {
        Self {
            _components: components,
            _history: history,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchObservationAuthorityDenial {
    ComponentCommitOrBasisMismatch,
    HistoryCommitOrOwnerMismatch,
}

#[derive(Debug)]
pub(crate) struct ProductBranchObservationAdmissionFailure {
    denial: ProductBranchObservationAuthorityDenial,
    snapshot: ProductBranchReferenceSnapshot,
    components: ObservationRetentionObligation,
    history: ExplicitCommitHistoryProtectionObligation,
}

impl ProductBranchObservationAdmissionFailure {
    pub(crate) const fn denial(&self) -> ProductBranchObservationAuthorityDenial {
        self.denial
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProductBranchReferenceSnapshot,
        ObservationRetentionObligation,
        ExplicitCommitHistoryProtectionObligation,
    ) {
        (self.snapshot, self.components, self.history)
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
        components: ObservationRetentionObligation,
        history: ExplicitCommitHistoryProtectionObligation,
    ) -> Result<Self, ProductBranchObservationAdmissionFailure> {
        if !components.matches_captured_head(snapshot.commit()) {
            return Err(ProductBranchObservationAdmissionFailure {
                denial: ProductBranchObservationAuthorityDenial::ComponentCommitOrBasisMismatch,
                snapshot,
                components,
                history,
            });
        }
        if !history.matches_commit(snapshot.commit())
            || history.owner_identity() != snapshot.owner()
        {
            return Err(ProductBranchObservationAdmissionFailure {
                denial: ProductBranchObservationAuthorityDenial::HistoryCommitOrOwnerMismatch,
                snapshot,
                components,
                history,
            });
        }
        Ok(Self {
            snapshot: Arc::new(snapshot),
            obligation: Arc::new(ProductBranchObservationObligation::owner_issued(
                components, history,
            )),
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
