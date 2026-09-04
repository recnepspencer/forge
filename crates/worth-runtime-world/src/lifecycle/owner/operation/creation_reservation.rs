use crate::branch::{
    CustodyComponent, LoweredBranchCreationPlan, ProductBranchCreationIntent,
    ProductBranchObservation, ReservedCustodySlot, RuntimeWorldBranchAdmissionDenial,
};
use crate::lifecycle::RuntimeWorldInstant;
use crate::publication::{
    NoEffectCause, ReservedAttemptCapacities, ReservedAttemptCapacityInputs,
    ReservedBranchCreationAttempt, ReservedBranchCreationInputs, RuntimeWorldCancellationToken,
};

use super::reservation_steps::{
    issue_publication_identities, reserve_publication_resources, IssuedPublicationIdentities,
    ReservedPublicationResources,
};
use super::RuntimeWorldOwnerRoot;

impl<D, I, E, Ctx, T> RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// Lower and reserve one branch creation. Every bounded resource the
    /// creation can consume, including one custody slot per owner fork it will
    /// ask for, is charged here before the first owner effect.
    pub(super) fn prepare_creation(
        &self,
        source: ProductBranchObservation,
        intent: ProductBranchCreationIntent,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedBranchCreationAttempt, RuntimeWorldBranchAdmissionDenial> {
        let plan = self.admit_creation_source(&source, intent, cancellation, deadline)?;
        let attempt = self.reserve_creation_attempt(source, plan, deadline)?;
        // The last boundary before the caller may ask an owner to fork.
        // Dropping the attempt on a denial releases every capacity it holds.
        creation_pre_effect_denial(self, cancellation, deadline)?;
        Ok(attempt)
    }

    /// The pre-reservation admission for a creation: this owner, an open and
    /// bootstrapped world, the exact current product head, and a plan whose
    /// two per-owner legs match the source it was lowered against.
    fn admit_creation_source(
        &self,
        source: &ProductBranchObservation,
        intent: ProductBranchCreationIntent,
        cancellation: &RuntimeWorldCancellationToken,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<LoweredBranchCreationPlan, RuntimeWorldBranchAdmissionDenial> {
        if source.owner_identity() != self.owner_identity() {
            return Err(RuntimeWorldBranchAdmissionDenial::ForeignOwner);
        }
        if !self.is_open_and_bootstrapped_for_creation() {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        creation_pre_effect_denial(self, cancellation, deadline)?;
        if !self.current_product_head_is(source) {
            return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
        }
        let plan = LoweredBranchCreationPlan::lower(source.clone(), intent)?;
        if plan.is_compatible_with(source) {
            Ok(plan)
        } else {
            Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)
        }
    }

    /// Charge every bounded resource the lowered plan can consume. Nothing
    /// here contacts a component owner.
    fn reserve_creation_attempt(
        &self,
        source: ProductBranchObservation,
        plan: LoweredBranchCreationPlan,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<ReservedBranchCreationAttempt, RuntimeWorldBranchAdmissionDenial> {
        let operation = self
            .reserve_creation_operation()
            .map_err(|()| RuntimeWorldBranchAdmissionDenial::OwnerUnavailable)?;
        let IssuedPublicationIdentities {
            attempt_identity,
            commit_identity,
            product_unpublished_identity,
        } = issue_publication_identities(self).map_err(map_reservation_cause)?;
        let ReservedPublicationResources {
            history,
            reserved_commit_capacity,
            reserved_recovery_slot,
            reserved_component_pin_pair,
            reserved_publication_capacity,
        } = reserve_publication_resources(self, &source, &commit_identity)
            .map_err(map_reservation_cause)?;
        let relational_custody = reserve_custody(
            self,
            CustodyComponent::Relational,
            plan.relational().requires_owner_effect(),
        )?;
        let signal_custody = reserve_custody(
            self,
            CustodyComponent::Signal,
            plan.signal().requires_owner_effect(),
        )?;
        Ok(ReservedBranchCreationAttempt::new(
            ReservedBranchCreationInputs {
                identity: attempt_identity,
                source,
                plan,
                capacities: ReservedAttemptCapacities::new(ReservedAttemptCapacityInputs {
                    reserved_commit_identity: commit_identity,
                    product_unpublished_identity,
                    reserved_commit_capacity,
                    reserved_recovery_slot,
                    reserved_component_pin_pair,
                    reserved_publication_capacity,
                    history,
                    operation,
                }),
                relational_custody,
                signal_custody,
                deadline,
            },
        ))
    }
}

fn reserve_custody<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    component: CustodyComponent,
    required: bool,
) -> Result<Option<ReservedCustodySlot>, RuntimeWorldBranchAdmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    if !required {
        return Ok(None);
    }
    owner.state.custody.reserve(component).map(Some)
}

fn creation_pre_effect_denial<D, I, E, Ctx, T>(
    owner: &RuntimeWorldOwnerRoot<D, I, E, Ctx, T>,
    cancellation: &RuntimeWorldCancellationToken,
    deadline: Option<RuntimeWorldInstant>,
) -> Result<(), RuntimeWorldBranchAdmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    if cancellation.is_cancelled() {
        return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
    }
    if owner.deadline_expired(deadline) {
        return Err(RuntimeWorldBranchAdmissionDenial::OwnerUnavailable);
    }
    Ok(())
}

fn map_reservation_cause(cause: NoEffectCause) -> RuntimeWorldBranchAdmissionDenial {
    match cause {
        NoEffectCause::CapacityExhausted => RuntimeWorldBranchAdmissionDenial::CapacityExhausted,
        NoEffectCause::ReferenceGenerationExhausted => {
            RuntimeWorldBranchAdmissionDenial::IdentityExhausted
        }
        _ => RuntimeWorldBranchAdmissionDenial::OwnerUnavailable,
    }
}
