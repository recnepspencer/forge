use super::super::ForgeQueryIntentAdmissionExecutionSeam;
use super::eligibility_evidence::ForgeQueryIntentEligibilityTraceEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentDecisionTraceEvidenceOwner {
    QueryIntentAuthoring,
    QueryIntentEligibility,
    QueryIntentDecision,
    QueryIntentExecution,
}

impl ForgeQueryIntentDecisionTraceEvidenceOwner {
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
pub enum ForgeQueryIntentDecisionTraceEvidence {
    Request {
        request_digest: String,
    },
    Eligibility(ForgeQueryIntentEligibilityTraceEvidence),
    AdmittedDecision {
        decision_digest: String,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    },
    NonAdmittedDecision {
        decision_digest: String,
    },
    ExecutionHandoff {
        handoff_digest: String,
        execution_seam: ForgeQueryIntentAdmissionExecutionSeam,
    },
    ObligationDispatch {
        envelope_digest: String,
    },
    ExecutionOutcome {
        outcome_digest: String,
        execution_kind: String,
    },
}

impl ForgeQueryIntentDecisionTraceEvidence {
    pub(crate) fn owner(&self) -> ForgeQueryIntentDecisionTraceEvidenceOwner {
        match self {
            Self::Request { .. } => {
                ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentAuthoring
            }
            Self::Eligibility(_) => {
                ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentEligibility
            }
            Self::AdmittedDecision { .. }
            | Self::NonAdmittedDecision { .. }
            | Self::ObligationDispatch { .. } => {
                ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentDecision
            }
            Self::ExecutionHandoff { .. } | Self::ExecutionOutcome { .. } => {
                ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentExecution
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
            Self::ObligationDispatch { envelope_digest } => envelope_digest,
            Self::ExecutionOutcome { outcome_digest, .. } => outcome_digest,
        }
    }
}
