use super::counters::PlanarBooleanLoopIdentityMintingCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopIdentityMintingDenialKind {
    RequestIdentityMismatch,
    MissingRoleOutcome,
    MissingDegenerateOutcome,
    MissingSplitNamingSeed,
    ForeignNamingLineage,
    DanglingNameReference,
    DuplicatePropagatedPersistentName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIdentityMintingDenial {
    kind: PlanarBooleanLoopIdentityMintingDenialKind,
    detail_identity: String,
    counters: PlanarBooleanLoopIdentityMintingCounters,
    human_reason: String,
}

impl PlanarBooleanLoopIdentityMintingDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopIdentityMintingDenialKind,
        detail_identity: String,
        counters: PlanarBooleanLoopIdentityMintingCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail_identity,
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopIdentityMintingDenialKind {
        self.kind
    }

    pub fn detail_identity(&self) -> &str {
        &self.detail_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopIdentityMintingCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
