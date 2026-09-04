#[path = "finalization/install.rs"]
mod install;
#[path = "finalization/recovery.rs"]
mod recovery;
#[path = "finalization/state.rs"]
mod state;
#[cfg(test)]
#[path = "finalization/test_control.rs"]
mod test_control;

use std::sync::Arc;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{ProductBranchObservation, RuntimeWorldBranchAdmissionDenial};
use crate::history::{CompositeHistoryCatalog, CompositeRuntimeWorldCommit};
use crate::identity::{
    CompositePublicationAttemptIdentity, ProductBranchIdentity, ProductBranchIncarnation,
    ProductUnpublishedOwnerEffectsIdentity,
};
use crate::lifecycle::{RuntimeWorldBranchCreationOutcome, RuntimeWorldInstant};
use crate::publication::{
    CompositeAttemptProgress, CompositeOwnerExecutionResults, ReservedBranchCreationAttempt,
};
use crate::recovery::ReservedProductUnpublishedSlot;

use super::super::super::RuntimeWorldOwnerRoot;

pub(super) struct ForkedBranchInstallation {
    pub(super) branch: ProductBranchIdentity,
    pub(super) lifecycle: ProductBranchIncarnation,
    pub(super) reservation: crate::branch::registry::ProductBranchRegistryReservation,
    pub(super) attempt: ReservedBranchCreationAttempt,
    pub(super) progress: CompositeAttemptProgress,
    pub(super) successor_basis: AdmittedCompositeRuntimeWorldBasis,
}

pub(super) struct ForkedBranchRecoveryContext {
    pub(super) identity: ProductUnpublishedOwnerEffectsIdentity,
    pub(super) attempt_identity: CompositePublicationAttemptIdentity,
    pub(super) expected_head: ProductBranchObservation,
    pub(super) progress: CompositeAttemptProgress,
    pub(super) successor_basis: AdmittedCompositeRuntimeWorldBasis,
    pub(super) owner_results: CompositeOwnerExecutionResults,
    pub(super) recovery_slot: ReservedProductUnpublishedSlot,
    pub(super) deadline: Option<RuntimeWorldInstant>,
}

use state::ForkedBranchFinalization;

struct DestinationCommitInput<'a> {
    identity: crate::identity::CompositeCommitIdentity,
    expected_head: &'a ProductBranchObservation,
    successor_basis: &'a AdmittedCompositeRuntimeWorldBasis,
    attempt_identity: &'a CompositePublicationAttemptIdentity,
    owner_results: &'a CompositeOwnerExecutionResults,
}

pub(super) fn install_forked_branch<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    installation: ForkedBranchInstallation,
) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let finalization = ForkedBranchFinalization::from_installation(installation);
    match finalization.bind_publication() {
        Ok(bound) => bound.install_commit_and_history().finish(owner),
        Err(pending) => Ok(pending.into_product_unpublished()),
    }
}

fn destination_commit(input: DestinationCommitInput<'_>) -> Arc<CompositeRuntimeWorldCommit> {
    let DestinationCommitInput {
        identity,
        expected_head,
        successor_basis,
        attempt_identity,
        owner_results,
    } = input;
    Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            identity,
            expected_head.snapshot().commit(),
            successor_basis.clone(),
            attempt_identity.clone(),
            owner_results,
            None,
        )
        .expect("owner-issued branch results form the destination commit occurrence"),
    )
}

fn install_commit_and_history(
    history: &CompositeHistoryCatalog,
    reserved_commit_capacity: crate::history::ReservedCompositeCommitCapacity,
    commit: &Arc<CompositeRuntimeWorldCommit>,
) -> crate::history::ProductHeadHistoryProtectionObligation {
    let entry = reserved_commit_capacity
        .install(Arc::clone(commit))
        .expect("the reserved destination commit installs into its exact history slot");
    let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
    let product_history = history
        .protect_product_head(entry.commit())
        .expect("the installed destination admits product-head history protection");
    installed_rollback.commit();
    product_history
}

fn issue_observation_authority<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    history: &CompositeHistoryCatalog,
    commit: &CompositeRuntimeWorldCommit,
) -> Result<
    (
        crate::retention::ObservationRetentionObligation,
        crate::history::ExplicitCommitHistoryProtectionObligation,
    ),
    (),
>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let components = owner
        .state
        .retention
        .issue_observation(commit)
        .map_err(|_| ())?;
    let history = match history.protect_explicit_commit(commit) {
        Ok(history) => history,
        Err(_) => return Err(()),
    };
    Ok((components, history))
}
