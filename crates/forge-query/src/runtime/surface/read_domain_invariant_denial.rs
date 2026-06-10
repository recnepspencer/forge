use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::read_composition_hooks::{
    read_invariant_pack_hook_family, ForgeQueryReadInvariantPackContext,
    ForgeQueryReadInvariantPackViolation,
};

use super::ForgeQueryReadDomainInvariantSummary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryReadDomainInvariantDenial {
    hook_family: String,
    invariant_family: String,
    message: String,
    domain_invariant_summary: ForgeQueryReadDomainInvariantSummary,
    denial_digest: String,
}

impl ForgeQueryReadDomainInvariantDenial {
    pub(crate) fn from_violation(
        violation: ForgeQueryReadInvariantPackViolation,
        context: &ForgeQueryReadInvariantPackContext<'_>,
    ) -> Self {
        let domain_invariant_summary = context.read_domain_invariant_summary();
        let hook_family = read_invariant_pack_hook_family().to_string();
        let invariant_family = violation.invariant_family().to_string();
        let message = violation.message().to_string();
        let denial_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ReadDomainInvariantDenial,
        )
        .field_shape(ForgeQueryEvidenceTag::new("hook_family"), hook_family.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("invariant_family"),
            invariant_family.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("summary_digest"),
            domain_invariant_summary.summary_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("violation_digest"),
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

    pub fn domain_invariant_summary(&self) -> &ForgeQueryReadDomainInvariantSummary {
        &self.domain_invariant_summary
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryReadDomainInvariantDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "read composition denied by {} for invariant `{}`: {}",
            self.hook_family, self.invariant_family, self.message
        )
    }
}
