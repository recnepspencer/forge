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

use worth_relational::facade::branch::RelationalForkDenial;

/// One owner fork was refused. A creation fork is the first effect its owner
/// performs, so the denial always names why no branch exists.
pub(super) struct ForkFailure {
    pub(super) denial: RuntimeWorldBranchAdmissionDenial,
}

/// Ask the Relational owner for the exact destination this plan named. Custody
/// is recorded only after the owner reports the branch exists.
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
    let custody = attempt.take_relational_custody().ok_or(ForkFailure {
        denial: RuntimeWorldBranchAdmissionDenial::CustodyCapacityExhausted,
    })?;
    let source_branch = attempt
        .source()
        .basis()
        .relational_basis()
        .identity()
        .branch_id()
        .clone();
    let fork_port = owner.state.relational.fork_port();
    let (_descriptor, source) = fork_port
        .observe_fork_source(&source_branch)
        .map_err(|denial| relational_fork_failure(&denial))?;
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
