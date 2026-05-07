use core::convert::Infallible;

use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis, StaleReadableBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Lowered, Recipe, Resolved, Unresolved};
use crate::transition::{
    CheckedAdmitExecutionReadyRecipeTransition, CheckedAdmitRecipeTransition,
    CheckedLowerRecipeTransition, CheckedResolveRecipeTransition, ContextualTransition,
    ExecuteReadyRecipeTransition, ExecutionReadinessContext, ExecutionReadyAdmissionReadiness,
    RecipeAdmissionReadiness, RecipeLoweringReadiness, RecipeResolutionContext,
    RecipeResolutionGate, Transition, TransitionReadiness,
};

use super::{checked_inputs::gate_ready, checked_outcome::ProofOutcome};

pub(crate) type ResolvedRecipe<T, B> =
    Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>;
pub(crate) type LoweredRecipe<T, B> =
    Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>;

pub trait CheckedUnresolvedRecipeDxExt<T> {
    fn try_resolve<B, Auth, D, De>(
        self,
        gate: RecipeResolutionGate<B, Auth, D, De>,
    ) -> ProofOutcome<ResolvedRecipe<T, B>, D, De>
    where
        Auth: AuthorityMarker;

    fn try_resolve_ready<B, Auth>(
        self,
        basis: B,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<ResolvedRecipe<T, B>>
    where
        Auth: AuthorityMarker;
}

impl<T> CheckedUnresolvedRecipeDxExt<T> for Recipe<Unresolved, T> {
    fn try_resolve<B, Auth, D, De>(
        self,
        gate: RecipeResolutionGate<B, Auth, D, De>,
    ) -> ProofOutcome<ResolvedRecipe<T, B>, D, De>
    where
        Auth: AuthorityMarker,
    {
        CheckedResolveRecipeTransition.transition(self, gate).into()
    }

    fn try_resolve_ready<B, Auth>(
        self,
        basis: B,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<ResolvedRecipe<T, B>>
    where
        Auth: AuthorityMarker,
    {
        self.try_resolve(gate_ready(RecipeResolutionContext::new(basis, authority)))
    }
}

pub trait CheckedResolvedRecipeDxExt<T, B> {
    fn try_lower<C, D, De, F>(
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
        Infallible,
        Infallible,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        C: CapabilityMarker;
}

impl<T, B> CheckedResolvedRecipeDxExt<T, B> for ResolvedRecipe<T, B> {
    fn try_lower<C, D, De, F>(
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
        CheckedLowerRecipeTransition::<C>::new()
            .transition(self, readiness)
            .into()
    }

    fn try_lower_ready<C>(
        self,
        capability: CapabilityWitness<C>,
    ) -> ProofOutcome<
        LoweredRecipe<T, B>,
        Infallible,
        Infallible,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        C: CapabilityMarker,
    {
        self.try_lower::<C, Infallible, Infallible, Infallible>(TransitionReadiness::ready(
            capability,
        ))
    }
}

pub trait CheckedLoweredRecipeDxExt<T, B> {
    fn try_admit<Auth, D, De, F>(
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
        Infallible,
        F,
    >
    where
        Auth: AuthorityMarker;

    fn try_ready<R, Auth, D, De, F>(
        self,
        readiness: ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
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
        Infallible,
        Infallible,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Infallible,
        Infallible,
    >
    where
        Auth: AuthorityMarker;

    fn try_ready_now<R, Auth>(
        self,
        runtime: R,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        Infallible,
        Infallible,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        Auth: AuthorityMarker;
}

impl<T, B> CheckedLoweredRecipeDxExt<T, B> for LoweredRecipe<T, B> {
    fn try_admit<Auth, D, De, F>(
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
        Infallible,
        F,
    >
    where
        Auth: AuthorityMarker,
    {
        CheckedAdmitRecipeTransition::<Auth>::new()
            .transition(self, readiness)
            .into()
    }

    fn try_ready<R, Auth, D, De, F>(
        self,
        readiness: ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>,
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
        CheckedAdmitExecutionReadyRecipeTransition
            .transition(self, readiness)
            .into()
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
        Infallible,
        Infallible,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Infallible,
        Infallible,
    >
    where
        Auth: AuthorityMarker,
    {
        self.try_admit::<Auth, Infallible, Infallible, Infallible>(TransitionReadiness::ready(
            authority,
        ))
    }

    fn try_ready_now<R, Auth>(
        self,
        runtime: R,
        authority: AuthorityWitness<Auth>,
    ) -> ProofOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        Infallible,
        Infallible,
        Recipe<Lowered, T, StaleReadableBasis<B>>,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        Infallible,
    >
    where
        Auth: AuthorityMarker,
    {
        self.try_ready::<R, Auth, Infallible, Infallible, Infallible>(TransitionReadiness::ready(
            ExecutionReadinessContext::new(runtime, authority),
        ))
    }
}

pub trait CheckedExecutionReadyRecipeDxExt<T, A> {
    fn try_execute(self) -> ProofOutcome<ExecutedRecipe<T, A>>;
}

impl<T, A> CheckedExecutionReadyRecipeDxExt<T, A> for ExecutionReadyRecipe<T, A> {
    fn try_execute(self) -> ProofOutcome<ExecutedRecipe<T, A>> {
        ExecuteReadyRecipeTransition.transition(self).into()
    }
}
