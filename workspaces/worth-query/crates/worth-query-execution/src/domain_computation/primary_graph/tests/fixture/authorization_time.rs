use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::domain_computation::authorization::{
    WorthQueryAuthorizationTimeSource, WorthQueryAuthorizationTimeSourceDenial,
};

#[derive(Clone, Default)]
pub(in crate::domain_computation::primary_graph) struct AuthorizationTimeController {
    state: Arc<Mutex<AuthorizationTimeState>>,
}

#[derive(Default)]
enum AuthorizationTimeState {
    #[default]
    System,
    Scripted(VecDeque<SystemTime>),
}

impl AuthorizationTimeController {
    pub(in crate::domain_computation::primary_graph) fn script(
        &self,
        samples: impl IntoIterator<Item = SystemTime>,
    ) {
        *self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) =
            AuthorizationTimeState::Scripted(samples.into_iter().collect());
    }
}

impl WorthQueryAuthorizationTimeSource for AuthorizationTimeController {
    fn current_time(&self) -> Result<SystemTime, WorthQueryAuthorizationTimeSourceDenial> {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match &mut *state {
            AuthorizationTimeState::System => Ok(SystemTime::now()),
            AuthorizationTimeState::Scripted(samples) => samples
                .pop_front()
                .ok_or(WorthQueryAuthorizationTimeSourceDenial::Unavailable),
        }
    }
}
