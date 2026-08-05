use super::super::WorthQueryIntentAdmissionExecutionSeam;
use super::eligibility_evidence::WorthQueryIntentEligibilityTraceEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentDecisionTraceEvidenceOwner {
    QueryIntentAuthoring,
    QueryIntentEligibility,
    QueryIntentDecision,
    QueryIntentExecution,
}

impl WorthQueryIntentDecisionTraceEvidenceOwner {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryIntentAuthoring => "query-intent-authoring",
            Self::QueryIntentEligibility => "query-intent-eligibility",
            Self::QueryIntentDecision => "query-intent-decision",
            Self::QueryIntentExecution => "query-intent-execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentDecisionTraceEvidence {
    Request {
        request_digest: String,
    },
    Eligibility(WorthQueryIntentEligibilityTraceEvidence),
    AdmittedDecision {
        decision_digest: String,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    },
    NonAdmittedDecision {
        decision_digest: String,
    },
    ExecutionHandoff {
        handoff_digest: String,
        execution_seam: WorthQueryIntentAdmissionExecutionSeam,
    },
    ExecutionOutcome {
        outcome_digest: String,
        execution_kind: String,
    },
}

impl WorthQueryIntentDecisionTraceEvidence {
    pub(crate) fn owner(&self) -> WorthQueryIntentDecisionTraceEvidenceOwner {
        match self {
            Self::Request { .. } => {
                WorthQueryIntentDecisionTraceEvidenceOwner::QueryIntentAuthoring
            }
            Self::Eligibility(_) => {
                WorthQueryIntentDecisionTraceEvidenceOwner::QueryIntentEligibility
            }
            Self::AdmittedDecision { .. } | Self::NonAdmittedDecision { .. } => {
                WorthQueryIntentDecisionTraceEvidenceOwner::QueryIntentDecision
            }
            Self::ExecutionHandoff { .. } | Self::ExecutionOutcome { .. } => {
                WorthQueryIntentDecisionTraceEvidenceOwner::QueryIntentExecution
            }
        }
    }

    pub(crate) fn primary_digest(&self) -> &str {
        match self {
            Self::Request { request_digest } => request_digest,
            Self::Eligibility(evidence) => evidence.eligibility_digest(),
            Self::AdmittedDecision {
                decision_digest, ..
            }
            | Self::NonAdmittedDecision { decision_digest } => decision_digest,
            Self::ExecutionHandoff { handoff_digest, .. } => handoff_digest,
            Self::ExecutionOutcome { outcome_digest, .. } => outcome_digest,
        }
    }
}
