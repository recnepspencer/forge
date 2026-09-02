use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::history::{
    CompositeHistoryCatalog, CompositeHistoryCatalogDenial, CompositeRuntimeWorldCommit,
    ProductHeadHistoryProtectionObligation,
};
use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
    RuntimeWorldOwnerIdentity,
};
use crate::retention::{RetentionObligationDenial, RuntimeWorldRetentionOwner};

use super::observation::{
    ProductBranchObservation, ProductBranchObservationAdmissionFailure,
    ProductBranchObservationMismatch,
};

#[derive(Debug)]
struct ProductBranchReferenceImage {
    snapshot: ProductBranchReferenceSnapshot,
    product_head_history: ProductHeadHistoryProtectionObligation,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchReferenceCellAdmissionDenial {
    ProductHeadOwnerMismatch,
    ProductHeadCommitMismatch,
}

#[derive(Debug)]
pub(crate) struct ProductBranchReferenceCellAdmissionFailure {
    denial: ProductBranchReferenceCellAdmissionDenial,
    product_head_history: ProductHeadHistoryProtectionObligation,
}

impl ProductBranchReferenceCellAdmissionFailure {
    pub(crate) const fn denial(&self) -> ProductBranchReferenceCellAdmissionDenial {
        self.denial
    }

    pub(crate) fn into_product_head_history(self) -> ProductHeadHistoryProtectionObligation {
        self.product_head_history
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
    SuccessorHistoryOwnerMismatch,
    SuccessorHistoryCommitMismatch,
    GenerationExhausted,
}

#[derive(Debug)]
pub(crate) struct ProductBranchReferencePublishFailure {
    denial: ProductBranchReferenceCellDenial,
    successor_history: ProductHeadHistoryProtectionObligation,
}

impl ProductBranchReferencePublishFailure {
    pub(crate) fn denial(&self) -> &ProductBranchReferenceCellDenial {
        &self.denial
    }

    pub(crate) fn into_successor_history(self) -> ProductHeadHistoryProtectionObligation {
        self.successor_history
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
}

impl ProductBranchReferenceMovement {
    pub(crate) fn before(&self) -> &ProductBranchReferenceSnapshot {
        &self.before
    }

    pub(crate) fn after(&self) -> &ProductBranchReferenceSnapshot {
        &self.after
    }
}

impl ProductBranchReferenceCell {
    pub(crate) fn new(
        initial: ProductBranchReferenceSnapshot,
        product_head_history: ProductHeadHistoryProtectionObligation,
    ) -> Result<Self, ProductBranchReferenceCellAdmissionFailure> {
        match validate_product_head(&initial, &product_head_history) {
            Ok(()) => Ok(Self {
                state: ReferenceCellState::new(ProductBranchReferenceImage {
                    snapshot: initial,
                    product_head_history,
                }),
            }),
            Err(denial) => Err(ProductBranchReferenceCellAdmissionFailure {
                denial,
                product_head_history,
            }),
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
        successor: ProductBranchReferenceSnapshot,
        successor_history: ProductHeadHistoryProtectionObligation,
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
                successor_history,
            });
        }
        if let Err(denial) = validate_successor(&current.snapshot, &successor, &successor_history) {
            return Err(ProductBranchReferencePublishFailure {
                denial,
                successor_history,
            });
        }

        let movement = ProductBranchReferenceMovement {
            before: current.snapshot.clone(),
            after: successor.clone(),
        };
        let old_image = std::mem::replace(
            &mut *current,
            ProductBranchReferenceImage {
                snapshot: successor,
                product_head_history: successor_history,
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

fn validate_product_head(
    snapshot: &ProductBranchReferenceSnapshot,
    product_head_history: &ProductHeadHistoryProtectionObligation,
) -> Result<(), ProductBranchReferenceCellAdmissionDenial> {
    if product_head_history.owner_identity() != snapshot.owner() {
        return Err(ProductBranchReferenceCellAdmissionDenial::ProductHeadOwnerMismatch);
    }
    if !product_head_history.matches_commit(snapshot.commit()) {
        return Err(ProductBranchReferenceCellAdmissionDenial::ProductHeadCommitMismatch);
    }
    Ok(())
}

fn validate_successor(
    current: &ProductBranchReferenceSnapshot,
    successor: &ProductBranchReferenceSnapshot,
    history: &ProductHeadHistoryProtectionObligation,
) -> Result<(), ProductBranchReferenceCellDenial> {
    if successor.owner() != current.owner() {
        return Err(ProductBranchReferenceCellDenial::SuccessorOwnerMismatch);
    }
    if successor.branch() != current.branch() {
        return Err(ProductBranchReferenceCellDenial::SuccessorBranchMismatch);
    }
    if successor.lifecycle() != current.lifecycle() {
        return Err(ProductBranchReferenceCellDenial::SuccessorLifecycleMismatch);
    }
    let expected_generation = current
        .generation()
        .advance()
        .map_err(|_| ProductBranchReferenceCellDenial::GenerationExhausted)?;
    if successor.generation() != expected_generation {
        return Err(
            ProductBranchReferenceCellDenial::SuccessorGenerationMismatch {
                expected: expected_generation,
                actual: successor.generation(),
            },
        );
    }
    if history.owner_identity() != successor.owner() {
        return Err(ProductBranchReferenceCellDenial::SuccessorHistoryOwnerMismatch);
    }
    if !history.matches_commit(successor.commit()) {
        return Err(ProductBranchReferenceCellDenial::SuccessorHistoryCommitMismatch);
    }
    Ok(())
}

#[cfg(test)]
#[path = "reference_test_fixture.rs"]
mod reference_test_fixture;

#[cfg(test)]
#[path = "reference_cell_tests.rs"]
mod tests;
