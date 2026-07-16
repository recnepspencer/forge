use super::{
    SharedAdmissionFrontier, SharedDurabilityFrontier, SharedFrontierAction,
    SharedQuarantineFrontier, SharedReachabilityFrontier, SharedVisibilityFrontier,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedFrontierDenial {
    RecoveryPrecedenceRequired,
    CrashRequired,
    QuarantineVerificationRequired,
    LiveLeaseBlocksRelease,
    QuarantineBlocksRelease,
    QuarantineBlocksReuse,
    ReleaseRequiredBeforeReuse,
    DurabilityAdmissionRequired,
    ExternalAdmissionRequired,
    ReopenRequiredAfterCrash,
    DivergenceBlocksPublication,
    QuarantineBlocksPublication,
    IllegalTransition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedFrontierModel {
    durability: SharedDurabilityFrontier,
    visibility: SharedVisibilityFrontier,
    reachability: SharedReachabilityFrontier,
    quarantine: SharedQuarantineFrontier,
    admission: SharedAdmissionFrontier,
    recovery_precedence_preserved: bool,
    crashed: bool,
}

impl SharedFrontierModel {
    pub const fn initial() -> Self {
        Self {
            durability: SharedDurabilityFrontier::Pending,
            visibility: SharedVisibilityFrontier::Stable,
            reachability: SharedReachabilityFrontier::Reachable,
            quarantine: SharedQuarantineFrontier::Clear,
            admission: SharedAdmissionFrontier::None,
            recovery_precedence_preserved: false,
            crashed: false,
        }
    }

    pub fn apply(&mut self, action: SharedFrontierAction) -> Result<(), SharedFrontierDenial> {
        match action {
            SharedFrontierAction::DurabilityAdmitted => {
                self.durability = SharedDurabilityFrontier::Admitted;
            }
            SharedFrontierAction::RecoveryPrecedencePreserved => {
                self.recovery_precedence_preserved = true;
            }
            SharedFrontierAction::LiveLeaseAcquired => {
                self.reachability = SharedReachabilityFrontier::LiveLease;
            }
            SharedFrontierAction::LeaseReleased => {
                if self.reachability != SharedReachabilityFrontier::LiveLease {
                    return Err(SharedFrontierDenial::IllegalTransition);
                }
                self.reachability = SharedReachabilityFrontier::Reachable;
            }
            SharedFrontierAction::CompactionCutover => {
                if !self.recovery_precedence_preserved {
                    return Err(SharedFrontierDenial::RecoveryPrecedenceRequired);
                }
                self.visibility = SharedVisibilityFrontier::CompactionCutover;
            }
            SharedFrontierAction::Crash => self.crashed = true,
            SharedFrontierAction::Reopen => {
                if !self.crashed {
                    return Err(SharedFrontierDenial::CrashRequired);
                }
                if self.visibility == SharedVisibilityFrontier::CompactionCutover
                    && !self.recovery_precedence_preserved
                {
                    return Err(SharedFrontierDenial::RecoveryPrecedenceRequired);
                }
                if self.visibility == SharedVisibilityFrontier::CompactionCutover {
                    self.visibility = SharedVisibilityFrontier::Reopened;
                }
                self.crashed = false;
            }
            SharedFrontierAction::QuarantineSealed => {
                self.quarantine = SharedQuarantineFrontier::Sealed;
            }
            SharedFrontierAction::QuarantineVerificationStarted => {
                if self.quarantine != SharedQuarantineFrontier::Sealed {
                    return Err(SharedFrontierDenial::IllegalTransition);
                }
                self.quarantine = SharedQuarantineFrontier::VerificationPending;
            }
            SharedFrontierAction::QuarantineReadmitted => {
                if self.quarantine != SharedQuarantineFrontier::VerificationPending {
                    return Err(SharedFrontierDenial::QuarantineVerificationRequired);
                }
                self.quarantine = SharedQuarantineFrontier::Released;
            }
            SharedFrontierAction::ReclaimDeferred => {}
            SharedFrontierAction::ReclaimReleased => {
                if self.reachability == SharedReachabilityFrontier::LiveLease {
                    return Err(SharedFrontierDenial::LiveLeaseBlocksRelease);
                }
                if self.reachability != SharedReachabilityFrontier::Reachable {
                    return Err(SharedFrontierDenial::IllegalTransition);
                }
                if matches!(
                    self.quarantine,
                    SharedQuarantineFrontier::Sealed
                        | SharedQuarantineFrontier::VerificationPending
                ) {
                    return Err(SharedFrontierDenial::QuarantineBlocksRelease);
                }
                self.reachability = SharedReachabilityFrontier::ReleaseEligible;
            }
            SharedFrontierAction::GenerationReused => {
                if matches!(
                    self.quarantine,
                    SharedQuarantineFrontier::Sealed
                        | SharedQuarantineFrontier::VerificationPending
                ) {
                    return Err(SharedFrontierDenial::QuarantineBlocksReuse);
                }
                if self.reachability != SharedReachabilityFrontier::ReleaseEligible {
                    return Err(SharedFrontierDenial::ReleaseRequiredBeforeReuse);
                }
                self.reachability = SharedReachabilityFrontier::Reused;
            }
            SharedFrontierAction::CheckpointPublicationRequested => {
                self.require_publication_frontiers(false)?;
            }
            SharedFrontierAction::ImportAdmissionPending => {
                self.admission = SharedAdmissionFrontier::ImportPending;
            }
            SharedFrontierAction::ReplicationAdmissionPending => {
                self.admission = SharedAdmissionFrontier::ReplicationPending;
            }
            SharedFrontierAction::ExternalDurabilityAdmitted => {
                if !matches!(
                    self.admission,
                    SharedAdmissionFrontier::ImportPending
                        | SharedAdmissionFrontier::ReplicationPending
                ) {
                    return Err(SharedFrontierDenial::ExternalAdmissionRequired);
                }
                self.durability = SharedDurabilityFrontier::Admitted;
                self.admission = SharedAdmissionFrontier::ExternalDurable;
            }
            SharedFrontierAction::ExternalPublicationRequested => {
                self.require_publication_frontiers(true)?;
                self.admission = SharedAdmissionFrontier::Published;
            }
            SharedFrontierAction::ReplicationDivergenceDetected => {
                if !matches!(
                    self.admission,
                    SharedAdmissionFrontier::ReplicationPending
                        | SharedAdmissionFrontier::ExternalDurable
                ) {
                    return Err(SharedFrontierDenial::IllegalTransition);
                }
                self.admission = SharedAdmissionFrontier::Divergence;
            }
        }
        Ok(())
    }

    fn require_publication_frontiers(&self, external: bool) -> Result<(), SharedFrontierDenial> {
        if self.crashed {
            return Err(SharedFrontierDenial::ReopenRequiredAfterCrash);
        }
        if matches!(
            self.quarantine,
            SharedQuarantineFrontier::Sealed | SharedQuarantineFrontier::VerificationPending
        ) {
            return Err(SharedFrontierDenial::QuarantineBlocksPublication);
        }
        if self.durability != SharedDurabilityFrontier::Admitted {
            return Err(SharedFrontierDenial::DurabilityAdmissionRequired);
        }
        if external {
            match self.admission {
                SharedAdmissionFrontier::Divergence => {
                    return Err(SharedFrontierDenial::DivergenceBlocksPublication);
                }
                SharedAdmissionFrontier::ExternalDurable => {}
                _ => return Err(SharedFrontierDenial::ExternalAdmissionRequired),
            }
        }
        Ok(())
    }

    pub const fn durability(&self) -> SharedDurabilityFrontier {
        self.durability
    }

    pub const fn visibility(&self) -> SharedVisibilityFrontier {
        self.visibility
    }

    pub const fn reachability(&self) -> SharedReachabilityFrontier {
        self.reachability
    }

    pub const fn quarantine(&self) -> SharedQuarantineFrontier {
        self.quarantine
    }

    pub const fn admission(&self) -> SharedAdmissionFrontier {
        self.admission
    }

    pub const fn recovery_precedence_preserved(&self) -> bool {
        self.recovery_precedence_preserved
    }
}
