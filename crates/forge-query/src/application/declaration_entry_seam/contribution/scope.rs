#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ForgeQueryDeclarationEntryContributionProofScope {
    admitted_plan_digest: Option<String>,
    lower_runtime_boundary_digest: Option<String>,
}

impl ForgeQueryDeclarationEntryContributionProofScope {
    pub(crate) fn with_admitted_plan(
        mut self,
        plan: crate::runtime::ForgeQueryAdmittedIntentPlan,
    ) -> Self {
        self.admitted_plan_digest = Some(plan.decision_digest().to_string());
        self
    }

    pub(crate) fn with_lower_runtime_boundary(
        mut self,
        envelope: crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.lower_runtime_boundary_digest =
            Some(envelope.envelope_identity().terminal_projection_for_reporting().to_string());
        self
    }

    pub(crate) fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
    }

    pub(crate) fn lower_runtime_boundary_digest(&self) -> Option<&str> {
        self.lower_runtime_boundary_digest.as_deref()
    }
}
