use std::sync::Arc;

use super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::counters::BridgeCausalEnvelopeCounters;
use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeCausalEnvelopeDenialKind {
    EmptyAssemblyRequestDigest,
    EmptyEvidenceReference,
    DuplicateEvidenceReference,
    EvidenceOwnerMismatch,
    EvidenceReferenceFamilyMismatch,
    MissingEvidenceReference,
    MissingQueryObservationAnchor,
    QueryObservationAnchorOverclaim,
    MissingRequiredBridgeRouteEvidence,
    MissingRetainedBridgeRecord,
}

impl BridgeCausalEnvelopeDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAssemblyRequestDigest => "empty_assembly_request_digest",
            Self::EmptyEvidenceReference => "empty_evidence_reference",
            Self::DuplicateEvidenceReference => "duplicate_evidence_reference",
            Self::EvidenceOwnerMismatch => "evidence_owner_mismatch",
            Self::EvidenceReferenceFamilyMismatch => "evidence_reference_family_mismatch",
            Self::MissingEvidenceReference => "missing_evidence_reference",
            Self::MissingQueryObservationAnchor => "missing_query_observation_anchor",
            Self::QueryObservationAnchorOverclaim => "query_observation_anchor_overclaim",
            Self::MissingRequiredBridgeRouteEvidence => "missing_required_bridge_route_evidence",
            Self::MissingRetainedBridgeRecord => "missing_retained_bridge_record",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeDenial {
    kind: BridgeCausalEnvelopeDenialKind,
    family: BridgeCausalEvidenceFamily,
    supplied_owner: BridgeCausalEvidenceOwner,
    expected_owner: BridgeCausalEvidenceOwner,
    reference_identity: Arc<str>,
    counters: BridgeCausalEnvelopeCounters,
    failure_digest: Arc<str>,
}

impl BridgeCausalEnvelopeDenial {
    pub(crate) fn new(
        kind: BridgeCausalEnvelopeDenialKind,
        family: BridgeCausalEvidenceFamily,
        supplied_owner: BridgeCausalEvidenceOwner,
        expected_owner: BridgeCausalEvidenceOwner,
        reference_identity: Arc<str>,
        counters: BridgeCausalEnvelopeCounters,
    ) -> Self {
        let failure_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::Denial,
            &[
                &format!("{kind:?}"),
                family.as_str(),
                supplied_owner.as_str(),
                expected_owner.as_str(),
                reference_identity.as_ref(),
                counters.counter_digest(),
            ],
        );
        Self {
            kind,
            family,
            supplied_owner,
            expected_owner,
            reference_identity,
            counters,
            failure_digest: Arc::from(failure_digest),
        }
    }

    pub fn kind(&self) -> BridgeCausalEnvelopeDenialKind {
        self.kind
    }

    pub fn family(&self) -> BridgeCausalEvidenceFamily {
        self.family
    }

    pub fn supplied_owner(&self) -> BridgeCausalEvidenceOwner {
        self.supplied_owner
    }

    pub fn expected_owner(&self) -> BridgeCausalEvidenceOwner {
        self.expected_owner
    }

    pub fn reference_identity(&self) -> &str {
        self.reference_identity.as_ref()
    }

    pub fn counters(&self) -> &BridgeCausalEnvelopeCounters {
        &self.counters
    }

    pub fn failure_digest(&self) -> &str {
        self.failure_digest.as_ref()
    }
}
