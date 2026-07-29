use super::WorthUiApplicationSessionState;

impl WorthUiApplicationSessionState {
    pub(crate) fn compile_rebind_plan(
        &self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        lifecycle: crate::runtime::rebind::UiResolvedIdentityLifecycle,
        policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    ) -> Result<crate::runtime::rebind::UiRebindPlan, crate::runtime::rebind::UiRebindPlanningDenial>
    {
        let context = crate::runtime::rebind::UiRebindPlanningContext::new(
            &self.runtime,
            session,
            self.app.prepared_authority(),
        );
        crate::runtime::rebind::UiRebindPlanCompiler::compile(context, lifecycle, policy)
    }

    pub(crate) fn compile_preservation_rebind(
        &self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        evidence: crate::runtime::observation::UiEvidenceOnlySourceChange,
        policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    ) -> Result<crate::runtime::rebind::UiRebindPlan, crate::runtime::rebind::UiRebindPlanningDenial>
    {
        let context = crate::runtime::rebind::UiRebindPlanningContext::new(
            &self.runtime,
            session,
            self.app.prepared_authority(),
        );
        crate::runtime::rebind::UiRebindPlanCompiler::compile_preservation(
            context, evidence, policy,
        )
    }
}
