use super::read_composition_hooks::{
    ForgeQueryReadInvariantPackContext, ForgeQueryReadInvariantPackViolation,
};
use super::{
    ForgeQueryGenericInspectionIntentTarget, ForgeQueryInspection, ForgeQueryInspectionTarget,
    ForgeQueryInstalledProgram, ForgeQueryProgram, ForgeQueryReadBuilder,
    ForgeQueryReadDomainInvariantDenial, ForgeQueryReadFamily,
    ForgeQueryReadFamilyInvariantEvidence, ForgeQueryReadGraph, ForgeQueryReadResult,
    ForgeQueryRuntimeError, ForgeQueryUnifiedInspectionResult, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn compose_read(
        &mut self,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.compose_read_with_invariant_pack(declaration, |_context| Ok(()))
    }

    pub fn compose_read_with_invariant_pack(
        &mut self,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &ForgeQueryReadInvariantPackContext<'_>,
        ) -> Result<(), ForgeQueryReadInvariantPackViolation>,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = ForgeQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                ForgeQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let family = ForgeQueryReadFamily::new_kernel_only("composed_read", read_graph);
        self.read_family_intent(&family).execute()
    }

    pub fn define_read_family(
        &mut self,
        family_name: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
    ) -> Result<ForgeQueryReadFamily, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        Ok(ForgeQueryReadFamily::new_kernel_only(
            family_name,
            read_graph,
        ))
    }

    pub fn define_read_family_with_invariant_pack(
        &mut self,
        family_name: impl Into<String>,
        invariant_family: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryReadBuilder,
        ) -> Result<ForgeQueryReadGraph, super::ForgeQueryReadDenial>,
        invariant_pack: impl FnOnce(
            &ForgeQueryReadInvariantPackContext<'_>,
        ) -> Result<(), ForgeQueryReadInvariantPackViolation>,
    ) -> Result<ForgeQueryReadFamily, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let read_graph = declaration(ForgeQueryReadBuilder::new())
            .map_err(ForgeQueryRuntimeError::ReadCompositionDenied)?;
        let invariant_context = ForgeQueryReadInvariantPackContext::new(&read_graph);
        invariant_pack(&invariant_context).map_err(|violation| {
            ForgeQueryRuntimeError::ReadCompositionDomainInvariantDenied(
                ForgeQueryReadDomainInvariantDenial::from_violation(violation, &invariant_context),
            )
        })?;
        let invariant_evidence = ForgeQueryReadFamilyInvariantEvidence::new(
            invariant_family,
            invariant_context.read_domain_invariant_summary(),
        );
        Ok(ForgeQueryReadFamily::new_domain_invariant_admitted(
            family_name,
            invariant_evidence,
            read_graph,
        ))
    }

    pub fn execute_read_family(
        &mut self,
        family: &ForgeQueryReadFamily,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        self.read_family_intent(family).execute()
    }

    pub fn execute_read_family_with_access_plan(
        &mut self,
        family: &ForgeQueryReadFamily,
        graph_read_access_plan: super::ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let handoff = self
            .read_family_intent(family)
            .review()?
            .admitted_handoff()
            .ok_or_else(|| {
                ForgeQueryRuntimeError::ReadCompositionDenied(super::ForgeQueryReadDenial::new(
                    super::ForgeQueryReadDenialKind::BasisPreflightDenied,
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
        family: &ForgeQueryReadFamily,
        context: &crate::query_context::AdmittedQueryBasisContext,
        graph_read_access_plan: super::ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        let handoff = self
            .read_family_in_basis_context_intent(family, context)
            .review()?
            .admitted_handoff()
            .ok_or_else(|| {
                ForgeQueryRuntimeError::ReadCompositionDenied(super::ForgeQueryReadDenial::new(
                    super::ForgeQueryReadDenialKind::BasisPreflightDenied,
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
        family: &ForgeQueryReadFamily,
        context: &crate::query_context::AdmittedQueryBasisContext,
    ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(super::ForgeQueryRuntimeFacadeFamily::Read)?;
        self.read_family_in_basis_context_intent(family, context)
            .execute()
    }

    pub fn explain_graph_read_access_shape(
        &self,
        family: &ForgeQueryReadFamily,
    ) -> Result<
        super::ForgeQueryGraphReadAccessShapeExplanation,
        super::ForgeQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_graph_read_access_shape_for_family(family)
    }

    pub fn admit_graph_read_access_authority(
        &self,
        request: super::ForgeQueryGraphReadAccessAuthorityRequest,
    ) -> Result<
        super::ForgeQueryGraphReadAccessAuthorityContext,
        super::ForgeQueryGraphReadAccessAuthorityDenial,
    > {
        super::admit_graph_read_access_authority(request)
    }

    pub fn admit_graph_read_access_authority_from_policy_tenant_request(
        &self,
        request: super::ForgeQueryGraphReadPolicyTenantAuthorityRequest,
    ) -> Result<
        super::ForgeQueryGraphReadAccessAuthorityContext,
        super::ForgeQueryGraphReadAccessAuthorityDenial,
    > {
        super::admit_graph_read_access_authority_from_policy_tenant_request(request)
    }

    pub fn explain_graph_read_access_shape_in_authority(
        &self,
        family: &ForgeQueryReadFamily,
        authority: &super::ForgeQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        super::ForgeQueryGraphReadAccessShapeExplanation,
        super::ForgeQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_graph_read_access_shape_for_family_in_authority(family, authority)
    }

    pub fn admit_graph_read_access_in_authority(
        &self,
        family: &ForgeQueryReadFamily,
        authority: &super::ForgeQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        super::ForgeQueryGraphReadAccessAdmission,
        super::ForgeQueryGraphReadAccessShapeExplanationError,
    > {
        self.admit_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn plan_graph_read_access_in_authority(
        &self,
        family: &ForgeQueryReadFamily,
        authority: &super::ForgeQueryGraphReadAccessAuthorityContext,
    ) -> Result<
        Option<super::ForgeQueryAdmittedGraphReadAccessPlan>,
        super::ForgeQueryGraphReadAccessShapeExplanationError,
    > {
        super::plan_admitted_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn explain_boolean_selectivity_shape(
        &self,
        family: &ForgeQueryReadFamily,
    ) -> Result<
        super::ForgeQueryBooleanSelectivityShape,
        super::ForgeQueryGraphReadAccessShapeExplanationError,
    > {
        super::explain_boolean_selectivity_shape_for_family(family)
    }

    pub fn plan_live_graph_read_access(
        &self,
        family: &ForgeQueryReadFamily,
        budget: super::ForgeQueryLiveGraphReadMaintenanceBudget,
    ) -> Result<super::ForgeQueryLiveGraphReadAccessPlan, super::ForgeQueryLiveGraphReadAccessDenial>
    {
        let one_shot = match super::plan_admitted_graph_read_access_for_family(family) {
            Ok(Some(one_shot)) => one_shot,
            Ok(None) => {
                return Err(super::ForgeQueryLiveGraphReadAccessDenial::new(
                    super::ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport,
                    family.family_digest(),
                    &budget,
                    "live graph read access planning requires an admitted one-shot access plan",
                ));
            }
            Err(error) => {
                return Err(super::ForgeQueryLiveGraphReadAccessDenial::new(
                    super::ForgeQueryLiveGraphReadAccessPosture::DeniedLiveMaintenanceSupport,
                    family.family_digest(),
                    &budget,
                    format!("live graph read access planning could not derive one-shot access: {error:?}"),
                ));
            }
        };
        super::ForgeQueryLiveGraphReadAccessPlan::from_one_shot_access_plan(&one_shot, budget)
    }

    pub fn inspect_intent<'a, T>(
        &'a self,
        target: T,
    ) -> crate::intent_admission::ForgeQueryRuntimeInspectionIntentAuthoring<'a>
    where
        T: ForgeQueryGenericInspectionIntentTarget<'a>,
    {
        self.runtime.inspect_intent(target)
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        self.runtime.install_program(program)
    }

    pub fn inspect<'a, T>(
        &'a self,
        target: T,
    ) -> Result<ForgeQueryInspection, ForgeQueryRuntimeError>
    where
        T: Into<ForgeQueryInspectionTarget<'a>>,
    {
        self.runtime.inspect(target)
    }

    pub fn inspect_live<T>(
        &self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.inspect_live_by_name(view.name())
    }

    pub(crate) fn inspect_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
        self.runtime.inspect_live_view_name_result(view_name)
    }
}
