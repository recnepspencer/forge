use std::sync::Arc;

use crate::branch::{ProductBranchObservation, ProductBranchReferenceSnapshot};
use crate::history::{CompositeHistoryCatalog, CompositeRuntimeWorldCommit};
use crate::identity::{
    ProductBranchIdentity, ProductBranchIncarnation, ProductBranchReferenceGeneration,
};
use crate::lifecycle::owner::RuntimeWorldOperationReservation;
use crate::lifecycle::RuntimeWorldBranchCreationOutcome;
use crate::publication::CompositeOwnerExecutionResults;
use crate::retention::{
    PublicationRetentionObligation, ReservedComponentPinPairCapacity, RetentionObligationDenial,
};

use super::super::super::super::RuntimeWorldOwnerRoot;
use super::{ForkedBranchInstallation, ForkedBranchRecoveryContext};

pub(super) struct ForkedBranchDestination {
    pub(super) branch: ProductBranchIdentity,
    pub(super) lifecycle: ProductBranchIncarnation,
    pub(super) reservation: crate::branch::registry::ProductBranchRegistryReservation,
}

struct SettledForkedAttempt {
    attempt_identity: crate::identity::CompositePublicationAttemptIdentity,
    expected_head: ProductBranchObservation,
    reserved_commit_identity: crate::identity::CompositeCommitIdentity,
    product_unpublished_identity: crate::identity::ProductUnpublishedOwnerEffectsIdentity,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    recovery_slot: crate::recovery::ReservedProductUnpublishedSlot,
    reserved_component_pin_pair: ReservedComponentPinPairCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
    progress: crate::publication::CompositeAttemptProgress,
    owner_results: CompositeOwnerExecutionResults,
}

impl SettledForkedAttempt {
    fn new(
        attempt: crate::publication::ReservedBranchCreationAttempt,
        progress: crate::publication::CompositeAttemptProgress,
    ) -> Self {
        let (progress, owner_results) = progress
            .into_ready_results()
            .expect("settled branch execution carries ready owner results");
        let crate::publication::ReservedBranchCreationParts {
            identity,
            source,
            plan,
            capacities,
            cancellation: _,
            deadline,
            progress: _reserved_progress,
            counters: _,
        } = attempt.into_parts();
        assert!(
            owner_results.matches_creation_plan(&plan),
            "settled branch owner results must match the reserved creation plan"
        );
        let crate::publication::ReservedAttemptCapacityInputs {
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            reserved_recovery_slot: recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity: _,
            history,
            operation,
        } = capacities.into_parts();
        Self {
            attempt_identity: identity,
            expected_head: source,
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            recovery_slot,
            reserved_component_pin_pair,
            history,
            operation,
            deadline,
            progress,
            owner_results,
        }
    }
}

pub(super) struct ForkedBranchFinalization {
    destination: ForkedBranchDestination,
    recovery: ForkedBranchRecoveryContext,
    commit: Arc<CompositeRuntimeWorldCommit>,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    reserved_component_pin_pair: ReservedComponentPinPairCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
}

pub(super) struct PublicationBoundForkedBranch {
    destination: ForkedBranchDestination,
    recovery: ForkedBranchRecoveryContext,
    commit: Arc<CompositeRuntimeWorldCommit>,
    publication: PublicationRetentionObligation,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
}

pub(super) struct PendingForkedBranch {
    recovery: ForkedBranchRecoveryContext,
    commit: Arc<CompositeRuntimeWorldCommit>,
    capacity: ReservedComponentPinPairCapacity,
    denial: RetentionObligationDenial,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    history: CompositeHistoryCatalog,
    operation: RuntimeWorldOperationReservation,
}

pub(super) struct HistoryInstalledForkedBranch {
    pub(super) destination: ForkedBranchDestination,
    pub(super) recovery: ForkedBranchRecoveryContext,
    pub(super) commit: Arc<CompositeRuntimeWorldCommit>,
    pub(super) publication: PublicationRetentionObligation,
    pub(super) product_history: crate::history::ProductHeadHistoryProtectionObligation,
    pub(super) history: CompositeHistoryCatalog,
    pub(super) operation: RuntimeWorldOperationReservation,
}

pub(super) struct ObservedForkedBranch {
    pub(super) state: HistoryInstalledForkedBranch,
    pub(super) snapshot: ProductBranchReferenceSnapshot,
    pub(super) observation: ProductBranchObservation,
}

impl ForkedBranchFinalization {
    pub(super) fn from_installation(installation: ForkedBranchInstallation) -> Self {
        let ForkedBranchInstallation {
            branch,
            lifecycle,
            reservation,
            attempt,
            progress,
            successor_basis,
        } = installation;
        let settled = SettledForkedAttempt::new(attempt, progress);
        let SettledForkedAttempt {
            attempt_identity,
            expected_head,
            reserved_commit_identity,
            product_unpublished_identity,
            reserved_commit_capacity,
            recovery_slot,
            reserved_component_pin_pair,
            history,
            operation,
            deadline,
            progress,
            owner_results,
        } = settled;
        let commit = super::destination_commit(super::DestinationCommitInput {
            identity: reserved_commit_identity,
            expected_head: &expected_head,
            successor_basis: &successor_basis,
            attempt_identity: &attempt_identity,
            owner_results: &owner_results,
        });
        let retained_destination = (branch.clone(), lifecycle);
        Self {
            destination: ForkedBranchDestination {
                branch,
                lifecycle,
                reservation,
            },
            recovery: ForkedBranchRecoveryContext {
                identity: product_unpublished_identity,
                attempt_identity,
                expected_head,
                progress,
                successor_basis,
                owner_results,
                recovery_slot,
                deadline,
                destination: retained_destination,
            },
            commit,
            reserved_commit_capacity,
            reserved_component_pin_pair,
            history,
            operation,
        }
    }

    pub(super) fn bind_publication(
        self,
    ) -> Result<PublicationBoundForkedBranch, PendingForkedBranch> {
        let Self {
            destination,
            recovery,
            commit,
            reserved_commit_capacity,
            reserved_component_pin_pair,
            history,
            operation,
        } = self;
        match reserved_component_pin_pair.bind_publication(commit.basis()) {
            Ok(publication) => Ok(PublicationBoundForkedBranch {
                destination,
                recovery,
                commit,
                publication,
                reserved_commit_capacity,
                history,
                operation,
            }),
            Err((capacity, denial)) => {
                drop(destination.reservation);
                Err(PendingForkedBranch {
                    recovery,
                    commit,
                    capacity,
                    denial,
                    reserved_commit_capacity,
                    history,
                    operation,
                })
            }
        }
    }
}

impl PublicationBoundForkedBranch {
    pub(super) fn install_commit_and_history(self) -> HistoryInstalledForkedBranch {
        let Self {
            destination,
            recovery,
            commit,
            publication,
            reserved_commit_capacity,
            history,
            operation,
        } = self;
        let product_history =
            super::install_commit_and_history(&history, reserved_commit_capacity, &commit);
        HistoryInstalledForkedBranch {
            destination,
            recovery,
            commit,
            publication,
            product_history,
            history,
            operation,
        }
    }
}

impl PendingForkedBranch {
    /// The recovering operation reservation is released only after the record
    /// exists. Dropping it earlier would clear the close-admission ledger while
    /// no installed recovery slot yet denies `close()`.
    pub(super) fn into_product_unpublished(self) -> RuntimeWorldBranchCreationOutcome {
        let Self {
            recovery,
            commit,
            capacity,
            denial,
            reserved_commit_capacity,
            history,
            mut operation,
        } = self;
        let product_history =
            super::install_commit_and_history(&history, reserved_commit_capacity, &commit);
        operation
            .begin_recovery()
            .expect("a retained branch attempt enters recovery");
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(
            super::recovery::product_unpublished_pending(
                recovery,
                capacity,
                denial,
                product_history,
            ),
        )
    }
}

impl HistoryInstalledForkedBranch {
    pub(super) fn finish<D, I, E, Ctx, T>(
        self,
        owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    ) -> Result<RuntimeWorldBranchCreationOutcome, crate::branch::RuntimeWorldBranchAdmissionDenial>
    where
        D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
        I: Copy + Ord + Send + Sync + 'static,
        T: Copy + Ord + Send + Sync + 'static,
    {
        match self.issue_observation(owner) {
            Ok(observed) => observed.install(),
            Err(effects) => Ok(RuntimeWorldBranchCreationOutcome::ProductUnpublished(
                effects,
            )),
        }
    }

    fn issue_observation<D, I, E, Ctx, T>(
        self,
        owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    ) -> Result<ObservedForkedBranch, crate::recovery::ProductUnpublishedOwnerEffects>
    where
        D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
        I: Copy + Ord + Send + Sync + 'static,
        T: Copy + Ord + Send + Sync + 'static,
    {
        let authority = super::issue_observation_authority(owner, &self.history, &self.commit);
        #[cfg(test)]
        let authority = super::test_control::withhold_observation_authority_under_rehearsal(
            &self.recovery.identity,
            authority,
        );
        let (observation_components, observation_history) = match authority {
            Ok(authority) => authority,
            Err(()) => return Err(self.into_retained()),
        };
        let snapshot = ProductBranchReferenceSnapshot::owner_issued(
            owner.owner_identity(),
            self.destination.branch.clone(),
            self.destination.lifecycle,
            ProductBranchReferenceGeneration::initial(),
            Arc::clone(&self.commit),
        )
        .expect("owner-issued destination identity admits its reference snapshot");
        let observation = match ProductBranchObservation::owner_issued(
            snapshot.clone(),
            observation_components,
            observation_history,
        ) {
            Ok(observation) => observation,
            Err(failure) => {
                drop(failure.into_parts());
                return Err(self.into_retained());
            }
        };
        Ok(ObservedForkedBranch {
            state: self,
            snapshot,
            observation,
        })
    }

    /// The recovering operation reservation is released only after the record
    /// exists. Dropping it earlier would clear the close-admission ledger while
    /// no installed recovery slot yet denies `close()`.
    fn into_retained(self) -> crate::recovery::ProductUnpublishedOwnerEffects {
        let Self {
            recovery,
            publication,
            product_history,
            mut operation,
            ..
        } = self;
        operation
            .begin_recovery()
            .expect("a retained branch attempt enters recovery");
        super::recovery::retain_forked_effects(recovery, publication, product_history)
    }
}
