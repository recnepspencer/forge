use std::sync::{Arc, Mutex};

use crate::data::proof::invalidation::progression::InvalidationWorkBindingAxes;
use crate::logic::transaction::{SignalObservationCaptureGate, SignalObservationSurface};

/// Optional performed-work retention, separate from numeric counter storage.
#[derive(Debug)]
pub(crate) struct PerformedWorkCaptureState {
    capture_gate: SignalObservationCaptureGate,
    bindings: Arc<Mutex<Vec<InvalidationWorkBindingAxes>>>,
}

impl Default for PerformedWorkCaptureState {
    fn default() -> Self {
        Self::with_capture_gate(SignalObservationCaptureGate::default())
    }
}

impl PerformedWorkCaptureState {
    pub(crate) fn with_capture_gate(capture_gate: SignalObservationCaptureGate) -> Self {
        Self {
            capture_gate,
            bindings: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(crate) fn reset(&self) {
        self.bindings
            .lock()
            .expect("performed work observation poisoned")
            .clear();
    }

    pub(crate) fn shared_bindings(&self) -> Arc<Mutex<Vec<InvalidationWorkBindingAxes>>> {
        Arc::clone(&self.bindings)
    }

    pub(crate) fn record(&self, binding: InvalidationWorkBindingAxes) {
        if !self
            .capture_gate
            .captures(SignalObservationSurface::PerformedWork)
        {
            return;
        }
        self.bindings
            .lock()
            .expect("performed work observation poisoned")
            .push(binding);
    }

    pub(crate) fn snapshot(&self) -> Vec<InvalidationWorkBindingAxes> {
        self.bindings
            .lock()
            .expect("performed work observation poisoned")
            .clone()
    }
}
