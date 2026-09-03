#[path = "finalization.rs"]
mod finalization;

use crate::branch::{ProductBranchObservation, RuntimeWorldBranchAdmissionDenial};
use crate::lifecycle::{
    RuntimeWorldBranchCreationOutcome, RuntimeWorldCancellationSource,
    RuntimeWorldOwnerExecutionService, RuntimeWorldPreparationService,
};
use crate::publication::{
    CompositeExecutionBorrow, NoEffectCause, NoEffectCompositePublication, OwnerExecutionOutcome,
    ProductBranchIntent,
};

use super::super::RuntimeWorldOwnerRoot;

pub(super) fn create_forked_branch<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    source: ProductBranchObservation,
    intent: ProductBranchIntent,
    signal: CompositeExecutionBorrow<'_, D, I, E, Ctx, T>,
) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let reservation = owner
        .state
        .branches
        .reserve_branch(owner.owner_identity(), intent.creation().name().clone())
        .map_err(super::map_registry_denial)?;
    let (branch, lifecycle) = owner
        .issue_branch_identities()
        .map_err(|_| RuntimeWorldBranchAdmissionDenial::IdentityExhausted)?;

    let cancellation_source = RuntimeWorldCancellationSource::new();
    let cancellation = cancellation_source.token();
    let plan =
        RuntimeWorldPreparationService::prepare(owner, source, intent).map_err(map_no_effect)?;
    let attempt = RuntimeWorldPreparationService::reserve(owner, plan, &cancellation, None)
        .map_err(map_no_effect)?;
    match RuntimeWorldOwnerExecutionService::execute(owner, attempt, signal, &cancellation) {
        OwnerExecutionOutcome::NoEffect(no_effect) => Err(map_no_effect(no_effect)),
        OwnerExecutionOutcome::ProductUnpublished(effects) => Ok(
            RuntimeWorldBranchCreationOutcome::ProductUnpublished(effects),
        ),
        OwnerExecutionOutcome::Settled(settlement) => {
            let successor_basis = settlement
                .successor_basis()
                .cloned()
                .expect("settled branch execution carries an admitted successor basis");
            let (attempt, progress) = settlement.into_parts();
            finalization::install_forked_branch(
                owner,
                finalization::ForkedBranchInstallation {
                    branch,
                    lifecycle,
                    reservation,
                    attempt,
                    progress,
                    successor_basis,
                },
            )
        }
    }
}

fn map_no_effect(no_effect: NoEffectCompositePublication) -> RuntimeWorldBranchAdmissionDenial {
    match no_effect.cause() {
        NoEffectCause::CapacityExhausted => RuntimeWorldBranchAdmissionDenial::CapacityExhausted,
        NoEffectCause::ReferenceGenerationExhausted => {
            RuntimeWorldBranchAdmissionDenial::IdentityExhausted
        }
        NoEffectCause::StaleExpectedProductHead
        | NoEffectCause::CancelledBeforeEffect
        | NoEffectCause::DeadlineBeforeEffect
        | NoEffectCause::OwnerDeniedBeforeEffect
        | NoEffectCause::CorrespondenceRebindRequired
        | NoEffectCause::OwnerUnavailable
        | NoEffectCause::PreEffectFailure => RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
    }
}
