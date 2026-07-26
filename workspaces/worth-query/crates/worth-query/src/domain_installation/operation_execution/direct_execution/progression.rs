use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;
mod publication_progression;
mod receipt_identity;
use super::bound_graph_execution::invoke_bound_graphs;
use super::{
    WorthQueryAdmittedDirectOperation, WorthQueryAdmittedExecutionResourcePlan,
    WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryBoundExecutionReceipt, WorthQueryBoundGraphExecutionReceipt,
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutableDomainOperation,
    WorthQueryExecutionProviderSession, WorthQueryOperationExecutionContext,
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
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
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
    pub fn resources(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        self.resource_attempt.resources()
    }
    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        self.resource_attempt.provider_session()
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
impl<D: 'static, O, F: 'static, L: BasisOperationLane> WorthQueryAdmittedDirectOperation<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = super::WorthQueryDirectOperation>,
{
    pub fn execute(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryBoundExecutionOutcome<D, O, F, L, O::Output> {
        let mut counters = WorthQueryOperationExecutionCounters {
            runtime_authority_checks: 1,
            ..Default::default()
        };
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.bound.operation().domain_authority()),
            );
        if let Err(denial) = workspace.validate_installed_domain_witness::<D>(&witness) {
            return TransitionOutcome::Stale(WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::RuntimeAuthority(denial.kind()),
                format!("{denial:?}"),
                counters,
            ));
        }
        let resource_evidence = self.resource_attempt.evidence().clone();
        let execution_snapshot = workspace.snapshot_identity();
        let execution_identity = format!(
            "{}:bound-capability:{}",
            self.bound.binding_identity(),
            self.bound.capability_identity()
        );
        let conditional = match crate::domain_installation::evaluate_bound_conditionals(
            &self.bound,
            crate::domain_installation::WorthQueryConditionalEvaluationPass {
                workspace,
                snapshot: &execution_snapshot,
                execution_identity: &execution_identity,
                scope: crate::domain_installation::WorthQueryConditionalEvaluationScope::Operation,
                workflow_run_identity: None,
                attempt: 1,
                resources: self.resource_attempt.resources(),
                resource_evidence: &resource_evidence,
                counters: &mut counters,
            },
        ) {
            Ok(conditional) => conditional,
            Err(crate::domain_installation::WorthQueryConditionalEvaluationStop::Deferred(
                conditional,
            )) => {
                return TransitionOutcome::Deferred(
                    crate::domain_installation::WorthQueryDeferredDomainOperation {
                        admitted: self,
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
        let graph_receipts = match invoke_bound_graphs(
            &self.bound,
            self.resource_attempt.resources(),
            &resource_evidence,
            self.resource_attempt.provider_session(),
            &execution_snapshot,
            &mut counters,
        ) {
            Ok(receipts) => receipts,
            Err(denial) => return TransitionOutcome::Denied(denial),
        };
        let context = WorthQueryOperationExecutionContext::new(
            self.bound.definition(),
            self.bound.binding_identity(),
            self.bound.basis().capability_digest(),
            self.bound.basis().normalized(),
            self.executor.installed_read.as_ref(),
            &graph_receipts,
            self.resource_attempt.resources(),
            self.resource_attempt.provider_session(),
        );
        counters.executor_contacts += 1;
        let (material, primary_read_contacts) = match self
            .executor
            .execute::<D, O, F>(self.input, &context, workspace)
        {
            Ok(material) => material,
            Err(failure) => {
                let kind = classify_executor_failure(&self.bound, failure.class().clone());
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
            .bound
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
        let evidence_binding = match self
            .resource_attempt
            .provider_session()
            .bind_direct_domain_evidence(
                execution_snapshot.evidence_identity().as_str(),
                &output_identity,
            ) {
            Ok(binding) => binding,
            Err(denial) => {
                return TransitionOutcome::Denied(
                    WorthQueryBoundExecutionDenial::new(
                        WorthQueryBoundExecutionDenialKind::DomainEvidenceBinding(denial),
                        "execution attempt could not bind domain evidence",
                        counters,
                    )
                    .with_graph_receipts(graph_receipts),
                )
            }
        };
        let domain_evidence =
            match super::admit_domain_evidence(super::WorthQueryDomainEvidenceAdmissionInput {
                material: domain_evidence_material,
                binding: evidence_binding,
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
            binding_identity: self.bound.binding_identity(),
            capability_identity: self.bound.capability_identity(),
            execution_snapshot: &execution_snapshot,
            result_state,
            warnings: &warnings,
            graph_receipts: &graph_receipts,
            output_identity: &output_identity,
            conditional: &conditional,
            domain_evidence: domain_evidence.as_ref(),
            execution_resources: &resource_evidence,
        });
        let receipt = WorthQueryBoundExecutionReceipt {
            identity,
            binding_identity: self.bound.binding_identity().into(),
            result_state,
            output_identity,
            domain_evidence,
            execution_resources: resource_evidence,
        };
        let phase_proof = mint_operation_phase_proof(
            receipt.identity().to_string(),
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        TransitionOutcome::Success(WorthQueryExecutedDomainOperation {
            bound: self.bound,
            output,
            receipt,
            warnings,
            counters,
            graph_receipts,
            execution_snapshot,
            phase_proof,
            conditional,
            resource_attempt: self.resource_attempt,
        })
    }
}

fn classify_executor_failure<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    class: crate::domain_installation::WorthQueryOperationFailureClass,
) -> WorthQueryBoundExecutionDenialKind {
    if bound
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
