use crate::declaration::{UiDeclarationArtifact, UiDeclarationIdentity};

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

    pub(crate) fn query_fact_change(
        prerequisites: &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    ) -> Self {
        let canonical = prerequisites.canonical_basis_digest();
        let authority_digest = canonical
            .value()
            .bytes()
            .iter()
            .take(8)
            .enumerate()
            .fold(0u64, |digest, (index, byte)| {
                digest | (u64::from(*byte) << (index * 8))
            });
        Self {
            class: UiGraphTouchOriginClass::QueryFactChange,
            authority_digest,
        }
    }

    pub(crate) fn installed_query_fact_change(
        authority: &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryBasisAuthority,
    ) -> Self {
        Self {
            class: UiGraphTouchOriginClass::QueryFactChange,
            authority_digest: authority.identity().as_u64(),
        }
    }

    pub(crate) fn settled_query_fact_change(
        view_binding_id: &crate::capability::ViewBindingId,
        query_binding_identity: &str,
    ) -> Self {
        Self {
            class: UiGraphTouchOriginClass::QueryFactChange,
            authority_digest: crate::declaration::stable_text_digest(view_binding_id.as_str())
                ^ crate::declaration::stable_text_digest(query_binding_identity).rotate_left(29),
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

    pub(crate) fn query_basis(
        receipt: UiGraphTouchOriginReceipt,
        prerequisites: worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::QueryBasis {
                prerequisites: Box::new(prerequisites),
            },
        }
    }

    pub(crate) fn installed_query_basis(
        receipt: UiGraphTouchOriginReceipt,
        authority: worth_ui_query_binding::compatibility::managed_live::WorthUiQueryBasisAuthority,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::InstalledQueryBasis { authority },
        }
    }

    pub(crate) fn settled_query_binding(
        receipt: UiGraphTouchOriginReceipt,
        view_binding_id: crate::capability::ViewBindingId,
        query_binding_identity: Box<str>,
    ) -> Self {
        Self {
            receipt,
            authority: UiGraphTouchOriginAuthority::SettledQueryBinding {
                view_binding_id,
                query_binding_identity,
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
    QueryBasis {
        prerequisites: Box<
            worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence,
        >,
    },
    InstalledQueryBasis {
        authority: worth_ui_query_binding::compatibility::managed_live::WorthUiQueryBasisAuthority,
    },
    SettledQueryBinding {
        view_binding_id: crate::capability::ViewBindingId,
        query_binding_identity: Box<str>,
    },
    AuthoredProvenanceDigests {
        digests: Vec<u64>,
    },
}
