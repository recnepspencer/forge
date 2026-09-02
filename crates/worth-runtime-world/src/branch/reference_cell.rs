use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::history::{
    CompositeHistoryCatalog, CompositeHistoryCatalogDenial, CompositeRuntimeWorldCommit,
};
use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
    RuntimeWorldOwnerIdentity,
};
use crate::retention::RetentionTransferReceipt;
use crate::retention::{RetentionObligationDenial, RuntimeWorldRetentionOwner};

use super::observation::{
    ProductBranchObservation, ProductBranchObservationAdmissionFailure,
    ProductBranchObservationMismatch,
};
pub(crate) use protection::ProductBranchHeadProtectionDenial;
pub(crate) use protection::{
    ProductBranchHeadProtection, ProductBranchHeadProtectionAdmissionFailure,
};

mod protection;

#[derive(Debug)]
struct ProductBranchReferenceImage {
    snapshot: ProductBranchReferenceSnapshot,
    protection: ProductBranchHeadProtection,
}

#[derive(Debug, Clone)]
struct ReferenceCellState {
    current: Arc<RwLock<ProductBranchReferenceImage>>,
}

impl ReferenceCellState {
    fn new(initial: ProductBranchReferenceImage) -> Self {
        Self {
            current: Arc::new(RwLock::new(initial)),
        }
    }

    fn read(&self) -> RwLockReadGuard<'_, ProductBranchReferenceImage> {
        self.current
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn write(&self) -> RwLockWriteGuard<'_, ProductBranchReferenceImage> {
        self.current
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

/// One owner-issued immutable product-reference image. Its commit supplies
/// both selected identity and basis, preventing mixed observations.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchReferenceSnapshot {
    owner: RuntimeWorldOwnerIdentity,
    branch: ProductBranchIdentity,
    lifecycle: ProductBranchLifecycleIncarnation,
    generation: ProductBranchReferenceGeneration,
    commit: Arc<CompositeRuntimeWorldCommit>,
}

impl PartialEq for ProductBranchReferenceSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.branch == other.branch
            && self.lifecycle == other.lifecycle
            && self.generation == other.generation
            && self.commit.identity() == other.commit.identity()
            && crate::basis::compare_exact(self.commit.basis(), other.commit.basis()).is_ok()
    }
}

impl Eq for ProductBranchReferenceSnapshot {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchReferenceSnapshotDenial {
    OwnerMismatch,
    BranchOwnerMismatch,
    LifecycleOwnerMismatch,
    CommitOwnerMismatch,
    BasisOwnerMismatch,
}

impl ProductBranchReferenceSnapshot {
    pub(crate) fn owner_issued(
        owner: RuntimeWorldOwnerIdentity,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        generation: ProductBranchReferenceGeneration,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<Self, ProductBranchReferenceSnapshotDenial> {
        if branch.owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::BranchOwnerMismatch);
        }
        if lifecycle.owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::LifecycleOwnerMismatch);
        }
        if commit.identity().owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::CommitOwnerMismatch);
        }
        if commit.basis().owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::BasisOwnerMismatch);
        }
        Ok(Self {
            owner,
            branch,
            lifecycle,
            generation,
            commit,
        })
    }

    pub(crate) const fn owner(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) fn branch(&self) -> &ProductBranchIdentity {
        &self.branch
    }

    pub(crate) const fn lifecycle(&self) -> ProductBranchLifecycleIncarnation {
        self.lifecycle
    }

    pub(crate) const fn generation(&self) -> ProductBranchReferenceGeneration {
        self.generation
    }

    pub(crate) fn commit(&self) -> &CompositeRuntimeWorldCommit {
        &self.commit
    }
}

/// Independently borrowable product-reference cell; clones share only this
/// branch's synchronization domain.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchReferenceCell {
    state: ReferenceCellState,
}

/// Why a reference movement could not replace the selected product head.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ProductBranchReferenceCellDenial {
    ExpectedHeadMismatch(ProductBranchObservationMismatch),
    SuccessorOwnerMismatch,
    SuccessorBranchMismatch,
    SuccessorLifecycleMismatch,
    SuccessorGenerationMismatch {
        expected: ProductBranchReferenceGeneration,
        actual: ProductBranchReferenceGeneration,
    },
    SuccessorProtectionMismatch,
    GenerationExhausted,
}

#[derive(Debug)]
pub(crate) struct ProductBranchReferencePublishFailure {
    denial: ProductBranchReferenceCellDenial,
    successor_protection: ProductBranchHeadProtection,
}

impl ProductBranchReferencePublishFailure {
    pub(crate) fn denial(&self) -> &ProductBranchReferenceCellDenial {
        &self.denial
    }

    pub(crate) fn into_successor_protection(self) -> ProductBranchHeadProtection {
        self.successor_protection
    }
}

#[derive(Debug)]
pub(crate) enum ProductBranchReferenceObservationFailure {
    HistoryProtection(CompositeHistoryCatalogDenial),
    Retention(RetentionObligationDenial),
    ObservationBinding(ProductBranchObservationAdmissionFailure),
}

/// Exact old/new images installed by one movement; it mints no owner artifact.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchReferenceMovement {
    before: ProductBranchReferenceSnapshot,
    after: ProductBranchReferenceSnapshot,
    retention_transfer: RetentionTransferReceipt,
}

impl ProductBranchReferenceMovement {
    pub(crate) fn before(&self) -> &ProductBranchReferenceSnapshot {
        &self.before
    }

    pub(crate) fn after(&self) -> &ProductBranchReferenceSnapshot {
        &self.after
    }

    pub(crate) fn retention_transfer(&self) -> &RetentionTransferReceipt {
        &self.retention_transfer
    }
}

impl ProductBranchReferenceCell {
    pub(crate) fn new(
        protection: ProductBranchHeadProtection,
    ) -> Result<Self, ProductBranchHeadProtectionAdmissionFailure> {
        let initial = protection.snapshot().clone();
        match protection.validate() {
            Ok(()) => Ok(Self {
                state: ReferenceCellState::new(ProductBranchReferenceImage {
                    snapshot: initial,
                    protection,
                }),
            }),
            Err(denial) => Err(protection.into_admission_failure(denial)),
        }
    }

    /// Capture an immutable image whose commit stays alive across movement.
    pub(crate) fn atomic_snapshot(&self) -> ProductBranchReferenceSnapshot {
        self.state.read().snapshot.clone()
    }

    /// Protect and recheck one candidate before asking the component owner for
    /// its exact observation claims. The equal recheck is the head
    /// linearization point; all owner calls happen after its read guard drops.
    pub(crate) fn observe<D, I, T>(
        &self,
        history: &CompositeHistoryCatalog,
        retention_owner: &RuntimeWorldRetentionOwner<D, I, T>,
    ) -> Result<ProductBranchObservation, ProductBranchReferenceObservationFailure>
    where
        D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
        I: Copy + Ord + Send + Sync + 'static,
        T: Copy + Ord + Send + Sync + 'static,
    {
        loop {
            let candidate = self.atomic_snapshot();
            let history_protection = match history.protect_explicit_commit(candidate.commit()) {
                Ok(protection) => protection,
                Err(denial) => {
                    let current = self.state.read();
                    if current.snapshot != candidate {
                        drop(current);
                        continue;
                    }
                    return Err(ProductBranchReferenceObservationFailure::HistoryProtection(
                        denial,
                    ));
                }
            };
            let current = self.state.read();
            let still_selected = current.snapshot == candidate;
            drop(current);
            if !still_selected {
                drop(history_protection);
                continue;
            }

            let components = match retention_owner.issue_observation(candidate.commit()) {
                Ok(components) => components,
                Err(denial) => {
                    drop(history_protection);
                    return Err(ProductBranchReferenceObservationFailure::Retention(denial));
                }
            };
            return ProductBranchObservation::owner_issued(
                candidate,
                components,
                history_protection,
            )
            .map_err(ProductBranchReferenceObservationFailure::ObservationBinding);
        }
    }

    /// Replace only if the complete expected observation is still selected.
    /// Expected-currentness, successor validation, and pair replacement use
    /// one short branch-local lock; no owner call occurs while it is held.
    pub(crate) fn compare_and_publish(
        &self,
        expected: &ProductBranchObservation,
        successor: ProductBranchHeadProtection,
    ) -> Result<ProductBranchReferenceMovement, ProductBranchReferencePublishFailure> {
        let expected_snapshot = expected.snapshot();
        let mut current = self.state.write();
        if &current.snapshot != expected_snapshot {
            return Err(ProductBranchReferencePublishFailure {
                denial: ProductBranchReferenceCellDenial::ExpectedHeadMismatch(
                    expected
                        .mismatch_against_snapshot(&current.snapshot)
                        .expect("snapshot equality and observation comparison must agree"),
                ),
                successor_protection: successor,
            });
        }
        if let Err(denial) = validate_successor(&current.snapshot, &successor) {
            return Err(ProductBranchReferencePublishFailure {
                denial,
                successor_protection: successor,
            });
        }

        let successor_snapshot = successor.snapshot().clone();
        let Some(successor_receipt) = successor.transfer_receipt().cloned() else {
            return Err(ProductBranchReferencePublishFailure {
                denial: ProductBranchReferenceCellDenial::SuccessorProtectionMismatch,
                successor_protection: successor,
            });
        };
        let movement = ProductBranchReferenceMovement {
            before: current.snapshot.clone(),
            after: successor_snapshot.clone(),
            retention_transfer: successor_receipt,
        };
        let old_image = std::mem::replace(
            &mut *current,
            ProductBranchReferenceImage {
                snapshot: successor_snapshot,
                protection: successor,
            },
        );
        drop(current);
        drop(old_image);
        Ok(movement)
    }

    #[cfg(test)]
    fn hold_for_test(&self) -> impl Drop + '_ {
        self.state.write()
    }
}

fn validate_successor(
    current: &ProductBranchReferenceSnapshot,
    successor: &ProductBranchHeadProtection,
) -> Result<(), ProductBranchReferenceCellDenial> {
    let successor_snapshot = successor.snapshot();
    if successor_snapshot.owner() != current.owner() {
        return Err(ProductBranchReferenceCellDenial::SuccessorOwnerMismatch);
    }
    if successor_snapshot.branch() != current.branch() {
        return Err(ProductBranchReferenceCellDenial::SuccessorBranchMismatch);
    }
    if successor_snapshot.lifecycle() != current.lifecycle() {
        return Err(ProductBranchReferenceCellDenial::SuccessorLifecycleMismatch);
    }
    let expected_generation = current
        .generation()
        .advance()
        .map_err(|_| ProductBranchReferenceCellDenial::GenerationExhausted)?;
    if successor_snapshot.generation() != expected_generation {
        return Err(
            ProductBranchReferenceCellDenial::SuccessorGenerationMismatch {
                expected: expected_generation,
                actual: successor_snapshot.generation(),
            },
        );
    }
    if successor.product_head().owner_identity() != successor_snapshot.owner()
        || !successor
            .product_head()
            .matches_basis(successor_snapshot.commit().basis())
        || successor.product_head_history().owner_identity() != successor_snapshot.owner()
        || !successor
            .product_head_history()
            .matches_commit(successor_snapshot.commit())
        || successor.transfer_receipt().is_none()
    {
        return Err(ProductBranchReferenceCellDenial::SuccessorProtectionMismatch);
    }
    Ok(())
}

#[cfg(test)]
#[path = "reference_cell_tests.rs"]
mod tests;
