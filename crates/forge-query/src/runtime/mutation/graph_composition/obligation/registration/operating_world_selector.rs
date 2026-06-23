use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphObligationOperatingWorldSelector {
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
    AnyOperatingWorld,
}

impl ForgeQueryGraphObligationOperatingWorldSelector {
    pub fn any_committed_authority() -> Self {
        Self::AnyCommittedAuthority
    }

    pub fn preview() -> Self {
        Self::Preview
    }

    pub fn branch() -> Self {
        Self::Branch
    }

    pub fn configured_domain_handle() -> Self {
        Self::ConfiguredDomainHandle
    }

    pub fn any_operating_world() -> Self {
        Self::AnyOperatingWorld
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::AnyCommittedAuthority => "any-committed-authority",
            Self::Preview => "preview",
            Self::Branch => "branch",
            Self::ConfiguredDomainHandle => "configured-domain-handle",
            Self::AnyOperatingWorld => "any-operating-world",
        }
    }

    pub fn matches_operating_world(self, observed: Self) -> bool {
        matches!(self, Self::AnyOperatingWorld) || self == observed
    }

    pub fn selector_digest(self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationOperatingWorldSelector,
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), self.as_str())
        .seal()
    }
}
