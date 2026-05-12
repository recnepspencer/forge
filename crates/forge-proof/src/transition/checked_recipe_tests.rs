use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, RebindRequiredBasis, StaleReadableBasis,
};
use crate::proof::{
    mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
};
use crate::recipe::{Admitted, Lowered, Recipe, Resolved, Unresolved};
use crate::transition::AdmitRecipeTransition;

use super::{
    resolve_checked_lower_and_admit_recipe, resolve_lower_and_admit_recipe,
    CheckedAdmitRecipeTransition, CheckedLowerRecipeTransition, CheckedResolveRecipeTransition,
    ContextualTransition, LowerRecipeTransition, PreConstructionGate, RecipeAdmissionReadiness,
    RecipeLoweringReadiness, RecipeResolutionContext, RecipeResolutionGate, TransitionOutcome,
    TransitionReadiness,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct AdmissionAuthority;
impl AuthorityMarker for AdmissionAuthority {}

#[test]
fn checked_resolution_can_deny_before_progression() {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let denied: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::denied("denied");

    let outcome = CheckedResolveRecipeTransition.transition(unresolved, denied);

    assert!(matches!(outcome, TransitionOutcome::Denied("denied")));
}

#[test]
fn checked_resolution_can_defer_before_progression() {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let deferred: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::deferred("deferred");

    let outcome = CheckedResolveRecipeTransition.transition(unresolved, deferred);

    assert!(matches!(outcome, TransitionOutcome::Deferred("deferred")));
}

#[test]
fn checked_resolution_composes_into_admitted_recipe_when_ready() {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let gate: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        ));
    let lower = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>());
    let admit = AdmitRecipeTransition::new(mint_authority_witness::<AdmissionAuthority>());

    let admitted = resolve_lower_and_admit_recipe(unresolved, gate, &lower, &admit);

    match admitted {
        TransitionOutcome::Success(admitted) => {
            assert_eq!(admitted.payload(), &"payload");
            assert_eq!(admitted.strong_basis().value(), &7_u8);
        }
        _ => panic!("ready gate should compose into success"),
    }
}

#[test]
fn checked_lowering_can_rebind_before_progression() {
    let resolved = Recipe::<Resolved, _, _>::with_stage(
        "payload",
        FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(7_u8)),
    );
    let rebind_recipe = Recipe::<Resolved, _, RebindRequiredBasis<u8>>::with_stage(
        "payload",
        RebindRequiredBasis::new(AssumptionBasis::new(7_u8)),
    );
    let readiness: RecipeLoweringReadiness<
        &str,
        u8,
        LoweringCapability,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::rebind_required(rebind_recipe);

    let outcome =
        CheckedLowerRecipeTransition::<LoweringCapability>::new().transition(resolved, readiness);

    assert!(matches!(outcome, TransitionOutcome::RebindRequired(_)));
}

#[test]
fn checked_admission_can_return_stale_before_progression() {
    let lowered = Recipe::<Lowered, _, _>::with_stage(
        "payload",
        FreshnessScopedBasis::<CurrentValidity, _>::new(AssumptionBasis::new(7_u8)),
    );
    let stale_recipe = Recipe::<Lowered, _, StaleReadableBasis<u8>>::with_stage(
        "payload",
        StaleReadableBasis::new(AssumptionBasis::new(7_u8)),
    );
    let readiness: RecipeAdmissionReadiness<
        &str,
        u8,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::stale(stale_recipe);

    let outcome =
        CheckedAdmitRecipeTransition::<AdmissionAuthority>::new().transition(lowered, readiness);

    assert!(matches!(outcome, TransitionOutcome::Stale(_)));
}

#[test]
fn checked_progression_can_fail_before_later_steps_run() {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolution_gate: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        ));
    let lowering_readiness: RecipeLoweringReadiness<
        &str,
        u8,
        LoweringCapability,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::failed("failed");
    let admission_readiness: RecipeAdmissionReadiness<
        &str,
        u8,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>());

    let outcome = resolve_checked_lower_and_admit_recipe(
        unresolved,
        resolution_gate,
        lowering_readiness,
        admission_readiness,
    );

    assert!(matches!(outcome, TransitionOutcome::Failed("failed")));
}

#[test]
fn checked_progression_composes_successfully_when_all_readiness_is_ready() {
    let unresolved = Recipe::<Unresolved, _>::new("payload");
    let resolution_gate: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        ));
    let lowering_readiness: RecipeLoweringReadiness<
        &str,
        u8,
        LoweringCapability,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>());
    let admission_readiness: RecipeAdmissionReadiness<
        &str,
        u8,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>());

    let outcome = resolve_checked_lower_and_admit_recipe(
        unresolved,
        resolution_gate,
        lowering_readiness,
        admission_readiness,
    );

    match outcome {
        TransitionOutcome::Success(admitted) => {
            assert_eq!(admitted.payload(), &"payload");
            assert_eq!(admitted.strong_basis().value(), &7_u8);
        }
        _ => panic!("all-ready checked progression should compose into success"),
    }
}

#[test]
fn transition_outcome_algebra_certification_equivalence_lane() {
    let direct_gate: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        ));
    let checked_gate: RecipeResolutionGate<u8, ResolutionAuthority, &'static str, &'static str> =
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        ));
    let lower = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>());
    let admit = AdmitRecipeTransition::new(mint_authority_witness::<AdmissionAuthority>());
    let lowering_readiness: RecipeLoweringReadiness<
        &str,
        u8,
        LoweringCapability,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>());
    let admission_readiness: RecipeAdmissionReadiness<
        &str,
        u8,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    > = TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>());

    let direct = resolve_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        direct_gate,
        &lower,
        &admit,
    );
    let checked = resolve_checked_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        checked_gate,
        lowering_readiness,
        admission_readiness,
    );

    match (direct, checked) {
        (TransitionOutcome::Success(direct), TransitionOutcome::Success(checked)) => {
            assert_eq!(direct.payload(), checked.payload());
            assert_eq!(
                direct.strong_basis().value(),
                checked.strong_basis().value()
            );
        }
        _ => panic!("all-ready direct and checked lanes should both succeed"),
    }
}

#[test]
fn transition_outcome_algebra_certification_divergence_lanes() {
    let denial: TransitionOutcome<
        Recipe<Admitted, &str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
        &'static str,
        &'static str,
        Recipe<Lowered, &str, StaleReadableBasis<u8>>,
        Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
        &'static str,
    > = resolve_checked_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        PreConstructionGate::<
            RecipeResolutionContext<u8, ResolutionAuthority>,
            &'static str,
            &'static str,
        >::denied("denied"),
        TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>()),
        TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>()),
    );
    let defer: TransitionOutcome<
        Recipe<Admitted, &str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
        &'static str,
        &'static str,
        Recipe<Lowered, &str, StaleReadableBasis<u8>>,
        Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
        &'static str,
    > = resolve_checked_lower_and_admit_recipe(
        Recipe::<Unresolved, _>::new("payload"),
        PreConstructionGate::<
            RecipeResolutionContext<u8, ResolutionAuthority>,
            &'static str,
            &'static str,
        >::deferred("deferred"),
        TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>()),
        TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>()),
    );
    let stale: TransitionOutcome<
        Recipe<Admitted, &str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
        &'static str,
        &'static str,
        Recipe<Lowered, &str, StaleReadableBasis<u8>>,
        Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
        &'static str,
    > = resolve_checked_lower_and_admit_recipe::<
        &str,
        u8,
        ResolutionAuthority,
        LoweringCapability,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    >(
        Recipe::<Unresolved, _>::new("payload"),
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        )),
        TransitionReadiness::ready(mint_capability_witness::<LoweringCapability>()),
        TransitionReadiness::stale(Recipe::<Lowered, _, StaleReadableBasis<u8>>::with_stage(
            "payload",
            StaleReadableBasis::new(AssumptionBasis::new(7_u8)),
        )),
    );
    let rebind: TransitionOutcome<
        Recipe<Admitted, &str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
        &'static str,
        &'static str,
        Recipe<Lowered, &str, StaleReadableBasis<u8>>,
        Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
        &'static str,
    > = resolve_checked_lower_and_admit_recipe::<
        &str,
        u8,
        ResolutionAuthority,
        LoweringCapability,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    >(
        Recipe::<Unresolved, _>::new("payload"),
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        )),
        TransitionReadiness::rebind_required(
            Recipe::<Resolved, _, RebindRequiredBasis<u8>>::with_stage(
                "payload",
                RebindRequiredBasis::new(AssumptionBasis::new(7_u8)),
            ),
        ),
        TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>()),
    );
    let failed: TransitionOutcome<
        Recipe<Admitted, &str, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<u8>>>,
        &'static str,
        &'static str,
        Recipe<Lowered, &str, StaleReadableBasis<u8>>,
        Recipe<Resolved, &str, RebindRequiredBasis<u8>>,
        &'static str,
    > = resolve_checked_lower_and_admit_recipe::<
        &str,
        u8,
        ResolutionAuthority,
        LoweringCapability,
        AdmissionAuthority,
        &'static str,
        &'static str,
        &'static str,
    >(
        Recipe::<Unresolved, _>::new("payload"),
        PreConstructionGate::ready(RecipeResolutionContext::new(
            7_u8,
            mint_authority_witness::<ResolutionAuthority>(),
        )),
        TransitionReadiness::failed("failed"),
        TransitionReadiness::ready(mint_authority_witness::<AdmissionAuthority>()),
    );

    assert!(matches!(denial, TransitionOutcome::Denied("denied")));
    assert!(matches!(defer, TransitionOutcome::Deferred("deferred")));
    assert!(matches!(stale, TransitionOutcome::Stale(_)));
    assert!(matches!(rebind, TransitionOutcome::RebindRequired(_)));
    assert!(matches!(failed, TransitionOutcome::Failed("failed")));
}
