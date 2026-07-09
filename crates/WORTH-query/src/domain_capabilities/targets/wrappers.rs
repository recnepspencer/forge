use crate::application::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::runtime::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration,
    WorthQueryLowerRuntimeBoundaryEnvelope,
};
use crate::target_binding::{
    WorthQueryAdmittedIntentPlanBindingTarget, WorthQueryBindingTargetWitness,
    WorthQueryIntentDeclarationBindingTarget, WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
};

use super::core::{WorthQueryDomainCapabilityTarget, WorthQueryDomainCapabilityTargetBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBoundContributionTarget {
    erased: WorthQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedPlanBoundContributionTarget {
    erased: WorthQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeBoundaryBoundContributionTarget {
    erased: WorthQueryDomainCapabilityTarget,
}

impl WorthQueryDeclarationBoundContributionTarget {
    pub fn for_intent_declaration(declaration: &WorthQueryIntentDeclaration) -> Self {
        let shared = WorthQueryIntentDeclarationBindingTarget::for_intent_declaration(declaration);
        Self::from_shared(shared)
    }

    pub fn for_canonical_declaration<
        D: WorthQueryDomainEntryMarker,
        I: WorthQueryDeclarationInput<D>,
    >(
        declaration: &WorthQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Self {
        let shared =
            WorthQueryIntentDeclarationBindingTarget::for_canonical_declaration(declaration);
        Self::from_shared(shared)
    }

    fn from_shared(shared: WorthQueryIntentDeclarationBindingTarget) -> Self {
        let erased =
            WorthQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                    "intent declaration target should project into the domain-capability veneer",
                );
        Self { erased }
    }
}

impl WorthQueryAdmittedPlanBoundContributionTarget {
    pub fn for_admitted_intent_plan(plan: &WorthQueryAdmittedIntentPlan) -> Self {
        let shared = WorthQueryAdmittedIntentPlanBindingTarget::for_admitted_intent_plan(plan);
        Self::from_shared(shared)
    }

    fn from_shared(shared: WorthQueryAdmittedIntentPlanBindingTarget) -> Self {
        let erased =
            WorthQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                    "admitted-intent-plan target should project into the domain-capability veneer",
                );
        Self { erased }
    }
}

impl WorthQueryLowerRuntimeBoundaryBoundContributionTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        let shared =
            WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget::for_lower_runtime_boundary_envelope(
                envelope,
            );
        Self::from_shared(shared)
    }

    fn from_shared(shared: WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget) -> Self {
        let erased =
            WorthQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                "lower-runtime boundary target should project into the domain-capability veneer",
            );
        Self { erased }
    }
}

impl WorthQueryDomainCapabilityTargetBinding for WorthQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &WorthQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> WorthQueryDomainCapabilityTarget {
        self.erased
    }
}

impl WorthQueryDomainCapabilityTargetBinding for WorthQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &WorthQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> WorthQueryDomainCapabilityTarget {
        self.erased
    }
}

impl WorthQueryDomainCapabilityTargetBinding
    for WorthQueryLowerRuntimeBoundaryBoundContributionTarget
{
    fn erased_target(&self) -> &WorthQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> WorthQueryDomainCapabilityTarget {
        self.erased
    }
}

impl crate::target_binding::sealed::Sealed for WorthQueryDeclarationBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed for WorthQueryAdmittedPlanBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed
    for WorthQueryLowerRuntimeBoundaryBoundContributionTarget
{
}

impl WorthQueryBindingTargetWitness for WorthQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::WorthQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::WorthQueryBindingTarget {
        self.erased.into_shared()
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::WorthQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::WorthQueryBindingTarget {
        self.erased.into_shared()
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryLowerRuntimeBoundaryBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::WorthQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::WorthQueryBindingTarget {
        self.erased.into_shared()
    }
}
