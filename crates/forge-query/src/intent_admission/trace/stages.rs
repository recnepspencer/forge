#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentDecisionTraceStage {
    RawIntent,
    Eligibility,
    AdmittedDecision,
    AdvisoryStop,
    ExecutionHandoff,
    ExecutionOutcome,
    ViolationStop,
}

impl ForgeQueryIntentDecisionTraceStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RawIntent => "raw-intent",
            Self::Eligibility => "eligibility",
            Self::AdmittedDecision => "admitted-decision",
            Self::AdvisoryStop => "advisory-stop",
            Self::ExecutionHandoff => "execution-handoff",
            Self::ExecutionOutcome => "execution-outcome",
            Self::ViolationStop => "violation-stop",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentDecisionTraceEnvelopeKind {
    AdmittedExecution,
    AdvisoryStop,
    ViolationStop,
}

impl ForgeQueryIntentDecisionTraceEnvelopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedExecution => "admitted-execution",
            Self::AdvisoryStop => "advisory-stop",
            Self::ViolationStop => "violation-stop",
        }
    }
}
