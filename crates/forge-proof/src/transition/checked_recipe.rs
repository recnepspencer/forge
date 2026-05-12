use std::convert::Infallible;
use std::marker::PhantomData;

use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis, StaleReadableBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{Admitted, Recipe, Resolved, Unresolved};

use super::composition::compose_success_transition;
use super::contract::{ContextualTransition, Transition};
use super::outcomes::{DeferredTransitionOutcome, TransitionOutcome};
use super::recipe::{
    AdmitRecipeTransition, LowerRecipeTransition, RecipeResolutionContext, ResolveRecipeTransition,
};
use super::rejection::{PreConstructionGate, TransitionReadiness};

pub struct CheckedResolveRecipeTransition;

pub type RecipeResolutionGate<B, Auth, D, De> =
    PreConstructionGate<RecipeResolutionContext<B, Auth>, D, De>;

pub type RecipeLoweringReadiness<T, B, Cap, D, De, F> = TransitionReadiness<
    CapabilityWitness<Cap>,
    D,
    De,
    Infallible,
    Recipe<Resolved, T, RebindRequiredBasis<B>>,
    F,
>;

pub type RecipeAdmissionReadiness<T, B, Auth, D, De, F> = TransitionReadiness<
    AuthorityWitness<Auth>,
    D,
    De,
    Recipe<crate::recipe::Lowered, T, StaleReadableBasis<B>>,
    Infallible,
    F,
>;

pub struct CheckedLowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    capability: PhantomData<fn() -> C>,
}

impl<C> CheckedLowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    pub fn new() -> Self {
        Self {
            capability: PhantomData,
        }
    }
}

pub struct CheckedAdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    authority: PhantomData<fn() -> Auth>,
}

impl<Auth> CheckedAdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    pub fn new() -> Self {
        Self {
            authority: PhantomData,
        }
    }
}

impl<T, B, Auth, D, De>
    ContextualTransition<Recipe<Unresolved, T>, RecipeResolutionGate<B, Auth, D, De>>
    for CheckedResolveRecipeTransition
where
    Auth: AuthorityMarker,
{
    type Output = DeferredTransitionOutcome<
        Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
    >;

    fn transition(
        &self,
        input: Recipe<Unresolved, T>,
        context: RecipeResolutionGate<B, Auth, D, De>,
    ) -> Self::Output {
        match context {
            PreConstructionGate::Ready(context) => {
                ResolveRecipeTransition.transition(input, context).into()
            }
            PreConstructionGate::Denied(reason) => TransitionOutcome::denied(reason),
            PreConstructionGate::Deferred(reason) => TransitionOutcome::deferred(reason),
        }
    }
}

impl<T, B, C, D, De, F>
    ContextualTransition<
        Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        RecipeLoweringReadiness<T, B, C, D, De, F>,
    > for CheckedLowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    type Output = TransitionOutcome<
        Recipe<
            crate::recipe::Lowered,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        D,
        De,
        Infallible,
        Recipe<Resolved, T, RebindRequiredBasis<B>>,
        F,
    >;

    fn transition(
        &self,
        input: Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        context: RecipeLoweringReadiness<T, B, C, D, De, F>,
    ) -> Self::Output {
        match context {
            TransitionReadiness::Ready(capability) => LowerRecipeTransition::new(capability)
                .transition(input)
                .into(),
            TransitionReadiness::Denied(reason) => TransitionOutcome::denied(reason),
            TransitionReadiness::Deferred(reason) => TransitionOutcome::deferred(reason),
            TransitionReadiness::Stale(impossible) => match impossible {},
            TransitionReadiness::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe)
            }
            TransitionReadiness::Failed(reason) => TransitionOutcome::failed(reason),
        }
    }
}

impl<T, B, Auth, D, De, F>
    ContextualTransition<
        Recipe<
            crate::recipe::Lowered,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        RecipeAdmissionReadiness<T, B, Auth, D, De, F>,
    > for CheckedAdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    type Output = TransitionOutcome<
        Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        D,
        De,
        Recipe<crate::recipe::Lowered, T, StaleReadableBasis<B>>,
        Infallible,
        F,
    >;

    fn transition(
        &self,
        input: Recipe<
            crate::recipe::Lowered,
            T,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>,
        >,
        context: RecipeAdmissionReadiness<T, B, Auth, D, De, F>,
    ) -> Self::Output {
        match context {
            TransitionReadiness::Ready(authority) => AdmitRecipeTransition::new(authority)
                .transition(input)
                .into(),
            TransitionReadiness::Denied(reason) => TransitionOutcome::denied(reason),
            TransitionReadiness::Deferred(reason) => TransitionOutcome::deferred(reason),
            TransitionReadiness::Stale(recipe) => TransitionOutcome::stale(recipe),
            TransitionReadiness::RebindRequired(impossible) => match impossible {},
            TransitionReadiness::Failed(reason) => TransitionOutcome::failed(reason),
        }
    }
}

pub fn resolve_lower_and_admit_recipe<T, B, ResolutionAuth, LoweringCap, AdmissionAuth, D, De>(
    unresolved: Recipe<Unresolved, T>,
    resolution_gate: RecipeResolutionGate<B, ResolutionAuth, D, De>,
    lower_transition: &LowerRecipeTransition<LoweringCap>,
    admit_transition: &AdmitRecipeTransition<AdmissionAuth>,
) -> DeferredTransitionOutcome<
    Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    D,
    De,
>
where
    ResolutionAuth: AuthorityMarker,
    LoweringCap: crate::proof::CapabilityMarker,
    AdmissionAuth: AuthorityMarker,
{
    let resolved = CheckedResolveRecipeTransition.transition(unresolved, resolution_gate);
    let lowered =
        compose_success_transition(resolved, |resolved| lower_transition.transition(resolved));

    compose_success_transition(lowered, |lowered| admit_transition.transition(lowered))
}

pub fn resolve_checked_lower_and_admit_recipe<
    T,
    B,
    ResolutionAuth,
    LoweringCap,
    AdmissionAuth,
    D,
    De,
    F,
>(
    unresolved: Recipe<Unresolved, T>,
    resolution_gate: RecipeResolutionGate<B, ResolutionAuth, D, De>,
    lowering_readiness: RecipeLoweringReadiness<T, B, LoweringCap, D, De, F>,
    admission_readiness: RecipeAdmissionReadiness<T, B, AdmissionAuth, D, De, F>,
) -> TransitionOutcome<
    Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    D,
    De,
    Recipe<crate::recipe::Lowered, T, StaleReadableBasis<B>>,
    Recipe<Resolved, T, RebindRequiredBasis<B>>,
    F,
>
where
    ResolutionAuth: AuthorityMarker,
    LoweringCap: CapabilityMarker,
    AdmissionAuth: AuthorityMarker,
{
    let resolved = CheckedResolveRecipeTransition.transition(unresolved, resolution_gate);
    let lowered = match resolved {
        TransitionOutcome::Success(resolved) => CheckedLowerRecipeTransition::<LoweringCap>::new()
            .transition(resolved, lowering_readiness),
        TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason),
        TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason),
        TransitionOutcome::Stale(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(impossible) => match impossible {},
        TransitionOutcome::Failed(impossible) => match impossible {},
    };

    match lowered {
        TransitionOutcome::Success(lowered) => {
            match CheckedAdmitRecipeTransition::<AdmissionAuth>::new()
                .transition(lowered, admission_readiness)
            {
                TransitionOutcome::Success(admitted) => TransitionOutcome::success(admitted),
                TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason),
                TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason),
                TransitionOutcome::Stale(recipe) => TransitionOutcome::stale(recipe),
                TransitionOutcome::RebindRequired(impossible) => match impossible {},
                TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason),
            }
        }
        TransitionOutcome::Denied(reason) => TransitionOutcome::denied(reason),
        TransitionOutcome::Deferred(reason) => TransitionOutcome::deferred(reason),
        TransitionOutcome::Stale(impossible) => match impossible {},
        TransitionOutcome::RebindRequired(recipe) => TransitionOutcome::rebind_required(recipe),
        TransitionOutcome::Failed(reason) => TransitionOutcome::failed(reason),
    }
}
