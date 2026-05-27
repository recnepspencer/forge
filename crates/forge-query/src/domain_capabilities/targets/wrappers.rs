use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};
use crate::target_binding::{
    ForgeQueryAdmittedIntentPlanBindingTarget, ForgeQueryBindingTargetWitness,
    ForgeQueryIntentDeclarationBindingTarget, ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget,
};

use super::core::{ForgeQueryDomainCapabilityTarget, ForgeQueryDomainCapabilityTargetBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBoundContributionTarget {
    erased: ForgeQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanBoundContributionTarget {
    erased: ForgeQueryDomainCapabilityTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    erased: ForgeQueryDomainCapabilityTarget,
}

impl ForgeQueryDeclarationBoundContributionTarget {
    pub fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        let shared = ForgeQueryIntentDeclarationBindingTarget::for_intent_declaration(declaration);
        Self::from_shared(shared)
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        let shared = ForgeQueryIntentDeclarationBindingTarget::from_digest(target_digest);
        Self::from_shared(shared)
    }

    fn from_shared(shared: ForgeQueryIntentDeclarationBindingTarget) -> Self {
        let erased =
            ForgeQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                    "intent declaration target should project into the domain-capability veneer",
                );
        Self { erased }
    }
}

impl ForgeQueryAdmittedPlanBoundContributionTarget {
    pub fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        let shared = ForgeQueryAdmittedIntentPlanBindingTarget::for_admitted_intent_plan(plan);
        Self::from_shared(shared)
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        Self::from_digest_parts(
            target_digest,
            "test.request",
            "test.eligibility",
            "test.decision",
        )
    }

    #[cfg(test)]
    pub(crate) fn from_digest_parts(
        target_digest: impl Into<String>,
        request_digest: impl Into<String>,
        eligibility_digest: impl Into<String>,
        decision_digest: impl Into<String>,
    ) -> Self {
        let shared = ForgeQueryAdmittedIntentPlanBindingTarget::from_digest_parts(
            target_digest,
            request_digest.into(),
            eligibility_digest.into(),
            decision_digest.into(),
        );
        Self::from_shared(shared)
    }

    fn from_shared(shared: ForgeQueryAdmittedIntentPlanBindingTarget) -> Self {
        let erased =
            ForgeQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                    "admitted-intent-plan target should project into the domain-capability veneer",
                );
        Self { erased }
    }
}

impl ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    pub fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        let shared =
            ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget::for_lower_runtime_boundary_envelope(
                envelope,
            );
        Self::from_shared(shared)
    }

    #[cfg(test)]
    pub(crate) fn from_digest(target_digest: impl Into<String>) -> Self {
        let shared =
            ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget::from_digest(target_digest);
        Self::from_shared(shared)
    }

    fn from_shared(shared: ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget) -> Self {
        let erased =
            ForgeQueryDomainCapabilityTarget::from_shared(shared.clone().into_erased_target())
                .expect(
                "lower-runtime boundary target should project into the domain-capability veneer",
            );
        Self { erased }
    }
}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl ForgeQueryDomainCapabilityTargetBinding for ForgeQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl ForgeQueryDomainCapabilityTargetBinding
    for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
{
    fn erased_target(&self) -> &ForgeQueryDomainCapabilityTarget {
        &self.erased
    }

    fn into_erased_target(self) -> ForgeQueryDomainCapabilityTarget {
        self.erased
    }
}

impl crate::target_binding::sealed::Sealed for ForgeQueryDeclarationBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryAdmittedPlanBoundContributionTarget {}
impl crate::target_binding::sealed::Sealed
    for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget
{
}

impl ForgeQueryBindingTargetWitness for ForgeQueryDeclarationBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.erased.into_shared()
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryAdmittedPlanBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.erased.into_shared()
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    fn erased_target(&self) -> &crate::target_binding::ForgeQueryBindingTarget {
        self.erased.shared()
    }

    fn into_erased_target(self) -> crate::target_binding::ForgeQueryBindingTarget {
        self.erased.into_shared()
    }
}
