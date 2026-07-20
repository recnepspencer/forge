use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;
use crate::identity::hash_parts;

use super::{WorthQueryExecutableDomainOperation, WorthQueryWorkflowOperation};
use super::{
    WorthQueryInstalledWorkflowGraph, WorthQueryInstalledWorkflowParallelAdmissionProvider,
    WorthQueryInstalledWorkflowStageExecutor, WorthQueryWorkflowRunCounters,
    WorthQueryWorkflowStageReceipt, WorthQueryWorkflowStartDenial,
};
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryOperationPhaseProof,
    WorthQueryWorkflowRunPhase,
};
use crate::domain_installation::WorthQueryBoundDomainOperation;
use worth_proof::TransitionOutcome;

pub type WorthQueryWorkflowStartOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryWorkflowRun<D, O, F, L>,
    WorthQueryWorkflowStartDenial,
    std::convert::Infallible,
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
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    WorthQueryBoundDomainOperation<D, O, F, L>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation>,
{
    pub fn start_workflow(self) -> WorthQueryWorkflowStartOutcome<D, O, F, L> {
        if !self.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryWorkflowStartDenial::StaleInstallationGeneration,
            );
        }
        let Some(graph) = self.operation().workflow_graph().cloned() else {
            return TransitionOutcome::Denied(WorthQueryWorkflowStartDenial::WorkflowNotDeclared);
        };
        let Some(executor) = self.workflow_executor().cloned() else {
            return TransitionOutcome::Denied(WorthQueryWorkflowStartDenial::StageExecutorMissing);
        };
        let parallel_admission_provider = self.workflow_parallel_admission_provider().cloned();
        let identity = hash_parts(&[
            "worth_query_installed_workflow_run_v1".into(),
            format!("binding:{}", self.binding_identity()),
            format!("capability:{}", self.capability_identity()),
            format!("operation:{}", self.definition().canonical_identity()),
        ]);
        let proof = mint_operation_phase_proof(
            identity.clone(),
            Some(self.authority_proof().payload().identity()),
            operation_phase_basis(self.authority_proof()).clone(),
        );
        let authority_proof = Arc::new(WorthQueryWorkflowRunAuthorityProof {
            domain_authority: Arc::clone(self.operation().domain_authority()),
            proof,
        });
        TransitionOutcome::Success(WorthQueryWorkflowRun {
            bound: self,
            graph,
            executor,
            identity,
            completed: BTreeSet::new(),
            receipt_index: BTreeMap::new(),
            receipts: Vec::new(),
            counters: WorthQueryWorkflowRunCounters::default(),
            parallel_admission_provider,
            active_parallel_admission: None,
            authority_proof,
        })
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
}
