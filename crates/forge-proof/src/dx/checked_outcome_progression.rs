use core::convert::Infallible;

use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis, StaleReadableBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Lowered, Recipe, Resolved};
use crate::transition::{
    ExecutionReadyAdmissionReadiness, RecipeAdmissionReadiness, RecipeLoweringReadiness,
    Transition, TransitionOutcome,
};

use super::checked_outcome::ProofOutcome;
use super::checked_recipe_progression::{
    CheckedLoweredRecipeDxExt, CheckedResolvedRecipeDxExt, LoweredRecipe, ResolvedRecipe,
};

pub trait CheckedProofOutcomeLowerExt<T, B, D, De> {
    fn try_lower<C, F>(
        self,
        readiness: RecipeLoweringReadiness<T, B, C, D, De, F>,
    ) -> ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        C: CapabilityMarker;

    fn try_lower_ready<C>(
        self,
        capability: CapabilityWitness<C>,
    ) -> ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        C: CapabilityMarker;
}

impl<T, B, D, De> CheckedProofOutcomeLowerExt<T, B, D, De>
    for ProofOutcome<ResolvedRecipe<T, B>, D, De>
{
    fn try_lower<C, F>(
        self,
        readiness: RecipeLoweringReadiness<T, B, C, D, De, F>,
    ) -> ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        C: CapabilityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(resolved) => resolved.try_lower(readiness),
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(impossible) => match impossible {},
            TransitionOutcome::Failed(impossible) => match impossible {},
        }
    }

    fn try_lower_ready<C>(
        self,
        capability: CapabilityWitness<C>,
    ) -> ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        C: CapabilityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(resolved) => match resolved
                .try_lower_ready(capability)
                .into_raw()
            {
                TransitionOutcome::Success(lowered) => TransitionOutcome::success(lowered).into(),
                TransitionOutcome::Denied(impossible) => match impossible {},
                TransitionOutcome::Deferred(impossible) => match impossible {},
                TransitionOutcome::Stale(impossible) => match impossible {},
                TransitionOutcome::RebindRequired(recipe) => {
                    TransitionOutcome::rebind_required(recipe).into()
                }
                TransitionOutcome::Failed(impossible) => match impossible {},
            },
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(impossible) => match impossible {},
            TransitionOutcome::Failed(impossible) => match impossible {},
        }
    }
}

pub trait CheckedProofOutcomeToAdmitExt<T, B, D, De, R, F> {
    fn try_admit<Auth>(
        self,
        readiness: RecipeAdmissionReadiness<T, B, Auth, D, De, F>,
    ) -> ProofOutcome<
        Recipe<
            crate::recipe::Admitted,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        R,
        F,
    >
    where
        Auth: AuthorityMarker;

    fn try_admit_ready<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        Recipe<
            crate::recipe::Admitted,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        R,
        F,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B, D, De, F>
    CheckedProofOutcomeToAdmitExt<T, B, D, De, Recipe<Resolved, T, RebindRequiredBasis<B>>, F>
    for ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
{
    fn try_admit<Auth>(
        self,
        readiness: RecipeAdmissionReadiness<T, B, Auth, D, De, F>,
    ) -> ProofOutcome<
        Recipe<
            crate::recipe::Admitted,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        Auth: AuthorityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(lowered) => match lowered.try_admit(readiness).into_raw() {
                TransitionOutcome::Success(admitted) => TransitionOutcome::success(admitted).into(),
                TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
                TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
                TransitionOutcome::Stale(recipe) => TransitionOutcome::stale(recipe).into(),
                TransitionOutcome::RebindRequired(impossible) => match impossible {},
                TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason).into(),
            },
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe).into()
            }
            TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason).into(),
        }
    }

    fn try_admit_ready<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        Recipe<
            crate::recipe::Admitted,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        Auth: AuthorityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(lowered) => match lowered
                .try_admit_ready(authority)
                .into_raw()
            {
                TransitionOutcome::Success(admitted) => TransitionOutcome::success(admitted).into(),
                TransitionOutcome::Denied(impossible) => match impossible {},
                TransitionOutcome::Deferred(impossible) => match impossible {},
                TransitionOutcome::Stale(recipe) => TransitionOutcome::stale(recipe).into(),
                TransitionOutcome::RebindRequired(impossible) => match impossible {},
                TransitionOutcome::Failed(impossible) => match impossible {},
            },
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe).into()
            }
            TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason).into(),
        }
    }
}

pub trait CheckedProofOutcomeReadyExt<T, B, D, De, R, F> {
    fn try_ready<Rt, Auth>(
        self,
        readiness: ExecutionReadyAdmissionReadiness<T, B, Rt, Auth, D, De, F>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        R,
        F,
    >
    where
        Auth: AuthorityMarker;

    fn try_ready_now<Rt, Auth>(
        self,
        runtime: Rt,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        R,
        F,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B, D, De, F>
    CheckedProofOutcomeReadyExt<T, B, D, De, Recipe<Resolved, T, RebindRequiredBasis<B>>, F>
    for ProofOutcome<
        LoweredRecipe<T, B>,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
{
    fn try_ready<Rt, Auth>(
        self,
        readiness: ExecutionReadyAdmissionReadiness<T, B, Rt, Auth, D, De, F>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        Auth: AuthorityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(lowered) => lowered.try_ready(readiness),
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe).into()
            }
            TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason).into(),
        }
    }

    fn try_ready_now<Rt, Auth>(
        self,
        runtime: Rt,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >
    where
        Auth: AuthorityMarker,
    {
        match self.into_raw() {
            TransitionOutcome::Success(lowered) => {
                match lowered.try_ready_now(runtime, authority).into_raw() {
                    TransitionOutcome::Success(ready) => TransitionOutcome::success(ready).into(),
                    TransitionOutcome::Denied(impossible) => match impossible {},
                    TransitionOutcome::Deferred(impossible) => match impossible {},
                    TransitionOutcome::Stale(recipe) => TransitionOutcome::stale(recipe).into(),
                    TransitionOutcome::RebindRequired(recipe) => {
                        TransitionOutcome::rebind_required(recipe).into()
                    }
                    TransitionOutcome::Failed(impossible) => match impossible {},
                }
            }
            TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason).into(),
            TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason).into(),
            TransitionOutcome::Stale(impossible) => match impossible {},
            TransitionOutcome::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe).into()
            }
            TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason).into(),
        }
    }
}

pub trait CheckedProofOutcomeExecuteExt<T, A, D, De, St, R, F> {
    fn try_execute(self) -> ProofOutcome<ExecutedRecipe<T, A>, D, De, St, R, F>;
}

impl<T, A, D, De, St, R, F> CheckedProofOutcomeExecuteExt<T, A, D, De, St, R, F>
    for ProofOutcome<ExecutionReadyRecipe<T, A>, D, De, St, R, F>
{
    fn try_execute(self) -> ProofOutcome<ExecutedRecipe<T, A>, D, De, St, R, F> {
        self.map_success(|ready| {
            crate::transition::ExecuteReadyRecipeTransition
                .transition(ready)
                .into_value()
        })
    }
}
