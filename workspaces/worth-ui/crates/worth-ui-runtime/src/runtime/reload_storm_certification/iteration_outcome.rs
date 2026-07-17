use crate::runtime::{
    WorthUiFileRustReplacementPipelineReport, WorthUiReloadFailure,
    WorthUiReloadStormCandidateDenialReason, WorthUiReloadStormReceiptBinding,
};

#[derive(Debug, PartialEq)]
pub enum WorthUiReloadStormIterationOutcome {
    Activated(Box<WorthUiReloadStormSuccessfulIteration>),
    EquivalentNoOp(WorthUiReloadStormNoOpIteration),
    DeniedPreserved(WorthUiReloadStormDeniedIteration),
}

#[derive(Debug, PartialEq)]
pub struct WorthUiReloadStormSuccessfulIteration {
    label: String,
    binding: WorthUiReloadStormReceiptBinding,
    report: WorthUiFileRustReplacementPipelineReport,
    active_plan_digest_after: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormNoOpIteration {
    label: String,
    binding: WorthUiReloadStormReceiptBinding,
    active_plan_digest: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormDeniedIteration {
    label: String,
    candidate_denial_reason: WorthUiReloadStormCandidateDenialReason,
    failure: WorthUiReloadFailure,
    active_plan_digest_after: u64,
    last_valid_plan_digest_after: u64,
}

impl WorthUiReloadStormSuccessfulIteration {
    pub(crate) fn new(
        label: impl Into<String>,
        binding: WorthUiReloadStormReceiptBinding,
        report: WorthUiFileRustReplacementPipelineReport,
        active_plan_digest_after: u64,
    ) -> Self {
        Self {
            label: label.into(),
            binding,
            report,
            active_plan_digest_after,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn binding(&self) -> WorthUiReloadStormReceiptBinding {
        self.binding
    }

    pub fn report(&self) -> &WorthUiFileRustReplacementPipelineReport {
        &self.report
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }
}

impl WorthUiReloadStormNoOpIteration {
    pub(crate) fn new(
        label: impl Into<String>,
        binding: WorthUiReloadStormReceiptBinding,
        active_plan_digest: u64,
    ) -> Self {
        Self {
            label: label.into(),
            binding,
            active_plan_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn binding(&self) -> WorthUiReloadStormReceiptBinding {
        self.binding
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }
}

impl WorthUiReloadStormDeniedIteration {
    pub(crate) fn new(
        label: impl Into<String>,
        candidate_denial_reason: WorthUiReloadStormCandidateDenialReason,
        failure: WorthUiReloadFailure,
        active_plan_digest_after: u64,
        last_valid_plan_digest_after: u64,
    ) -> Self {
        Self {
            label: label.into(),
            candidate_denial_reason,
            failure,
            active_plan_digest_after,
            last_valid_plan_digest_after,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn failure(&self) -> &WorthUiReloadFailure {
        &self.failure
    }

    pub fn candidate_denial_reason(&self) -> &WorthUiReloadStormCandidateDenialReason {
        &self.candidate_denial_reason
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }

    pub fn last_valid_plan_digest_after(&self) -> u64 {
        self.last_valid_plan_digest_after
    }
}
