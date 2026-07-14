use crate::policy_basis::PolicyTenantAdmissionError;
use crate::policy_narrowing::PolicyNarrowingError;
use crate::relationship_proof::RelationshipProofError;
use crate::runtime::WorthQueryGraphReadAccessAuthorityDenial;

use super::WorthQueryReadContextAdmissionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReadContextDenialSource {
    MissingRelationshipProof,
    PolicyTenant(PolicyTenantAdmissionError),
    PolicyNarrowing(PolicyNarrowingError),
    RelationshipProof(RelationshipProofError),
    GraphAuthority(WorthQueryGraphReadAccessAuthorityDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReadContextDenial {
    source: WorthQueryReadContextDenialSource,
    counters: WorthQueryReadContextAdmissionCounters,
}

impl WorthQueryReadContextDenial {
    pub(crate) fn missing_relationship_proof(
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        Self {
            source: WorthQueryReadContextDenialSource::MissingRelationshipProof,
            counters,
        }
    }

    pub fn source(&self) -> &WorthQueryReadContextDenialSource {
        &self.source
    }

    pub fn counters(&self) -> &WorthQueryReadContextAdmissionCounters {
        &self.counters
    }

    pub(crate) fn policy_tenant(
        source: PolicyTenantAdmissionError,
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        Self {
            source: WorthQueryReadContextDenialSource::PolicyTenant(source),
            counters,
        }
    }

    pub(crate) fn relationship_proof(
        source: RelationshipProofError,
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        Self {
            source: WorthQueryReadContextDenialSource::RelationshipProof(source),
            counters,
        }
    }

    pub(crate) fn policy_narrowing(
        source: PolicyNarrowingError,
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        Self {
            source: WorthQueryReadContextDenialSource::PolicyNarrowing(source),
            counters,
        }
    }

    pub(crate) fn graph_authority(
        source: WorthQueryGraphReadAccessAuthorityDenial,
        counters: WorthQueryReadContextAdmissionCounters,
    ) -> Self {
        Self {
            source: WorthQueryReadContextDenialSource::GraphAuthority(source),
            counters,
        }
    }
}
