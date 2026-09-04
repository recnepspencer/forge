use crate::branch::{
    ComponentBranchTarget, RelationalBranchCreationPlan, RuntimeWorldBranchAdmissionDenial,
    SignalBranchCreationPlan,
};
use crate::publication::{
    RelationalAttemptProgress, ReservedBranchCreationAttempt, RuntimeWorldCancellationToken,
    SignalAttemptProgress,
};

use super::super::super::super::RuntimeWorldOwnerRoot;
use super::{install_custody, CreationDestination};

use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, AdmittedRelationalForkSourceBasis, RelationalForkDenial,
    RelationalForkSourceDescriptor,
};

/// One owner fork was refused. A creation fork is the first effect its owner
/// performs, so the denial always names why no branch exists.
pub(super) struct ForkFailure {
    pub(super) denial: RuntimeWorldBranchAdmissionDenial,
}

/// Ask the Relational owner for the exact destination this plan named, from a
/// source Runtime World observes for itself and proves is still the admitted
/// one. Custody is recorded only after the owner reports the branch exists.
pub(super) fn fork_relational<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    attempt: &mut ReservedBranchCreationAttempt,
    destination: &CreationDestination,
) -> Result<RelationalAttemptProgress, ForkFailure>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let target = match attempt.plan().relational() {
        RelationalBranchCreationPlan::ReuseExact => {
            return Ok(RelationalAttemptProgress::untouched())
        }
        RelationalBranchCreationPlan::ForkExact { target } => target.clone(),
    };
    let source = observe_exact_fork_source(owner, attempt)?;
    let custody = attempt.take_relational_custody().ok_or(ForkFailure {
        denial: RuntimeWorldBranchAdmissionDenial::CustodyCapacityExhausted,
    })?;
    let fork_port = owner.state.relational.fork_port();
    let reservation = fork_port
        .reserve_fork_target(target.clone())
        .map_err(|denial| relational_fork_failure(&denial))?;
    let (fork, target_basis) = fork_port
        .fork_reserved_with_basis(reservation, source)
        .map_err(|denial| relational_fork_failure(&denial))?;
    install_custody(
        custody,
        destination,
        ComponentBranchTarget::Relational(target),
    );
    Ok(RelationalAttemptProgress::forked(fork, target_basis))
}

/// Observe the fork source the admitted basis names and prove it is still that
/// exact source. Runtime World issues the fork token itself: no caller-held
/// source token exists, and a source that moved between admission and this call
/// denies here, before the destination is reserved or any branch is created.
fn observe_exact_fork_source<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    attempt: &ReservedBranchCreationAttempt,
) -> Result<AdmittedRelationalForkSourceBasis, ForkFailure>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let admitted = attempt.source().basis().relational_basis();
    let (descriptor, source) = owner
        .state
        .relational
        .fork_port()
        .observe_fork_source(admitted.descriptor().branch_id())
        .map_err(|denial| relational_fork_failure(&denial))?;
    if source_is_the_admitted_basis(&descriptor, admitted) {
        Ok(source)
    } else {
        Err(ForkFailure {
            denial: RuntimeWorldBranchAdmissionDenial::ForkSourceChanged,
        })
    }
}

/// Every axis the fresh source descriptor shares with the admitted source
/// basis. A single differing axis means the branch the owner would fork is not
/// the branch this creation was admitted against.
fn source_is_the_admitted_basis(
    descriptor: &RelationalForkSourceDescriptor,
    admitted: &AdmittedRelationalBranchBasis,
) -> bool {
    let basis = admitted.descriptor();
    descriptor.runtime_instance_id() == basis.runtime_instance_id()
        && descriptor.source_branch() == basis.branch_id()
        && descriptor.observation() == basis.reference()
        && descriptor.truth_version() == basis.truth_version()
}

/// Ask the Signal owner for the exact destination this plan named. The
/// reservation and the fork both run under the Runtime World token's own
/// Signal token; no detached cancellation source exists on this path.
pub(super) fn fork_signal<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    attempt: &mut ReservedBranchCreationAttempt,
    destination: &CreationDestination,
    cancellation: &RuntimeWorldCancellationToken,
) -> Result<SignalAttemptProgress, ForkFailure>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let target = match attempt.plan().signal() {
        SignalBranchCreationPlan::ReuseExact => return Ok(SignalAttemptProgress::untouched()),
        SignalBranchCreationPlan::ForkExact { target } => target.clone(),
    };
    let custody = attempt.take_signal_custody().ok_or(ForkFailure {
        denial: RuntimeWorldBranchAdmissionDenial::CustodyCapacityExhausted,
    })?;
    let mutation_port = owner.state.signal.mutation_port();
    let reservation = mutation_port
        .reserve_fork_exact(target.clone(), attempt.source().basis().signal_basis())
        .map_err(|denial| signal_fork_failure(&denial))?;
    let fork = mutation_port
        .fork_reserved_exact(reservation, cancellation.signal_token())
        .map_err(|denial| signal_fork_failure(&denial))?;
    install_custody(custody, destination, ComponentBranchTarget::Signal(target));
    Ok(SignalAttemptProgress::forked(fork))
}

fn relational_fork_failure(denial: &RelationalForkDenial) -> ForkFailure {
    let denial = match denial {
        RelationalForkDenial::RetentionCapacityExhausted
        | RelationalForkDenial::RetentionIdentityExhausted => {
            RuntimeWorldBranchAdmissionDenial::CapacityExhausted
        }
        _ => RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
    };
    ForkFailure { denial }
}

fn signal_fork_failure(
    denial: &worth_signal::facade::branch::SignalBranchForkOperationDenial,
) -> ForkFailure {
    let denial = match super::super::super::super::execution_service::map_fork_no_effect(denial) {
        crate::publication::NoEffectCause::CapacityExhausted => {
            RuntimeWorldBranchAdmissionDenial::CapacityExhausted
        }
        _ => RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
    };
    ForkFailure { denial }
}
