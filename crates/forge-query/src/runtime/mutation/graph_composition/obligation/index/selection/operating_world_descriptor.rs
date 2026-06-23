use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::lookup::ForgeQueryGraphObligationOperatingWorldLookupKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationOperatingWorldDescriptorKind {
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
}

impl ForgeQueryGraphObligationOperatingWorldDescriptorKind {
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
pub struct ForgeQueryGraphObligationOperatingWorldDescriptor {
    kind: ForgeQueryGraphObligationOperatingWorldDescriptorKind,
    descriptor_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationOperatingWorldDescriptor {
    pub fn any_committed_authority() -> Self {
        Self::new(ForgeQueryGraphObligationOperatingWorldDescriptorKind::AnyCommittedAuthority)
    }

    pub fn preview() -> Self {
        Self::new(ForgeQueryGraphObligationOperatingWorldDescriptorKind::Preview)
    }

    pub fn branch() -> Self {
        Self::new(ForgeQueryGraphObligationOperatingWorldDescriptorKind::Branch)
    }

    pub fn configured_domain_handle() -> Self {
        Self::new(ForgeQueryGraphObligationOperatingWorldDescriptorKind::ConfiguredDomainHandle)
    }

    fn new(kind: ForgeQueryGraphObligationOperatingWorldDescriptorKind) -> Self {
        let descriptor_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationOperatingWorldDescriptor,
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .seal();
        Self {
            kind,
            descriptor_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryGraphObligationOperatingWorldDescriptorKind {
        self.kind
    }

    pub fn descriptor_digest(&self) -> &str {
        self.descriptor_digest.as_str()
    }

    pub(crate) fn descriptor_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.descriptor_digest
    }

    pub(super) fn lookup_keys(&self) -> Vec<ForgeQueryGraphObligationOperatingWorldLookupKey> {
        vec![
            ForgeQueryGraphObligationOperatingWorldLookupKey::from_descriptor_kind(self.kind),
            ForgeQueryGraphObligationOperatingWorldLookupKey::AnyOperatingWorld,
        ]
    }
}
