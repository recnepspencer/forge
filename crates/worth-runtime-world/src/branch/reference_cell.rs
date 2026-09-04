use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::history::{CompositeHistoryCatalog, CompositeHistoryCatalogDenial};
use crate::identity::ProductBranchReferenceGeneration;
use crate::retention::RetentionTransferReceipt;
use crate::retention::{RetentionObligationDenial, RuntimeWorldRetentionOwner};

use super::observation::{
    ProductBranchObservation, ProductBranchObservationAdmissionFailure,
    ProductBranchObservationMismatch,
};
use super::reference_snapshot::ProductBranchReferenceSnapshot;
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
    observed_head: ProductBranchReferenceSnapshot,
    successor_protection: ProductBranchHeadProtection,
}

impl ProductBranchReferencePublishFailure {
    pub(crate) fn denial(&self) -> &ProductBranchReferenceCellDenial {
        &self.denial
    }

    pub(crate) fn observed_head(&self) -> &ProductBranchReferenceSnapshot {
        &self.observed_head
    }

    pub(crate) fn into_successor_protection(self) -> ProductBranchHeadProtection {
        self.successor_protection
    }

    pub(crate) fn into_recovery_parts(
        self,
    ) -> (ProductBranchReferenceSnapshot, ProductBranchHeadProtection) {
        (self.observed_head, self.successor_protection)
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

    /// Run `f` on `argument` while this cell provably still carries
    /// `expected`. The branch-local read guard is held across `f`, so nothing
    /// can publish past the expected head until `f` returns: what `f`
    /// installs from that head is installed from a current head, not from one
    /// that was current when it was last checked. `f` must not touch this
    /// cell. A displaced head returns the head the cell carries, with the
    /// argument untouched, so the caller keeps whatever custody it holds.
    pub(crate) fn while_current<A, R>(
        &self,
        expected: &ProductBranchObservation,
        argument: A,
        f: impl FnOnce(A) -> R,
    ) -> Result<R, (ProductBranchReferenceSnapshot, A)> {
        let current = self.state.read();
        if expected
            .mismatch_against_snapshot(&current.snapshot)
            .is_some()
        {
            return Err((current.snapshot.clone(), argument));
        }
        Ok(f(argument))
    }

    /// Give back the protection of a cell nobody else shares. A cell the
    /// registry never installed has exactly one holder, so for it this is
    /// total; a shared cell keeps its protection and answers `None`.
    pub(crate) fn into_protection(self) -> Option<ProductBranchHeadProtection> {
        Arc::try_unwrap(self.state.current).ok().map(|image| {
            image
                .into_inner()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .protection
        })
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
                observed_head: current.snapshot.clone(),
                successor_protection: successor,
            });
        }
        if let Err(denial) = validate_successor(&current.snapshot, &successor) {
            return Err(ProductBranchReferencePublishFailure {
                denial,
                observed_head: current.snapshot.clone(),
                successor_protection: successor,
            });
        }

        let successor_snapshot = successor.snapshot().clone();
        let Some(successor_receipt) = successor.transfer_receipt().cloned() else {
            return Err(ProductBranchReferencePublishFailure {
                denial: ProductBranchReferenceCellDenial::SuccessorProtectionMismatch,
                observed_head: current.snapshot.clone(),
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

    /// Whether a writer would block right now: true exactly while some guard
    /// on this cell is held. Non-blocking, so a guarded section may ask it.
    #[cfg(test)]
    fn writers_are_locked_out_for_test(&self) -> bool {
        matches!(
            self.state.current.try_write(),
            Err(std::sync::TryLockError::WouldBlock)
        )
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
