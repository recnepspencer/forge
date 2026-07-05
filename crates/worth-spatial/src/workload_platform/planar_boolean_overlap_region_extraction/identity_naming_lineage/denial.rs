use super::counters::PlanarBooleanOverlapRegionIdentityLineageCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionIdentityLineageDenialKind {
    InputIdentityMismatchDenied,
    DuplicateRegionIdentityDenied,
    ConflictingPersistentNamePropagationDenied,
    DanglingPersistentNameReferenceDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionIdentityLineageDenial {
    kind: PlanarBooleanOverlapRegionIdentityLineageDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapRegionIdentityLineageCounters,
    message: &'static str,
}

impl PlanarBooleanOverlapRegionIdentityLineageDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapRegionIdentityLineageDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapRegionIdentityLineageCounters,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionIdentityLineageDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionIdentityLineageCounters {
        self.counters
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
