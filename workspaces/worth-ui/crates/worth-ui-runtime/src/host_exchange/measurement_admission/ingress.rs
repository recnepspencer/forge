use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use worth_ui_host_contract::UiHostMeasurementObservation;

const COMPLETION_LIMIT: usize = 64;
const COMPLETION_BYTE_LIMIT: usize = 64 * 1024;
const COMPLETION_ENVELOPE_BYTES: usize = 48;

#[derive(Clone, Debug, PartialEq)]
pub struct UiHostMeasurementCompletion {
    observation: UiHostMeasurementObservation,
    observed_at: u64,
}

#[derive(Clone)]
pub struct WorthUiHostMeasurementIngress {
    state: Rc<RefCell<UiHostMeasurementIngressState>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementIngressDenial {
    Shutdown,
    CapacityExceeded,
}

struct UiHostMeasurementIngressState {
    shutdown: bool,
    completions: VecDeque<UiHostMeasurementCompletion>,
    byte_count: usize,
}

impl UiHostMeasurementCompletion {
    pub fn new(observation: UiHostMeasurementObservation, observed_at: u64) -> Self {
        Self {
            observation,
            observed_at,
        }
    }

    pub fn observation(&self) -> &UiHostMeasurementObservation {
        &self.observation
    }

    pub const fn observed_at(&self) -> u64 {
        self.observed_at
    }

    fn encoded_len(&self) -> usize {
        COMPLETION_ENVELOPE_BYTES + self.observation.request().encoded_len()
    }
}

impl WorthUiHostMeasurementIngress {
    pub(crate) fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(UiHostMeasurementIngressState {
                shutdown: false,
                completions: VecDeque::new(),
                byte_count: 0,
            })),
        }
    }

    pub fn enqueue(
        &self,
        completion: UiHostMeasurementCompletion,
    ) -> Result<(), UiHostMeasurementIngressDenial> {
        let mut state = self.state.borrow_mut();
        if state.shutdown {
            return Err(UiHostMeasurementIngressDenial::Shutdown);
        }
        let encoded_len = completion.encoded_len();
        if state.completions.len() >= COMPLETION_LIMIT
            || state.byte_count.saturating_add(encoded_len) > COMPLETION_BYTE_LIMIT
        {
            return Err(UiHostMeasurementIngressDenial::CapacityExceeded);
        }
        state.byte_count += encoded_len;
        state.completions.push_back(completion);
        Ok(())
    }

    pub fn pending_completion_count(&self) -> usize {
        self.state.borrow().completions.len()
    }

    pub(crate) fn drain(&self) -> Vec<UiHostMeasurementCompletion> {
        let mut state = self.state.borrow_mut();
        state.byte_count = 0;
        state.completions.drain(..).collect()
    }

    pub(crate) fn shutdown(&self) {
        let mut state = self.state.borrow_mut();
        state.shutdown = true;
        state.completions.clear();
        state.byte_count = 0;
    }
}
