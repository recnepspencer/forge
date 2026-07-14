use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::lookup::WorthQueryGraphObligationOperatingWorldLookupKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationOperatingWorldDescriptorKind {
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
}

impl WorthQueryGraphObligationOperatingWorldDescriptorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AnyCommittedAuthority => "any-committed-authority",
            Self::Preview => "preview",
            Self::Branch => "branch",
            Self::ConfiguredDomainHandle => "configured-domain-handle",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationOperatingWorldDescriptor {
    kind: WorthQueryGraphObligationOperatingWorldDescriptorKind,
    descriptor_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationOperatingWorldDescriptor {
    pub fn any_committed_authority() -> Self {
        Self::new(WorthQueryGraphObligationOperatingWorldDescriptorKind::AnyCommittedAuthority)
    }

    pub fn preview() -> Self {
        Self::new(WorthQueryGraphObligationOperatingWorldDescriptorKind::Preview)
    }

    pub fn branch() -> Self {
        Self::new(WorthQueryGraphObligationOperatingWorldDescriptorKind::Branch)
    }

    pub fn configured_domain_handle() -> Self {
        Self::new(WorthQueryGraphObligationOperatingWorldDescriptorKind::ConfiguredDomainHandle)
    }

    fn new(kind: WorthQueryGraphObligationOperatingWorldDescriptorKind) -> Self {
        let descriptor_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationOperatingWorldDescriptor,
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), kind.as_str())
        .seal();
        Self {
            kind,
            descriptor_digest,
        }
    }

    pub fn kind(&self) -> WorthQueryGraphObligationOperatingWorldDescriptorKind {
        self.kind
    }

    pub fn descriptor_digest(&self) -> &str {
        self.descriptor_digest.as_str()
    }

    pub(crate) fn descriptor_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.descriptor_digest
    }

    pub(super) fn lookup_keys(&self) -> Vec<WorthQueryGraphObligationOperatingWorldLookupKey> {
        vec![
            WorthQueryGraphObligationOperatingWorldLookupKey::from_descriptor_kind(self.kind),
            WorthQueryGraphObligationOperatingWorldLookupKey::AnyOperatingWorld,
        ]
    }
}
