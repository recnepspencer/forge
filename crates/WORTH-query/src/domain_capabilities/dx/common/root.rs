use crate::domain_capabilities::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::runtime::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration,
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::admitted_plan::WorthQueryAdmittedPlanDomainContributionSurface;
use super::intent::WorthQueryIntentDomainContributionSurface;
use super::lower_runtime::WorthQueryLowerRuntimeDomainContributionSurface;

pub fn worth_query_domain(domain: impl Into<String>) -> WorthQueryDomainContributionSurface {
    WorthQueryDomainContributionSurface {
        domain: domain.into(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainContributionSurface {
    domain: String,
}

impl WorthQueryDomainContributionSurface {
    pub fn for_intent(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> WorthQueryIntentDomainContributionSurface {
        WorthQueryIntentDomainContributionSurface {
            domain: self.domain.clone(),
            target: WorthQueryDeclarationBoundContributionTarget::for_intent_declaration(
                declaration,
            ),
        }
    }

    pub fn for_admitted_intent_plan(
        &self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> WorthQueryAdmittedPlanDomainContributionSurface {
        WorthQueryAdmittedPlanDomainContributionSurface {
            domain: self.domain.clone(),
            target: WorthQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        }
    }

    pub fn for_lower_runtime_boundary_envelope(
        &self,
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> WorthQueryLowerRuntimeDomainContributionSurface {
        WorthQueryLowerRuntimeDomainContributionSurface {
            domain: self.domain.clone(),
            target: WorthQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(envelope),
        }
    }

    pub fn for_lower_runtime_boundary_source<S>(
        &self,
        source: &S,
    ) -> WorthQueryLowerRuntimeDomainContributionSurface
    where
        S: WorthQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }
}
