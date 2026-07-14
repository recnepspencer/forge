use super::WorthQueryWorkflowCounters;
use crate::runtime::WorthQueryRuntimeError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStopSource {
    CrossSession,
    ForeignAuthority,
    StalePreview,
    UnsupportedWriteback,
    InspectionUnavailable,
    LowerRuntime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowViolationKind {
    CrossSession,
    ForeignAuthority,
    StalePreview,
    UnsupportedWriteback,
    InspectionUnavailable,
    LowerRuntime,
}

#[derive(Debug)]
pub struct WorthQueryWorkflowViolation {
    kind: WorthQueryWorkflowViolationKind,
    error: Option<WorthQueryRuntimeError>,
}

impl WorthQueryWorkflowViolation {
    pub fn kind(&self) -> WorthQueryWorkflowViolationKind {
        self.kind
    }

    pub fn runtime_error(&self) -> Option<&WorthQueryRuntimeError> {
        self.error.as_ref()
    }

    fn denied(source: WorthQueryWorkflowStopSource) -> Self {
        Self {
            kind: violation_kind(source),
            error: None,
        }
    }

    fn runtime(error: WorthQueryRuntimeError) -> Self {
        Self {
            kind: WorthQueryWorkflowViolationKind::LowerRuntime,
            error: Some(error),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowNextAction {
    ProvideAuthority,
    RefreshPreview,
    UseMatchingSession,
    RebindAuthoritativeWriteback,
    UseOperationalReceipt,
    InspectRuntimeDenial,
}

pub struct WorthQueryWorkflowStop {
    source: WorthQueryWorkflowStopSource,
    violation: WorthQueryWorkflowViolation,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryWorkflowStop {
    pub fn source(&self) -> WorthQueryWorkflowStopSource {
        self.source
    }

    pub fn violation(&self) -> &WorthQueryWorkflowViolation {
        &self.violation
    }

    pub fn error(&self) -> Option<&WorthQueryRuntimeError> {
        self.violation.runtime_error()
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub fn next_action(&self) -> WorthQueryWorkflowNextAction {
        match self.source {
            WorthQueryWorkflowStopSource::CrossSession => {
                WorthQueryWorkflowNextAction::UseMatchingSession
            }
            WorthQueryWorkflowStopSource::ForeignAuthority => {
                WorthQueryWorkflowNextAction::ProvideAuthority
            }
            WorthQueryWorkflowStopSource::StalePreview => {
                WorthQueryWorkflowNextAction::RefreshPreview
            }
            WorthQueryWorkflowStopSource::UnsupportedWriteback => {
                WorthQueryWorkflowNextAction::RebindAuthoritativeWriteback
            }
            WorthQueryWorkflowStopSource::InspectionUnavailable => {
                WorthQueryWorkflowNextAction::UseOperationalReceipt
            }
            WorthQueryWorkflowStopSource::LowerRuntime => {
                WorthQueryWorkflowNextAction::InspectRuntimeDenial
            }
        }
    }

    pub(crate) fn denied(
        source: WorthQueryWorkflowStopSource,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source,
            violation: WorthQueryWorkflowViolation::denied(source),
            counters,
        }
    }

    pub(crate) fn runtime(
        error: WorthQueryRuntimeError,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source: WorthQueryWorkflowStopSource::LowerRuntime,
            violation: WorthQueryWorkflowViolation::runtime(error),
            counters,
        }
    }

    pub(crate) fn inspection_unavailable(
        error: WorthQueryRuntimeError,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            source: WorthQueryWorkflowStopSource::InspectionUnavailable,
            violation: WorthQueryWorkflowViolation {
                kind: WorthQueryWorkflowViolationKind::InspectionUnavailable,
                error: Some(error),
            },
            counters,
        }
    }
}

fn violation_kind(source: WorthQueryWorkflowStopSource) -> WorthQueryWorkflowViolationKind {
    match source {
        WorthQueryWorkflowStopSource::CrossSession => WorthQueryWorkflowViolationKind::CrossSession,
        WorthQueryWorkflowStopSource::ForeignAuthority => {
            WorthQueryWorkflowViolationKind::ForeignAuthority
        }
        WorthQueryWorkflowStopSource::StalePreview => WorthQueryWorkflowViolationKind::StalePreview,
        WorthQueryWorkflowStopSource::UnsupportedWriteback => {
            WorthQueryWorkflowViolationKind::UnsupportedWriteback
        }
        WorthQueryWorkflowStopSource::InspectionUnavailable => {
            WorthQueryWorkflowViolationKind::InspectionUnavailable
        }
        WorthQueryWorkflowStopSource::LowerRuntime => WorthQueryWorkflowViolationKind::LowerRuntime,
    }
}
