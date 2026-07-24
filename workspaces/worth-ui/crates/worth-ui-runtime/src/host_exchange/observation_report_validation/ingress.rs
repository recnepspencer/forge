use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use worth_ui_host_contract::UiHostObservationBatch;

const INGRESS_BATCH_LIMIT: usize = 16;
const INGRESS_REPORT_LIMIT: usize = 256;
const INGRESS_BYTE_LIMIT: usize = 64 * 1024;

#[derive(Clone)]
pub struct WorthUiHostObservationIngress {
    state: Rc<RefCell<UiHostObservationIngressState>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationIngressDenial {
    Shutdown,
    MalformedBatch,
    CapacityExceeded,
}

struct UiHostObservationIngressState {
    shutdown: bool,
    batches: VecDeque<UiHostObservationBatch>,
    report_count: usize,
    byte_count: usize,
}

impl WorthUiHostObservationIngress {
    pub(crate) fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(UiHostObservationIngressState {
                shutdown: false,
                batches: VecDeque::new(),
                report_count: 0,
                byte_count: 0,
            })),
        }
    }

    pub fn enqueue(
        &self,
        batch: UiHostObservationBatch,
    ) -> Result<(), UiHostObservationIngressDenial> {
        batch
            .validate_shape()
            .map_err(|_| UiHostObservationIngressDenial::MalformedBatch)?;
        let core = batch.canonical_core();
        let mut state = self.state.borrow_mut();
        if state.shutdown {
            return Err(UiHostObservationIngressDenial::Shutdown);
        }
        if state.batches.len() >= INGRESS_BATCH_LIMIT
            || state.report_count.saturating_add(core.report_count()) > INGRESS_REPORT_LIMIT
            || state.byte_count.saturating_add(core.byte_count()) > INGRESS_BYTE_LIMIT
        {
            return Err(UiHostObservationIngressDenial::CapacityExceeded);
        }
        state.report_count += core.report_count();
        state.byte_count += core.byte_count();
        state.batches.push_back(batch);
        Ok(())
    }

    pub fn pending_batch_count(&self) -> usize {
        self.state.borrow().batches.len()
    }

    pub(crate) fn drain(&self) -> Vec<UiHostObservationBatch> {
        let mut state = self.state.borrow_mut();
        state.report_count = 0;
        state.byte_count = 0;
        state.batches.drain(..).collect()
    }

    pub(crate) fn shutdown(&self) {
        let mut state = self.state.borrow_mut();
        state.shutdown = true;
        state.batches.clear();
        state.report_count = 0;
        state.byte_count = 0;
    }
}
