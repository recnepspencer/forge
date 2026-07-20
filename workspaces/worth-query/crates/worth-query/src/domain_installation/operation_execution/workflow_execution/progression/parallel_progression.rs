use std::collections::BTreeSet;
use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;

use super::workflow_progression::WorthQueryWorkflowAdvanceStep;
use super::{
    WorthQueryWorkflowAdvanceDenial, WorthQueryWorkflowAdvanceDenialKind,
    WorthQueryWorkflowParallelAdmissionCall, WorthQueryWorkflowParallelAdmissionReceipt,
    WorthQueryWorkflowParallelFrontierStage, WorthQueryWorkflowRun, WorthQueryWorkflowValue,
};

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn advance_admitted_frontier(
        mut self,
        stages: impl IntoIterator<Item = (String, WorthQueryWorkflowValue)>,
        workspace: &mut WorthQueryWorkspace,
    ) -> super::workflow_progression::WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        match self.advance_frontier_once(stages, workspace) {
            Ok(WorthQueryWorkflowAdvanceStep::Advanced) => TransitionOutcome::Success(self),
            Ok(WorthQueryWorkflowAdvanceStep::Deferred(conditional)) => {
                TransitionOutcome::Deferred(
                    crate::domain_installation::WorthQueryDeferredWorkflowStage {
                        run: self,
                        conditional,
                    },
                )
            }
            Err(denial) => self.outcome_from_denial(denial),
        }
    }

    fn advance_frontier_once(
        &mut self,
        stages: impl IntoIterator<Item = (String, WorthQueryWorkflowValue)>,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryWorkflowAdvanceStep, WorthQueryWorkflowAdvanceDenial> {
        let mut stages = stages.into_iter().collect::<Vec<_>>();
        stages.sort_by(|left, right| left.0.cmp(&right.0));
        if stages.len() < 2
            || stages
                .windows(2)
                .any(|pair| pair[0].0.as_str() == pair[1].0.as_str())
        {
            return Err(self.denial(WorthQueryWorkflowAdvanceDenialKind::ParallelFrontierShape));
        }
        if !self.bound.definition().semantics().lowering.deterministic {
            return Err(self.denial(WorthQueryWorkflowAdvanceDenialKind::NonDeterministicLowering));
        }

        self.counters.runtime_authority_checks += 1;
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                Arc::clone(self.bound.operation().domain_authority()),
            );
        workspace
            .validate_installed_domain_witness::<D>(&witness)
            .map_err(|denial| {
                self.denial(WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
                    denial.kind(),
                ))
            })?;

        let mut frontier = Vec::with_capacity(stages.len());
        for (stage_identity, input) in &stages {
            self.counters.stage_admission_checks += 1;
            self.counters.stage_index_lookups += 1;
            let stage =
                self.graph.stage(stage_identity).cloned().ok_or_else(|| {
                    self.denial(WorthQueryWorkflowAdvanceDenialKind::UnknownStage)
                })?;
            if self.completed.contains(stage_identity) {
                return Err(self.denial(WorthQueryWorkflowAdvanceDenialKind::StageAlreadyCompleted));
            }
            let mut predecessor_receipt_identities = Vec::new();
            for predecessor in stage.predecessors() {
                self.counters.predecessor_checks += 1;
                if !self.completed.contains(predecessor) {
                    return Err(self.denial(
                        WorthQueryWorkflowAdvanceDenialKind::PredecessorIncomplete(
                            predecessor.clone(),
                        ),
                    ));
                }
                let Some(receipt_identity) = self
                    .receipt_index
                    .get(predecessor)
                    .map(|index| self.receipts[*index].identity())
                else {
                    return Err(self.denial(
                        WorthQueryWorkflowAdvanceDenialKind::PredecessorAuthorityMissing(
                            predecessor.clone(),
                        ),
                    ));
                };
                predecessor_receipt_identities.push(receipt_identity.to_string());
            }
            if !input.satisfies(stage.semantics().input) {
                return Err(self.denial(WorthQueryWorkflowAdvanceDenialKind::InputContract));
            }
            self.validate_frontier_requirements(&stage)?;
            predecessor_receipt_identities.sort();
            frontier.push(WorthQueryWorkflowParallelFrontierStage::new(
                stage_identity.clone(),
                predecessor_receipt_identities,
                stage.semantics().graph_read_roles.clone(),
                stage.semantics().touch_roles.clone(),
                stage.semantics().effect_roles.clone(),
            ));
        }

        let call = WorthQueryWorkflowParallelAdmissionCall::new(
            self.bound.definition().canonical_identity(),
            self.bound.binding_identity(),
            &self.identity,
            self.bound.basis().capability_digest(),
            frontier,
        );
        let provider = self.parallel_admission_provider.as_ref().ok_or_else(|| {
            self.denial(WorthQueryWorkflowAdvanceDenialKind::ParallelProviderMissing)
        })?;
        self.counters.parallel_admission_checks += 1;
        let lower_receipt = provider.admit(&call).map_err(|failure| {
            self.denial(WorthQueryWorkflowAdvanceDenialKind::ParallelProvider(
                failure.detail().into(),
            ))
        })?;
        if let Some(reason) = lower_receipt.serial_fallback_reason() {
            return Err(
                self.denial(WorthQueryWorkflowAdvanceDenialKind::ParallelNotAdmitted(
                    reason,
                )),
            );
        }
        self.active_parallel_admission = Some(Arc::new(
            WorthQueryWorkflowParallelAdmissionReceipt::mint(&call, lower_receipt),
        ));
        for (stage_identity, input) in stages {
            if let WorthQueryWorkflowAdvanceStep::Deferred(conditional) =
                self.advance_once(&stage_identity, input, workspace)?
            {
                self.active_parallel_admission = None;
                return Ok(WorthQueryWorkflowAdvanceStep::Deferred(conditional));
            }
        }
        self.active_parallel_admission = None;
        Ok(WorthQueryWorkflowAdvanceStep::Advanced)
    }

    fn validate_frontier_requirements(
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
                return Err(
                    self.denial(WorthQueryWorkflowAdvanceDenialKind::RequiredCapability(
                        required.as_str().into(),
                    )),
                );
            }
        }
        let bound_roles = self.bound.required_domain_roles().collect::<BTreeSet<_>>();
        for role in &stage.semantics().required_domain_roles {
            self.counters.required_domain_checks += 1;
            if !bound_roles.contains(role.as_str()) {
                return Err(
                    self.denial(WorthQueryWorkflowAdvanceDenialKind::RequiredDomain(
                        role.as_str().into(),
                    )),
                );
            }
        }
        Ok(())
    }

    fn denial(&self, kind: WorthQueryWorkflowAdvanceDenialKind) -> WorthQueryWorkflowAdvanceDenial {
        WorthQueryWorkflowAdvanceDenial::new(kind, self.counters)
    }
}
