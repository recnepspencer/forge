use super::{WorthQueryProviderSessionProtocolCounters, WorthQueryProviderSessionProtocolStage};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProviderSessionControlStopKind {
    Cancelled,
    TimedOut,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderSessionCommitControlStopped {
    kind: WorthQueryProviderSessionControlStopKind,
    stage: WorthQueryProviderSessionProtocolStage,
    detail: String,
    counters: WorthQueryProviderSessionProtocolCounters,
}

impl WorthQueryProviderSessionCommitControlStopped {
    pub(in crate::domain_computation) fn new(
        kind: WorthQueryProviderSessionControlStopKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            stage: WorthQueryProviderSessionProtocolStage::Commit,
            detail: detail.into(),
            counters: WorthQueryProviderSessionProtocolCounters::default(),
        }
    }

    pub const fn kind(&self) -> WorthQueryProviderSessionControlStopKind {
        self.kind
    }

    pub const fn stage(&self) -> WorthQueryProviderSessionProtocolStage {
        self.stage
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }

    pub(in crate::domain_computation) fn at_stage(
        mut self,
        stage: WorthQueryProviderSessionProtocolStage,
        counters: WorthQueryProviderSessionProtocolCounters,
    ) -> Self {
        self.stage = stage;
        self.counters = counters;
        self
    }
}
