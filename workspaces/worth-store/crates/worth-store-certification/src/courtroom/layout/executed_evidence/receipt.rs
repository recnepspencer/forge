use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LayoutExecutedEvidenceKind {
    HiddenBroadScanDenied,
    BTreeReadinessStale,
    BTreePhysicalReadDenied,
    CrossTenantScopeDenied,
    CorruptionQuarantined,
    RollbackPublicationSourceDenied,
    RollbackRebindRequired,
    BTreeLeafOrderDenied,
    BTreeLeftPartitionDenied,
    BTreeRightPartitionDenied,
    LsmTombstoneRequired,
    LsmCacheArtifactInvalid,
    CompatibilityWindowMismatch,
    MaintenanceDeferred,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct LayoutExecutedEvidenceReceipt {
    observed: BTreeSet<LayoutExecutedEvidenceKind>,
}

impl LayoutExecutedEvidenceReceipt {
    pub(in crate::courtroom::layout) fn record(&mut self, evidence: LayoutExecutedEvidenceKind) {
        self.observed.insert(evidence);
    }

    pub(crate) fn contains(&self, evidence: LayoutExecutedEvidenceKind) -> bool {
        self.observed.contains(&evidence)
    }
}
