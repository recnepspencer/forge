use crate::declaration::{stable_text_digest, UiDeclarationArtifact, UiDeclarationIdentity};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchOriginClass {
    DeclarationChange,
    QueryFactChange,
    HostObservation,
    ServiceEvent,
    IntentSubmission,
    DiagnosticOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchOriginReceipt {
    class: UiGraphTouchOriginClass,
    authority_digest: u64,
}

impl UiGraphTouchOriginReceipt {
    pub(crate) fn declaration_change(artifact: &UiDeclarationArtifact) -> Self {
        Self {
            class: UiGraphTouchOriginClass::DeclarationChange,
            authority_digest: artifact.identity().digest().raw(),
        }
    }

    pub(crate) fn query_fact_change(authority_projection: &str) -> Self {
        Self {
            class: UiGraphTouchOriginClass::QueryFactChange,
            authority_digest: stable_text_digest(authority_projection),
        }
    }

    pub(crate) const fn host_observation(authority_digest: u64) -> Self {
        Self {
            class: UiGraphTouchOriginClass::HostObservation,
            authority_digest,
        }
    }

    pub(crate) const fn service_event(authority_digest: u64) -> Self {
        Self {
            class: UiGraphTouchOriginClass::ServiceEvent,
            authority_digest,
        }
    }

    pub(crate) const fn intent_submission(authority_digest: u64) -> Self {
        Self {
            class: UiGraphTouchOriginClass::IntentSubmission,
            authority_digest,
        }
    }

    pub(crate) const fn diagnostic_only(authority_digest: u64) -> Self {
        Self {
            class: UiGraphTouchOriginClass::DiagnosticOnly,
            authority_digest,
        }
    }

    pub fn class(&self) -> UiGraphTouchOriginClass {
        self.class
    }

    pub fn authority_digest(&self) -> u64 {
        self.authority_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchOriginWitness {
    receipt: UiGraphTouchOriginReceipt,
    authority: UiGraphTouchOriginAuthority,
}

impl UiGraphTouchOriginWitness {
    pub(crate) const fn declaration_instances(
        receipt: UiGraphTouchOriginReceipt,
        declaration_identity: UiDeclarationIdentity,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::DeclarationInstances {
                declaration_identity,
            },
        }
    }

    pub(crate) const fn mounted_receipt_transition_only(
        receipt: UiGraphTouchOriginReceipt,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::MountedReceiptTransitionOnly,
        }
    }

    pub(crate) fn authored_provenance_digests(
        receipt: UiGraphTouchOriginReceipt,
        mut digests: Vec<u64>,
    ) -> Self {
        digests.sort_unstable();
        digests.dedup();
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::AuthoredProvenanceDigests { digests },
        }
    }

    pub fn receipt(&self) -> &UiGraphTouchOriginReceipt {
        &self.receipt
    }

    pub(crate) const fn authority(&self) -> &UiGraphTouchOriginAuthority {
        &self.authority
    }
}

impl From<UiGraphTouchOriginWitness> for UiGraphTouchOriginReceipt {
    fn from(value: UiGraphTouchOriginWitness) -> Self {
        value.receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum UiGraphTouchOriginAuthority {
    DeclarationInstances {
        declaration_identity: UiDeclarationIdentity,
    },
    MountedReceiptTransitionOnly,
    AuthoredProvenanceDigests {
        digests: Vec<u64>,
    },
}
