use std::sync::Arc;

use super::authority::{BridgeCausalEvidenceFamily, BridgeCausalEvidenceOwner};
use super::evidence_reference::BridgeCausalEvidenceReference;
use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeCausalEvidenceBindingClass {
    RetainedBridgeRecord,
    ExternalAuthorityReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEvidenceBinding {
    owner: BridgeCausalEvidenceOwner,
    family: BridgeCausalEvidenceFamily,
    binding_class: BridgeCausalEvidenceBindingClass,
    reference_identity: Arc<str>,
    retained_record_digest: Option<Arc<str>>,
    binding_digest: Arc<str>,
}

impl BridgeCausalEvidenceBinding {
    pub(super) fn retained(
        reference: &BridgeCausalEvidenceReference,
        retained_record_digest: String,
    ) -> Self {
        Self::new(
            reference,
            BridgeCausalEvidenceBindingClass::RetainedBridgeRecord,
            Some(Arc::from(retained_record_digest)),
        )
    }

    pub(super) fn external(reference: &BridgeCausalEvidenceReference) -> Self {
        Self::new(
            reference,
            BridgeCausalEvidenceBindingClass::ExternalAuthorityReference,
            None,
        )
    }

    fn new(
        reference: &BridgeCausalEvidenceReference,
        binding_class: BridgeCausalEvidenceBindingClass,
        retained_record_digest: Option<Arc<str>>,
    ) -> Self {
        let retained = retained_record_digest.as_deref().unwrap_or("external");
        let binding_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::EvidenceBinding,
            &[
                reference.owner().as_str(),
                reference.family().as_str(),
                reference.reference_identity(),
                retained,
            ],
        );
        Self {
            owner: reference.owner(),
            family: reference.family(),
            binding_class,
            reference_identity: Arc::from(reference.reference_identity()),
            retained_record_digest,
            binding_digest: Arc::from(binding_digest),
        }
    }

    pub fn owner(&self) -> BridgeCausalEvidenceOwner {
        self.owner
    }

    pub fn family(&self) -> BridgeCausalEvidenceFamily {
        self.family
    }

    pub fn binding_class(&self) -> BridgeCausalEvidenceBindingClass {
        self.binding_class
    }

    pub fn reference_identity(&self) -> &str {
        self.reference_identity.as_ref()
    }

    pub fn retained_record_digest(&self) -> Option<&str> {
        self.retained_record_digest.as_deref()
    }

    pub fn binding_digest(&self) -> &str {
        self.binding_digest.as_ref()
    }
}
