use crate::{IntegrityHandoffCounters, IntegrityHandoffPayload};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4RecoveryPhysicsIntegrityReadiness {
    payload: IntegrityHandoffPayload,
}

impl S4RecoveryPhysicsIntegrityReadiness {
    pub(crate) fn from_admitted_s3_handoff(payload: IntegrityHandoffPayload) -> Self {
        Self { payload }
    }

    pub const fn payload(&self) -> &IntegrityHandoffPayload {
        &self.payload
    }

    pub const fn counters(&self) -> IntegrityHandoffCounters {
        self.payload().counters()
    }

    pub const fn proves_no_raw_bytes_crossed(&self) -> bool {
        self.payload().proves_no_raw_bytes_crossed()
    }

    pub const fn claims_recovery(&self) -> bool {
        false
    }
}
