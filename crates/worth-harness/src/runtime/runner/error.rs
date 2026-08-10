use std::fmt;

use crate::capture::{DiagnosticsLevel, ExecutionMode};
use crate::comparison::ComparisonMode;
use crate::timeline::{ClockDomain, ExecutionPhase};

use super::super::capability::CaptureDepth;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessError<AdapterError> {
    UnsupportedExecutionMode(ExecutionMode),
    UnsupportedDiagnosticsLevel(DiagnosticsLevel),
    UnsupportedCaptureDepth(CaptureDepth),
    UnsupportedComparisonMode(ComparisonMode),
    UnsupportedClockDomain(ClockDomain),
    UnsupportedExecutionPhase(ExecutionPhase),
    UnsupportedWorkBudget,
    UnsupportedReplay,
    Adapter(AdapterError),
}

impl<AdapterError: fmt::Display> fmt::Display for HarnessError<AdapterError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedExecutionMode(mode) => {
                write!(f, "unsupported execution mode: {mode:?}")
            }
            Self::UnsupportedDiagnosticsLevel(level) => {
                write!(f, "unsupported diagnostics level: {level:?}")
            }
            Self::UnsupportedCaptureDepth(depth) => {
                write!(f, "unsupported capture depth: {depth:?}")
            }
            Self::UnsupportedComparisonMode(mode) => {
                write!(f, "unsupported comparison mode: {mode:?}")
            }
            Self::UnsupportedClockDomain(domain) => {
                write!(f, "unsupported clock domain: {domain:?}")
            }
            Self::UnsupportedExecutionPhase(phase) => {
                write!(f, "unsupported execution phase: {phase:?}")
            }
            Self::UnsupportedWorkBudget => write!(f, "unsupported work budget"),
            Self::UnsupportedReplay => write!(f, "unsupported replay"),
            Self::Adapter(error) => write!(f, "{error}"),
        }
    }
}

impl<AdapterError: fmt::Debug + fmt::Display> std::error::Error for HarnessError<AdapterError> {}
