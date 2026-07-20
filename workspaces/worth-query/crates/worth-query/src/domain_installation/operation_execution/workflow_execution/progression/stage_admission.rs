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
        self.counters.runtime_authority_checks += 1;
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.bound.operation().domain_authority()),
            );
        workspace
            .validate_installed_domain_witness::<D>(&witness)
            .map_err(|denial| {
                WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(denial.kind()),
                    self.counters,
                )
            })?;
        self.counters.stage_admission_checks += 1;
        self.counters.stage_index_lookups += 1;
        let stage = self.graph.stage(stage_identity).cloned().ok_or_else(|| {
            WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::UnknownStage,
                self.counters,
            )
        })?;
        if self.completed.contains(stage_identity) {
            return Err(WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::StageAlreadyCompleted,
                self.counters,
            ));
        }
        for predecessor in stage.predecessors() {
            self.counters.predecessor_checks += 1;
            if !self.completed.contains(predecessor) {
                return Err(WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::PredecessorIncomplete(predecessor.clone()),
                    self.counters,
                ));
            }
        }
        if !input.satisfies(stage.semantics().input) {
            return Err(WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::InputContract,
                self.counters,
            ));
        }
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
                return Err(WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::RequiredCapability(
                        required.as_str().into(),
                    ),
                    self.counters,
                ));
            }
        }
        for role in &stage.semantics().required_domain_roles {
            self.counters.required_domain_checks += 1;
            if !self
                .bound
                .required_domain_roles()
                .any(|bound| bound == role.as_str())
            {
                return Err(WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::RequiredDomain(role.as_str().into()),
                    self.counters,
                ));
            }
        }
        Ok(stage)
    }
}
