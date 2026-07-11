pub mod bounded_wal_tail_family;
pub mod checkpoint_cutover_family;
pub mod crash_boundary_family;
mod readmission_family;
pub mod recovery_source_family;
pub mod replay_index_family;

pub use bounded_wal_tail_family::{
    AdmittedBoundedWalTailLayoutFamily, AdmittedBoundedWalTailLayoutRule,
    BoundedWalTailLayoutFamilyHome, BoundedWalTailLayoutReport,
};
pub use checkpoint_cutover_family::{
    AdmittedCheckpointCutoverLayoutFamily, AdmittedRecoveryManifestLayoutRule,
    CheckpointCutoverLayoutFamilyHome, CheckpointCutoverLayoutReport,
    CheckpointRecoveryManifestLayoutReport,
};
pub use crash_boundary_family::{
    AdmittedCrashBoundaryLayoutFamily, AdmittedCrashBoundaryLayoutRule,
    CrashBoundaryLayoutFamilyHome, CrashBoundaryLayoutReport,
};
pub use readmission_family::{
    AdmittedReadmissionLayoutFamily, AdmittedReadmissionLayoutRule, ReadmissionLayoutFamilyHome,
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionClass,
    RecoveryLayoutReadmissionIdentity, RecoveryLayoutReadmissionWitness,
    RecoveryReadmissionLayoutReport,
};
pub use recovery_source_family::{
    AdmittedRecoverySourceLayoutFamily, AdmittedRecoverySourceLayoutRule,
    RecoverySourceLayoutFamilyHome, RecoverySourceLayoutReport,
};
pub use replay_index_family::{
    AdmittedReplayIndexLayoutFamily, AdmittedReplayIndexLayoutRule, ReplayIndexLayoutCounters,
    ReplayIndexLayoutFamilyHome, ReplayIndexLayoutReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLayoutAccessDenialKind {
    LocatorProjectionCannotStandInForCheckpointAuthority,
    RecoveryBlockedByIntegrityDamage,
    ReplayProjectionCannotStandInForWalAuthority,
    RecoverySourceRowCannotStandInForRecoveryAuthority,
    BackendResidueCannotStandInForCrashBoundaryAuthority,
    AmbiguousResidueCannotStandInForCrashBoundaryAuthority,
    DerivedRollbackCannotStandInForCrashBoundaryAuthority,
    BoundedWalTailLookupOutOfRange,
    ReplayTailCheckpointGap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLayoutAccessDenial {
    kind: RecoveryLayoutAccessDenialKind,
}

impl RecoveryLayoutAccessDenial {
    pub const fn new(kind: RecoveryLayoutAccessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> RecoveryLayoutAccessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryLayoutAccess;

impl RecoveryLayoutAccess {
    pub const fn s8() -> Self {
        Self
    }

    pub fn checkpoint_cutover_layout(
        self,
        rule: &AdmittedRecoveryManifestLayoutRule,
    ) -> Result<AdmittedCheckpointCutoverLayoutFamily, RecoveryLayoutAccessDenial> {
        let admission = CheckpointCutoverLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedCheckpointCutoverLayoutFamily::new(admission))
    }

    pub fn replay_index_layout(
        self,
        rule: &AdmittedReplayIndexLayoutRule,
    ) -> Result<AdmittedReplayIndexLayoutFamily, RecoveryLayoutAccessDenial> {
        let admission = ReplayIndexLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedReplayIndexLayoutFamily::new(admission))
    }

    pub fn recovery_source_layout(
        self,
        rule: &AdmittedRecoverySourceLayoutRule,
    ) -> Result<AdmittedRecoverySourceLayoutFamily, RecoveryLayoutAccessDenial> {
        let admission = RecoverySourceLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedRecoverySourceLayoutFamily::new(admission))
    }

    pub fn crash_boundary_layout(
        self,
        rule: &AdmittedCrashBoundaryLayoutRule,
    ) -> Result<AdmittedCrashBoundaryLayoutFamily, RecoveryLayoutAccessDenial> {
        let admission = CrashBoundaryLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedCrashBoundaryLayoutFamily::new(admission))
    }

    pub fn readmission_layout(
        self,
        rule: &AdmittedReadmissionLayoutRule,
    ) -> Result<AdmittedReadmissionLayoutFamily, RecoveryLayoutReadmissionAdmissionDenial> {
        let admission = ReadmissionLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedReadmissionLayoutFamily::new(admission))
    }

    pub fn bounded_wal_tail_layout(
        self,
        rule: &AdmittedBoundedWalTailLayoutRule,
    ) -> Result<AdmittedBoundedWalTailLayoutFamily, RecoveryLayoutAccessDenial> {
        let admission = BoundedWalTailLayoutFamilyHome::s8().admit(rule)?;
        Ok(AdmittedBoundedWalTailLayoutFamily::new(admission))
    }

    pub(crate) fn phase22_replay_index_family(self) -> AdmittedReplayIndexLayoutFamily {
        AdmittedReplayIndexLayoutFamily::new(
            ReplayIndexLayoutFamilyHome::s8()
                .admit(&AdmittedReplayIndexLayoutRule::internal_phase22())
                .expect("internal phase-22 replay index admission must stay valid"),
        )
    }

    pub(crate) fn phase22_recovery_source_family(self) -> AdmittedRecoverySourceLayoutFamily {
        AdmittedRecoverySourceLayoutFamily::new(
            RecoverySourceLayoutFamilyHome::s8()
                .admit(&AdmittedRecoverySourceLayoutRule::internal_phase22())
                .expect("internal phase-22 recovery source admission must stay valid"),
        )
    }

    pub(crate) fn phase22_crash_boundary_family(self) -> AdmittedCrashBoundaryLayoutFamily {
        AdmittedCrashBoundaryLayoutFamily::new(
            CrashBoundaryLayoutFamilyHome::s8()
                .admit(&AdmittedCrashBoundaryLayoutRule::internal_phase22())
                .expect("internal phase-22 crash boundary admission must stay valid"),
        )
    }

    pub(crate) fn phase22_bounded_wal_tail_family(self) -> AdmittedBoundedWalTailLayoutFamily {
        AdmittedBoundedWalTailLayoutFamily::new(
            BoundedWalTailLayoutFamilyHome::s8()
                .admit(&AdmittedBoundedWalTailLayoutRule::internal_phase22())
                .expect("internal phase-22 bounded tail admission must stay valid"),
        )
    }
}
