use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphObligationOperatingWorldSelector {
    AnyCommittedAuthority,
    Preview,
    Branch,
    ConfiguredDomainHandle,
    AnyOperatingWorld,
}

impl WorthQueryGraphObligationOperatingWorldSelector {
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

    pub fn selector_digest(self) -> WorthQueryEvidenceIdentity {
        worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationOperatingWorldSelector,
        )
        .field_shape(WorthQueryEvidenceTag::new("kind"), self.as_str())
        .seal()
    }
}
