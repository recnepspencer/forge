use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;
mod publication_progression;
mod receipt_identity;
use super::bound_graph_execution::invoke_bound_graphs;
use super::{
    WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryBoundExecutionReceipt, WorthQueryBoundGraphExecutionReceipt,
    WorthQueryExecutableDomainOperation, WorthQueryOperationExecutionContext,
    WorthQueryOperationExecutionCounters, WorthQueryOperationExecutionWarning,
    WorthQueryOperationOutput, WorthQueryTerminalOperation,
};
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryExecutedOperationPhase,
    WorthQueryOperationPhaseProof,
};
use crate::domain_installation::WorthQueryBoundDomainOperation;
pub use publication_progression::WorthQueryPublicationDenial;
use receipt_identity::{direct_execution_receipt_identity, DirectExecutionIdentityInput};
pub struct WorthQueryExecutedDomainOperation<D, O, F, L: BasisOperationLane, Output> {
    pub(super) bound: WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(super) output: Output,
    receipt: WorthQueryBoundExecutionReceipt,
    warnings: Vec<WorthQueryOperationExecutionWarning>,
    pub(super) counters: WorthQueryOperationExecutionCounters,
    graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    execution_snapshot: crate::memory_workspace::WorthQuerySnapshotIdentity,
    phase_proof: WorthQueryOperationPhaseProof<WorthQueryExecutedOperationPhase>,
    conditional: Vec<crate::domain_installation::WorthQueryConditionalProvenance>,
}
pub type WorthQueryBoundExecutionOutcome<D, O, F, L, Output> = TransitionOutcome<
    WorthQueryExecutedDomainOperation<D, O, F, L, Output>,
    WorthQueryBoundExecutionDenial,
    crate::domain_installation::WorthQueryDeferredDomainOperation<D, O, F, L>,
    WorthQueryBoundExecutionDenial,
    WorthQueryBoundExecutionDenial,
    WorthQueryBoundExecutionDenial,
>;
impl<D, O, F, L: BasisOperationLane, Output> WorthQueryExecutedDomainOperation<D, O, F, L, Output> {
    pub fn receipt(&self) -> &WorthQueryBoundExecutionReceipt {
        &self.receipt
    }
    pub fn warnings(&self) -> &[WorthQueryOperationExecutionWarning] {
        &self.warnings
    }
    pub fn counters(&self) -> WorthQueryOperationExecutionCounters {
        self.counters
    }
    pub fn graph_receipts(&self) -> &[WorthQueryBoundGraphExecutionReceipt] {
        &self.graph_receipts
    }
    pub fn conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        &self.conditional
    }
}
impl<D, O, F, L: BasisOperationLane, Output> WorthQueryExecutedDomainOperation<D, O, F, L, Output>
where
    O: WorthQueryExecutableDomainOperation<
        D,
        F,
        Output = Output,
        Publication = WorthQueryTerminalOperation,
        Execution = super::WorthQueryDirectOperation,
    >,
{
    pub fn output(&self) -> &Output {
        &self.output
    }
}
impl<D: 'static, O, F: 'static, L: BasisOperationLane> WorthQueryBoundDomainOperation<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = super::WorthQueryDirectOperation>,
{
    pub fn execute(
        self,
        input: O::Input,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryBoundExecutionOutcome<D, O, F, L, O::Output> {
        let mut counters = WorthQueryOperationExecutionCounters {
            runtime_authority_checks: 1,
            ..Default::default()
        };
        if let Err(denial) = self.admit_direct_runtime_authority(workspace) {
            let kind = denial.kind();
            let stop = WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::RuntimeAuthority(kind),
                format!("{denial:?}"),
                counters,
            );
            return match kind {
                crate::domain_installation::WorthQueryDomainHandleDenialKind::StaleInstallationGeneration => {
                    TransitionOutcome::Stale(stop)
                }
                crate::domain_installation::WorthQueryDomainHandleDenialKind::PackageIdentityChanged => {
                    TransitionOutcome::RebindRequired(stop)
                }
                crate::domain_installation::WorthQueryDomainHandleDenialKind::DomainNotInstalled
                | crate::domain_installation::WorthQueryDomainHandleDenialKind::ForeignRuntime => {
                    TransitionOutcome::Denied(stop)
                }
            };
        }
        if let Err(denial) = self.admit_direct_execution_contract(&input, &mut counters) {
            return TransitionOutcome::Denied(denial);
        }
        let execution_snapshot = workspace.snapshot_identity();
        let execution_identity = format!(
            "{}:bound-capability:{}",
            self.binding_identity(),
            self.capability_identity()
        );
        let conditional = match crate::domain_installation::evaluate_bound_conditionals(
            &self,
            crate::domain_installation::WorthQueryConditionalEvaluationPass {
                workspace,
                snapshot: &execution_snapshot,
                execution_identity: &execution_identity,
                scope: crate::domain_installation::WorthQueryConditionalEvaluationScope::Operation,
                workflow_run_identity: None,
                attempt: 1,
                counters: &mut counters,
            },
        ) {
            Ok(conditional) => conditional,
            Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Deferred(
                conditional,
            )) => {
                return TransitionOutcome::Deferred(
                    crate::domain_installation::WorthQueryDeferredDomainOperation {
                        bound: self,
                        conditional,
                        counters,
                    },
                )
            }
            Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Failed {
                kind,
                detail,
            }) => {
                return TransitionOutcome::Failed(WorthQueryBoundExecutionDenial::new(
                    WorthQueryBoundExecutionDenialKind::ConditionalExecution(kind),
                    detail,
                    counters,
                ))
            }
            Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Reentry(
                denial,
            )) => {
                return TransitionOutcome::Denied(WorthQueryBoundExecutionDenial::new(
                    WorthQueryBoundExecutionDenialKind::ConditionalReentry(denial),
                    "Signal decision did not re-enter the exact bound Query operation",
                    counters,
                ))
            }
        };
        let graph_receipts = match invoke_bound_graphs(&self, &execution_snapshot, &mut counters) {
            Ok(receipts) => receipts,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let Some(executor) = self.executor().cloned() else {
            return TransitionOutcome::Failed(
                WorthQueryBoundExecutionDenial::new(
                    WorthQueryBoundExecutionDenialKind::ExecutorRegistrationMissing,
                    "the bound direct operation no longer retains its admitted executor",
                    counters,
                )
                .with_graph_receipts(graph_receipts),
            );
        };
        let context = WorthQueryOperationExecutionContext::new(
            self.definition(),
            self.binding_identity(),
            self.basis().capability_digest(),
            self.basis().normalized(),
            executor.installed_read.as_ref(),
            &graph_receipts,
        );
        counters.executor_contacts += 1;
        let (material, primary_read_contacts) =
            match executor.execute::<D, O, F>(input, &context, workspace) {
                Ok(material) => material,
                Err(failure) => {
                    let kind = self.classify_executor_failure(failure.class().clone());
                    return TransitionOutcome::Failed(
                        WorthQueryBoundExecutionDenial::new(kind, failure.detail(), counters)
                            .with_graph_receipts(graph_receipts),
                    );
                }
            };
        counters.primary_read_contacts += primary_read_contacts;
        let (output, result_state, warnings, domain_evidence_material) = material.into_parts();
        counters.terminal_posture_checks += 1;
        if !self
            .definition()
            .semantics()
            .terminal
            .result_states
            .contains(&result_state)
        {
            return TransitionOutcome::Denied(
                WorthQueryBoundExecutionDenial::new(
                    WorthQueryBoundExecutionDenialKind::UndeclaredResultState,
                    "executor returned a result state absent from the installed terminal contract",
                    counters,
                )
                .with_graph_receipts(graph_receipts),
            );
        }
        let output_identity = output.operation_output_identity();
        let domain_evidence =
            match super::admit_domain_evidence(super::WorthQueryDomainEvidenceAdmissionInput {
                contract: self
                    .operation()
                    .evidence_contract()
                    .map(std::sync::Arc::as_ref),
                material: domain_evidence_material,
                binding: super::WorthQueryDomainEvidenceBindingParts {
                    operation_identity: self.definition().canonical_identity().to_owned(),
                    binding_identity: self.binding_identity().to_owned(),
                    run_identity: None,
                    stage_identity: None,
                    basis_identity: self.basis().capability_digest().to_owned(),
                    execution_snapshot_identity: execution_snapshot
                        .evidence_identity()
                        .as_str()
                        .to_owned(),
                    output_occurrence_identity: output_identity.clone(),
                },
                ledger: None,
            }) {
                Ok(evidence) => evidence,
                Err(denial) => {
                    return TransitionOutcome::Denied(
                        WorthQueryBoundExecutionDenial::new(
                            WorthQueryBoundExecutionDenialKind::DomainEvidence(denial.kind()),
                            denial.subject(),
                            counters,
                        )
                        .with_graph_receipts(graph_receipts),
                    )
                }
            };
        let identity = direct_execution_receipt_identity(DirectExecutionIdentityInput {
            binding_identity: self.binding_identity(),
            capability_identity: self.capability_identity(),
            execution_snapshot: &execution_snapshot,
            result_state,
            warnings: &warnings,
            graph_receipts: &graph_receipts,
            output_identity: &output_identity,
            conditional: &conditional,
            domain_evidence: domain_evidence.as_ref(),
        });
        let receipt = WorthQueryBoundExecutionReceipt {
            identity,
            binding_identity: self.binding_identity().into(),
            result_state,
            output_identity,
            domain_evidence,
        };
        let phase_proof = mint_operation_phase_proof(
            receipt.identity().to_string(),
            Some(self.authority_proof().payload().identity()),
            operation_phase_basis(self.authority_proof()).clone(),
        );
        TransitionOutcome::Success(WorthQueryExecutedDomainOperation {
            bound: self,
            output,
            receipt,
            warnings,
            counters,
            graph_receipts,
            execution_snapshot,
            phase_proof,
            conditional,
        })
    }

    fn admit_direct_runtime_authority(
        &self,
        workspace: &WorthQueryWorkspace,
    ) -> Result<(), crate::domain_installation::WorthQueryDomainHandleDenial> {
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.operation().domain_authority()),
            );
        workspace.validate_installed_domain_witness::<D>(&witness)
    }

    fn admit_direct_execution_contract(
        &self,
        input: &O::Input,
        counters: &mut WorthQueryOperationExecutionCounters,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        counters.input_contract_checks += 1;
        if !super::operation_input::input_satisfies_contract(
            input,
            &self.definition().semantics().parameters,
        ) {
            return Err(WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::InputContract,
                "operation input does not satisfy the installed parameter contract",
                *counters,
            ));
        }
        let semantics = self.definition().semantics();
        if direct_graph_evidence_can_realize(semantics)
            && matches!(
                semantics.invariants,
                crate::domain_installation::WorthQueryOperationInvariantContract::NotRequired
            )
            && matches!(
                semantics.lineage,
                crate::domain_installation::WorthQueryOperationLineageContract::NotRequired
            )
        {
            return Ok(());
        }
        Err(WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::WorkflowEvidenceRequired,
            "direct execution lacks an admitted evidence route for the declared effects, invariants, or lineage",
            *counters,
        ))
    }

    fn classify_executor_failure(
        &self,
        class: crate::domain_installation::WorthQueryOperationFailureClass,
    ) -> WorthQueryBoundExecutionDenialKind {
        if self
            .definition()
            .semantics()
            .terminal
            .failure_classes
            .contains(&class)
        {
            WorthQueryBoundExecutionDenialKind::Executor(class)
        } else {
            WorthQueryBoundExecutionDenialKind::UndeclaredFailureClass(class)
        }
    }
}

fn direct_graph_evidence_can_realize(
    semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
) -> bool {
    use crate::domain_installation::{
        WorthQueryOperationEffectContract as Effects,
        WorthQueryOperationEffectFamily as EffectFamily,
        WorthQueryOperationTouchContract as Touches,
    };

    match (&semantics.touches, &semantics.effects) {
        (Touches::NotRequired, Effects::NotRequired) => true,
        (Touches::Declared { graph_roles, .. }, Effects::Declared { effect_families }) => {
            !graph_roles.is_empty()
                && !effect_families.is_empty()
                && effect_families
                    .iter()
                    .all(|family| *family == EffectFamily::Mutation)
        }
        (Touches::Declared { graph_roles, .. }, Effects::NotRequired) => !graph_roles.is_empty(),
        (Touches::NotRequired, Effects::Declared { .. }) => false,
    }
}
