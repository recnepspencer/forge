use crate::domain_capabilities::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::admitted_plan::ForgeQueryAdmittedPlanDomainContributionSurface;
use super::intent::ForgeQueryIntentDomainContributionSurface;
use super::lower_runtime::ForgeQueryLowerRuntimeDomainContributionSurface;

pub fn forge_query_domain(domain: impl Into<String>) -> ForgeQueryDomainContributionSurface {
    ForgeQueryDomainContributionSurface {
        domain: domain.into(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainContributionSurface {
    domain: String,
}

impl ForgeQueryDomainContributionSurface {
    pub fn for_intent(
        &self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryIntentDomainContributionSurface {
        ForgeQueryIntentDomainContributionSurface {
            domain: self.domain.clone(),
            target: ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(
                declaration,
            ),
        }
    }

    pub fn for_admitted_intent_plan(
        &self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryAdmittedPlanDomainContributionSurface {
        ForgeQueryAdmittedPlanDomainContributionSurface {
            domain: self.domain.clone(),
            target: ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        }
    }

    pub fn for_lower_runtime_boundary_envelope(
        &self,
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> ForgeQueryLowerRuntimeDomainContributionSurface {
        ForgeQueryLowerRuntimeDomainContributionSurface {
            domain: self.domain.clone(),
            target: ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(envelope),
        }
    }

    pub fn for_lower_runtime_boundary_source<S>(
        &self,
        source: &S,
    ) -> ForgeQueryLowerRuntimeDomainContributionSurface
    where
        S: ForgeQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }
}
