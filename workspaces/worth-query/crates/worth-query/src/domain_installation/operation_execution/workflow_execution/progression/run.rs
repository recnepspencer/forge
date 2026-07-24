use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;
use crate::identity::hash_parts;

use super::{
    WorthQueryAdmittedWorkflowOperation, WorthQueryAdmittedWorkflowResourcePlan,
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
    WorthQueryInstalledWorkflowGraph, WorthQueryInstalledWorkflowParallelAdmissionProvider,
    WorthQueryInstalledWorkflowStageExecutor, WorthQueryWorkflowRunCounters,
    WorthQueryWorkflowStageReceipt, WorthQueryWorkflowStartDenial,
    WorthQueryWorkflowStartDenialKind,
};
use super::{WorthQueryExecutableDomainOperation, WorthQueryWorkflowOperation};
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryOperationPhaseProof,
    WorthQueryWorkflowRunPhase,
};
use crate::domain_installation::WorthQueryBoundDomainOperation;
use worth_proof::TransitionOutcome;

pub type WorthQueryWorkflowStartOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryWorkflowRun<D, O, F, L>,
    WorthQueryWorkflowStartDenial,
    crate::domain_installation::WorthQueryDeferredWorkflowStart<D, O, F, L>,
    WorthQueryWorkflowStartDenial,
    WorthQueryWorkflowStartDenial,
    WorthQueryWorkflowStartDenial,
>;

#[derive(Debug)]
pub(super) struct WorthQueryWorkflowRunAuthorityProof {
    pub(super) domain_authority:
        Arc<crate::domain_installation::WorthQueryInstalledDomainAuthority>,
    pub(super) proof: WorthQueryOperationPhaseProof<WorthQueryWorkflowRunPhase>,
}

impl WorthQueryWorkflowRunAuthorityProof {
    pub(super) fn operation_identity(&self) -> &str {
        &operation_phase_basis(&self.proof).operation_identity
    }

    pub(super) fn binding_identity(&self) -> &str {
        &operation_phase_basis(&self.proof).binding_identity
    }

    pub(super) fn basis_identity(&self) -> &str {
        &operation_phase_basis(&self.proof).basis_identity
    }

    pub(super) fn capability_identity(&self) -> u64 {
        operation_phase_basis(&self.proof).capability_identity
    }
}

pub struct WorthQueryWorkflowRun<D, O, F, L: BasisOperationLane> {
    pub(super) bound: WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(super) graph: Arc<WorthQueryInstalledWorkflowGraph>,
    pub(super) executor: Arc<WorthQueryInstalledWorkflowStageExecutor>,
    pub(super) identity: String,
    pub(super) completed: BTreeSet<String>,
    pub(super) receipt_index: BTreeMap<String, usize>,
    pub(super) receipts: Vec<WorthQueryWorkflowStageReceipt>,
    pub(super) counters: WorthQueryWorkflowRunCounters,
    pub(super) parallel_admission_provider:
        Option<Arc<WorthQueryInstalledWorkflowParallelAdmissionProvider>>,
    pub(super) active_parallel_admission:
        Option<Arc<super::WorthQueryWorkflowParallelAdmissionReceipt>>,
    pub(super) authority_proof: Arc<WorthQueryWorkflowRunAuthorityProof>,
    pub(super) operation_conditional:
        Vec<crate::domain_installation::WorthQueryConditionalProvenance>,
    pub(super) artifact_registry: crate::domain_installation::WorthQueryWorkflowArtifactRegistry,
    pub(super) domain_evidence_ledger: super::WorthQueryDomainEvidenceAdmissionLedger,
    pub(super) resources: WorthQueryAdmittedWorkflowResourcePlan,
    pub(super) provider_session: WorthQueryExecutionProviderSession,
    pub(super) operation_resource_evidence: WorthQueryExecutionResourceAttemptEvidence,
}

struct DeclaredWorkflowRuntime {
    graph: Arc<WorthQueryInstalledWorkflowGraph>,
    executor: Arc<WorthQueryInstalledWorkflowStageExecutor>,
    parallel_admission_provider: Option<Arc<WorthQueryInstalledWorkflowParallelAdmissionProvider>>,
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    WorthQueryAdmittedWorkflowOperation<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
{
    pub fn start_workflow(
        self,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
    ) -> WorthQueryWorkflowStartOutcome<D, O, F, L> {
        self.start_workflow_attempt(workspace, 1)
    }

    pub(super) fn start_workflow_attempt(
        self,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
        attempt: u64,
    ) -> WorthQueryWorkflowStartOutcome<D, O, F, L> {
        let mut counters = WorthQueryWorkflowRunCounters::default();
        let operation_resource_evidence = WorthQueryExecutionResourceAttemptEvidence::capture(
            self.resources.operation(),
            &self.provider_session,
        );
        let declared_runtime = match self.declared_workflow_runtime() {
            Ok(runtime) => runtime,
            Err(kind) => {
                return TransitionOutcome::Denied(WorthQueryWorkflowStartDenial::new(
                    kind, counters,
                ))
            }
        };
        let snapshot = workspace.snapshot_identity();
        let identity = self.workflow_run_identity(&snapshot, attempt);
        let operation_conditional =
            match super::workflow_conditional_start_evaluation::evaluate(
                &self.bound,
                super::workflow_conditional_start_evaluation::ConditionalWorkflowStartEvaluationPass {
                    workspace,
                    snapshot: &snapshot,
                    run_identity: &identity,
                    attempt,
                    run_counters: &mut counters,
                },
            ) {
                Ok(conditional) => conditional,
                Err(super::workflow_conditional_start_evaluation::ConditionalWorkflowStartStop::Deferred(conditional)) => {
                    return TransitionOutcome::Deferred(
                        crate::domain_installation::WorthQueryDeferredWorkflowStart {
                            admitted: self,
                            conditional,
                            counters,
                            run_identity: identity,
                            attempt,
                        },
                    );
                }
                Err(super::workflow_conditional_start_evaluation::ConditionalWorkflowStartStop::Denied(kind)) => {
                    let failed = matches!(
                        kind,
                        WorthQueryWorkflowStartDenialKind::ConditionalExecution(_)
                    );
                    let denial = WorthQueryWorkflowStartDenial::new(kind, counters);
                    return if failed {
                        TransitionOutcome::Failed(denial)
                    } else {
                        TransitionOutcome::Denied(denial)
                    };
                }
            };
        let proof = mint_operation_phase_proof(
            identity.clone(),
            Some(self.phase_proof.payload().identity()),
            operation_phase_basis(&self.phase_proof).clone(),
        );
        let authority_proof = Arc::new(WorthQueryWorkflowRunAuthorityProof {
            domain_authority: Arc::clone(self.bound.operation().domain_authority()),
            proof,
        });
        TransitionOutcome::Success(WorthQueryWorkflowRun {
            bound: self.bound,
            graph: declared_runtime.graph,
            executor: declared_runtime.executor,
            identity: identity.clone(),
            completed: BTreeSet::new(),
            receipt_index: BTreeMap::new(),
            receipts: Vec::new(),
            counters,
            parallel_admission_provider: declared_runtime.parallel_admission_provider,
            active_parallel_admission: None,
            authority_proof,
            operation_conditional,
            artifact_registry: crate::domain_installation::WorthQueryWorkflowArtifactRegistry::new(
                identity,
            ),
            domain_evidence_ledger: super::WorthQueryDomainEvidenceAdmissionLedger::default(),
            resources: self.resources,
            provider_session: self.provider_session,
            operation_resource_evidence,
        })
    }

    fn declared_workflow_runtime(
        &self,
    ) -> Result<DeclaredWorkflowRuntime, WorthQueryWorkflowStartDenialKind> {
        let graph = self
            .bound
            .operation()
            .workflow_graph()
            .cloned()
            .ok_or(WorthQueryWorkflowStartDenialKind::WorkflowNotDeclared)?;
        let executor = self
            .bound
            .workflow_executor()
            .cloned()
            .ok_or(WorthQueryWorkflowStartDenialKind::StageExecutorMissing)?;
        Ok(DeclaredWorkflowRuntime {
            graph,
            executor,
            parallel_admission_provider: self.bound.workflow_parallel_admission_provider().cloned(),
        })
    }

    fn workflow_run_identity(
        &self,
        snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
        attempt: u64,
    ) -> String {
        hash_parts(&[
            "worth_query_installed_workflow_run_v2".into(),
            format!("binding:{}", self.bound.binding_identity()),
            format!("capability:{}", self.bound.capability_identity()),
            format!("operation:{}", self.bound.definition().canonical_identity()),
            format!("snapshot:{}", snapshot.evidence_identity().as_str()),
            format!("attempt:{attempt}"),
            format!("resources:{}", self.resources.identity()),
            format!("provider-session:{}", self.provider_session.identity()),
        ])
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn receipts(&self) -> &[WorthQueryWorkflowStageReceipt] {
        &self.receipts
    }
    pub fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.counters
    }
    pub fn installed_graph(&self) -> &WorthQueryInstalledWorkflowGraph {
        &self.graph
    }
    pub fn operation_conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        &self.operation_conditional
    }
    pub fn resources(&self) -> &WorthQueryAdmittedWorkflowResourcePlan {
        &self.resources
    }
    pub fn provider_session(&self) -> &WorthQueryExecutionProviderSession {
        &self.provider_session
    }
    pub fn operation_resource_evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.operation_resource_evidence
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    crate::domain_installation::WorthQueryDeferredWorkflowStart<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
{
    pub fn retry(
        self,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
    ) -> WorthQueryWorkflowStartOutcome<D, O, F, L> {
        self.admitted
            .start_workflow_attempt(workspace, self.attempt + 1)
    }
}
