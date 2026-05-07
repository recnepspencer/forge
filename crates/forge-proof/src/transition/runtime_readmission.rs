use std::convert::Infallible;

use crate::assumption::{
    AssumptionBasis, BoundaryBridgedStaleReadableBasis, CurrentValidity, FreshnessScopedBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness};
use crate::recipe::{ExecutedRecipe, ExecutionReadyRecipe, Lowered, Recipe};

use super::composition::compose_success_transition;
use super::contract::{ContextualTransition, Transition};
use super::outcomes::{SuccessfulTransitionOutcome, TransitionOutcome};
use super::readiness::{
    AdmitExecutionReadyRecipeTransition, ExecuteReadyRecipeTransition, ExecutionReadinessContext,
};
use super::rejection::TransitionReadiness;

pub struct LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    basis: NextB,
    readmission_authority: AuthorityWitness<ReadmitAuth>,
    readiness: ExecutionReadinessContext<Runtime, ReadinessAuth>,
}

impl<NextB, ReadmitAuth, Runtime, ReadinessAuth>
    LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    pub fn new(
        basis: NextB,
        readmission_authority: AuthorityWitness<ReadmitAuth>,
        runtime: Runtime,
        readiness_authority: AuthorityWitness<ReadinessAuth>,
    ) -> Self {
        Self {
            basis,
            readmission_authority,
            readiness: ExecutionReadinessContext::new(runtime, readiness_authority),
        }
    }
}

pub type LoweredReadmissionReadiness<
    T,
    PrevB,
    NextB,
    ReadmitAuth,
    Runtime,
    ReadinessAuth,
    D,
    De,
    F,
> = TransitionReadiness<
    LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>,
    D,
    De,
    Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
    Infallible,
    F,
>;

pub struct ReadmitLoweredForExecutionReadyTransition;

pub struct CheckedReadmitLoweredForExecutionReadyTransition;

impl<T, PrevB, NextB, ReadmitAuth, Runtime, ReadinessAuth>
    ContextualTransition<
        Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
        LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>,
    > for ReadmitLoweredForExecutionReadyTransition
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    type Output = SuccessfulTransitionOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<NextB>>>,
    >;

    fn transition(
        &self,
        input: Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
        context: LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>,
    ) -> Self::Output {
        let LoweredReadmissionContext {
            basis,
            readmission_authority,
            readiness,
        } = context;
        let readmitted = input.readmit_with_authority(basis, readmission_authority);

        AdmitExecutionReadyRecipeTransition.transition(readmitted, readiness)
    }
}

impl<T, PrevB, NextB, ReadmitAuth, Runtime, ReadinessAuth, D, De, F>
    ContextualTransition<
        Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
        LoweredReadmissionReadiness<T, PrevB, NextB, ReadmitAuth, Runtime, ReadinessAuth, D, De, F>,
    > for CheckedReadmitLoweredForExecutionReadyTransition
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    type Output = TransitionOutcome<
        ExecutionReadyRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<NextB>>>,
        D,
        De,
        Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
        Infallible,
        F,
    >;

    fn transition(
        &self,
        input: Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
        context: LoweredReadmissionReadiness<
            T,
            PrevB,
            NextB,
            ReadmitAuth,
            Runtime,
            ReadinessAuth,
            D,
            De,
            F,
        >,
    ) -> Self::Output {
        match context {
            TransitionReadiness::Ready(context) => ReadmitLoweredForExecutionReadyTransition
                .transition(input, context)
                .into(),
            TransitionReadiness::Denied(reason) => TransitionOutcome::denied(reason),
            TransitionReadiness::Deferred(reason) => TransitionOutcome::deferred(reason),
            TransitionReadiness::Stale(recipe) => TransitionOutcome::stale(recipe),
            TransitionReadiness::RebindRequired(impossible) => match impossible {},
            TransitionReadiness::Failed(reason) => TransitionOutcome::failed(reason),
        }
    }
}

pub fn readmit_ready_and_execute_recipe<T, PrevB, NextB, ReadmitAuth, Runtime, ReadinessAuth>(
    bridged: Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
    context: LoweredReadmissionContext<NextB, ReadmitAuth, Runtime, ReadinessAuth>,
) -> SuccessfulTransitionOutcome<
    ExecutedRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<NextB>>>,
>
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    let ready = ReadmitLoweredForExecutionReadyTransition
        .transition(bridged, context)
        .into_value();

    ExecuteReadyRecipeTransition.transition(ready)
}

pub fn checked_readmit_ready_and_execute_recipe<
    T,
    PrevB,
    NextB,
    ReadmitAuth,
    Runtime,
    ReadinessAuth,
    D,
    De,
    F,
>(
    bridged: Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
    context: LoweredReadmissionReadiness<
        T,
        PrevB,
        NextB,
        ReadmitAuth,
        Runtime,
        ReadinessAuth,
        D,
        De,
        F,
    >,
) -> TransitionOutcome<
    ExecutedRecipe<T, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<NextB>>>,
    D,
    De,
    Recipe<Lowered, T, BoundaryBridgedStaleReadableBasis<PrevB>>,
    Infallible,
    F,
>
where
    ReadmitAuth: AuthorityMarker,
    ReadinessAuth: AuthorityMarker,
{
    let ready = CheckedReadmitLoweredForExecutionReadyTransition.transition(bridged, context);

    compose_success_transition(ready, |ready| {
        ExecuteReadyRecipeTransition.transition(ready)
    })
}

#[cfg(test)]
mod tests {
    use crate::proof::{
        mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    };
    use crate::recipe::{Recipe, Unresolved};
    use crate::transition::recipe::{
        LowerRecipeTransition, RecipeResolutionContext, ResolveRecipeTransition,
    };
    use crate::transition::{
        ContextualTransition, Transition, TransitionOutcome, TransitionReadiness,
    };

    use super::{
        checked_readmit_ready_and_execute_recipe, readmit_ready_and_execute_recipe,
        CheckedReadmitLoweredForExecutionReadyTransition, LoweredReadmissionContext,
    };

    struct ResolutionAuthority;
    impl AuthorityMarker for ResolutionAuthority {}

    struct LoweringCapability;
    impl CapabilityMarker for LoweringCapability {}

    struct ReadmissionAuthority;
    impl AuthorityMarker for ReadmissionAuthority {}

    struct ReadinessAuthority;
    impl AuthorityMarker for ReadinessAuthority {}

    fn lowered_recipe(
        basis: u8,
    ) -> crate::recipe::Recipe<
        crate::recipe::Lowered,
        &'static str,
        crate::assumption::FreshnessScopedBasis<
            crate::assumption::CurrentValidity,
            crate::assumption::AssumptionBasis<u8>,
        >,
    > {
        let unresolved = Recipe::<Unresolved, _>::new("payload");
        let resolved = ResolveRecipeTransition.transition(
            unresolved,
            RecipeResolutionContext::new(basis, mint_authority_witness::<ResolutionAuthority>()),
        );

        LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
            .transition(resolved.into_value())
            .into_value()
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_same_basis_runtime_readmission_equivalence_lane(
    ) {
        let direct = crate::transition::admit_ready_and_execute_recipe(
            lowered_recipe(13_u8),
            crate::transition::ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();

        let shifted = readmit_ready_and_execute_recipe(
            lowered_recipe(13_u8).bridge_trust_boundary(),
            LoweredReadmissionContext::new(
                13_u8,
                mint_authority_witness::<ReadmissionAuthority>(),
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();

        assert_eq!(direct.payload(), shifted.payload());
        assert_eq!(
            direct.strong_basis().value(),
            shifted.strong_basis().value()
        );
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_shifted_basis_runtime_readmission_progression(
    ) {
        let executed = readmit_ready_and_execute_recipe(
            lowered_recipe(17_u8).bridge_trust_boundary(),
            LoweredReadmissionContext::new(
                19_u16,
                mint_authority_witness::<ReadmissionAuthority>(),
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();

        assert_eq!(executed.payload(), &"payload");
        assert_eq!(executed.strong_basis().value(), &19_u16);
    }

    #[test]
    fn lowering_and_execution_readiness_boundary_certification_boundary_bridged_divergence_lanes() {
        let bridged = lowered_recipe(23_u8).bridge_trust_boundary();

        let denied = CheckedReadmitLoweredForExecutionReadyTransition.transition(
            lowered_recipe(23_u8).bridge_trust_boundary(),
            TransitionReadiness::<
                LoweredReadmissionContext<
                    u8,
                    ReadmissionAuthority,
                    &'static str,
                    ReadinessAuthority,
                >,
                &'static str,
                &'static str,
                crate::recipe::Recipe<
                    crate::recipe::Lowered,
                    &'static str,
                    crate::assumption::BoundaryBridgedStaleReadableBasis<u8>,
                >,
                std::convert::Infallible,
                &'static str,
            >::denied("denied"),
        );
        let deferred = CheckedReadmitLoweredForExecutionReadyTransition.transition(
            lowered_recipe(23_u8).bridge_trust_boundary(),
            TransitionReadiness::<
                LoweredReadmissionContext<
                    u8,
                    ReadmissionAuthority,
                    &'static str,
                    ReadinessAuthority,
                >,
                &'static str,
                &'static str,
                crate::recipe::Recipe<
                    crate::recipe::Lowered,
                    &'static str,
                    crate::assumption::BoundaryBridgedStaleReadableBasis<u8>,
                >,
                std::convert::Infallible,
                &'static str,
            >::deferred("deferred"),
        );
        let stale = checked_readmit_ready_and_execute_recipe(
            bridged,
            TransitionReadiness::<
                LoweredReadmissionContext<
                    u16,
                    ReadmissionAuthority,
                    &'static str,
                    ReadinessAuthority,
                >,
                &'static str,
                &'static str,
                crate::recipe::Recipe<
                    crate::recipe::Lowered,
                    &'static str,
                    crate::assumption::BoundaryBridgedStaleReadableBasis<u8>,
                >,
                std::convert::Infallible,
                &'static str,
            >::stale(lowered_recipe(23_u8).bridge_trust_boundary()),
        );
        let failed = checked_readmit_ready_and_execute_recipe(
            lowered_recipe(23_u8).bridge_trust_boundary(),
            TransitionReadiness::<
                LoweredReadmissionContext<
                    u16,
                    ReadmissionAuthority,
                    &'static str,
                    ReadinessAuthority,
                >,
                &'static str,
                &'static str,
                crate::recipe::Recipe<
                    crate::recipe::Lowered,
                    &'static str,
                    crate::assumption::BoundaryBridgedStaleReadableBasis<u8>,
                >,
                std::convert::Infallible,
                &'static str,
            >::failed("failed"),
        );

        assert!(matches!(denied, TransitionOutcome::Denied("denied")));
        assert!(matches!(deferred, TransitionOutcome::Deferred("deferred")));
        assert!(matches!(stale, TransitionOutcome::Stale(_)));
        assert!(matches!(failed, TransitionOutcome::Failed("failed")));
    }
}
