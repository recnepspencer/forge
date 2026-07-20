use std::cell::RefCell;

use super::{
    OperationalRecoveryDriverState, OperationalRecoveryProductionDriver,
    OperationalRecoveryYieldpoint,
};
use crate::operational_recovery_trace::OperationalRecoveryDriverTrace;

impl OperationalRecoveryProductionDriver {
    pub const fn uninterrupted() -> Self {
        Self::with_pause(None)
    }

    pub const fn pause_once_at(yieldpoint: OperationalRecoveryYieldpoint) -> Self {
        Self::with_pause(Some(yieldpoint))
    }

    const fn with_pause(pause_at: Option<OperationalRecoveryYieldpoint>) -> Self {
        Self {
            state: RefCell::new(OperationalRecoveryDriverState {
                pause_at,
                process_crash: None,
                reached: Vec::new(),
                operation_identities: Vec::new(),
                control_artifact_identities: Vec::new(),
                inspection_evidence_identity: None,
                truth_evidence_identity: None,
                latest_control_observation: None,
            }),
        }
    }

    pub fn trace(&self) -> OperationalRecoveryDriverTrace {
        let state = self.state.borrow();
        OperationalRecoveryDriverTrace::from_observations(
            state.reached.clone(),
            state.operation_identities.clone(),
            state.control_artifact_identities.clone(),
            state.inspection_evidence_identity,
            state.truth_evidence_identity,
        )
    }
}
