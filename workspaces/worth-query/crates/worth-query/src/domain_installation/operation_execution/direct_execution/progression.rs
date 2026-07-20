use crate::basis_lifecycle::BasisOperationLane;
use crate::identity::hash_parts;
use crate::ordinary::read::WorthQueryReadCompletion;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;

use super::bound_graph_execution::invoke_bound_graphs;
use super::{
    WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryBoundExecutionReceipt, WorthQueryBoundGraphExecutionReceipt,
    WorthQueryDerivedPublicationReceipt, WorthQueryExecutableDomainOperation,
    WorthQueryOperationExecutionContext, WorthQueryOperationExecutionCounters,
    WorthQueryOperationExecutionWarning, WorthQueryOperationOutput,
    WorthQueryPublishedDomainOperation, WorthQueryPublishingOperation, WorthQueryTerminalOperation,
};
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryExecutedOperationPhase,
    WorthQueryOperationPhaseProof,
};
use crate::domain_installation::WorthQueryBoundDomainOperation;

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
        let witness =
            crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
                std::sync::Arc::clone(self.operation().domain_authority()),
            );
        if let Err(denial) = workspace.validate_installed_domain_witness::<D>(&witness) {
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
        counters.input_contract_checks += 1;
        if !super::operation_input::input_satisfies_contract(
            &input,
            &self.definition().semantics().parameters,
        ) {
            return TransitionOutcome::Denied(WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::InputContract,
                "operation input does not satisfy the installed parameter contract",
                counters,
            ));
        }
        let execution_snapshot = workspace.snapshot_identity();
        let execution_identity = format!(
            "{}:bound-capability:{}",
            self.binding_identity(),
            self.capability_identity()
        );
        let conditional = match crate::domain_installation::evaluate_bound_conditionals(
            &self,
            workspace,
            &execution_snapshot,
            &execution_identity,
            None,
            None,
            1,
            &mut counters,
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
                    let class = failure.class().clone();
                    let kind = if self
                        .definition()
                        .semantics()
                        .terminal
                        .failure_classes
                        .contains(&class)
                    {
                        WorthQueryBoundExecutionDenialKind::Executor(class)
                    } else {
                        WorthQueryBoundExecutionDenialKind::UndeclaredFailureClass(class)
                    };
                    return TransitionOutcome::Failed(
                        WorthQueryBoundExecutionDenial::new(kind, failure.detail(), counters)
                            .with_graph_receipts(graph_receipts),
                    );
                }
            };
        counters.primary_read_contacts += primary_read_contacts;
        let (output, result_state, warnings) = material.into_parts();
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
        let graph_evidence = graph_receipts
            .iter()
            .map(|receipt| {
                format!(
                    "{}:{:?}:{}:{}",
                    receipt.role(),
                    receipt.kind(),
                    receipt.evidence_identity(),
                    receipt
                        .projection()
                        .map(|projection| projection.receipt().result_digest())
                        .unwrap_or("not-projected")
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let warning_evidence = warnings
            .iter()
            .map(|warning| format!("{warning:?}"))
            .collect::<Vec<_>>()
            .join(",");
        let output_identity = output.operation_output_identity();
        let conditional_evidence = conditional
            .iter()
            .map(|item| item.signal_identity())
            .collect::<Vec<_>>()
            .join(",");
        let identity = hash_parts(&[
            "worth_query_bound_execution_v1".into(),
            format!("binding:{}", self.binding_identity()),
            format!("capability:{}", self.capability_identity()),
            format!(
                "snapshot:{}",
                execution_snapshot.evidence_identity().as_str()
            ),
            format!("result_state:{result_state:?}"),
            format!("warnings:{warning_evidence}"),
            format!("graph_evidence:{graph_evidence}"),
            format!("output:{output_identity}"),
            format!("conditional:{conditional_evidence}"),
        ]);
        let receipt = WorthQueryBoundExecutionReceipt {
            identity,
            binding_identity: self.binding_identity().into(),
            result_state,
            output_identity,
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
}

impl<D, O, F, L: BasisOperationLane>
    WorthQueryExecutedDomainOperation<D, O, F, L, WorthQueryReadCompletion>
where
    O: WorthQueryExecutableDomainOperation<
        D,
        F,
        Output = WorthQueryReadCompletion,
        Publication = WorthQueryPublishingOperation,
    >,
{
    pub fn publish(
        mut self,
    ) -> TransitionOutcome<
        WorthQueryPublishedDomainOperation<D, O, F, L>,
        WorthQueryPublicationDenial,
        std::convert::Infallible,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
        WorthQueryPublicationDenial,
    > {
        if !self.bound.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryPublicationDenial::StaleInstallationGeneration,
            );
        }
        self.counters.publication_checks += 1;
        let canonical = &self.bound.definition().semantics().canonical_query;
        if !self.output.validates_installed_publication(
            canonical,
            self.bound.basis().normalized().family(),
            &self.execution_snapshot,
            self.bound
                .operation()
                .domain_authority()
                .runtime_authority(),
        ) {
            return TransitionOutcome::Denied(
                WorthQueryPublicationDenial::ExecutionMaterialMismatch,
            );
        }
        let identity = hash_parts(&[
            "worth_query_derived_publication_v1".into(),
            format!("execution:{}", self.receipt.identity()),
            format!("query:{}", canonical.query().digest().as_str()),
            format!(
                "result_shape:{}",
                canonical.result_shape().digest().as_str()
            ),
        ]);
        let receipt = WorthQueryDerivedPublicationReceipt {
            identity,
            execution_identity: self.receipt.identity().into(),
        };
        let phase_proof = mint_operation_phase_proof(
            receipt.identity().to_string(),
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        TransitionOutcome::Success(WorthQueryPublishedDomainOperation::mint(
            self,
            receipt,
            phase_proof,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicationDenial {
    StaleInstallationGeneration,
    ExecutionMaterialMismatch,
}
