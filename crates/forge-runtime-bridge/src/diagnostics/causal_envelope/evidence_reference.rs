use std::sync::Arc;

use super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::counters::BridgeCausalEnvelopeCounters;
use super::denial::{BridgeCausalEnvelopeDenial, BridgeCausalEnvelopeDenialKind};
use super::digest;

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
        reference_identity: impl Into<Arc<str>>,
    ) -> Result<Self, BridgeCausalEnvelopeDenial> {
        let reference_identity = reference_identity.into();
        if reference_identity.is_empty() {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EmptyEvidenceReference,
                family,
                owner,
                family.expected_owner(),
                reference_identity,
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        if owner != family.expected_owner() {
            return Err(BridgeCausalEnvelopeDenial::new(
                BridgeCausalEnvelopeDenialKind::EvidenceOwnerMismatch,
                family,
                owner,
                family.expected_owner(),
                reference_identity,
                BridgeCausalEnvelopeCounters::empty(),
            ));
        }
        let reference_digest = digest(
            "bridge-causal-evidence-reference",
            &[owner.as_str(), family.as_str(), reference_identity.as_ref()],
        );
        Ok(Self {
            owner,
            family,
            reference_digest: Arc::from(reference_digest),
            reference_identity,
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
