use crate::{S4IntegrityHandoffCounters, S4IntegrityHandoffPayload};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4RecoveryPhysicsIntegrityReadiness {
    payload: S4IntegrityHandoffPayload,
}

impl S4RecoveryPhysicsIntegrityReadiness {
    pub fn from_s3_integrity_handoff(payload: S4IntegrityHandoffPayload) -> Self {
        Self { payload }
    }

    pub const fn payload(&self) -> &S4IntegrityHandoffPayload {
        &self.payload
    }

    pub const fn counters(&self) -> S4IntegrityHandoffCounters {
        self.payload.counters()
    }

    pub fn proves_no_raw_bytes_crossed(&self) -> bool {
        self.payload.proves_no_raw_bytes_crossed()
    }

    pub const fn claims_recovery(&self) -> bool {
        false
    }
}
