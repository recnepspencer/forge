use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::workflow_graph_execution::invoke_stage_graphs;
use super::workflow_stage_evidence_validation::WorthQueryWorkflowStageValidationInput;
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenialKind, WorthQueryWorkflowRun,
    WorthQueryWorkflowStageExecutionAuthority, WorthQueryWorkflowStageExecutionContext,
    WorthQueryWorkflowStageExecutionScope, WorthQueryWorkflowStageMaterialParts,
    WorthQueryWorkflowStageReceipt, WorthQueryWorkflowStageRuntimeAdmission,
    WorthQueryWorkflowValue,
};
use worth_proof::TransitionOutcome;

pub type WorthQueryWorkflowAdvanceOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryWorkflowRun<D, O, F, L>,
    WorthQueryWorkflowAdvanceDenial,
    crate::domain_installation::WorthQueryDeferredWorkflowStage<D, O, F, L>,
    WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenial,
>;

pub(super) enum WorthQueryWorkflowAdvanceStep {
    Advanced,
    Deferred(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
}

enum WorthQueryStageConditionAdmission {
    Admitted(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
    Deferred(Vec<crate::domain_installation::WorthQueryConditionalProvenance>),
}

struct WorthQueryExecutedWorkflowStage {
    predecessor_receipt_identities: Vec<String>,
    material: WorthQueryWorkflowStageMaterialParts,
    effect_workflow_binding: crate::workflow::WorkflowContextBinding,
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn advance(
        mut self,
        stage_identity: &str,
        input: WorthQueryWorkflowValue,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        let runtime_admission = match self.admit_stage_runtime_authority(workspace) {
            Ok(admission) => admission,
            Err(denial) => return self.outcome_from_denial(denial),
        };
        self.advance_with_runtime_admission(stage_identity, input, workspace, runtime_admission)
    }

    pub(super) fn advance_with_runtime_admission(
        mut self,
        stage_identity: &str,
        input: WorthQueryWorkflowValue,
        workspace: &mut WorthQueryWorkspace,
        runtime_admission: WorthQueryWorkflowStageRuntimeAdmission,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        match self.advance_once_with_runtime_admission(
            stage_identity,
            input,
            workspace,
            runtime_admission,
        ) {
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

    pub(super) fn outcome_from_denial(
        mut self,
        denial: WorthQueryWorkflowAdvanceDenial,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        let stale = match denial.kind() {
            WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
                crate::domain_installation::WorthQueryDomainHandleDenialKind::StaleInstallationGeneration,
            ) => true,
            WorthQueryWorkflowAdvanceDenialKind::ArtifactCarriage(artifact) => {
                artifact.kind()
                    == crate::domain_installation::WorthQueryArtifactDenialKind::StaleInstallationGeneration
            }
            _ => false,
        };
        let rebind = matches!(
            denial.kind(),
            WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
                crate::domain_installation::WorthQueryDomainHandleDenialKind::PackageIdentityChanged
            )
        );
        let failed = matches!(
            denial.kind(),
            WorthQueryWorkflowAdvanceDenialKind::StageExecutor { .. }
                | WorthQueryWorkflowAdvanceDenialKind::UndeclaredFailureClass(_)
                | WorthQueryWorkflowAdvanceDenialKind::PredecessorAuthorityMissing(_)
                | WorthQueryWorkflowAdvanceDenialKind::ResourceAdmissionMissing
                | WorthQueryWorkflowAdvanceDenialKind::ConditionalExecution(_)
        );
        for receipt in self.receipts.iter_mut().rev() {
            receipt.cancel_artifact_output();
        }
        let completed_effects = self
            .receipts
            .iter()
            .flat_map(|receipt| receipt.effect_evidence().iter().cloned())
            .collect();
        let stop = denial
            .prepend_executed_effects(completed_effects)
            .with_completed_stage_receipts(self.receipts);
        if stale {
            TransitionOutcome::Stale(stop)
        } else if rebind {
            TransitionOutcome::RebindRequired(stop)
        } else if failed {
            TransitionOutcome::Failed(stop)
        } else {
            TransitionOutcome::Denied(stop)
        }
    }

    pub(super) fn advance_once(
        &mut self,
        stage_identity: &str,
        input: WorthQueryWorkflowValue,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryWorkflowAdvanceStep, WorthQueryWorkflowAdvanceDenial> {
        let runtime_admission = self.admit_stage_runtime_authority(workspace)?;
        self.advance_once_with_runtime_admission(
            stage_identity,
            input,
            workspace,
            runtime_admission,
        )
    }

    fn advance_once_with_runtime_admission(
        &mut self,
        stage_identity: &str,
        input: WorthQueryWorkflowValue,
        workspace: &mut WorthQueryWorkspace,
        runtime_admission: WorthQueryWorkflowStageRuntimeAdmission,
    ) -> Result<WorthQueryWorkflowAdvanceStep, WorthQueryWorkflowAdvanceDenial> {
        let counters_before = self.counters;
        let stage = self.admit_stage(stage_identity, &input, runtime_admission)?;
        let resources = self.resources.shared_stage(stage_identity).ok_or_else(|| {
            WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::ResourceAdmissionMissing,
                self.counters,
            )
        })?;
        let resource_evidence = super::WorthQueryExecutionResourceAttemptEvidence::capture(
            &resources,
            &self.provider_session,
        );
        let semantic_input = input.semantic_value();
        let graph_snapshot = workspace.snapshot_identity();
        let conditional = match self.admit_stage_condition(
            stage_identity,
            &resources,
            &resource_evidence,
            &graph_snapshot,
            workspace,
        )? {
            WorthQueryStageConditionAdmission::Admitted(conditional) => conditional,
            WorthQueryStageConditionAdmission::Deferred(conditional) => {
                return Ok(WorthQueryWorkflowAdvanceStep::Deferred(conditional));
            }
        };
        let graph_receipts = invoke_stage_graphs(
            &self.bound,
            &self.identity,
            &stage,
            &resources,
            &resource_evidence,
            &graph_snapshot,
            &mut self.counters,
        )?;
        let executed = self.execute_admitted_stage(
            &stage,
            &resources,
            &resource_evidence,
            input,
            &graph_receipts,
            workspace,
        )?;
        let evidence = self.validate_stage_evidence(
            &stage,
            WorthQueryWorkflowStageValidationInput {
                semantic_input,
                material: executed.material,
                graph_receipts,
                conditional,
                execution_snapshot: workspace.snapshot_identity(),
                effect_workflow_binding: executed.effect_workflow_binding,
                counters_before,
                resource_evidence,
            },
        )?;
        self.retain_admitted_stage(&stage, executed.predecessor_receipt_identities, evidence);
        Ok(WorthQueryWorkflowAdvanceStep::Advanced)
    }

    fn admit_stage_condition(
        &mut self,
        stage_identity: &str,
        resources: &super::WorthQueryAdmittedExecutionResourcePlan,
        resource_evidence: &super::WorthQueryExecutionResourceAttemptEvidence,
        graph_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryStageConditionAdmission, WorthQueryWorkflowAdvanceDenial> {
        match super::workflow_conditional_stage_evaluation::evaluate(
            &self.bound,
            workspace,
            graph_snapshot,
            stage_identity,
            &self.identity,
            self.receipts.len() as u64 + 1,
            resources,
            resource_evidence,
            &mut self.counters,
        ) {
            Ok(conditional) => Ok(WorthQueryStageConditionAdmission::Admitted(conditional)),
            Err(super::workflow_conditional_stage_evaluation::ConditionalStageStop::Deferred(
                conditional,
            )) => Ok(WorthQueryStageConditionAdmission::Deferred(conditional)),
            Err(super::workflow_conditional_stage_evaluation::ConditionalStageStop::Denied(
                kind,
            )) => Err(WorthQueryWorkflowAdvanceDenial::new(kind, self.counters)),
        }
    }

    fn execute_admitted_stage(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        resources: &super::WorthQueryAdmittedExecutionResourcePlan,
        resource_evidence: &super::WorthQueryExecutionResourceAttemptEvidence,
        input: WorthQueryWorkflowValue,
        graph_receipts: &[WorthQueryBoundGraphExecutionReceipt],
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryExecutedWorkflowStage, WorthQueryWorkflowAdvanceDenial> {
        let predecessor_indices = self.predecessor_receipt_indices(stage)?;
        let predecessor_receipts = predecessor_indices
            .iter()
            .map(|index| &self.receipts[*index])
            .collect::<Vec<_>>();
        self.assert_predecessor_authority(&predecessor_receipts);
        self.counters.stage_executor_contacts += 1;
        let effect_workflow_binding =
            self.stage_effect_workflow_binding(stage, workspace.snapshot_identity());
        let context = self.stage_execution_context(
            stage,
            &predecessor_receipts,
            graph_receipts,
            resources,
            resource_evidence,
            effect_workflow_binding.clone(),
        );
        let material = self
            .executor
            .execute(input, &context, &self.artifact_registry, workspace)
            .map_err(|failure| {
                let class = failure.class().clone();
                let kind = if stage.semantics().failure_classes.contains(&class) {
                    WorthQueryWorkflowAdvanceDenialKind::StageExecutor {
                        class,
                        detail: failure.detail().into(),
                    }
                } else {
                    WorthQueryWorkflowAdvanceDenialKind::UndeclaredFailureClass(class)
                };
                WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                    kind,
                    self.counters,
                    failure.executed_effects().to_vec(),
                )
                .with_graph_receipts(graph_receipts.to_vec())
            })?;
        Ok(WorthQueryExecutedWorkflowStage {
            predecessor_receipt_identities: predecessor_receipts
                .iter()
                .map(|receipt| receipt.identity().to_string())
                .collect(),
            material: material.into_parts(),
            effect_workflow_binding,
        })
    }

    fn stage_effect_workflow_binding(
        &self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        snapshot: crate::memory_workspace::WorthQuerySnapshotIdentity,
    ) -> crate::workflow::WorkflowContextBinding {
        let effect_binding_scope = format!(
            "{}:{}:{}",
            self.bound.binding_identity(),
            self.identity,
            stage.identity()
        );
        crate::workflow::synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
            self.bound.definition().canonical_identity(),
            &effect_binding_scope,
            snapshot,
        )
    }

    fn stage_execution_context<'a>(
        &'a self,
        stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        predecessor_receipts: &'a [&'a WorthQueryWorkflowStageReceipt],
        graph_receipts: &'a [WorthQueryBoundGraphExecutionReceipt],
        resources: &'a super::WorthQueryAdmittedExecutionResourcePlan,
        resource_evidence: &'a super::WorthQueryExecutionResourceAttemptEvidence,
        effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    ) -> WorthQueryWorkflowStageExecutionContext<'a> {
        WorthQueryWorkflowStageExecutionContext::new(
            WorthQueryWorkflowStageExecutionScope {
                operation_identity: self.bound.definition().canonical_identity(),
                binding_identity: self.bound.binding_identity(),
                run_identity: &self.identity,
                stage,
                predecessor_receipts,
            },
            WorthQueryWorkflowStageExecutionAuthority {
                effect_workflow_binding,
                basis: self.bound.basis().normalized().family(),
                installed_read: self.executor.installed_read.as_ref(),
                operation_graph_reads: self.bound.definition().semantics().graph_reads.roles(),
                graph_receipts,
                resources,
                resource_evidence,
                provider_session: &self.provider_session,
                query_authority: self
                    .bound
                    .definition()
                    .semantics()
                    .canonical_query
                    .query()
                    .authority(),
                identity_evolution_basis_identity: self
                    .bound
                    .basis()
                    .capability_digest()
                    .to_owned(),
                domain_authority: std::sync::Arc::clone(self.bound.operation().domain_authority()),
                output_artifact_contract: self
                    .graph
                    .artifact_contracts(stage.identity())
                    .and_then(super::WorthQueryInstalledWorkflowArtifactContracts::output)
                    .map(std::sync::Arc::clone),
                input_artifact_contract: self
                    .graph
                    .artifact_contracts(stage.identity())
                    .and_then(super::WorthQueryInstalledWorkflowArtifactContracts::input)
                    .map(std::sync::Arc::clone),
            },
        )
    }
}
