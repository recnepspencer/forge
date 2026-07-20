#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryDeclarationEntryContributionProofScope {
    admitted_plan_digest: Option<String>,
    lower_runtime_boundary_digest: Option<String>,
}

impl WorthQueryDeclarationEntryContributionProofScope {
    pub(crate) fn with_admitted_plan(
        mut self,
        plan: crate::runtime::WorthQueryAdmittedIntentPlan,
    ) -> Self {
        self.admitted_plan_digest = Some(plan.decision_digest().to_string());
        self
    }

    pub(crate) fn with_lower_runtime_boundary(
        mut self,
        envelope: crate::runtime::WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.lower_runtime_boundary_digest = Some(
            envelope
                .envelope_identity()
                .terminal_projection_for_reporting()
                .to_string(),
        );
        self
    }

    #[cfg(test)]
    pub(crate) fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn lower_runtime_boundary_digest(&self) -> Option<&str> {
        self.lower_runtime_boundary_digest.as_deref()
    }
}
