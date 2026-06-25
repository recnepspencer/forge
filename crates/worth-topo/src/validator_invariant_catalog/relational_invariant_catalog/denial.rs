#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopologyRelationalInvariantCatalogDenialKind {
    NoInvariantFamilies,
    NoSelectedInvariantFamilies,
    QueryRegistrationArtifactMissing,
    RejectedNonQueryAuthority,
    OldPackOrdinaryPathResidue,
    SourceFirewallViolation,
    ValidatorSeedMismatch,
}

impl WorthTopologyRelationalInvariantCatalogDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoInvariantFamilies => "no-invariant-families",
            Self::NoSelectedInvariantFamilies => "no-selected-invariant-families",
            Self::QueryRegistrationArtifactMissing => "query-registration-artifact-missing",
            Self::RejectedNonQueryAuthority => "rejected-non-query-authority",
            Self::OldPackOrdinaryPathResidue => "old-pack-ordinary-path-residue",
            Self::SourceFirewallViolation => "source-firewall-violation",
            Self::ValidatorSeedMismatch => "validator-seed-mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantCatalogDenial {
    kind: WorthTopologyRelationalInvariantCatalogDenialKind,
    subject_digest: String,
    detail: String,
    denial_digest: String,
}

impl WorthTopologyRelationalInvariantCatalogDenial {
    pub(in crate::validator_invariant_catalog) fn new(
        kind: WorthTopologyRelationalInvariantCatalogDenialKind,
        subject_digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let subject_digest = subject_digest.into();
        let detail = detail.into();
        let denial_digest = [
            "worth-topo-relational-invariant-catalog-denial-v1",
            kind.as_str(),
            subject_digest.as_str(),
            detail.as_str(),
        ]
        .join("|");
        Self {
            kind,
            subject_digest,
            detail,
            denial_digest,
        }
    }

    pub const fn kind(&self) -> WorthTopologyRelationalInvariantCatalogDenialKind {
        self.kind
    }

    pub fn subject_digest(&self) -> &str {
        &self.subject_digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for WorthTopologyRelationalInvariantCatalogDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} denied `{}`: {}",
            self.kind.as_str(),
            self.subject_digest,
            self.detail
        )
    }
}
