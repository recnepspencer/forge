use crate::basis_lifecycle::BasisOperationLane;

use super::{
    WorthQueryWorkflowAdvanceDenial, WorthQueryWorkflowAdvanceDenialKind, WorthQueryWorkflowRun,
    WorthQueryWorkflowValue,
};

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub(super) fn admit_stage(
        &mut self,
        stage_identity: &str,
        input: &WorthQueryWorkflowValue,
        workspace: &crate::runtime::WorthQueryWorkspace,
    ) -> Result<
        worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        WorthQueryWorkflowAdvanceDenial,
    > {
        self.validate_stage_runtime_authority(workspace)?;
        let stage = self.pending_stage(stage_identity)?;
        self.validate_stage_predecessors(&stage)?;
        if !input.satisfies(&stage.semantics().input) {
            return Err(
                self.stage_admission_denial(WorthQueryWorkflowAdvanceDenialKind::InputContract)
            );
        }
        self.validate_stage_capabilities(&stage)?;
        self.validate_stage_domains(&stage)?;
        Ok(stage)
    }

    fn validate_stage_runtime_authority(
        &mut self,
        workspace: &crate::runtime::WorthQueryWorkspace,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        self.counters.runtime_authority_checks += 1;
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.bound.operation().domain_authority()),
            );
        workspace
            .validate_installed_domain_witness::<D>(&witness)
            .map_err(|denial| {
                self.stage_admission_denial(WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
                    denial.kind(),
                ))
            })
    }

    fn pending_stage(
        &mut self,
        stage_identity: &str,
    ) -> Result<
        worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        WorthQueryWorkflowAdvanceDenial,
    > {
        self.counters.stage_admission_checks += 1;
        self.counters.stage_index_lookups += 1;
        let stage = self.graph.stage(stage_identity).cloned().ok_or_else(|| {
            self.stage_admission_denial(WorthQueryWorkflowAdvanceDenialKind::UnknownStage)
        })?;
        if self.completed.contains(stage_identity) {
            return Err(self.stage_admission_denial(
                WorthQueryWorkflowAdvanceDenialKind::StageAlreadyCompleted,
            ));
        }
        Ok(stage)
    }

    fn validate_stage_predecessors(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        for predecessor in stage.predecessors() {
            self.counters.predecessor_checks += 1;
            if !self.completed.contains(predecessor) {
                return Err(self.stage_admission_denial(
                    WorthQueryWorkflowAdvanceDenialKind::PredecessorIncomplete(predecessor.clone()),
                ));
            }
        }
        Ok(())
    }

    fn validate_stage_capabilities(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        for required in stage.required_capabilities() {
            self.counters.required_capability_checks += 1;
            if !self
                .bound
                .operation()
                .domain_authority()
                .required_capabilities()
                .iter()
                .any(|installed| installed.satisfies_operation_requirement(*required))
            {
                return Err(self.stage_admission_denial(
                    WorthQueryWorkflowAdvanceDenialKind::RequiredCapability(
                        required.as_str().into(),
                    ),
                ));
            }
        }
        Ok(())
    }

    fn validate_stage_domains(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        for role in &stage.semantics().required_domain_roles {
            self.counters.required_domain_checks += 1;
            if !self
                .bound
                .required_domain_roles()
                .any(|bound| bound == role.as_str())
            {
                return Err(self.stage_admission_denial(
                    WorthQueryWorkflowAdvanceDenialKind::RequiredDomain(role.as_str().into()),
                ));
            }
        }
        Ok(())
    }

    fn stage_admission_denial(
        &self,
        kind: WorthQueryWorkflowAdvanceDenialKind,
    ) -> WorthQueryWorkflowAdvanceDenial {
        WorthQueryWorkflowAdvanceDenial::new(kind, self.counters)
    }
}
