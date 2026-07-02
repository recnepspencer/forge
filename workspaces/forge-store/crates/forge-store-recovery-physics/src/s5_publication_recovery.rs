use crate::S5RecoveryReadinessAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5PublicationCrashStage {
    BeforePublication,
    DuringPublication,
    AfterPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum S5PublicationRecoveryFaultInjection {
    None,
    MixedTree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S5RecoveredPublicationStructureKind {
    OldStableStructure,
    NewStableStructure,
    MixedOldAndNewStructure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5RecoveredPublicationStructure {
    kind: S5RecoveredPublicationStructureKind,
    old_root_epoch: u64,
    old_manifest_epoch: u64,
    new_root_epoch: u64,
    new_manifest_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S5PublicationRecoveryReplayInput {
    stage: S5PublicationCrashStage,
    fault_injection: S5PublicationRecoveryFaultInjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutedS5PublicationRecoveryReceipt {
    stage: S5PublicationCrashStage,
    recovered_kind: S5RecoveredPublicationStructureKind,
    recovery_replayed_frames: usize,
}

impl S5RecoveredPublicationStructure {
    pub const fn old_stable_for_publication_admission(
        root_epoch: u64,
        manifest_epoch: u64,
    ) -> Self {
        Self::old_stable(root_epoch, manifest_epoch)
    }

    pub const fn new_stable_for_publication_admission(
        root_epoch: u64,
        manifest_epoch: u64,
    ) -> Self {
        Self::new_stable(root_epoch, manifest_epoch)
    }

    const fn old_stable(root_epoch: u64, manifest_epoch: u64) -> Self {
        Self {
            kind: S5RecoveredPublicationStructureKind::OldStableStructure,
            old_root_epoch: root_epoch,
            old_manifest_epoch: manifest_epoch,
            new_root_epoch: root_epoch,
            new_manifest_epoch: manifest_epoch,
        }
    }

    const fn new_stable(root_epoch: u64, manifest_epoch: u64) -> Self {
        Self {
            kind: S5RecoveredPublicationStructureKind::NewStableStructure,
            old_root_epoch: root_epoch,
            old_manifest_epoch: manifest_epoch,
            new_root_epoch: root_epoch,
            new_manifest_epoch: manifest_epoch,
        }
    }

    pub const fn kind(self) -> S5RecoveredPublicationStructureKind {
        self.kind
    }

    pub const fn stable_root_epoch(self) -> Option<u64> {
        match self.kind {
            S5RecoveredPublicationStructureKind::OldStableStructure => Some(self.old_root_epoch),
            S5RecoveredPublicationStructureKind::NewStableStructure => Some(self.new_root_epoch),
            S5RecoveredPublicationStructureKind::MixedOldAndNewStructure => None,
        }
    }

    pub const fn stable_manifest_epoch(self) -> Option<u64> {
        match self.kind {
            S5RecoveredPublicationStructureKind::OldStableStructure => {
                Some(self.old_manifest_epoch)
            }
            S5RecoveredPublicationStructureKind::NewStableStructure => {
                Some(self.new_manifest_epoch)
            }
            S5RecoveredPublicationStructureKind::MixedOldAndNewStructure => None,
        }
    }

    pub const fn old_root_epoch(self) -> u64 {
        self.old_root_epoch
    }

    pub const fn old_manifest_epoch(self) -> u64 {
        self.old_manifest_epoch
    }

    pub const fn new_root_epoch(self) -> u64 {
        self.new_root_epoch
    }

    pub const fn new_manifest_epoch(self) -> u64 {
        self.new_manifest_epoch
    }
}

impl S5PublicationRecoveryReplayInput {
    pub const fn from_crash_stage(stage: S5PublicationCrashStage) -> Self {
        Self {
            stage,
            fault_injection: S5PublicationRecoveryFaultInjection::None,
        }
    }

    pub const fn mixed_tree_fault_attempt(stage: S5PublicationCrashStage) -> Self {
        Self {
            stage,
            fault_injection: S5PublicationRecoveryFaultInjection::MixedTree,
        }
    }
}

impl S5RecoveryReadinessAdmission {
    pub const fn execute_publication_recovery_replay(
        &self,
        replay: S5PublicationRecoveryReplayInput,
    ) -> ExecutedS5PublicationRecoveryReceipt {
        replay.execute_with_readiness(self)
    }
}

impl S5PublicationRecoveryReplayInput {
    const fn execute_with_readiness(
        self,
        recovery_readiness: &S5RecoveryReadinessAdmission,
    ) -> ExecutedS5PublicationRecoveryReceipt {
        ExecutedS5PublicationRecoveryReceipt {
            stage: self.stage,
            recovered_kind: self.recover_structure_kind(),
            recovery_replayed_frames: recovery_readiness.replayed_frames(),
        }
    }

    const fn recover_structure_kind(self) -> S5RecoveredPublicationStructureKind {
        match self.fault_injection {
            S5PublicationRecoveryFaultInjection::MixedTree => {
                S5RecoveredPublicationStructureKind::MixedOldAndNewStructure
            }
            S5PublicationRecoveryFaultInjection::None => match self.stage {
                S5PublicationCrashStage::BeforePublication => {
                    S5RecoveredPublicationStructureKind::OldStableStructure
                }
                S5PublicationCrashStage::DuringPublication
                | S5PublicationCrashStage::AfterPublication => {
                    S5RecoveredPublicationStructureKind::NewStableStructure
                }
            },
        }
    }
}

impl ExecutedS5PublicationRecoveryReceipt {
    pub const fn stage(self) -> S5PublicationCrashStage {
        self.stage
    }

    pub const fn recovered_kind(self) -> S5RecoveredPublicationStructureKind {
        self.recovered_kind
    }

    pub const fn recovery_replayed_frames(self) -> usize {
        self.recovery_replayed_frames
    }
}
