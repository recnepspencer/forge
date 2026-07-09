use worth_store_physical_backend::BackendDurabilityProfileId;

use crate::{LogSequenceNumber, PageLsn};

use super::{CheckpointRecoveryCounterSnapshot, CheckpointRootPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointValidationDenialKind {
    TornManifest,
    MissingCheckpointLocator,
    LocatorCheckpointMismatch,
    MissingRoot,
    StaleRoot,
    StalePageLsnFrontier,
    RedoBoundaryOutsideCoveredRange,
    RecoveryBlockingIntegrityDamage,
    FuzzyCheckpointModeUnsupported,
    CutoverDurabilityProfileMismatch,
    CutoverDurabilityRangeMismatch,
    CutoverDurabilityCheckpointMismatch,
    CutoverDurabilityArtifactMismatch,
    CutoverDurabilityRoleReuse,
    RecoveredCheckpointEvidenceMismatch,
    AmbiguousCheckpointCutover,
    WalRetentionWithoutCoveringCheckpoint,
    WalRetentionWithoutContiguousTail,
    WalRetentionCheckpointMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointValidationDenial {
    kind: CheckpointValidationDenialKind,
    counters: CheckpointRecoveryCounterSnapshot,
    expected_lsn: Option<LogSequenceNumber>,
    observed_lsn: Option<LogSequenceNumber>,
    expected_page_lsn: Option<PageLsn>,
    observed_page_lsn: Option<PageLsn>,
    root_posture: Option<CheckpointRootPosture>,
    profile_id: Option<BackendDurabilityProfileId>,
}

impl CheckpointValidationDenial {
    pub(crate) const fn new(
        kind: CheckpointValidationDenialKind,
        counters: CheckpointRecoveryCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            counters,
            expected_lsn: None,
            observed_lsn: None,
            expected_page_lsn: None,
            observed_page_lsn: None,
            root_posture: None,
            profile_id: None,
        }
    }

    pub(crate) const fn with_lsn_pair(
        mut self,
        expected: LogSequenceNumber,
        observed: LogSequenceNumber,
    ) -> Self {
        self.expected_lsn = Some(expected);
        self.observed_lsn = Some(observed);
        self
    }

    pub(crate) const fn with_page_lsn_pair(mut self, expected: PageLsn, observed: PageLsn) -> Self {
        self.expected_page_lsn = Some(expected);
        self.observed_page_lsn = Some(observed);
        self
    }

    pub(crate) const fn with_root_posture(mut self, posture: CheckpointRootPosture) -> Self {
        self.root_posture = Some(posture);
        self
    }

    pub(crate) const fn with_profile_id(mut self, profile_id: BackendDurabilityProfileId) -> Self {
        self.profile_id = Some(profile_id);
        self
    }

    pub const fn kind(&self) -> CheckpointValidationDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }

    pub const fn expected_lsn(&self) -> Option<LogSequenceNumber> {
        self.expected_lsn
    }

    pub const fn observed_lsn(&self) -> Option<LogSequenceNumber> {
        self.observed_lsn
    }

    pub const fn expected_page_lsn(&self) -> Option<PageLsn> {
        self.expected_page_lsn
    }

    pub const fn observed_page_lsn(&self) -> Option<PageLsn> {
        self.observed_page_lsn
    }

    pub const fn root_posture(&self) -> Option<CheckpointRootPosture> {
        self.root_posture
    }

    pub const fn profile_id(&self) -> Option<BackendDurabilityProfileId> {
        self.profile_id
    }
}
