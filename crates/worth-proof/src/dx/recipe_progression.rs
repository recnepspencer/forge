use crate::assumption::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis};
use crate::proof::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
use crate::recipe::{Admitted, ExecutionReadyRecipe, Lowered, Recipe, Resolved, Unresolved};
use crate::transition::{
    AdmitExecutionReadyRecipeTransition, AdmitRecipeTransition, ContextualTransition,
    ExecutionReadinessContext, LowerRecipeTransition, RecipeResolutionContext,
    ResolveRecipeTransition, Transition,
};

pub trait UnresolvedRecipeDxExt<T> {
    fn resolve_with<B, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: B,
    ) -> Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker;
}

impl<T> UnresolvedRecipeDxExt<T> for Recipe<Unresolved, T> {
    fn resolve_with<B, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        basis: B,
    ) -> Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker,
    {
        ResolveRecipeTransition
            .transition(self, RecipeResolutionContext::new(basis, authority))
            .into_value()
    }
}

pub trait ResolvedRecipeDxExt<T, B> {
    fn lower_with<C>(
        self,
        capability: CapabilityWitness<C>,
    ) -> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        C: CapabilityMarker;
}

impl<T, B> ResolvedRecipeDxExt<T, B>
    for Recipe<Resolved, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
{
    fn lower_with<C>(
        self,
        capability: CapabilityWitness<C>,
    ) -> Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        C: CapabilityMarker,
    {
        LowerRecipeTransition::new(capability)
            .transition(self)
            .into_value()
    }
}

pub trait LoweredRecipeDxExt<T, B> {
    fn admit_with<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker;

    fn ready_with<R, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        runtime: R,
    ) -> ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker;
}

impl<T, B> LoweredRecipeDxExt<T, B>
    for Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
{
    fn admit_with<Auth>(
        self,
        authority: AuthorityWitness<Auth>,
    ) -> Recipe<Admitted, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker,
    {
        AdmitRecipeTransition::new(authority)
            .transition(self)
            .into_value()
    }

    fn ready_with<R, Auth>(
        self,
        authority: AuthorityWitness<Auth>,
        runtime: R,
    ) -> ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>
    where
        Auth: AuthorityMarker,
    {
        AdmitExecutionReadyRecipeTransition
            .transition(self, ExecutionReadinessContext::new(runtime, authority))
            .into_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::proof::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    };
    use crate::recipe::{Recipe, Unresolved};
    use crate::transition::{
        AdmitExecutionReadyRecipeTransition, AdmitRecipeTransition, ContextualTransition,
        ExecutionReadinessContext, LowerRecipeTransition, RecipeResolutionContext,
        ResolveRecipeTransition, Transition,
    };

    use super::{LoweredRecipeDxExt, ResolvedRecipeDxExt, UnresolvedRecipeDxExt};

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct AdmissionAuthority;
    impl AuthorityMarker for AdmissionAuthority {}

    struct ReadinessAuthority;
    impl AuthorityMarker for ReadinessAuthority {}

    #[test]
    fn pleasant_progression_matches_raw_admitted_progression() {
        let pleasant = Recipe::<Unresolved, _>::new("payload")
            .resolve_with(mint_authority_witness::<ResolutionAuthority>(), 7_u8)
            .lower_with(mint_capability_witness::<LoweringCapability>())
            .admit_with(mint_authority_witness::<AdmissionAuthority>());

        let raw_resolved = ResolveRecipeTransition.transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(7_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let raw_lowered =
            LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
                .transition(raw_resolved.into_value());
        let raw_admitted =
            AdmitRecipeTransition::new(mint_authority_witness::<AdmissionAuthority>())
                .transition(raw_lowered.into_value())
                .into_value();

        assert_eq!(pleasant.payload(), raw_admitted.payload());
        assert_eq!(
            pleasant.strong_basis().value(),
            raw_admitted.strong_basis().value()
        );
    }

    #[test]
    fn pleasant_progression_matches_raw_ready_progression() {
        let pleasant = Recipe::<Unresolved, _>::new("payload")
            .resolve_with(mint_authority_witness::<ResolutionAuthority>(), 11_u8)
            .lower_with(mint_capability_witness::<LoweringCapability>())
            .ready_with(
                mint_authority_witness::<ReadinessAuthority>(),
                "runtime admission",
            );

        let raw_resolved = ResolveRecipeTransition.transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(11_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let raw_lowered =
            LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
                .transition(raw_resolved.into_value())
                .into_value();
        let raw_ready = AdmitExecutionReadyRecipeTransition.transition(
            raw_lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        );
        let raw_ready = raw_ready.into_value();

        assert_eq!(pleasant.payload(), raw_ready.payload());
        assert_eq!(
            pleasant.strong_basis().value(),
            raw_ready.strong_basis().value()
        );
    }
}
