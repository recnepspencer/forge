use worth_query_installation::facade::WorthQueryDomainHandleDenialKind;

use super::WorthQueryExecutionResourceAdmissionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryExecutionResourceAdmissionDenialKind {
    RuntimeAuthority(WorthQueryDomainHandleDenialKind),
    InputContract,
    DirectExecutionContract,
    ResourceContract,
    ExecutorSupportMissing,
    DifferentProviderRequired,
    DifferentAccessProductRequired,
    DifferentAllocatorRequired,
    ExecutionModeUnsupported,
    CancellationSafePointUnsupported,
    DegradationPostureUnsupported,
    AsyncExecutionRequired,
    Backpressured,
    ResourceCeilingExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceAdmissionDenial {
    kind: WorthQueryExecutionResourceAdmissionDenialKind,
    detail: String,
    counters: WorthQueryExecutionResourceAdmissionCounters,
}

impl WorthQueryExecutionResourceAdmissionDenial {
    pub fn new(
        kind: WorthQueryExecutionResourceAdmissionDenialKind,
        detail: impl Into<String>,
        counters: WorthQueryExecutionResourceAdmissionCounters,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters,
        }
    }

    pub fn kind(&self) -> &WorthQueryExecutionResourceAdmissionDenialKind {
        &self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> WorthQueryExecutionResourceAdmissionCounters {
        self.counters
    }
}
