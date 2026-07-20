use crate::domain_installation::WorthQueryOperationResultState;
use std::collections::BTreeMap;

use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowEffectEvidence,
    WorthQueryWorkflowInvariantOutcome, WorthQueryWorkflowPrimaryReadEvidence,
    WorthQueryWorkflowStageWarning, WorthQueryWorkflowValue,
};

#[derive(Debug)]
pub struct WorthQueryInstalledWorkflowGraph {
    definition: worth_query_installation::facade::WorthQueryPortableWorkflowDefinition,
    stage_index: BTreeMap<String, usize>,
}

impl WorthQueryInstalledWorkflowGraph {
    pub(crate) fn compile(
        operation: &worth_query_installation::facade::WorthQueryPortableDomainOperationDefinition,
    ) -> Option<Self> {
        match &operation.semantics().workflow {
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(
                definition,
            ) => {
                let definition = definition.clone();
                let stage_index = definition
                    .stages()
                    .iter()
                    .enumerate()
                    .map(|(index, stage)| (stage.identity().to_string(), index))
                    .collect();
                Some(Self {
                    definition,
                    stage_index,
                })
            }
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::NotRequired => {
                None
            }
        }
    }

    pub fn entry_stage(&self) -> &str {
        self.definition.entry_stage()
    }

    pub fn stages(&self) -> &[worth_query_installation::facade::WorthQueryPortableWorkflowStage] {
        self.definition.stages()
    }

    pub(super) fn stage(
        &self,
        identity: &str,
    ) -> Option<&worth_query_installation::facade::WorthQueryPortableWorkflowStage> {
        self.stage_index
            .get(identity)
            .map(|index| &self.definition.stages()[*index])
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryWorkflowRunCounters {
    pub runtime_authority_checks: usize,
    pub stage_index_lookups: usize,
    pub stage_admission_checks: usize,
    pub predecessor_checks: usize,
    pub predecessor_receipt_lookups: usize,
    pub required_capability_checks: usize,
    pub required_domain_checks: usize,
    pub graph_read_contacts: usize,
    pub touch_effect_contacts: usize,
    pub effect_receipt_checks: usize,
    pub commit_admission_contacts: usize,
    pub invariant_checks: usize,
    pub parallel_admission_checks: usize,
    pub stage_executor_contacts: usize,
    pub output_contract_checks: usize,
    pub terminal_contract_checks: usize,
    pub consumption_contacts: usize,
    pub unrelated_run_scans: usize,
    pub conditional_dependency_checks: usize,
    pub conditional_semantic_reads: usize,
    pub conditional_condition_checks: usize,
    pub conditional_comparator_checks: usize,
    pub conditional_compute_contacts: usize,
    pub conditional_semantic_changes: usize,
    pub conditional_reuse_checks: usize,
}

impl WorthQueryWorkflowRunCounters {
    pub(super) fn delta_since(self, before: Self) -> Self {
        Self {
            runtime_authority_checks: self.runtime_authority_checks
                - before.runtime_authority_checks,
            stage_index_lookups: self.stage_index_lookups - before.stage_index_lookups,
            stage_admission_checks: self.stage_admission_checks - before.stage_admission_checks,
            predecessor_checks: self.predecessor_checks - before.predecessor_checks,
            predecessor_receipt_lookups: self.predecessor_receipt_lookups
                - before.predecessor_receipt_lookups,
            required_capability_checks: self.required_capability_checks
                - before.required_capability_checks,
            required_domain_checks: self.required_domain_checks - before.required_domain_checks,
            graph_read_contacts: self.graph_read_contacts - before.graph_read_contacts,
            touch_effect_contacts: self.touch_effect_contacts - before.touch_effect_contacts,
            effect_receipt_checks: self.effect_receipt_checks - before.effect_receipt_checks,
            commit_admission_contacts: self.commit_admission_contacts
                - before.commit_admission_contacts,
            invariant_checks: self.invariant_checks - before.invariant_checks,
            parallel_admission_checks: self.parallel_admission_checks
                - before.parallel_admission_checks,
            stage_executor_contacts: self.stage_executor_contacts - before.stage_executor_contacts,
            output_contract_checks: self.output_contract_checks - before.output_contract_checks,
            terminal_contract_checks: self.terminal_contract_checks
                - before.terminal_contract_checks,
            consumption_contacts: self.consumption_contacts - before.consumption_contacts,
            unrelated_run_scans: self.unrelated_run_scans - before.unrelated_run_scans,
            conditional_dependency_checks: self.conditional_dependency_checks
                - before.conditional_dependency_checks,
            conditional_semantic_reads: self.conditional_semantic_reads
                - before.conditional_semantic_reads,
            conditional_condition_checks: self.conditional_condition_checks
                - before.conditional_condition_checks,
            conditional_comparator_checks: self.conditional_comparator_checks
                - before.conditional_comparator_checks,
            conditional_compute_contacts: self.conditional_compute_contacts
                - before.conditional_compute_contacts,
            conditional_semantic_changes: self.conditional_semantic_changes
                - before.conditional_semantic_changes,
            conditional_reuse_checks: self.conditional_reuse_checks
                - before.conditional_reuse_checks,
        }
    }

    pub(super) fn observed_cost_roles(
        self,
    ) -> Vec<worth_query_installation::facade::WorthQueryWorkflowCostRole> {
        use worth_query_installation::facade::WorthQueryWorkflowCostRole as Role;
        let mut roles = vec![Role::Admission];
        if self.graph_read_contacts > 0 {
            roles.push(Role::GraphRead);
        }
        if self.touch_effect_contacts > 0 {
            roles.push(Role::TouchEffect);
        }
        if self.commit_admission_contacts > 0 {
            roles.push(Role::CommitAdmission);
        }
        if self.effect_receipt_checks > 0 {
            roles.push(Role::Effect);
        }
        if self.invariant_checks > 0 {
            roles.push(Role::Invariant);
        }
        if self.stage_executor_contacts > 0 {
            roles.push(Role::Execution);
        }
        if self.output_contract_checks > 0 || self.terminal_contract_checks > 0 {
            roles.push(Role::ResultValidation);
        }
        roles.sort();
        roles
    }
}

#[derive(Debug)]
pub struct WorthQueryWorkflowStageReceipt {
    pub(super) identity: String,
    pub(super) run_identity: String,
    pub(super) binding_identity: String,
    pub(super) stage_identity: String,
    pub(super) predecessor_stage_identities: Vec<String>,
    pub(super) predecessor_receipt_identities: Vec<String>,
    pub(super) output: WorthQueryWorkflowValue,
    pub(super) result_state: Option<WorthQueryOperationResultState>,
    pub(super) warnings: Vec<WorthQueryWorkflowStageWarning>,
    pub(super) graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    pub(super) primary_read_evidence: Vec<WorthQueryWorkflowPrimaryReadEvidence>,
    pub(super) effect_evidence: Vec<WorthQueryWorkflowEffectEvidence>,
    pub(super) invariant_outcomes: Vec<WorthQueryWorkflowInvariantOutcome>,
    pub(super) parallel_admission:
        Option<std::sync::Arc<super::WorthQueryWorkflowParallelAdmissionReceipt>>,
    pub(super) counters: WorthQueryWorkflowRunCounters,
    pub(super) authority_proof: std::sync::Arc<super::WorthQueryWorkflowRunAuthorityProof>,
    pub(super) stage_authority_proof: std::sync::Arc<WorthQueryWorkflowStageAuthorityProof>,
    pub(super) predecessor_authority_proofs:
        Vec<std::sync::Arc<WorthQueryWorkflowStageAuthorityProof>>,
    pub(super) execution_snapshot: crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) conditional: Vec<crate::domain_installation::WorthQueryConditionalProvenance>,
}

#[derive(Debug)]
pub(super) struct WorthQueryWorkflowStageAuthorityProof {
    pub(super) stage_identity: String,
    pub(super) run_authority: std::sync::Arc<super::WorthQueryWorkflowRunAuthorityProof>,
    pub(super) proof:
        crate::domain_installation::operation_authority_chain::WorthQueryOperationPhaseProof<
            crate::domain_installation::operation_authority_chain::WorthQueryWorkflowStagePhase,
        >,
}

impl WorthQueryWorkflowStageReceipt {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn run_identity(&self) -> &str {
        &self.run_identity
    }
    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }
    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }
    pub fn predecessor_receipt_identities(&self) -> &[String] {
        &self.predecessor_receipt_identities
    }
    pub fn predecessor_stage_identities(&self) -> &[String] {
        &self.predecessor_stage_identities
    }
    pub fn predecessor_proof_count(&self) -> usize {
        self.predecessor_authority_proofs.len()
    }
    pub(crate) fn output(&self) -> &WorthQueryWorkflowValue {
        &self.output
    }
    pub fn result_state(&self) -> Option<WorthQueryOperationResultState> {
        self.result_state
    }
    pub fn warnings(&self) -> &[WorthQueryWorkflowStageWarning] {
        &self.warnings
    }
    pub fn graph_receipts(&self) -> &[WorthQueryBoundGraphExecutionReceipt] {
        &self.graph_receipts
    }
    pub fn primary_read_evidence(&self) -> &[WorthQueryWorkflowPrimaryReadEvidence] {
        &self.primary_read_evidence
    }
    pub fn effect_evidence(&self) -> &[WorthQueryWorkflowEffectEvidence] {
        &self.effect_evidence
    }
    pub fn invariant_outcomes(&self) -> &[WorthQueryWorkflowInvariantOutcome] {
        &self.invariant_outcomes
    }
    pub fn parallel_admission(&self) -> Option<&super::WorthQueryWorkflowParallelAdmissionReceipt> {
        self.parallel_admission.as_deref()
    }
    pub fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.counters
    }
    pub fn installation_generation(
        &self,
    ) -> crate::domain_installation::WorthQueryDomainInstallationGeneration {
        self.authority_proof
            .domain_authority
            .installation_generation()
    }
    pub fn operation_identity(&self) -> &str {
        self.authority_proof.operation_identity()
    }
    pub fn basis_identity(&self) -> &str {
        self.authority_proof.basis_identity()
    }
    pub fn execution_snapshot(&self) -> &crate::memory_workspace::WorthQuerySnapshotIdentity {
        &self.execution_snapshot
    }
    pub fn conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        &self.conditional
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStartDenial {
    StaleInstallationGeneration,
    WorkflowNotDeclared,
    StageExecutorMissing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowAdvanceDenialKind {
    RuntimeAuthority(crate::domain_installation::WorthQueryDomainHandleDenialKind),
    UnknownStage,
    StageAlreadyCompleted,
    PredecessorIncomplete(String),
    PredecessorAuthorityMissing(String),
    RequiredCapability(String),
    RequiredDomain(String),
    InputContract,
    GraphProvider(String),
    StageExecutor {
        class: worth_query_installation::facade::WorthQueryOperationFailureClass,
        detail: String,
    },
    UndeclaredFailureClass(worth_query_installation::facade::WorthQueryOperationFailureClass),
    PrimaryReadEvidence,
    EffectEvidence,
    InvariantEvidence,
    CostContract,
    OutputContract,
    TerminalContract,
    ParallelFrontierShape,
    NonDeterministicLowering,
    ParallelProviderMissing,
    ParallelProvider(String),
    ParallelNotAdmitted(worth_signal::facade::adapters::FrontierRouteSerialFallbackReason),
    ConditionalExecution(worth_runtime_bridge::facade::BridgeConditionalDenialKind),
    ConditionalReentry(crate::domain_installation::WorthQueryConditionalAdmissionDenial),
}

#[derive(Debug)]
pub struct WorthQueryWorkflowAdvanceDenial {
    kind: WorthQueryWorkflowAdvanceDenialKind,
    evidence: Box<WorthQueryWorkflowAdvanceDenialEvidence>,
}

#[derive(Debug)]
struct WorthQueryWorkflowAdvanceDenialEvidence {
    counters: WorthQueryWorkflowRunCounters,
    executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    completed_stage_receipts: Vec<WorthQueryWorkflowStageReceipt>,
}

impl WorthQueryWorkflowAdvanceDenial {
    pub(super) fn new(
        kind: WorthQueryWorkflowAdvanceDenialKind,
        counters: WorthQueryWorkflowRunCounters,
    ) -> Self {
        Self {
            kind,
            evidence: Box::new(WorthQueryWorkflowAdvanceDenialEvidence {
                counters,
                executed_effects: Vec::new(),
                graph_receipts: Vec::new(),
                completed_stage_receipts: Vec::new(),
            }),
        }
    }
    pub(super) fn with_executed_effects(
        kind: WorthQueryWorkflowAdvanceDenialKind,
        counters: WorthQueryWorkflowRunCounters,
        executed_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    ) -> Self {
        Self {
            kind,
            evidence: Box::new(WorthQueryWorkflowAdvanceDenialEvidence {
                counters,
                executed_effects,
                graph_receipts: Vec::new(),
                completed_stage_receipts: Vec::new(),
            }),
        }
    }
    pub fn kind(&self) -> &WorthQueryWorkflowAdvanceDenialKind {
        &self.kind
    }
    pub fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.evidence.counters
    }
    pub fn executed_effects(&self) -> &[WorthQueryWorkflowEffectEvidence] {
        &self.evidence.executed_effects
    }
    pub fn graph_receipts(&self) -> &[WorthQueryBoundGraphExecutionReceipt] {
        &self.evidence.graph_receipts
    }
    pub fn completed_stage_receipts(&self) -> &[WorthQueryWorkflowStageReceipt] {
        &self.evidence.completed_stage_receipts
    }

    pub(super) fn with_graph_receipts(
        mut self,
        graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    ) -> Self {
        self.evidence.graph_receipts = graph_receipts;
        self
    }

    pub(super) fn prepend_executed_effects(
        mut self,
        mut prior_effects: Vec<WorthQueryWorkflowEffectEvidence>,
    ) -> Self {
        prior_effects.append(&mut self.evidence.executed_effects);
        self.evidence.executed_effects = prior_effects;
        self
    }

    pub(super) fn with_completed_stage_receipts(
        mut self,
        receipts: Vec<WorthQueryWorkflowStageReceipt>,
    ) -> Self {
        self.evidence.completed_stage_receipts = receipts;
        self
    }
}
