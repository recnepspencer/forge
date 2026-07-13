use super::read_composition_hooks::{
    WorthQueryReadInvariantPackContext, WorthQueryReadInvariantPackViolation,
};
use super::{
    WorthQueryGenericInspectionIntentTarget, WorthQueryInspection, WorthQueryInspectionTarget,
    WorthQueryInstalledProgram, WorthQueryProgram, WorthQueryReadBuilder,
    WorthQueryReadDomainInvariantDenial, WorthQueryReadFamily,
    WorthQueryReadFamilyInvariantEvidence, WorthQueryReadGraph, WorthQueryReadResult,
    WorthQueryRuntimeError, WorthQueryUnifiedInspectionResult, WorthQueryWorkspace,
};

impl WorthQueryWorkspace {
    pub(crate) fn execute_declared_read_graph_in_authority(
        &mut self,
        read_graph: WorthQueryReadGraph,
        authority: &super::WorthQueryGraphReadAccessAuthorityContext,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let family = WorthQueryReadFamily::new_kernel_only("declared_read", read_graph);
        self.read_family_intent_in_graph_read_authority(&family, authority)
            .execute()
    }

    pub fn compose_read(
        &mut self,
        declaration: impl FnOnce(
            WorthQueryReadBuilder,
        ) -> Result<WorthQueryReadGraph, super::WorthQueryReadDenial>,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.compose_read_with_invariant_pack(declaration, |_context| Ok(()))
    }

    pub fn compose_read_with_invariant_pack(
        &mut self,
        declaration: impl FnOnce(
            WorthQueryReadBuilder,
        ) -> Result<WorthQueryReadGraph, super::WorthQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &WorthQueryReadInvariantPackContext<'_>,
        ) -> Result<(), WorthQueryReadInvariantPackViolation>,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(WorthQueryReadBuilder::new())
            .map_err(WorthQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = WorthQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            WorthQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                WorthQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let family = WorthQueryReadFamily::new_kernel_only("composed_read", read_graph);
        self.read_family_intent(&family).execute()
    }

    pub fn define_read_family(
        &mut self,
        family_name: impl Into<String>,
        declaration: impl FnOnce(
            WorthQueryReadBuilder,
        ) -> Result<WorthQueryReadGraph, super::WorthQueryReadDenial>,
    ) -> Result<WorthQueryReadFamily, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(WorthQueryReadBuilder::new())
            .map_err(WorthQueryRuntimeError::ReadCompositionDenied)?;
        Ok(WorthQueryReadFamily::new_kernel_only(
            family_name,
            read_graph,
        ))
    }

    pub fn define_read_family_with_invariant_pack(
        &mut self,
        family_name: impl Into<String>,
        invariant_family: impl Into<String>,
        declaration: impl FnOnce(
            WorthQueryReadBuilder,
        ) -> Result<WorthQueryReadGraph, super::WorthQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &WorthQueryReadInvariantPackContext<'_>,
        ) -> Result<(), WorthQueryReadInvariantPackViolation>,
    ) -> Result<WorthQueryReadFamily, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(WorthQueryReadBuilder::new())
            .map_err(WorthQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = WorthQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            WorthQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                WorthQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let invariant_evidence = WorthQueryReadFamilyInvariantEvidence::new(
            invariant_family,
            invariant_context.read_domain_invariant_summary(),
        );
        Ok(WorthQueryReadFamily::new_domain_invariant_admitted(
            family_name,
            invariant_evidence,
            read_graph,
        ))
    }

    pub fn execute_read_family(
        &mut self,
        family: &WorthQueryReadFamily,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        self.read_family_intent(family).execute()
    }

    pub fn execute_read_family_with_access_plan(
        &mut self,
        family: &WorthQueryReadFamily,
        graph_read_access_plan: super::WorthQueryAdmittedGraphReadAccessPlan,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let handoff = self
            .read_family_intent(family)
            .review()?
            .admitted_handoff()
            .ok_or_else(|| {
                WorthQueryRuntimeError::ReadCompositionDenied(super::WorthQueryReadDenial::new(
                    super::WorthQueryReadDenialKind::BasisPreflightDenied,
                    "read family was not admitted for access-plan execution",
                ))
            })?;
        let binding = self.into_runtime_read_execution_binding_with_access_plan(
            handoff,
            graph_read_access_plan,
        )?;
        self.execute_bound_read_execution(binding)
    }

    pub fn execute_read_family_in_basis_context_with_access_plan(
        &mut self,
        family: &WorthQueryReadFamily,
        context: &crate::query_context::ScopedQueryBasisContext,
        graph_read_access_plan: super::WorthQueryAdmittedGraphReadAccessPlan,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        let handoff = self
            .read_family_in_basis_context_intent(family, context)
            .review()?
            .admitted_handoff()
            .ok_or_else(|| {
                WorthQueryRuntimeError::ReadCompositionDenied(super::WorthQueryReadDenial::new(
                    super::WorthQueryReadDenialKind::BasisPreflightDenied,
                    "read family basis context was not admitted for access-plan execution",
                ))
            })?;
        let binding = self.into_runtime_read_execution_binding_with_access_plan(
            handoff,
            graph_read_access_plan,
        )?;
        self.execute_bound_read_execution(binding)
    }

    pub fn execute_read_family_in_basis_context(
        &mut self,
        family: &WorthQueryReadFamily,
        context: &crate::query_context::ScopedQueryBasisContext,
    ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::WorthQueryRuntimeFacadeFamily::Read)?;
        self.read_family_in_basis_context_intent(family, context)
            .execute()
    }

    pub fn explain_graph_read_access_shape(
        &self,
        family: &WorthQueryReadFamily,
    ) -> Result<
        super::WorthQueryGraphReadAccessShapeExplanation,
        super::WorthQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_graph_read_access_shape_for_family(family)
    }

    pub fn admit_graph_read_access_authority(
        &self,
        request: super::WorthQueryGraphReadAccessAuthorityRequest,
    ) -> Result<
        super::WorthQueryGraphReadAccessAuthorityContext,
        super::WorthQueryGraphReadAccessAuthorityDenial,
    > {
        super::admit_graph_read_access_authority(request)
    }

    pub fn admit_graph_read_access_authority_from_policy_tenant_request(
        &self,
        request: super::WorthQueryGraphReadPolicyTenantAuthorityRequest,
    ) -> Result<
        super::WorthQueryGraphReadAccessAuthorityContext,
        super::WorthQueryGraphReadAccessAuthorityDenial,
    > {
        super::admit_graph_read_access_authority_from_policy_tenant_request(request)
    }

    pub fn explain_graph_read_access_shape_in_authority(
        &self,
        family: &WorthQueryReadFamily,
        authority: &super::WorthQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        super::WorthQueryGraphReadAccessShapeExplanation,
        super::WorthQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_graph_read_access_shape_for_family_in_authority(family, authority)
    }

    pub fn admit_graph_read_access_in_authority(
        &self,
        family: &WorthQueryReadFamily,
        authority: &super::WorthQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        super::WorthQueryGraphReadAccessAdmission,
        super::WorthQueryGraphReadAccessShapeExplanationError,
    > {
        self.admit_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn plan_graph_read_access_in_authority(
        &self,
        family: &WorthQueryReadFamily,
        authority: &super::WorthQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        Option<super::WorthQueryAdmittedGraphReadAccessPlan>,
        super::WorthQueryGraphReadAccessShapeExplanationError,
    > {
        super::plan_admitted_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn explain_boolean_selectivity_shape(
        &self,
        family: &WorthQueryReadFamily,
    ) -> Result<
        super::WorthQueryBooleanSelectivityShape,
        super::WorthQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_boolean_selectivity_shape_for_family(family)
    }

    pub fn plan_live_graph_read_access(
        &self,
        family: &WorthQueryReadFamily,
        budget: super::WorthQueryLiveGraphReadMaintenanceBudget,
    ) -> Result<super::WorthQueryLiveGraphReadAccessPlan, super::WorthQueryLiveGraphReadAccessDenial>
    {
        let one_shot = match super::plan_admitted_graph_read_access_for_family(family) {
            Ok(Some(one_shot)) => one_shot,
            Ok(None) => {
                return Err(super::WorthQueryLiveGraphReadAccessDenial::new(
                    super::WorthQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport,
                    family.family_digest(),
                    &budget,
                    "live graph read access planning requires an admitted one-shot access plan",
                ));
            }
            Err(error) => {
                return Err(super::WorthQueryLiveGraphReadAccessDenial::new(
                    super::WorthQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport,
                    family.family_digest(),
                    &budget,
                    format!("live graph read access planning could not derive one-shot access: {error:?}"),
                ));
            }
        };
        super::WorthQueryLiveGraphReadAccessPlan::from_one_shot_access_plan(&one_shot, budget)
    }

    pub fn inspect_intent<'a, T>(
        &'a self,
        target: T,
    ) -> crate::intent_admission::WorthQueryRuntimeInspectionIntentAuthoring<'a>
    where
        T: WorthQueryGenericInspectionIntentTarget<'a>,
    {
        self.runtime.inspect_intent(target)
    }

    pub fn install_program(
        &mut self,
        program: WorthQueryProgram,
    ) -> Result<WorthQueryInstalledProgram, WorthQueryRuntimeError> {
        self.runtime.install_program(program)
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<WorthQueryInspection, WorthQueryRuntimeError>
    where
        T: Into<WorthQueryInspectionTarget<'a>>,
    {
        self.runtime.inspect(target)
    }

    pub fn inspect_live<T>(
        &self,
        view: &super::WorthQueryLiveView<T>,
    ) -> Result<WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
        self.inspect_live_by_name(view.name())
    }

    pub fn inspect_live_target(
        &self,
        target: &super::WorthQueryLiveArtifactTarget,
    ) -> Result<WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
        let resolved = self.resolve_live_artifact_target(target.terminal_view_name_projection())?;
        self.inspect_live_by_name(resolved.terminal_view_name_projection())
    }

    pub(crate) fn inspect_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
        self.runtime.inspect_live_view_name_result(view_name)
    }
}
