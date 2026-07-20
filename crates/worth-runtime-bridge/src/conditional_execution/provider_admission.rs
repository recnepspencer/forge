use worth_query_installation::facade::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryOutputEquivalenceRequirement,
    WorthQueryPortableConditionalNodeDeclaration,
};

use super::{BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet};

pub(super) fn validate_provider_shape(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    providers: &BridgeConditionalProviderSet,
) -> Result<(), BridgeConditionalDenial> {
    require_exact(
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::DomainSpecific
        ),
        providers.condition.is_some(),
        BridgeConditionalDenialKind::MissingConditionProvider,
        BridgeConditionalDenialKind::ExtraConditionProvider,
    )?;
    require_exact(
        matches!(
            declaration.dependency_comparator(),
            WorthQueryComparatorRequirement::Registered(_)
        ),
        providers.dependency_comparator.is_some(),
        BridgeConditionalDenialKind::MissingDependencyComparator,
        BridgeConditionalDenialKind::ExtraDependencyComparator,
    )?;
    require_exact(
        matches!(
            declaration.output_equivalence(),
            WorthQueryOutputEquivalenceRequirement::Registered(_)
        ),
        providers.output_comparator.is_some(),
        BridgeConditionalDenialKind::MissingOutputComparator,
        BridgeConditionalDenialKind::ExtraOutputComparator,
    )?;
    require_exact(
        matches!(
            declaration.artifact_reuse_equivalence(),
            WorthQueryArtifactReuseEquivalence::Registered(_)
        ),
        providers.reuse_comparator.is_some(),
        BridgeConditionalDenialKind::MissingReuseComparator,
        BridgeConditionalDenialKind::ExtraReuseComparator,
    )?;
    require_exact(
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::OnDemand
        ),
        providers.trigger.is_some(),
        BridgeConditionalDenialKind::MissingTriggerProvider,
        BridgeConditionalDenialKind::ExtraTriggerProvider,
    )?;
    require_exact(
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::Temporal
        ),
        providers.wake.is_some(),
        BridgeConditionalDenialKind::MissingWakeProvider,
        BridgeConditionalDenialKind::ExtraWakeProvider,
    )?;
    if providers.compute.is_none() {
        return Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::MissingComputeProvider,
            "every installed conditional node requires one exact volatile compute provider",
        ));
    }
    Ok(())
}

fn require_exact(
    required: bool,
    present: bool,
    missing: BridgeConditionalDenialKind,
    extra: BridgeConditionalDenialKind,
) -> Result<(), BridgeConditionalDenial> {
    match (required, present) {
        (true, false) => Err(BridgeConditionalDenial::new(
            missing,
            "required provider is absent",
        )),
        (false, true) => Err(BridgeConditionalDenial::new(
            extra,
            "provider was registered for a declaration that does not require it",
        )),
        _ => Ok(()),
    }
}

pub(super) const PROVIDER_DIMENSION_CHECK_COUNT: usize = 7;
