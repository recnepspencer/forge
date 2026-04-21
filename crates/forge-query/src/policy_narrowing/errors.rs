use crate::authorized_projection::AuthorizedProjectionFailureClass;
use crate::relationship_proof::RelationshipProofFailureClass;

use super::PolicyNarrowingCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyNarrowingFailureClass {
    CanonicalQueryDigestMismatch,
    PolicyMaskAuthorityMismatch,
    AuthorizedProjectionDenied(AuthorizedProjectionFailureClass),
    RelationshipProofDenied(RelationshipProofFailureClass),
    UnknownNarrowingCost,
    UnboundedDerivedInfluence,
    UnboundedProofTopology,
    DigestPartBudgetExceeded,
}

impl PolicyNarrowingFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CanonicalQueryDigestMismatch => "canonical_query_digest_mismatch",
            Self::PolicyMaskAuthorityMismatch => "policy_mask_authority_mismatch",
            Self::AuthorizedProjectionDenied(failure) => failure.as_str(),
            Self::RelationshipProofDenied(failure) => failure.as_str(),
            Self::UnknownNarrowingCost => "unknown_narrowing_cost",
            Self::UnboundedDerivedInfluence => "unbounded_derived_influence",
            Self::UnboundedProofTopology => "unbounded_proof_topology",
            Self::DigestPartBudgetExceeded => "digest_part_budget_exceeded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyNarrowingError {
    failure_class: PolicyNarrowingFailureClass,
    message: &'static str,
    counters: PolicyNarrowingCounters,
}

impl PolicyNarrowingError {
    pub(crate) fn new(
        failure_class: PolicyNarrowingFailureClass,
        message: &'static str,
        counters: PolicyNarrowingCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> PolicyNarrowingFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &PolicyNarrowingCounters {
        &self.counters
    }
}
