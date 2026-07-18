use std::cell::RefCell;

use worth_store_operations::OperationalControlSessionObservation;

use super::{
    OperationalRecoveryDriverState, OperationalRecoveryProductionDriver,
    OperationalRecoveryYieldpoint,
};
use crate::OperationalRecoveryProcessCrashConfig;

impl OperationalRecoveryProductionDriver {
    pub fn crash_once_at(config: OperationalRecoveryProcessCrashConfig) -> Self {
        Self {
            state: RefCell::new(OperationalRecoveryDriverState {
                pause_at: None,
                process_crash: Some(config),
                reached: Vec::new(),
                operation_identities: Vec::new(),
                control_artifact_identities: Vec::new(),
                inspection_evidence_identity: None,
                truth_evidence_identity: None,
                latest_control_observation: None,
            }),
        }
    }

    pub(crate) fn crash_at_control_cut_if_scheduled(
        &self,
        point: OperationalRecoveryYieldpoint,
        observation: OperationalControlSessionObservation,
    ) {
        let config = {
            let mut state = self.state.borrow_mut();
            if state
                .process_crash
                .as_ref()
                .is_some_and(|config| config.yieldpoint() == point)
            {
                state.process_crash.take()
            } else {
                None
            }
        };
        if let Some(config) = config {
            config.crash_with_control_observation(observation, &self.trace());
        }
    }

    pub(super) fn crash_at_latest_control_if_scheduled(
        &self,
        point: OperationalRecoveryYieldpoint,
    ) {
        let (config, observation) = {
            let mut state = self.state.borrow_mut();
            let scheduled = state
                .process_crash
                .as_ref()
                .is_some_and(|config| config.yieldpoint() == point);
            if scheduled {
                (state.process_crash.take(), state.latest_control_observation)
            } else {
                (None, None)
            }
        };
        if let Some(config) = config {
            let observation = observation
                .expect("fresh-process crash certification requires a registered control session");
            config.crash_with_control_observation(observation, &self.trace());
        }
    }
}
