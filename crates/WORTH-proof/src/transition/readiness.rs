use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis, StaleReadableBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness};
use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Lowered, Recipe, Resolved};

use super::composition::compose_success_transition;
use super::contract::{ContextualTransition, Transition};
use super::outcomes::{SuccessfulTransitionOutcome, TransitionOutcome};
use super::rejection::TransitionReadiness;

pub struct ExecutionReadinessContext<R, Auth>
where
    Auth: AuthorityMarker,
{
    runtime: R,
    authority: AuthorityWitness<Auth>,
}

impl<R, Auth> ExecutionReadinessContext<R, Auth>
where
    Auth: AuthorityMarker,
{
    pub fn new(runtime: R, authority: AuthorityWitness<Auth>) -> Self {
        Self { runtime, authority }
    }
}

pub struct AdmitExecutionReadyRecipeTransition;

pub type ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F> = TransitionReadiness<
    ExecutionReadinessContext<R, Auth>,
    D,
    De,
    Recipe<Lowered, T, StaleReadableBasis<B>>,
    Recipe<Resolved, T, RebindRequiredBasis<B>>,
    F,
>;

pub type CurrentExecutionReadyRecipe<T, B> =
    ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>;

pub type CurrentExecutedRecipe<T, B> =
    ExecutedRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>;

pub type CheckedExecutionReadyOutcome<T, B, D, De, F> = TransitionOutcome<
    CurrentExecutionReadyRecipe<T, B>,
    D,
    De,
    Recipe<Lowered, T, StaleReadableBasis<B>>,
    Recipe<Resolved, T, RebindRequiredBasis<B>>,
    F,
>;

pub type CheckedExecutedOutcome<T, B, D, De, F> = TransitionOutcome<
    CurrentExecutedRecipe<T, B>,
    D,
    De,
    Recipe<Lowered, T, StaleReadableBasis<B>>,
    Recipe<Resolved, T, RebindRequiredBasis<B>>,
    F,
>;

pub struct CheckedAdmitExecutionReadyRecipeTransition;

impl<T, B, R, Auth>
    ContextualTransition<
        Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        ExecutionReadinessContext<R, Auth>,
    > for AdmitExecutionReadyRecipeTransition
where
    Auth: AuthorityMarker,
{
    type Output = SuccessfulTransitionOutcome<CurrentExecutionReadyRecipe<T, B>>;

    fn transition(
        &self,
        input: Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        context: ExecutionReadinessContext<R, Auth>,
    ) -> Self::Output {
        let ExecutionReadinessContext { runtime, authority } = context;
        let _ = runtime;
        let _ = authority;

        SuccessfulTransitionOutcome::new(ExecutionReadyRecipe::new(input))
    }
}

impl<T, B, R, Auth, D, De, F>
    ContextualTransition<
        Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>,
    > for CheckedAdmitExecutionReadyRecipeTransition
where
    Auth: AuthorityMarker,
{
    type Output = CheckedExecutionReadyOutcome<T, B, D, De, F>;

    fn transition(
        &self,
        input: Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
        context: ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>,
    ) -> Self::Output {
        match context {
            TransitionReadiness::Ready(context) => AdmitExecutionReadyRecipeTransition
                .transition(input, context)
                .into(),
            TransitionReadiness::Denied(reason) => TransitionOutcome::denied(reason),
            TransitionReadiness::Deferred(reason) => TransitionOutcome::deferred(reason),
            TransitionReadiness::Stale(recipe) => TransitionOutcome::stale(recipe),
            TransitionReadiness::RebindRequired(recipe) => {
                TransitionOutcome::rebind_required(recipe)
            }
            TransitionReadiness::Failed(reason) => TransitionOutcome::failed(reason),
        }
    }
}

pub struct ExecuteReadyRecipeTransition;

impl<T, A> Transition<ExecutionReadyRecipe<T, A>> for ExecuteReadyRecipeTransition {
    type Output = SuccessfulTransitionOutcome<ExecutedRecipe<T, A>>;

    fn transition(&self, input: ExecutionReadyRecipe<T, A>) -> Self::Output {
        SuccessfulTransitionOutcome::new(ExecutedRecipe::new(input))
    }
}

pub fn admit_ready_and_execute_recipe<T, B, R, Auth>(
    lowered: Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    readiness: ExecutionReadinessContext<R, Auth>,
) -> SuccessfulTransitionOutcome<CurrentExecutedRecipe<T, B>>
where
    Auth: AuthorityMarker,
{
    let ready = AdmitExecutionReadyRecipeTransition
        .transition(lowered, readiness)
        .into_value();

    ExecuteReadyRecipeTransition.transition(ready)
}

pub fn checked_admit_ready_and_execute_recipe<T, B, R, Auth, D, De, F>(
    lowered: Recipe<Lowered, T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>>,
    readiness: ExecutionReadyAdmissionReadiness<T, B, R, Auth, D, De, F>,
) -> CheckedExecutedOutcome<T, B, D, De, F>
where
    Auth: AuthorityMarker,
{
    let ready = CheckedAdmitExecutionReadyRecipeTransition.transition(lowered, readiness);

    compose_success_transition(ready, |ready| {
        ExecuteReadyRecipeTransition.transition(ready)
    })
}

#[cfg(test)]
mod tests {
    use crate::assumption::{
        AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis,
        StaleReadableBasis,
    };
    use crate::proof::{mint_authority_witness, AuthorityMarker};
    use crate::recipe::{ExecutionReadyRecipe, Lowered, Recipe, Resolved, Unresolved};
    use crate::transition::recipe::{
        LowerRecipeTransition, RecipeResolutionContext, ResolveRecipeTransition,
    };
    use crate::transition::{
        ContextualTransition, Transition, TransitionOutcome, TransitionReadiness,
    };
    use crate::{
        proof::{mint_capability_witness, CapabilityMarker},
        transition::{
            admit_ready_and_execute_recipe, checked_admit_ready_and_execute_recipe,
            ExecutionReadinessContext,
        },
    };

    use super::{
        AdmitExecutionReadyRecipeTransition, CheckedAdmitExecutionReadyRecipeTransition,
        ExecuteReadyRecipeTransition,
    };

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct ReadinessAuthority;
    impl AuthorityMarker for ReadinessAuthority {}

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_runtime_progression() {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let resolved = ResolveRecipeTransition.transition(
            unresolved,
            RecipeResolutionContext::new(17_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let lowered = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
            .transition(resolved.into_value())
            .into_value();

        let ready = AdmitExecutionReadyRecipeTransition.transition(
            lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        );
        let executed = ExecuteReadyRecipeTransition.transition(ready.into_value());
        let executed = executed.into_value();

        assert_eq!(executed.payload(), &"payload");
        assert_eq!(executed.strong_basis().value(), &17_u8);
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_equivalence_lane() {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let resolved = ResolveRecipeTransition.transition(
            unresolved,
            RecipeResolutionContext::new(9_u8, mint_authority_witness::<ResolutionAuthority>()),
        );
        let lowered = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
            .transition(resolved.into_value())
            .into_value();

        let executed = admit_ready_and_execute_recipe(
            lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();

        assert_eq!(executed.payload(), &"payload");
        assert_eq!(executed.strong_basis().value(), &9_u8);
    }

    #[test]
    fn executed_recipe_requires_ready_wrapper() {
        let lowered = Recipe::<
            Lowered,
            _,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>,
        >::with_stage(
            "payload",
            FreshnessScopedBasis::new(AssumptionBasis::new(3_u8)),
        );
        let ready = ExecutionReadyRecipe::new(lowered);

        assert_eq!(
            ExecuteReadyRecipeTransition
                .transition(ready)
                .into_value()
                .strong_basis()
                .value(),
            &3_u8
        );
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_category_divergence_lanes() {
        let lowered = Recipe::<
            Lowered,
            _,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>,
        >::with_stage(
            "payload",
            FreshnessScopedBasis::new(AssumptionBasis::new(23_u8)),
        );
        let stale_lowered = Recipe::<Lowered, _, _>::with_stage(
            "payload",
            StaleReadableBasis::new(AssumptionBasis::new(23_u8)),
        );
        let rebind_resolved = Recipe::<Resolved, _, _>::with_stage(
            "payload",
            RebindRequiredBasis::new(AssumptionBasis::new(23_u8)),
        );

        let denied = CheckedAdmitExecutionReadyRecipeTransition.transition(
            Recipe::with_stage(
                "payload",
                FreshnessScopedBasis::new(AssumptionBasis::new(23_u8)),
            ),
            TransitionReadiness::<
                ExecutionReadinessContext<&'static str, ReadinessAuthority>,
                &'static str,
                &'static str,
                Recipe<Lowered, &str, StaleReadableBasis<u8>>,
                Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
                &'static str,
            >::denied("denied"),
        );
        let deferred = CheckedAdmitExecutionReadyRecipeTransition.transition(
            Recipe::with_stage(
                "payload",
                FreshnessScopedBasis::new(AssumptionBasis::new(23_u8)),
            ),
            TransitionReadiness::<
                ExecutionReadinessContext<&'static str, ReadinessAuthority>,
                &'static str,
                &'static str,
                Recipe<Lowered, &str, StaleReadableBasis<u8>>,
                Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
                &'static str,
            >::deferred("deferred"),
        );
        let stale = checked_admit_ready_and_execute_recipe(
            lowered,
            TransitionReadiness::<
                ExecutionReadinessContext<&'static str, ReadinessAuthority>,
                &'static str,
                &'static str,
                Recipe<Lowered, &str, StaleReadableBasis<u8>>,
                Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
                &'static str,
            >::stale(stale_lowered),
        );
        let rebind = checked_admit_ready_and_execute_recipe(
            Recipe::with_stage(
                "payload",
                FreshnessScopedBasis::new(AssumptionBasis::new(23_u8)),
            ),
            TransitionReadiness::<
                ExecutionReadinessContext<&'static str, ReadinessAuthority>,
                &'static str,
                &'static str,
                Recipe<Lowered, &str, StaleReadableBasis<u8>>,
                Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
                &'static str,
            >::rebind_required(rebind_resolved),
        );
        let failed = checked_admit_ready_and_execute_recipe(
            Recipe::with_stage(
                "payload",
                FreshnessScopedBasis::new(AssumptionBasis::new(23_u8)),
            ),
            TransitionReadiness::<
                ExecutionReadinessContext<&'static str, ReadinessAuthority>,
                &'static str,
                &'static str,
                Recipe<Lowered, &str, StaleReadableBasis<u8>>,
                Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
                &'static str,
            >::failed("failed"),
        );

        assert!(matches!(denied, TransitionOutcome::Denied("denied")));
        assert!(matches!(deferred, TransitionOutcome::Deferred("deferred")));
        assert!(matches!(stale, TransitionOutcome::Stale(_)));
        assert!(matches!(rebind, TransitionOutcome::RebindRequired(_)));
        assert!(matches!(failed, TransitionOutcome::Failed("failed")));
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_checked_composition_success_lane() {
        let lowered = Recipe::<
            Lowered,
            _,
            FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>,
        >::with_stage(
            "payload",
            FreshnessScopedBasis::new(AssumptionBasis::new(41_u8)),
        );

        let executed = checked_admit_ready_and_execute_recipe::<
            _,
            _,
            _,
            ReadinessAuthority,
            &'static str,
            &'static str,
            &'static str,
        >(
            lowered,
            TransitionReadiness::ready(ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            )),
        );
        let executed = match executed {
            TransitionOutcome::Success(executed) => executed,
            _ => panic!("expected success from checked readiness execution composition"),
        };

        assert_eq!(executed.payload(), &"payload");
        assert_eq!(executed.strong_basis().value(), &41_u8);
    }
}
