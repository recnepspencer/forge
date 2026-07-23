use worth_query_installation::facade::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryOutputEquivalenceRequirement,
    WorthQueryPortableConditionalNodeDeclaration,
};

use super::provider_semantics::BridgeConditionalProviderSemanticContracts;
use super::{BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet};

#[derive(Clone)]
pub(super) struct BridgeConditionalProviderAdmission {
    declaration: WorthQueryPortableConditionalNodeDeclaration,
    required_roles: [bool; PROVIDER_DIMENSION_CHECK_COUNT],
    semantic_contracts: BridgeConditionalProviderSemanticContracts,
}

impl BridgeConditionalProviderAdmission {
    pub(super) fn declaration(&self) -> &WorthQueryPortableConditionalNodeDeclaration {
        &self.declaration
    }

    pub(super) const fn required_roles(&self) -> &[bool; PROVIDER_DIMENSION_CHECK_COUNT] {
        &self.required_roles
    }

    pub(super) fn semantic_contracts(&self) -> &BridgeConditionalProviderSemanticContracts {
        &self.semantic_contracts
    }
}

pub(super) fn admit_provider_set(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    providers: &BridgeConditionalProviderSet,
) -> Result<BridgeConditionalProviderAdmission, BridgeConditionalDenial> {
    let required_roles = [
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::DomainSpecific
        ),
        matches!(
            declaration.dependency_comparator(),
            WorthQueryComparatorRequirement::Registered(_)
        ),
        matches!(
            declaration.output_equivalence(),
            WorthQueryOutputEquivalenceRequirement::Registered(_)
        ),
        matches!(
            declaration.artifact_reuse_equivalence(),
            WorthQueryArtifactReuseEquivalence::Registered(_)
        ),
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::OnDemand
        ),
        matches!(
            declaration.condition().class(),
            WorthQueryConditionalConditionClass::Temporal
        ),
        true,
    ];
    let present_roles = [
        providers.condition.is_some(),
        providers.dependency_comparator.is_some(),
        providers.output_comparator.is_some(),
        providers.reuse_comparator.is_some(),
        providers.trigger.is_some(),
        providers.wake.is_some(),
        providers.compute.is_some(),
    ];
    let denials = [
        (
            BridgeConditionalDenialKind::MissingConditionProvider,
            BridgeConditionalDenialKind::ExtraConditionProvider,
        ),
        (
            BridgeConditionalDenialKind::MissingDependencyComparator,
            BridgeConditionalDenialKind::ExtraDependencyComparator,
        ),
        (
            BridgeConditionalDenialKind::MissingOutputComparator,
            BridgeConditionalDenialKind::ExtraOutputComparator,
        ),
        (
            BridgeConditionalDenialKind::MissingReuseComparator,
            BridgeConditionalDenialKind::ExtraReuseComparator,
        ),
        (
            BridgeConditionalDenialKind::MissingTriggerProvider,
            BridgeConditionalDenialKind::ExtraTriggerProvider,
        ),
        (
            BridgeConditionalDenialKind::MissingWakeProvider,
            BridgeConditionalDenialKind::ExtraWakeProvider,
        ),
        (
            BridgeConditionalDenialKind::MissingComputeProvider,
            BridgeConditionalDenialKind::ExtraComputeProvider,
        ),
    ];
    for ((required, present), (missing, extra)) in
        required_roles.into_iter().zip(present_roles).zip(denials)
    {
        require_exact(required, present, missing, extra)?;
    }
    Ok(BridgeConditionalProviderAdmission {
        declaration: declaration.clone(),
        required_roles,
        semantic_contracts: providers.semantic_contracts.clone(),
    })
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
