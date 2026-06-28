use crate::{
    BackendResidueRejection, CheckpointValidationDenial, CheckpointValidationDenialKind,
    IllegalAcknowledgmentDenial, IllegalAcknowledgmentDenialKind, RecoveryBudgetDenial,
    RecoveryBudgetDenialKind, RecoveryEntryAdmissionDenial, RecoveryEntryAdmissionDenialKind,
    RedoRecordGrammarDenial, RedoRecordGrammarDenialKind, RuntimeRecoveryReportDenial,
};

use super::RecoveryPhysicsCloseoutDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntheticRecoveryShortcutKind {
    RawBytes,
    LiveStateReuse,
    BackendResidueGuessing,
    UnsupportedDurabilityClaim,
    InvalidPageLsn,
    TornCheckpoint,
    UnboundedRecoveryPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticRecoveryShortcutRejectionBoundary {
    RecoveryEntryAdmission,
    FreshRuntimeCrashIsolation,
    RecoverySourcePrecedence,
    DurabilityBarrierAndAck,
    WalBeforeDataPageLsn,
    CheckpointManifestPublication,
    BoundedRecoveryBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntheticRecoveryShortcutRejection {
    kind: SyntheticRecoveryShortcutKind,
    boundary: SyntheticRecoveryShortcutRejectionBoundary,
}

impl SyntheticRecoveryShortcutRejection {
    const fn new(
        kind: SyntheticRecoveryShortcutKind,
        boundary: SyntheticRecoveryShortcutRejectionBoundary,
    ) -> Self {
        Self { kind, boundary }
    }

    pub const fn kind(self) -> SyntheticRecoveryShortcutKind {
        self.kind
    }

    pub const fn boundary(self) -> SyntheticRecoveryShortcutRejectionBoundary {
        self.boundary
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticRecoveryShortcutEvidence {
    rejection: SyntheticRecoveryShortcutRejection,
}

impl SyntheticRecoveryShortcutEvidence {
    pub fn from_raw_recovery_bytes(
        denial: RecoveryEntryAdmissionDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        if denial.kind() != RecoveryEntryAdmissionDenialKind::RawBytesCrossedIntegrityBoundary {
            return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::RawBytes,
                SyntheticRecoveryShortcutRejectionBoundary::RecoveryEntryAdmission,
            ),
        ))
    }

    pub const fn from_same_process_live_state_reuse(
        denial: RuntimeRecoveryReportDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        match denial {
            RuntimeRecoveryReportDenial::SameProcessLiveStateReuse => Ok(Self::from_rejection(
                SyntheticRecoveryShortcutRejection::new(
                    SyntheticRecoveryShortcutKind::LiveStateReuse,
                    SyntheticRecoveryShortcutRejectionBoundary::FreshRuntimeCrashIsolation,
                ),
            )),
            _ => Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence),
        }
    }

    pub fn from_backend_residue_guessing(
        denial: BackendResidueRejection,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        match denial.kind() {
            crate::BackendResidueKind::BackendDirectoryResidue
            | crate::BackendResidueKind::OrphanedCheckpointManifest => {}
            _ => {
                return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
            }
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::BackendResidueGuessing,
                SyntheticRecoveryShortcutRejectionBoundary::RecoverySourcePrecedence,
            ),
        ))
    }

    pub fn from_unsupported_durability_claim(
        denial: IllegalAcknowledgmentDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        if denial.kind() != IllegalAcknowledgmentDenialKind::UnsupportedDurabilityCapability {
            return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::UnsupportedDurabilityClaim,
                SyntheticRecoveryShortcutRejectionBoundary::DurabilityBarrierAndAck,
            ),
        ))
    }

    pub fn from_invalid_page_lsn(
        denial: RedoRecordGrammarDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        if denial.kind() != RedoRecordGrammarDenialKind::MissingPageLsnBasis {
            return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::InvalidPageLsn,
                SyntheticRecoveryShortcutRejectionBoundary::WalBeforeDataPageLsn,
            ),
        ))
    }

    pub fn from_torn_checkpoint(
        denial: CheckpointValidationDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        if denial.kind() != CheckpointValidationDenialKind::TornManifest {
            return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::TornCheckpoint,
                SyntheticRecoveryShortcutRejectionBoundary::CheckpointManifestPublication,
            ),
        ))
    }

    pub fn from_unbounded_recovery_plan(
        denial: RecoveryBudgetDenial,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        match denial.kind() {
            RecoveryBudgetDenialKind::CheckpointIntervalMismatch { .. }
            | RecoveryBudgetDenialKind::WalTailFrameBudgetExceeded { .. }
            | RecoveryBudgetDenialKind::WalTailSegmentBudgetExceeded { .. }
            | RecoveryBudgetDenialKind::PageRedoBudgetExceeded { .. }
            | RecoveryBudgetDenialKind::ForbiddenFullStoreScan { .. } => {}
            _ => {
                return Err(RecoveryPhysicsCloseoutDenial::UnsupportedSyntheticShortcutEvidence);
            }
        }
        Ok(Self::from_rejection(
            SyntheticRecoveryShortcutRejection::new(
                SyntheticRecoveryShortcutKind::UnboundedRecoveryPlan,
                SyntheticRecoveryShortcutRejectionBoundary::BoundedRecoveryBudget,
            ),
        ))
    }

    const fn from_rejection(rejection: SyntheticRecoveryShortcutRejection) -> Self {
        Self { rejection }
    }

    pub const fn rejection(&self) -> SyntheticRecoveryShortcutRejection {
        self.rejection
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntheticRecoveryShortcutRejectionReport {
    rejections: Vec<SyntheticRecoveryShortcutRejection>,
}

impl SyntheticRecoveryShortcutRejectionReport {
    pub fn from_denial_evidence(
        evidence: Vec<SyntheticRecoveryShortcutEvidence>,
    ) -> Result<Self, RecoveryPhysicsCloseoutDenial> {
        let report = Self {
            rejections: evidence
                .into_iter()
                .map(|evidence| evidence.rejection())
                .collect(),
        };
        if report.all_required_shortcuts_denied() {
            return Ok(report);
        }
        Err(RecoveryPhysicsCloseoutDenial::MissingSyntheticShortcutRejection)
    }

    pub fn all_required_shortcuts_denied(&self) -> bool {
        required_shortcuts().iter().all(|kind| self.denies(*kind))
    }

    pub fn denies(&self, kind: SyntheticRecoveryShortcutKind) -> bool {
        self.rejections
            .iter()
            .any(|rejection| rejection.kind() == kind)
    }

    pub fn rejections(&self) -> &[SyntheticRecoveryShortcutRejection] {
        &self.rejections
    }
}

const fn required_shortcuts() -> [SyntheticRecoveryShortcutKind; 7] {
    [
        SyntheticRecoveryShortcutKind::RawBytes,
        SyntheticRecoveryShortcutKind::LiveStateReuse,
        SyntheticRecoveryShortcutKind::BackendResidueGuessing,
        SyntheticRecoveryShortcutKind::UnsupportedDurabilityClaim,
        SyntheticRecoveryShortcutKind::InvalidPageLsn,
        SyntheticRecoveryShortcutKind::TornCheckpoint,
        SyntheticRecoveryShortcutKind::UnboundedRecoveryPlan,
    ]
}
