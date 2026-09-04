use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::branch::{
    ComponentBranchTarget, OwnerCreatedComponentCustodyRecord, ReservedCustodySlot,
    RuntimeWorldBranchAdmissionDenial,
};
use crate::identity::{ProductBranchIdentity, ProductBranchIncarnation};
use crate::publication::{
    CompositeAttemptProgress, ReservedBranchCreationAttempt, RuntimeWorldCancellationToken,
    SignalAttemptProgress,
};
use crate::recovery::{ProductUnpublishedCause, ProductUnpublishedOwnerEffects};

use super::super::super::RuntimeWorldOwnerRoot;

#[path = "execution/forks.rs"]
mod forks;
#[path = "execution/terminal.rs"]
mod terminal;

use forks::{fork_relational, fork_signal, ForkFailure};

/// Where the two owner fork calls of one creation ended.
pub(super) enum BranchCreationExecution {
    NoEffect(RuntimeWorldBranchAdmissionDenial),
    ProductUnpublished(ProductUnpublishedOwnerEffects),
    Settled {
        attempt: ReservedBranchCreationAttempt,
        progress: CompositeAttemptProgress,
        successor_basis: AdmittedCompositeRuntimeWorldBasis,
    },
}

/// The destination identity the created custody records are keyed by. It is
/// issued before the first owner fork, so a performed fork is always
/// attributable to an exact product-branch occurrence.
pub(super) struct CreationDestination {
    pub(super) branch: ProductBranchIdentity,
    pub(super) incarnation: ProductBranchIncarnation,
}

/// Run the Relational fork, then the Signal fork, in that fixed order. No
/// cross-owner lock is held across the two calls; the boundary between them is
/// rechecked exactly as a publication rechecks it.
pub(super) fn execute_creation<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    mut attempt: ReservedBranchCreationAttempt,
    destination: &CreationDestination,
    cancellation: &RuntimeWorldCancellationToken,
) -> BranchCreationExecution
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    attempt.begin_owner_execution();
    if cancellation.is_cancelled() || owner.deadline_expired(attempt.deadline()) {
        return BranchCreationExecution::NoEffect(
            RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
        );
    }
    if !owner.current_product_head_is(attempt.source()) {
        return BranchCreationExecution::NoEffect(
            RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
        );
    }

    let relational = match fork_relational(owner, &mut attempt, destination) {
        Ok(progress) => progress,
        Err(ForkFailure { denial }) => return BranchCreationExecution::NoEffect(denial),
    };
    let progress = CompositeAttemptProgress::new(relational, SignalAttemptProgress::untouched());

    if let Err(denial) = owner.creation_boundary_gate(&attempt, cancellation) {
        return terminal::retain_or_no_effect(
            owner,
            attempt,
            progress,
            terminal::CreationDenialNaming {
                cause: ProductUnpublishedCause::StaleProductHead,
                denial,
            },
        );
    }

    let signal = match fork_signal(owner, &mut attempt, destination, cancellation) {
        Ok(progress) => progress,
        Err(ForkFailure { denial }) => {
            return terminal::retain_or_no_effect(
                owner,
                attempt,
                progress,
                terminal::CreationDenialNaming {
                    cause: ProductUnpublishedCause::SiblingOwnerDenied,
                    denial,
                },
            )
        }
    };
    let (relational, _) = progress.into_parts();
    let progress = CompositeAttemptProgress::new(relational, signal);

    terminal::settle_creation(owner, attempt, progress, cancellation)
}

/// Attribute one performed owner fork to the destination product branch. The
/// slot was charged before the fork, so installation is total.
fn install_custody(
    slot: ReservedCustodySlot,
    destination: &CreationDestination,
    target: ComponentBranchTarget,
) {
    slot.install(OwnerCreatedComponentCustodyRecord::new(
        destination.branch.clone(),
        destination.incarnation,
        target,
    ));
}

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// The between-owners boundary for a creation. A denial here has already
    /// moved the Relational owner, so the caller must retain, not discard.
    fn creation_boundary_gate(
        &self,
        attempt: &ReservedBranchCreationAttempt,
        cancellation: &RuntimeWorldCancellationToken,
    ) -> Result<(), RuntimeWorldBranchAdmissionDenial> {
        if cancellation.is_cancelled() || self.deadline_expired(attempt.deadline()) {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        if !self.current_product_head_is(attempt.source()) {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        Ok(())
    }
}
