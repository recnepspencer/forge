use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{Admitted, Lowered, Recipe, Resolved, Unresolved};

use super::contract::{ContextualTransition, Transition};
use super::outcomes::SuccessfulTransitionOutcome;

pub struct ResolveRecipeTransition;

pub struct RecipeResolutionContext<B, Auth>
where
    Auth: AuthorityMarker,
{
    basis: B,
    authority: AuthorityWitness<Auth>,
}

impl<B, Auth> RecipeResolutionContext<B, Auth>
where
    Auth: AuthorityMarker,
{
    pub fn new(basis: B, authority: AuthorityWitness<Auth>) -> Self {
        Self { basis, authority }
    }
}

impl<T, B, Auth> ContextualTransition<Recipe<Unresolved, T>, RecipeResolutionContext<B, Auth>>
    for ResolveRecipeTransition
where
    Auth: AuthorityMarker,
{
    type Output = SuccessfulTransitionOutcome<
        Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    >;

    fn transition(
        &self,
        input: Recipe<Unresolved, T>,
        context: RecipeResolutionContext<B, Auth>,
    ) -> Self::Output {
        let RecipeResolutionContext { basis, authority } = context;
        let _ = authority;

        SuccessfulTransitionOutcome::new(Recipe::with_stage(
            input.payload,
            FreshnessScopedBasis::new(AssumptionBasis::new(basis)),
        ))
    }
}

pub struct LowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    capability: CapabilityWitness<C>,
}

impl<C> LowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    pub fn new(capability: CapabilityWitness<C>) -> Self {
        Self { capability }
    }
}

impl<T, B, C>
    Transition<Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>>
    for LowerRecipeTransition<C>
where
    C: CapabilityMarker,
{
    type Output = SuccessfulTransitionOutcome<
        Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    >;

    fn transition(
        &self,
        input: Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    ) -> Self::Output {
        let _ = self.capability;

        SuccessfulTransitionOutcome::new(Recipe::with_stage(input.payload, input.basis))
    }
}

pub struct AdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    authority: AuthorityWitness<Auth>,
}

impl<Auth> AdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    pub fn new(authority: AuthorityWitness<Auth>) -> Self {
        Self { authority }
    }
}

impl<T, B, Auth>
    Transition<Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>>
    for AdmitRecipeTransition<Auth>
where
    Auth: AuthorityMarker,
{
    type Output = SuccessfulTransitionOutcome<
        Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    >;

    fn transition(
        &self,
        input: Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    ) -> Self::Output {
        let _ = self.authority;

        SuccessfulTransitionOutcome::new(Recipe::with_stage(input.payload, input.basis))
    }
}

#[cfg(test)]
mod tests {
    use crate::proof::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    };

    use super::{
        AdmitRecipeTransition, ContextualTransition, LowerRecipeTransition,
        RecipeResolutionContext, ResolveRecipeTransition, Transition,
    };
    use crate::recipe::{Recipe, Unresolved};

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct AdmissionAuthority;
    impl AuthorityMarker for AdmissionAuthority {}

    #[test]
    fn recipe_transitions_progress_through_success_outcomes() {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let resolved = ResolveRecipeTransition.transition(
            unresolved,
            RecipeResolutionContext::new(7_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let resolved = resolved.into_value();

        let lowered = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
            .transition(resolved);
        let lowered = lowered.into_value();

        let admitted = AdmitRecipeTransition::new(mint_authority_witness::<AdmissionAuthority>())
            .transition(lowered);
        let admitted = admitted.into_value();

        assert_eq!(admitted.payload(), &"payload");
        assert_eq!(admitted.strong_basis().value(), &7_u8);
    }
}
