#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryExecutionResourceAdmissionDenialKind {
    RuntimeAuthority(crate::domain_installation::WorthQueryDomainHandleDenialKind),
    InputContract,
    ResourceContract,
    ExecutorSupportMissing,
    DifferentProviderRequired,
    DifferentAccessProductRequired,
    DifferentAllocatorRequired,
    AsyncExecutionRequired,
    Backpressured,
    ResourceCeilingExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceAdmissionDenial {
    kind: WorthQueryExecutionResourceAdmissionDenialKind,
    detail: String,
    counters: super::WorthQueryExecutionResourceAdmissionCounters,
}

impl WorthQueryExecutionResourceAdmissionDenial {
    pub(crate) fn new(
        kind: WorthQueryExecutionResourceAdmissionDenialKind,
        detail: impl Into<String>,
        counters: super::WorthQueryExecutionResourceAdmissionCounters,
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

    pub fn counters(&self) -> super::WorthQueryExecutionResourceAdmissionCounters {
        self.counters
    }
}
