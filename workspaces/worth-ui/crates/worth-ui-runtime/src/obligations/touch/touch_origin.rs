use crate::declaration::{UiDeclarationArtifact, UiDeclarationIdentity};
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchOriginClass {
    DeclarationChange,
    QueryBindingChange,
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

    pub(crate) fn settled_query_binding_change(
        view_binding_id: &crate::capability::ViewBindingId,
        binding_reference: &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    ) -> Self {
        Self {
            class: UiGraphTouchOriginClass::QueryBindingChange,
            authority_digest: crate::declaration::stable_text_digest(view_binding_id.as_str())
                ^ opaque_reference_digest(binding_reference).rotate_left(29),
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

    pub(crate) fn settled_query_binding(
        receipt: UiGraphTouchOriginReceipt,
        view_binding_id: crate::capability::ViewBindingId,
        binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::SettledQueryBinding {
                view_binding_id,
                binding_reference,
            },
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
    SettledQueryBinding {
        view_binding_id: crate::capability::ViewBindingId,
        binding_reference: worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
    },
    AuthoredProvenanceDigests {
        digests: Vec<u64>,
    },
}

pub(super) fn opaque_reference_digest(
    reference: &worth_ui_query_binding::WorthUiAdmittedQueryBindingReference,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    reference.hash(&mut hasher);
    hasher.finish()
}
