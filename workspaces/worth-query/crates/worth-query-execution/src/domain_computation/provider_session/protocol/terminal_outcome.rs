use super::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolCounters,
    WorthQueryProviderSessionRecoveryPosture,
};

#[derive(Debug)]
pub struct WorthQueryClosedProviderSessionDisposition {
    provider_receipt: String,
    counters: WorthQueryProviderSessionProtocolCounters,
    terminal_binding: super::WorthQueryProviderSessionTerminalBinding,
}

#[derive(Debug)]
pub enum WorthQuerySessionCommitOrAbortOutcome {
    Committed(WorthQueryClosedProviderSessionDisposition),
    Aborted(WorthQueryClosedProviderSessionDisposition),
    CommitRecoveryRequired(WorthQueryProviderSessionFailure),
    AbortRecoveryRequired(WorthQueryProviderSessionFailure),
}

impl WorthQuerySessionCommitOrAbortOutcome {
    pub fn recovery_posture(&self) -> WorthQueryProviderSessionRecoveryPosture {
        match self {
            Self::Committed(_) | Self::Aborted(_) => {
                WorthQueryProviderSessionRecoveryPosture::Closed
            }
            Self::CommitRecoveryRequired(_) | Self::AbortRecoveryRequired(_) => {
                WorthQueryProviderSessionRecoveryPosture::RecoveryRequired
            }
        }
    }

    pub fn failure(&self) -> Option<&WorthQueryProviderSessionFailure> {
        match self {
            Self::CommitRecoveryRequired(failure) | Self::AbortRecoveryRequired(failure) => {
                Some(failure)
            }
            Self::Committed(_) | Self::Aborted(_) => None,
        }
    }
}

impl WorthQueryClosedProviderSessionDisposition {
    pub(in crate::domain_computation::provider_session::protocol) fn close(
        provider_receipt: String,
        counters: WorthQueryProviderSessionProtocolCounters,
        terminal_binding: super::WorthQueryProviderSessionTerminalBinding,
    ) -> Self {
        Self {
            provider_receipt,
            counters,
            terminal_binding,
        }
    }

    pub fn provider_receipt(&self) -> &str {
        &self.provider_receipt
    }

    pub fn counters(&self) -> WorthQueryProviderSessionProtocolCounters {
        self.counters
    }

    pub(in crate::domain_computation) const fn terminal_binding(
        &self,
    ) -> &super::WorthQueryProviderSessionTerminalBinding {
        &self.terminal_binding
    }
}
