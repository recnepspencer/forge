use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::read_composition_hooks::{
    read_invariant_pack_hook_family, WorthQueryReadInvariantPackContext,
    WorthQueryReadInvariantPackViolation,
};

use super::WorthQueryReadDomainInvariantSummary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadDomainInvariantDenial {
    hook_family: String,
    invariant_family: String,
    message: String,
    domain_invariant_summary: WorthQueryReadDomainInvariantSummary,
    denial_digest: String,
}

impl WorthQueryReadDomainInvariantDenial {
    pub(crate) fn from_violation(
        violation: WorthQueryReadInvariantPackViolation,
        context: &WorthQueryReadInvariantPackContext<'_>,
    ) -> Self {
        let domain_invariant_summary = context.read_domain_invariant_summary();
        let hook_family = read_invariant_pack_hook_family().to_string();
        let invariant_family = violation.invariant_family().to_string();
        let message = violation.message().to_string();
        let denial_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::ReadDomainInvariantDenial)
                .field_shape(
                    WorthQueryEvidenceTag::new("hook_family"),
                    hook_family.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("invariant_family"),
                    invariant_family.as_str(),
                )
                .field_value(
                    WorthQueryEvidenceTag::new("summary_digest"),
                    domain_invariant_summary.summary_digest(),
                )
                .field_value(
                    WorthQueryEvidenceTag::new("violation_digest"),
                    violation.violation_digest(),
                )
                .seal()
                .as_str()
                .to_string();
        Self {
            hook_family,
            invariant_family,
            message,
            domain_invariant_summary,
            denial_digest,
        }
    }

    pub fn hook_family(&self) -> &str {
        &self.hook_family
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn domain_invariant_summary(&self) -> &WorthQueryReadDomainInvariantSummary {
        &self.domain_invariant_summary
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for WorthQueryReadDomainInvariantDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "read composition denied by {} for invariant `{}`: {}",
            self.hook_family, self.invariant_family, self.message
        )
    }
}
