#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentDecisionTraceStage {
    RawIntent,
    Eligibility,
    AdmittedDecision,
    AdvisoryStop,
    ExecutionHandoff,
    ExecutionOutcome,
    ViolationStop,
}

impl WorthQueryIntentDecisionTraceStage {
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
pub enum WorthQueryIntentDecisionTraceEnvelopeKind {
    AdmittedExecution,
    AdvisoryStop,
    ViolationStop,
}

impl WorthQueryIntentDecisionTraceEnvelopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedExecution => "admitted-execution",
            Self::AdvisoryStop => "advisory-stop",
            Self::ViolationStop => "violation-stop",
        }
    }
}
