use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::workflow_graph_execution::invoke_stage_graphs;
use super::workflow_stage_receipt::WorthQueryAdmittedWorkflowStageEvidence;
use super::{
    WorthQueryWorkflowAdvanceDenial, WorthQueryWorkflowAdvanceDenialKind,
    WorthQueryWorkflowInvariantOutcome, WorthQueryWorkflowRun,
    WorthQueryWorkflowStageExecutionAuthority, WorthQueryWorkflowStageExecutionContext,
    WorthQueryWorkflowValue,
};
use crate::domain_installation::WorthQueryOperationGraphParticipation;
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

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn advance(
        mut self,
        stage_identity: &str,
        input: WorthQueryWorkflowValue,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        match self.advance_once(stage_identity, input, workspace) {
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
        self,
        denial: WorthQueryWorkflowAdvanceDenial,
    ) -> WorthQueryWorkflowAdvanceOutcome<D, O, F, L> {
        let stale = matches!(
            denial.kind(),
            WorthQueryWorkflowAdvanceDenialKind::RuntimeAuthority(
                crate::domain_installation::WorthQueryDomainHandleDenialKind::StaleInstallationGeneration
            )
        );
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
                | WorthQueryWorkflowAdvanceDenialKind::ConditionalExecution(_)
        );
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
        let counters_before = self.counters;
        let stage = self.admit_stage(stage_identity, &input, workspace)?;
        let semantic_input = input.semantic_value();
        let graph_snapshot = workspace.snapshot_identity();
        let conditional = match super::workflow_conditional_stage_evaluation::evaluate(
            &self.bound,
            workspace,
            &graph_snapshot,
            stage_identity,
            &self.identity,
            self.receipts.len() as u64 + 1,
            &mut self.counters,
        ) {
            Ok(conditional) => conditional,
            Err(super::workflow_conditional_stage_evaluation::ConditionalStageStop::Deferred(
                conditional,
            )) => {
                return Ok(WorthQueryWorkflowAdvanceStep::Deferred(conditional));
            }
            Err(super::workflow_conditional_stage_evaluation::ConditionalStageStop::Denied(
                kind,
            )) => {
                return Err(WorthQueryWorkflowAdvanceDenial::new(kind, self.counters));
            }
        };
        let graph_receipts = invoke_stage_graphs(
            &self.bound,
            &self.identity,
            &stage,
            &graph_snapshot,
            &mut self.counters,
        )?;
        let mut predecessor_receipts = Vec::with_capacity(stage.predecessors().len());
        for predecessor in stage.predecessors() {
            self.counters.predecessor_receipt_lookups += 1;
            let Some(receipt) = self
                .receipt_index
                .get(predecessor)
                .map(|index| &self.receipts[*index])
            else {
                return Err(WorthQueryWorkflowAdvanceDenial::new(
                    WorthQueryWorkflowAdvanceDenialKind::PredecessorAuthorityMissing(
                        predecessor.clone(),
                    ),
                    self.counters,
                ));
            };
            predecessor_receipts.push(receipt);
        }
        debug_assert!(predecessor_receipts.iter().all(|receipt| {
            std::sync::Arc::ptr_eq(&receipt.authority_proof, &self.authority_proof)
                && std::sync::Arc::ptr_eq(
                    &receipt.stage_authority_proof.run_authority,
                    &self.authority_proof,
                )
                && receipt.stage_authority_proof.proof.payload().identity() == receipt.identity
                && receipt.stage_authority_proof.stage_identity == receipt.stage_identity
                && receipt.authority_proof.binding_identity() == self.bound.binding_identity()
                && receipt.authority_proof.capability_identity() == self.bound.capability_identity()
        }));
        let effect_binding_scope = format!(
            "{}:{}:{}",
            self.bound.binding_identity(),
            self.identity,
            stage_identity
        );
        let effect_workflow_binding =
            crate::workflow::synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
                self.bound.definition().canonical_identity(),
                &effect_binding_scope,
                workspace.snapshot_identity(),
            );
        let context = WorthQueryWorkflowStageExecutionContext::new(
            self.bound.definition().canonical_identity(),
            &self.identity,
            &stage,
            &predecessor_receipts,
            WorthQueryWorkflowStageExecutionAuthority {
                effect_workflow_binding,
                basis: self.bound.basis().normalized().family(),
                installed_read: self.executor.installed_read.as_ref(),
                operation_graph_reads: self.bound.definition().semantics().graph_reads.roles(),
                graph_receipts: &graph_receipts,
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
            },
        );
        self.counters.stage_executor_contacts += 1;
        let material = self
            .executor
            .execute(input, &context, workspace)
            .map_err(|failure| {
                let executed_effects = failure.executed_effects().to_vec();
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
                    executed_effects,
                )
                .with_graph_receipts(graph_receipts.clone())
            })?;
        let material = material.into_parts();
        let output = material.output;
        let warnings = material.warnings;
        let result_state = material.result_state;
        let mut primary_read_evidence = material.primary_graph_reads;
        let mut effect_evidence = material.effects;
        let executed_effects = material.executed_effects;
        let lineage = material.lineage;
        if !super::workflow_lineage_validation::valid_stage_lineage(
            &lineage,
            self.bound.definition().semantics().lineage,
            &conditional,
            self.bound.definition().canonical_identity(),
            &self.identity,
            stage_identity,
            &effect_evidence,
        ) {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::LineageEvidence,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        primary_read_evidence.sort_by(|left, right| left.role().cmp(right.role()));
        let mut expected_primary_reads = self
            .bound
            .definition()
            .semantics()
            .graph_reads
            .roles()
            .iter()
            .filter(|read| {
                stage.semantics().graph_read_roles.contains(&read.role)
                    && read.participation
                        == WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
            })
            .map(|read| read.role.as_str())
            .collect::<Vec<_>>();
        expected_primary_reads.sort();
        let evidence_roles = primary_read_evidence
            .iter()
            .map(|evidence| evidence.role())
            .collect::<Vec<_>>();
        let valid_primary_reads = evidence_roles == expected_primary_reads
            && primary_read_evidence.iter().all(|evidence| {
                evidence.validates(
                    &self.bound.definition().semantics().canonical_query,
                    self.bound.basis().normalized().family(),
                    &workspace.snapshot_identity(),
                    self.bound
                        .operation()
                        .domain_authority()
                        .runtime_authority(),
                )
            });
        if !valid_primary_reads {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::PrimaryReadEvidence,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        self.counters.graph_read_contacts += primary_read_evidence.len();
        effect_evidence.sort_by_key(|effect| effect.family());
        let mut expected_effects = stage.semantics().effect_roles.clone();
        expected_effects.sort();
        let evidence_effects = effect_evidence
            .iter()
            .map(|effect| effect.family())
            .collect::<Vec<_>>();
        let valid_effects = evidence_effects == expected_effects
            && effect_evidence.iter().all(|effect| {
                effect.binds_workflow(
                    context.effect_workflow_binding(),
                    self.bound.basis().normalized().family(),
                )
            });
        if !valid_effects {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::EffectEvidence,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        self.counters.effect_receipt_checks += effect_evidence.len();
        if !stage.semantics().invariant_roles.is_empty() && effect_evidence.is_empty() {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::InvariantEvidence,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        let invariant_outcomes = stage
            .semantics()
            .invariant_roles
            .iter()
            .map(|role| {
                let installed_identity = self
                    .bound
                    .operation()
                    .domain_authority()
                    .installed_invariant_identity(role)
                    .ok_or_else(|| {
                        WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                            WorthQueryWorkflowAdvanceDenialKind::InvariantEvidence,
                            self.counters,
                            executed_effects.clone(),
                        )
                        .with_graph_receipts(graph_receipts.clone())
                    })?;
                Ok(WorthQueryWorkflowInvariantOutcome::from_query_commits(
                    role,
                    installed_identity,
                    &effect_evidence,
                ))
            })
            .collect::<Result<Vec<_>, WorthQueryWorkflowAdvanceDenial>>()?;
        self.counters.invariant_checks += invariant_outcomes.len();
        self.counters.output_contract_checks += 1;
        if !output.satisfies(stage.semantics().output) {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::OutputContract,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        self.counters.terminal_contract_checks += 1;
        if stage.is_terminal() != result_state.is_some()
            || result_state
                .is_some_and(|state| !stage.semantics().terminal_result_states.contains(&state))
        {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::TerminalContract,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        let stage_counters = self.counters.delta_since(counters_before);
        let mut declared_cost_roles = stage.semantics().cost_roles.clone();
        declared_cost_roles.sort();
        if declared_cost_roles != stage_counters.observed_cost_roles() {
            return Err(WorthQueryWorkflowAdvanceDenial::with_executed_effects(
                WorthQueryWorkflowAdvanceDenialKind::CostContract,
                self.counters,
                executed_effects,
            )
            .with_graph_receipts(graph_receipts));
        }
        let predecessor_receipt_identities = predecessor_receipts
            .iter()
            .map(|receipt| receipt.identity().to_string())
            .collect::<Vec<_>>();
        self.retain_admitted_stage(
            &stage,
            predecessor_receipt_identities,
            WorthQueryAdmittedWorkflowStageEvidence {
                input: semantic_input,
                output,
                result_state,
                warnings,
                graph_receipts,
                primary_read_evidence,
                effect_evidence,
                invariant_outcomes,
                counters: stage_counters,
                execution_snapshot: workspace.snapshot_identity(),
                conditional,
                lineage,
            },
        );
        Ok(WorthQueryWorkflowAdvanceStep::Advanced)
    }
}
