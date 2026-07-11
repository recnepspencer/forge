use super::{
    AdmittedBoundedWalTailLayoutFamily, AdmittedBoundedWalTailLayoutRule,
    AdmittedCheckpointCutoverLayoutFamily, AdmittedCrashBoundaryLayoutFamily,
    AdmittedCrashBoundaryLayoutRule, AdmittedRecoveryManifestLayoutRule,
    AdmittedRecoverySourceLayoutFamily, AdmittedRecoverySourceLayoutRule,
    AdmittedReplayIndexLayoutFamily, AdmittedReplayIndexLayoutRule,
    BoundedWalTailLayoutFamilyHome, CheckpointCutoverLayoutFamilyHome,
    CrashBoundaryLayoutFamilyHome, RecoveryLayoutAccessDenial, RecoverySourceLayoutFamilyHome,
    ReplayIndexLayoutFamilyHome,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryLayoutAccess;

impl RecoveryLayoutAccess {
    pub const fn s8() -> Self { Self }

    pub fn checkpoint_cutover_layout(self, rule: &AdmittedRecoveryManifestLayoutRule) -> Result<AdmittedCheckpointCutoverLayoutFamily, RecoveryLayoutAccessDenial> {
        Ok(AdmittedCheckpointCutoverLayoutFamily::new(CheckpointCutoverLayoutFamilyHome::s8().admit(rule)?))
    }

    pub fn replay_index_layout(self, rule: &AdmittedReplayIndexLayoutRule) -> Result<AdmittedReplayIndexLayoutFamily, RecoveryLayoutAccessDenial> {
        Ok(AdmittedReplayIndexLayoutFamily::new(ReplayIndexLayoutFamilyHome::s8().admit(rule)?))
    }

    pub fn recovery_source_layout(self, rule: &AdmittedRecoverySourceLayoutRule) -> Result<AdmittedRecoverySourceLayoutFamily, RecoveryLayoutAccessDenial> {
        Ok(AdmittedRecoverySourceLayoutFamily::new(RecoverySourceLayoutFamilyHome::s8().admit(rule)?))
    }

    pub fn crash_boundary_layout(self, rule: &AdmittedCrashBoundaryLayoutRule) -> Result<AdmittedCrashBoundaryLayoutFamily, RecoveryLayoutAccessDenial> {
        Ok(AdmittedCrashBoundaryLayoutFamily::new(CrashBoundaryLayoutFamilyHome::s8().admit(rule)?))
    }

    pub fn bounded_wal_tail_layout(self, rule: &AdmittedBoundedWalTailLayoutRule) -> Result<AdmittedBoundedWalTailLayoutFamily, RecoveryLayoutAccessDenial> {
        Ok(AdmittedBoundedWalTailLayoutFamily::new(BoundedWalTailLayoutFamilyHome::s8().admit(rule)?))
    }
}
