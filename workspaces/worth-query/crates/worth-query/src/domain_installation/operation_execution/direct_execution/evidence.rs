use crate::domain_installation::{WorthQueryGraphProviderCallKind, WorthQueryOperationResultState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryOperationExecutionCounters {
    pub runtime_authority_checks: usize,
    pub input_contract_checks: usize,
    pub graph_provider_contacts: usize,
    pub primary_read_contacts: usize,
    pub executor_contacts: usize,
    pub terminal_posture_checks: usize,
    pub publication_checks: usize,
    pub consumption_contacts: usize,
    pub conditional_dependency_checks: usize,
    pub conditional_semantic_reads: usize,
    pub conditional_condition_checks: usize,
    pub conditional_comparator_checks: usize,
    pub conditional_compute_contacts: usize,
    pub conditional_semantic_changes: usize,
    pub conditional_reuse_checks: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundExecutionReceipt {
    pub(super) identity: String,
    pub(super) binding_identity: String,
    pub(super) result_state: WorthQueryOperationResultState,
    pub(super) output_identity: String,
}

impl WorthQueryBoundExecutionReceipt {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }
    pub fn result_state(&self) -> WorthQueryOperationResultState {
        self.result_state
    }
    pub fn output_identity(&self) -> &str {
        &self.output_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryBoundGraphExecutionReceipt {
    pub(super) role: String,
    pub(super) kind: WorthQueryGraphProviderCallKind,
    pub(super) provider_receipt: String,
    pub(super) evidence_identity: String,
    pub(super) projection: Option<Box<crate::runtime::WorthQueryReadResult>>,
}

impl WorthQueryBoundGraphExecutionReceipt {
    pub fn role(&self) -> &str {
        &self.role
    }
    pub fn kind(&self) -> WorthQueryGraphProviderCallKind {
        self.kind
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub(crate) fn projection(&self) -> Option<&crate::runtime::WorthQueryReadResult> {
        self.projection.as_deref()
    }
    pub fn has_projection_material(&self) -> bool {
        self.projection.is_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedPublicationReceipt {
    pub(super) identity: String,
    pub(super) execution_identity: String,
}

impl WorthQueryDerivedPublicationReceipt {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn execution_identity(&self) -> &str {
        &self.execution_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundExecutionDenialKind {
    RuntimeAuthority(crate::domain_installation::WorthQueryDomainHandleDenialKind),
    InputContract,
    GraphProvider,
    ExecutorRegistrationMissing,
    Executor(crate::domain_installation::WorthQueryOperationFailureClass),
    UndeclaredFailureClass(crate::domain_installation::WorthQueryOperationFailureClass),
    UndeclaredResultState,
    ConditionalExecution(worth_runtime_bridge::facade::BridgeConditionalDenialKind),
    ConditionalReentry(crate::domain_installation::WorthQueryConditionalAdmissionDenial),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryBoundExecutionDenial {
    kind: WorthQueryBoundExecutionDenialKind,
    detail: String,
    evidence: Box<WorthQueryBoundExecutionDenialEvidence>,
}

#[derive(Clone, Debug, PartialEq)]
struct WorthQueryBoundExecutionDenialEvidence {
    counters: WorthQueryOperationExecutionCounters,
    graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
}

impl WorthQueryBoundExecutionDenial {
    pub(super) fn new(
        kind: WorthQueryBoundExecutionDenialKind,
        detail: impl Into<String>,
        counters: WorthQueryOperationExecutionCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            evidence: Box::new(WorthQueryBoundExecutionDenialEvidence {
                counters,
                graph_receipts: Vec::new(),
            }),
        }
    }
    pub(super) fn with_graph_receipts(
        mut self,
        graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    ) -> Self {
        self.evidence.graph_receipts = graph_receipts;
        self
    }
    pub fn kind(&self) -> &WorthQueryBoundExecutionDenialKind {
        &self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
    pub fn counters(&self) -> WorthQueryOperationExecutionCounters {
        self.evidence.counters
    }
    pub fn graph_receipts(&self) -> &[WorthQueryBoundGraphExecutionReceipt] {
        &self.evidence.graph_receipts
    }
}
