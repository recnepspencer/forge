#![cfg(test)]

use core::convert::Infallible;

use crate::assumption::{AssumptionBasis, RebindRequiredBasis, StaleReadableBasis};
use crate::proof::{
    mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
    CapabilityWitness,
};
use crate::recipe::{Recipe, Resolved, Unresolved};
use crate::transition::{
    checked_admit_ready_and_execute_recipe, CheckedLowerRecipeTransition, ContextualTransition,
    ExecutionReadinessContext, PreConstructionGate, RecipeResolutionContext,
    ResolveRecipeTransition, TransitionOutcome, TransitionReadiness,
};

use super::{
    CheckedProofOutcomeExecuteExt, CheckedProofOutcomeLowerExt, CheckedProofOutcomeReadyExt,
    CheckedResolvedRecipeDxExt, CheckedUnresolvedRecipeDxExt, ProofOutcomeKind,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

#[test]
fn pleasant_checked_progression_matches_raw_checked_ready_and_execute_success_lane() {
    let pleasant = Recipe::<Unresolved, _>::new("payload")
        .try_resolve::<u8, ResolutionAuthority, &'static str, &'static str>(
            PreConstructionGate::ready(RecipeResolutionContext::new(
                41_u8,
                mint_authority_witness::<ResolutionAuthority>(),
            )),
        )
        .try_lower::<LoweringCapability, &'static str>(TransitionReadiness::ready(
            mint_capability_witness::<LoweringCapability>(),
        ))
        .try_ready(TransitionReadiness::ready(ExecutionReadinessContext::new(
            "runtime admission",
            mint_authority_witness::<ReadinessAuthority>(),
        )))
        .try_execute();

    let raw_resolved = ResolveRecipeTransition
        .transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(41_u8, mint_authority_witness::<ResolutionAuthority>()),
        )
        .into_value();
    let raw_lowered = match CheckedLowerRecipeTransition::<LoweringCapability>::new().transition(
        raw_resolved,
        TransitionReadiness::<
            CapabilityWitness<LoweringCapability>,
            &'static str,
            &'static str,
            Infallible,
            Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
            &'static str,
        >::ready(mint_capability_witness::<LoweringCapability>()),
    ) {
        TransitionOutcome::Success(lowered) => lowered,
        _ => panic!("expected checked lowering success"),
    };
    let raw = checked_admit_ready_and_execute_recipe::<
        _,
        _,
        _,
        ReadinessAuthority,
        &'static str,
        &'static str,
        &'static str,
    >(
        raw_lowered,
        TransitionReadiness::ready(ExecutionReadinessContext::new(
            "runtime admission",
            mint_authority_witness::<ReadinessAuthority>(),
        )),
    );

    assert_eq!(pleasant.kind(), ProofOutcomeKind::Success);
    match (pleasant.into_raw(), raw) {
        (
            TransitionOutcome::Success(pleasant_executed),
            TransitionOutcome::Success(raw_executed),
        ) => {
            assert_eq!(pleasant_executed.payload(), raw_executed.payload());
            assert_eq!(
                pleasant_executed.strong_basis().value(),
                raw_executed.strong_basis().value()
            );
        }
        _ => panic!("expected success equivalence for pleasant checked progression"),
    }
}

#[test]
fn pleasant_checked_progression_preserves_topology_kinds() {
    let denied = Recipe::<Unresolved, _>::new("payload")
        .try_resolve::<u8, ResolutionAuthority, &'static str, &'static str>(
            PreConstructionGate::denied("denied"),
        );
    let rebind = ResolveRecipeTransition
        .transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(7_u8, mint_authority_witness::<ResolutionAuthority>()),
        )
        .into_value()
        .try_lower::<LoweringCapability, &'static str, &'static str, &'static str>(
            TransitionReadiness::rebind_required(Recipe::with_stage(
                "payload",
                RebindRequiredBasis::new(AssumptionBasis::new(7_u8)),
            )),
        );
    let stale = ResolveRecipeTransition
        .transition(
            Recipe::<Unresolved, _>::new("payload"),
            RecipeResolutionContext::new(9_u8, mint_authority_witness::<ResolutionAuthority>()),
        )
        .into_value()
        .try_lower::<LoweringCapability, &'static str, &'static str, &'static str>(
            TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>()),
        )
        .try_ready::<&'static str, ReadinessAuthority>(TransitionReadiness::stale(
            Recipe::with_stage(
                "payload",
                StaleReadableBasis::new(AssumptionBasis::new(9_u8)),
            ),
        ));

    assert_eq!(denied.kind(), ProofOutcomeKind::Denied);
    assert_eq!(rebind.kind(), ProofOutcomeKind::RebindRequired);
    assert_eq!(stale.kind(), ProofOutcomeKind::Stale);
}
