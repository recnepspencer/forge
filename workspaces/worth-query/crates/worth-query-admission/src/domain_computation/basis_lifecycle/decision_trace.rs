use crate::admission_digest::hash_parts;

use super::proofs::NormalizedBasisIntent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibilityDecisionTrace {
    normalized_digest: String,
    outcome: String,
    message: &'static str,
    trace_digest: String,
}

impl BasisEligibilityDecisionTrace {
    pub(crate) fn new(
        normalized: &NormalizedBasisIntent,
        outcome: impl Into<String>,
        message: &'static str,
    ) -> Self {
        let outcome = outcome.into();
        let trace_digest = hash_parts(&[
            format!("normalized:{}", normalized.normalized_digest()),
            format!("outcome:{outcome}"),
            format!("message:{message}"),
        ]);
        Self {
            normalized_digest: normalized.normalized_digest().to_string(),
            outcome,
            message,
            trace_digest,
        }
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }

    pub(crate) fn new_lower_runtime_readmission(
        scoped_basis_digest: &str,
        evidence_digest: &str,
        outcome: impl Into<String>,
        message: &'static str,
    ) -> Self {
        let outcome = outcome.into();
        let trace_digest = hash_parts(&[
            format!("scoped_basis:{scoped_basis_digest}"),
            format!("lower_runtime_evidence:{evidence_digest}"),
            format!("outcome:{outcome}"),
            format!("message:{message}"),
        ]);
        Self {
            normalized_digest: scoped_basis_digest.to_string(),
            outcome,
            message,
            trace_digest,
        }
    }
}
