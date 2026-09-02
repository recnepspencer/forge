use std::sync::{Arc, RwLock, RwLockWriteGuard};

use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
    RuntimeWorldOwnerIdentity,
};
use crate::retention::ObservationRetentionObligation;

use super::observation::{ProductBranchObservation, ProductBranchObservationMismatch};

/// Branch-local synchronization protects only one immutable snapshot pointer;
/// no owner call can occur while the lock is held.
#[derive(Debug)]
struct ReferenceCellState<T> {
    current: Arc<RwLock<Arc<T>>>,
}

impl<T> Clone for ReferenceCellState<T> {
    fn clone(&self) -> Self {
        Self {
            current: Arc::clone(&self.current),
        }
    }
}

impl<T> ReferenceCellState<T> {
    fn new(initial: T) -> Self {
        Self {
            current: Arc::new(RwLock::new(Arc::new(initial))),
        }
    }

    fn snapshot(&self) -> Arc<T> {
        self.current
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    fn write(&self) -> RwLockWriteGuard<'_, Arc<T>> {
        self.current
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn compare_and_replace(&self, expected: &T, successor: T) -> Result<(T, T), T>
    where
        T: Clone + PartialEq,
    {
        let mut current = self.write();
        if current.as_ref() != expected {
            return Err(current.as_ref().clone());
        }
        let before = current.as_ref().clone();
        let after = successor.clone();
        *current = Arc::new(successor);
        Ok((before, after))
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
    state: ReferenceCellState<ProductBranchReferenceSnapshot>,
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
    GenerationExhausted,
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
    pub(crate) fn new(initial: ProductBranchReferenceSnapshot) -> Self {
        Self {
            state: ReferenceCellState::new(initial),
        }
    }

    /// Capture an immutable image whose commit stays alive across movement.
    pub(crate) fn atomic_snapshot(&self) -> ProductBranchReferenceSnapshot {
        self.state.snapshot().as_ref().clone()
    }

    /// Admit a managed observation from one exact image and opaque,
    /// owner-issued retention handoff.
    pub(crate) fn observe(
        &self,
        retention: ObservationRetentionObligation,
    ) -> ProductBranchObservation {
        ProductBranchObservation::owner_issued(self.atomic_snapshot(), retention)
    }

    /// Replace only if the complete expected observation is still selected.
    /// Validation and the swap use this branch's short lock; no external work.
    pub(crate) fn compare_and_publish(
        &self,
        expected: &ProductBranchObservation,
        successor: ProductBranchReferenceSnapshot,
    ) -> Result<ProductBranchReferenceMovement, ProductBranchReferenceCellDenial> {
        let expected_snapshot = expected.snapshot();
        validate_successor(expected_snapshot, &successor)?;
        match self.state.compare_and_replace(expected_snapshot, successor) {
            Ok((before, after)) => Ok(ProductBranchReferenceMovement { before, after }),
            Err(observed) => Err(ProductBranchReferenceCellDenial::ExpectedHeadMismatch(
                expected
                    .mismatch_against_snapshot(&observed)
                    .expect("snapshot equality and observation comparison must agree"),
            )),
        }
    }
}

fn validate_successor(
    current: &ProductBranchReferenceSnapshot,
    successor: &ProductBranchReferenceSnapshot,
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    use super::ReferenceCellState;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestReferenceImage {
        generation: u64,
        commit: u64,
    }

    fn replace_if_current(
        state: &ReferenceCellState<TestReferenceImage>,
        expected: &TestReferenceImage,
        successor: TestReferenceImage,
    ) -> bool {
        state.compare_and_replace(expected, successor).is_ok()
    }

    #[test]
    fn cloned_reference_handles_share_one_immutable_image() {
        let state = ReferenceCellState::new(TestReferenceImage {
            generation: 0,
            commit: 7,
        });
        let clone = state.clone();
        let old = state.snapshot();

        assert!(replace_if_current(
            &clone,
            old.as_ref(),
            TestReferenceImage {
                generation: 1,
                commit: 8,
            },
        ));

        assert_eq!(old.generation, 0);
        assert_eq!(old.commit, 7);
        assert_eq!(state.snapshot().generation, 1);
        assert_eq!(state.snapshot().commit, 8);
    }

    #[test]
    fn same_head_contention_wins_one_compare_and_replace() {
        let state = ReferenceCellState::new(TestReferenceImage {
            generation: 0,
            commit: 7,
        });
        let expected = state.snapshot();
        let start = Arc::new(Barrier::new(3));
        let (results, receive) = mpsc::channel();

        let first = state.clone();
        let first_start = Arc::clone(&start);
        let first_expected = Arc::clone(&expected);
        let first_results = results.clone();
        let first_worker = thread::spawn(move || {
            first_start.wait();
            first_results
                .send(replace_if_current(
                    &first,
                    first_expected.as_ref(),
                    TestReferenceImage {
                        generation: 1,
                        commit: 8,
                    },
                ))
                .expect("first contention result receiver remains live");
        });

        let second = state.clone();
        let second_start = Arc::clone(&start);
        let second_expected = Arc::clone(&expected);
        let second_worker = thread::spawn(move || {
            second_start.wait();
            results
                .send(replace_if_current(
                    &second,
                    second_expected.as_ref(),
                    TestReferenceImage {
                        generation: 1,
                        commit: 9,
                    },
                ))
                .expect("second contention result receiver remains live");
        });

        start.wait();
        let first_won = receive
            .recv_timeout(Duration::from_secs(1))
            .expect("first contention worker completes");
        let second_won = receive
            .recv_timeout(Duration::from_secs(1))
            .expect("second contention worker completes");
        first_worker.join().expect("first worker does not panic");
        second_worker.join().expect("second worker does not panic");

        assert_ne!(first_won, second_won);
        assert_eq!(state.snapshot().generation, 1);
        assert!(matches!(state.snapshot().commit, 8 | 9));
    }

    #[test]
    fn unrelated_reference_progress_does_not_wait_on_another_cell() {
        let held = ReferenceCellState::new(TestReferenceImage {
            generation: 0,
            commit: 7,
        });
        let unrelated = ReferenceCellState::new(TestReferenceImage {
            generation: 0,
            commit: 11,
        });
        let held_guard = held.write();
        let (completed, receive) = mpsc::channel();
        let unrelated_worker_state = unrelated.clone();
        let worker = thread::spawn(move || {
            let expected = unrelated_worker_state.snapshot();
            let did_replace = replace_if_current(
                &unrelated_worker_state,
                expected.as_ref(),
                TestReferenceImage {
                    generation: 1,
                    commit: 12,
                },
            );
            completed
                .send(did_replace)
                .expect("unrelated completion receiver remains live");
        });

        let did_replace = receive
            .recv_timeout(Duration::from_secs(1))
            .expect("an unrelated reference completes while this cell is held");
        drop(held_guard);
        worker.join().expect("unrelated worker does not panic");

        assert!(did_replace);
        assert_eq!(held.snapshot().generation, 0);
        assert_eq!(unrelated.snapshot().generation, 1);
    }
}
