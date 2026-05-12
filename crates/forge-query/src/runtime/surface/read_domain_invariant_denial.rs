use crate::identity::hash_parts;
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
        let denial_digest = hash_parts(&[
            "forge_query_read_domain_invariant_denial_v1".to_string(),
            format!("hook:{hook_family}"),
            format!("invariant:{invariant_family}"),
            format!("message:{message}"),
            format!("summary:{}", domain_invariant_summary.summary_digest()),
            format!("violation:{}", violation.violation_digest()),
        ]);
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
