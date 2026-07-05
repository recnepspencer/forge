use super::counters::PlanarBooleanOverlapRegionLedgerAssemblyCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionLedgerAssemblyDenialKind {
    InputIdentityMismatchDenied,
    MissingCanonicalWindingProofDenied,
    MissingPriorProofProductDenied,
    ForeignPriorProofLineageDenied,
    SyntheticOverlapRowDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedgerAssemblyDenial {
    kind: PlanarBooleanOverlapRegionLedgerAssemblyDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapRegionLedgerAssemblyCounters,
    message: &'static str,
}

impl PlanarBooleanOverlapRegionLedgerAssemblyDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapRegionLedgerAssemblyDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapRegionLedgerAssemblyCounters,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionLedgerAssemblyDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionLedgerAssemblyCounters {
        self.counters
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
