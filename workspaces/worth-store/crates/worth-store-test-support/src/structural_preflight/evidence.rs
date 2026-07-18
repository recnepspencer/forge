use serde::{Deserialize, Serialize};

use super::{StructuralPredicate, StructuralPreflightPlan};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct PreflightEvidenceIdentity(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPredicateFailure {
    pub predicate: StructuralPredicate,
    pub failure_code: String,
    pub message: String,
    pub invalidated_inputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "verdict", rename_all = "snake_case")]
pub enum StructuralPredicateVerdict {
    Passed { authority_identity: String },
    Failed { failure: StructuralPredicateFailure },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPredicateEvidence {
    pub predicate: StructuralPredicate,
    pub input_identity: String,
    pub tool_identity: Option<String>,
    pub verdict: StructuralPredicateVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralPreflightEvidence {
    pub schema_version: u32,
    pub plan: StructuralPreflightPlan,
    pub predicates: Vec<StructuralPredicateEvidence>,
    pub evidence_identity: PreflightEvidenceIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "freshness", rename_all = "snake_case")]
pub enum PreflightEvidenceFreshness {
    Fresh {
        evidence_identity: PreflightEvidenceIdentity,
    },
    Stale {
        failures: Vec<StructuralPredicateFailure>,
    },
}

impl StructuralPreflightEvidence {
    pub fn failures(&self) -> Vec<&StructuralPredicateFailure> {
        self.predicates
            .iter()
            .filter_map(|evidence| match &evidence.verdict {
                StructuralPredicateVerdict::Passed { .. } => None,
                StructuralPredicateVerdict::Failed { failure } => Some(failure),
            })
            .collect()
    }

    pub fn passed_identity(&self, predicate: StructuralPredicate) -> Option<&str> {
        self.predicates.iter().find_map(|evidence| {
            if evidence.predicate != predicate {
                return None;
            }
            match &evidence.verdict {
                StructuralPredicateVerdict::Passed { authority_identity } => {
                    Some(authority_identity.as_str())
                }
                StructuralPredicateVerdict::Failed { .. } => None,
            }
        })
    }
}
