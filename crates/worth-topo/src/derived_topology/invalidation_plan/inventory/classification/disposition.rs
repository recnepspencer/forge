use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedInvalidationAuthorityDisposition {
    Migrate,
    Delete,
    CertificationBootstrapResidue,
    TrueQueryCapabilityGap,
}

impl DerivedInvalidationAuthorityDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Migrate => "migrate",
            Self::Delete => "delete",
            Self::CertificationBootstrapResidue => "certification_bootstrap_residue",
            Self::TrueQueryCapabilityGap => "true_query_capability_gap",
        }
    }

    pub const fn can_satisfy_ordinary_invalidation(self) -> bool {
        matches!(self, Self::Migrate)
    }
}
