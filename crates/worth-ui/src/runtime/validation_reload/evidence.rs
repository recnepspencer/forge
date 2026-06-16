#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiValidationReloadStage {
    EmptyRequest,
    SourceIngress,
    CandidateSubmission,
    CandidateAdmission,
    ArtifactComparison,
    ImpactClassification,
    ImpactNarrowing,
    IdentityMatching,
    NodeReplacement,
    StateInventory,
    DurableStateReconciliation,
    QueryBindingComparison,
    QueryLiveRebind,
    ActivationStaging,
    PlanLowering,
    HandleAllocation,
    TopologyAssembly,
    ReadyActivation,
    RuntimeInstanceMismatch,
    MissingReadyActivation,
    PlanSwap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiValidationReloadStatus {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied(WorthUiValidationReloadStage),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiValidationReloadEvidence {
    runtime_instance_witness: u64,
    status: WorthUiValidationReloadStatus,
    denial_detail: Option<String>,
    active_artifact_digest_before: u64,
    active_artifact_digest_after: u64,
    active_plan_digest_before: u64,
    active_plan_digest_after: u64,
    source_revision_digest: Option<u64>,
    ordering_receipt_digest: Option<u64>,
    candidate_artifact_digest: Option<u64>,
    candidate_plan_digest: Option<u64>,
    raw_events_observed: usize,
    events_coalesced: usize,
    provider_reads: usize,
    source_revisions_emitted: usize,
    candidate_submissions_emitted: usize,
    frame_path_work: usize,
    active_runtime_mutations_before_activation: usize,
    query_bindings_compared: usize,
    query_rebind_entries: usize,
    durable_state_reconciliation_receipts: usize,
    query_binding_planning_ran: bool,
    durable_state_planning_ran: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiValidationReloadEvidenceBuilder {
    evidence: WorthUiValidationReloadEvidence,
}

impl WorthUiValidationReloadEvidence {
    pub(crate) fn builder(
        runtime_instance_witness: u64,
        active_artifact_digest_before: u64,
        active_plan_digest_before: u64,
    ) -> WorthUiValidationReloadEvidenceBuilder {
        WorthUiValidationReloadEvidenceBuilder {
            evidence: Self {
                runtime_instance_witness,
                status: WorthUiValidationReloadStatus::Denied(
                    WorthUiValidationReloadStage::SourceIngress,
                ),
                denial_detail: None,
                active_artifact_digest_before,
                active_artifact_digest_after: active_artifact_digest_before,
                active_plan_digest_before,
                active_plan_digest_after: active_plan_digest_before,
                source_revision_digest: None,
                ordering_receipt_digest: None,
                candidate_artifact_digest: None,
                candidate_plan_digest: None,
                raw_events_observed: 0,
                events_coalesced: 0,
                provider_reads: 0,
                source_revisions_emitted: 0,
                candidate_submissions_emitted: 0,
                frame_path_work: 0,
                active_runtime_mutations_before_activation: 0,
                query_bindings_compared: 0,
                query_rebind_entries: 0,
                durable_state_reconciliation_receipts: 0,
                query_binding_planning_ran: false,
                durable_state_planning_ran: false,
            },
        }
    }

    pub fn status(&self) -> WorthUiValidationReloadStatus {
        self.status
    }

    pub fn runtime_instance_witness(&self) -> u64 {
        self.runtime_instance_witness
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    pub fn active_artifact_digest_before(&self) -> u64 {
        self.active_artifact_digest_before
    }

    pub fn active_artifact_digest_after(&self) -> u64 {
        self.active_artifact_digest_after
    }

    pub fn active_plan_digest_before(&self) -> u64 {
        self.active_plan_digest_before
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }

    pub fn source_revision_digest(&self) -> Option<u64> {
        self.source_revision_digest
    }

    pub fn ordering_receipt_digest(&self) -> Option<u64> {
        self.ordering_receipt_digest
    }

    pub fn candidate_artifact_digest(&self) -> Option<u64> {
        self.candidate_artifact_digest
    }

    pub fn candidate_plan_digest(&self) -> Option<u64> {
        self.candidate_plan_digest
    }

    pub fn raw_events_observed(&self) -> usize {
        self.raw_events_observed
    }

    pub fn events_coalesced(&self) -> usize {
        self.events_coalesced
    }

    pub fn provider_reads(&self) -> usize {
        self.provider_reads
    }

    pub fn source_revisions_emitted(&self) -> usize {
        self.source_revisions_emitted
    }

    pub fn candidate_submissions_emitted(&self) -> usize {
        self.candidate_submissions_emitted
    }

    pub fn frame_path_work(&self) -> usize {
        self.frame_path_work
    }

    pub fn active_runtime_mutations_before_activation(&self) -> usize {
        self.active_runtime_mutations_before_activation
    }

    pub fn query_bindings_compared(&self) -> usize {
        self.query_bindings_compared
    }

    pub fn query_rebind_entries(&self) -> usize {
        self.query_rebind_entries
    }

    pub fn durable_state_reconciliation_receipts(&self) -> usize {
        self.durable_state_reconciliation_receipts
    }

    pub fn query_binding_planning_ran(&self) -> bool {
        self.query_binding_planning_ran
    }

    pub fn durable_state_planning_ran(&self) -> bool {
        self.durable_state_planning_ran
    }

    pub(crate) fn mark_activated(
        mut self,
        active_artifact_digest_after: u64,
        active_plan_digest_after: u64,
    ) -> Self {
        self.status = WorthUiValidationReloadStatus::Activated;
        self.active_artifact_digest_after = active_artifact_digest_after;
        self.active_plan_digest_after = active_plan_digest_after;
        self
    }
}

impl WorthUiValidationReloadEvidenceBuilder {
    pub(crate) fn record_source_ingress(
        mut self,
        source_revision_digest: u64,
        ordering_receipt_digest: u64,
        counters: crate::runtime::WorthUiSourceIngressCounters,
    ) -> Self {
        self.evidence.source_revision_digest = Some(source_revision_digest);
        self.evidence.ordering_receipt_digest = Some(ordering_receipt_digest);
        self.evidence.raw_events_observed = counters.raw_events_observed();
        self.evidence.events_coalesced = counters.events_coalesced();
        self.evidence.provider_reads = counters.provider_reads();
        self.evidence.source_revisions_emitted = counters.source_revisions_emitted();
        self.evidence.frame_path_work = counters.frame_path_work();
        self.evidence.active_runtime_mutations_before_activation =
            counters.active_runtime_mutations();
        self
    }

    pub(crate) fn record_candidate_submission(
        mut self,
        counters: crate::runtime::WorthUiSourceIngressCounters,
    ) -> Self {
        self.evidence.candidate_submissions_emitted = counters.candidate_submissions_emitted();
        self.evidence.frame_path_work = counters.frame_path_work();
        self.evidence.active_runtime_mutations_before_activation =
            counters.active_runtime_mutations();
        self
    }

    pub(crate) fn record_candidate_artifact(mut self, digest: u64) -> Self {
        self.evidence.candidate_artifact_digest = Some(digest);
        self
    }

    pub(crate) fn record_query_and_state_planning(
        mut self,
        query_bindings_compared: usize,
        query_rebind_entries: usize,
        durable_state_reconciliation_receipts: usize,
    ) -> Self {
        self.evidence.query_bindings_compared = query_bindings_compared;
        self.evidence.query_rebind_entries = query_rebind_entries;
        self.evidence.durable_state_reconciliation_receipts = durable_state_reconciliation_receipts;
        self.evidence.query_binding_planning_ran = true;
        self.evidence.durable_state_planning_ran = true;
        self
    }

    pub(crate) fn record_candidate_plan(mut self, digest: u64) -> Self {
        self.evidence.candidate_plan_digest = Some(digest);
        self
    }

    pub(crate) fn finish(
        mut self,
        status: WorthUiValidationReloadStatus,
        active_artifact_digest_after: u64,
        active_plan_digest_after: u64,
    ) -> WorthUiValidationReloadEvidence {
        self.evidence.status = status;
        self.evidence.active_artifact_digest_after = active_artifact_digest_after;
        self.evidence.active_plan_digest_after = active_plan_digest_after;
        self.evidence
    }

    pub(crate) fn finish_denied(
        mut self,
        stage: WorthUiValidationReloadStage,
        detail: impl Into<String>,
        active_artifact_digest_after: u64,
        active_plan_digest_after: u64,
    ) -> WorthUiValidationReloadEvidence {
        self.evidence.status = WorthUiValidationReloadStatus::Denied(stage);
        self.evidence.denial_detail = Some(detail.into());
        self.evidence.active_artifact_digest_after = active_artifact_digest_after;
        self.evidence.active_plan_digest_after = active_plan_digest_after;
        self.evidence
    }
}
