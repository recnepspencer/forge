use std::sync::Arc;

use super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::counters::BridgeCausalEnvelopeCounters;
use super::denial::{BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind};
use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEvidenceReferenceIdentity {
    family: BridgeCausalEvidenceFamily,
    identity: Arc<str>,
}

impl BridgeCausalEvidenceReferenceIdentity {
    fn for_owner(
        owner: BridgeCausalEvidenceOwner,
        family: BridgeCausalEvidenceFamily,
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        let identity = identity.into();
        if identity.is_empty() {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EmptyEvidenceReference,
                family,
                owner,
                family.expected_owner(),
                identity,
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        if owner != family.expected_owner() {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EvidenceOwnerMismatch,
                family,
                owner,
                family.expected_owner(),
                identity,
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        Ok(Self { family, identity })
    }

    pub fn query_observation(
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::for_owner(
            BridgeCausalEvidenceOwner::Query,
            BridgeCausalEvidenceFamily::QueryObservation,
            identity,
        )
    }

    pub fn runtime_bridge(
        family: BridgeCausalEvidenceFamily,
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::for_owner(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
    }

    pub fn relational_authority(
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::for_owner(
            BridgeCausalEvidenceOwner::Relational,
            BridgeCausalEvidenceFamily::RelationalAuthority,
            identity,
        )
    }

    pub fn signal(
        family: BridgeCausalEvidenceFamily,
        identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        Self::for_owner(BridgeCausalEvidenceOwner::Signal, family, identity)
    }

    pub fn family(&self) -> BridgeCausalEvidenceFamily {
        self.family
    }

    pub fn as_str(&self) -> &str {
        self.identity.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEvidenceReference {
    owner: BridgeCausalEvidenceOwner,
    family: BridgeCausalEvidenceFamily,
    reference_digest: Arc<str>,
    reference_identity: Arc<str>,
}

impl BridgeCausalEvidenceReference {
    pub fn new(
        owner: BridgeCausalEvidenceOwner,
        family: BridgeCausalEvidenceFamily,
        reference_identity: BridgeCausalEvidenceReferenceIdentity,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        if reference_identity.family() != family {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EvidenceReferenceFamilyMismatch,
                family,
                owner,
                reference_identity.family().expected_owner(),
                Arc::from(reference_identity.as_str()),
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        if owner != family.expected_owner() {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EvidenceOwnerMismatch,
                family,
                owner,
                family.expected_owner(),
                Arc::from(reference_identity.as_str()),
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        let reference_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::EvidenceReference,
            &[owner.as_str(), family.as_str(), reference_identity.as_str()],
        );
        Ok(Self {
            owner,
            family,
            reference_digest: Arc::from(reference_digest),
            reference_identity: Arc::from(reference_identity.as_str()),
        })
    }

    pub fn owner(&self) -> BridgeCausalEvidenceOwner {
        self.owner
    }

    pub fn family(&self) -> BridgeCausalEvidenceFamily {
        self.family
    }

    pub fn reference_digest(&self) -> &str {
        self.reference_digest.as_ref()
    }

    pub fn reference_identity(&self) -> &str {
        self.reference_identity.as_ref()
    }
}
