use worth_store_contracts::StableDigest;
use worth_store_recovery_physics::{
    S4IntegrityHandoffCounters, S4RecoveryPhysicsIntegrityReadiness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S3S4HandoffCloseoutEvidence {
    handoff_identity: StableDigest,
    counters: S4IntegrityHandoffCounters,
    no_raw_bytes_crossed: bool,
    recovery_claimed: bool,
}

impl S3S4HandoffCloseoutEvidence {
    pub fn from_readiness(readiness: &S4RecoveryPhysicsIntegrityReadiness) -> Self {
        Self {
            handoff_identity: readiness.payload().identity().clone(),
            counters: readiness.counters(),
            no_raw_bytes_crossed: readiness.proves_no_raw_bytes_crossed(),
            recovery_claimed: readiness.claims_recovery(),
        }
    }

    pub const fn handoff_identity(&self) -> &StableDigest {
        &self.handoff_identity
    }

    pub const fn counters(&self) -> S4IntegrityHandoffCounters {
        self.counters
    }

    pub const fn proves_no_raw_bytes_crossed(&self) -> bool {
        self.no_raw_bytes_crossed
    }

    pub const fn claims_recovery(&self) -> bool {
        self.recovery_claimed
    }

    pub fn matches_readiness(&self, readiness: &S4RecoveryPhysicsIntegrityReadiness) -> bool {
        self.handoff_identity == *readiness.payload().identity()
            && self.counters == readiness.counters()
            && self.no_raw_bytes_crossed == readiness.proves_no_raw_bytes_crossed()
            && self.recovery_claimed == readiness.claims_recovery()
    }
}
