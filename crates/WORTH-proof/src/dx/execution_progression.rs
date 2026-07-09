use crate::recipe::ExecutedRecipe;
use crate::recipe::ExecutionReadyRecipe;
use crate::transition::{ExecuteReadyRecipeTransition, Transition};

pub trait ExecutionReadyRecipeDxExt<T, A> {
    fn execute(self) -> ExecutedRecipe<T, A>;
}

impl<T, A> ExecutionReadyRecipeDxExt<T, A> for ExecutionReadyRecipe<T, A> {
    fn execute(self) -> ExecutedRecipe<T, A> {
        ExecuteReadyRecipeTransition.transition(self).into_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::proof::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    };
    use crate::recipe::{Recipe, Unresolved};
    use crate::transition::{
        ContextualTransition, ExecutionReadinessContext, LowerRecipeTransition,
        RecipeResolutionContext, ResolveRecipeTransition, Transition,
    };

    use super::ExecutionReadyRecipeDxExt;
    use crate::dx::{LoweredRecipeDxExt, ResolvedRecipeDxExt, UnresolvedRecipeDxExt};

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct ReadinessAuthority;
    impl AuthorityMarker for ReadinessAuthority {}

    #[test]
    fn pleasant_execution_matches_raw_ready_and_executed_progression() {
        let pleasant = Recipe::<Unresolved, _>::new("payload")
            .resolve_with(mint_authority_witness::<ResolutionAuthority>(), 11_u8)
            .lower_with(mint_capability_witness::<LoweringCapability>())
            .ready_with(
                mint_authority_witness::<ReadinessAuthority>(),
                "runtime admission",
            )
            .execute();

        let raw_resolved = ResolveRecipeTransition.transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(11_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let raw_lowered =
            LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
                .transition(raw_resolved.into_value())
                .into_value();
        let raw_ready = crate::transition::AdmitExecutionReadyRecipeTransition.transition(
            raw_lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        );
        let raw_executed = crate::transition::ExecuteReadyRecipeTransition
            .transition(raw_ready.into_value())
            .into_value();

        assert_eq!(pleasant.payload(), raw_executed.payload());
        assert_eq!(
            pleasant.strong_basis().value(),
            raw_executed.strong_basis().value()
        );
    }
}
