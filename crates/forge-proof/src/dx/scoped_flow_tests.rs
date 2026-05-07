use crate::dx::{proof_flow, ExecutionReadyRecipeDxExt};
use crate::proof::{
    mint_authority_witness, mint_capability_witness, AuthorityMarker, CapabilityMarker,
};
use crate::recipe::{Recipe, Unresolved};
use crate::transition::{
    ContextualTransition, ExecutionReadinessContext, LowerRecipeTransition,
    RecipeResolutionContext, ResolveRecipeTransition, Transition,
};

struct ResolutionAuthority;
impl AuthorityMarker for ResolutionAuthority {}

struct AlternateResolutionAuthority;
impl AuthorityMarker for AlternateResolutionAuthority {}

struct LoweringCapability;
impl CapabilityMarker for LoweringCapability {}

struct AlternateLoweringCapability;
impl CapabilityMarker for AlternateLoweringCapability {}

struct ReadinessAuthority;
impl AuthorityMarker for ReadinessAuthority {}

struct AlternateReadinessAuthority;
impl AuthorityMarker for AlternateReadinessAuthority {}

#[test]
fn scoped_defaults_match_raw_full_progression() {
    let pleasant = proof_flow()
        .resolution_authority(mint_authority_witness::<ResolutionAuthority>())
        .lowering_capability(mint_capability_witness::<LoweringCapability>())
        .readiness_authority(mint_authority_witness::<ReadinessAuthority>())
        .recipe("payload")
        .resolve(7_u8)
        .lower()
        .ready("runtime admission")
        .execute();

    let raw_resolved = ResolveRecipeTransition.transition(
        Recipe::<Unresolved, _>::new("payload"),
        RecipeResolutionContext::new(7_u8, mint_authority_witness::<ResolutionAuthority>()),
    );
    let raw_lowered = LowerRecipeTransition::new(mint_capability_witness::<LoweringCapability>())
        .transition(raw_resolved.into_value())
        .into_value();
    let raw_ready = crate::transition::AdmitExecutionReadyRecipeTransition
        .transition(
            raw_lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();
    let raw_executed = crate::transition::ExecuteReadyRecipeTransition
        .transition(raw_ready)
        .into_value();

    assert_eq!(pleasant.payload(), raw_executed.payload());
    assert_eq!(
        pleasant.strong_basis().value(),
        raw_executed.strong_basis().value()
    );
}

#[test]
fn scoped_defaults_allow_visible_local_overrides() {
    let pleasant = proof_flow()
        .resolution_authority(mint_authority_witness::<ResolutionAuthority>())
        .lowering_capability(mint_capability_witness::<LoweringCapability>())
        .readiness_authority(mint_authority_witness::<ReadinessAuthority>())
        .recipe("payload")
        .resolve_with(
            mint_authority_witness::<AlternateResolutionAuthority>(),
            11_u16,
        )
        .lower_with(mint_capability_witness::<AlternateLoweringCapability>())
        .ready_with(
            mint_authority_witness::<AlternateReadinessAuthority>(),
            "runtime admission",
        )
        .execute();

    let raw_resolved = ResolveRecipeTransition.transition(
        Recipe::<Unresolved, _>::new("payload"),
        RecipeResolutionContext::new(
            11_u16,
            mint_authority_witness::<AlternateResolutionAuthority>(),
        ),
    );
    let raw_lowered =
        LowerRecipeTransition::new(mint_capability_witness::<AlternateLoweringCapability>())
            .transition(raw_resolved.into_value())
            .into_value();
    let raw_ready = crate::transition::AdmitExecutionReadyRecipeTransition
        .transition(
            raw_lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<AlternateReadinessAuthority>(),
            ),
        )
        .into_value();
    let raw_executed = crate::transition::ExecuteReadyRecipeTransition
        .transition(raw_ready)
        .into_value();

    assert_eq!(pleasant.payload(), raw_executed.payload());
    assert_eq!(
        pleasant.strong_basis().value(),
        raw_executed.strong_basis().value()
    );
}

#[test]
fn scoped_defaults_preserve_mixed_inheritance_and_override_semantics() {
    let pleasant = proof_flow()
        .resolution_authority(mint_authority_witness::<ResolutionAuthority>())
        .lowering_capability(mint_capability_witness::<LoweringCapability>())
        .readiness_authority(mint_authority_witness::<ReadinessAuthority>())
        .recipe("payload")
        .resolve(7_u8)
        .lower_with(mint_capability_witness::<AlternateLoweringCapability>())
        .ready("runtime admission")
        .execute();

    let raw_resolved = ResolveRecipeTransition.transition(
        Recipe::<Unresolved, _>::new("payload"),
        RecipeResolutionContext::new(7_u8, mint_authority_witness::<ResolutionAuthority>()),
    );
    let raw_lowered =
        LowerRecipeTransition::new(mint_capability_witness::<AlternateLoweringCapability>())
            .transition(raw_resolved.into_value())
            .into_value();
    let raw_ready = crate::transition::AdmitExecutionReadyRecipeTransition
        .transition(
            raw_lowered,
            ExecutionReadinessContext::new(
                "runtime admission",
                mint_authority_witness::<ReadinessAuthority>(),
            ),
        )
        .into_value();
    let raw_executed = crate::transition::ExecuteReadyRecipeTransition
        .transition(raw_ready)
        .into_value();

    assert_eq!(pleasant.payload(), raw_executed.payload());
    assert_eq!(
        pleasant.strong_basis().value(),
        raw_executed.strong_basis().value()
    );
}
