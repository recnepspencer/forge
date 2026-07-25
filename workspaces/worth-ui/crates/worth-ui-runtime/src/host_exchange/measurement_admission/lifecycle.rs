use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui_host_contract::{
    UiHostMeasurementDeadline, UiHostMeasurementObservation, UiHostMeasurementRequest,
    UiMeasurementRequestIdentity, WorthUiHostCapabilityReport,
};

use super::model::UiHostMeasurementDependencyBasis;
use super::{
    UiHostMeasurementCurrentTruth, UiHostMeasurementDenial, UiHostMeasurementIntent,
    UiHostMeasurementOutcome, UiRequestedHostMeasurement, UiSolicitedHostMeasurementResult,
};

const PENDING_REQUEST_LIMIT: usize = 64;
const PENDING_REQUEST_BYTE_LIMIT: usize = 64 * 1024;
const TERMINAL_REQUEST_LIMIT: usize = 128;
static NEXT_MEASUREMENT_REQUEST: AtomicU64 = AtomicU64::new(1);

struct UiPendingHostMeasurement {
    request: Rc<UiHostMeasurementRequest>,
    basis: UiHostMeasurementDependencyBasis,
    deadline: UiHostMeasurementDeadline,
}

pub(crate) struct UiHostMeasurementAdmission {
    ingress: super::WorthUiHostMeasurementIngress,
    pending: BTreeMap<UiMeasurementRequestIdentity, UiPendingHostMeasurement>,
    pending_bytes: usize,
    terminal: BTreeSet<UiMeasurementRequestIdentity>,
    terminal_order: VecDeque<UiMeasurementRequestIdentity>,
    shutdown: bool,
}

impl Default for UiHostMeasurementAdmission {
    fn default() -> Self {
        Self {
            ingress: super::WorthUiHostMeasurementIngress::new(),
            pending: BTreeMap::new(),
            pending_bytes: 0,
            terminal: BTreeSet::new(),
            terminal_order: VecDeque::new(),
            shutdown: false,
        }
    }
}

impl UiHostMeasurementAdmission {
    pub(crate) fn begin(
        &mut self,
        intent: UiHostMeasurementIntent,
        current: UiHostMeasurementCurrentTruth,
        capability_report: &WorthUiHostCapabilityReport,
        now: u64,
    ) -> UiHostMeasurementOutcome {
        if self.shutdown {
            return denied(UiHostMeasurementDenial::Shutdown);
        }
        let identity = match next_request_identity() {
            Some(identity) => identity,
            None => return denied(UiHostMeasurementDenial::IdentityExhausted),
        };
        let (binding, request_intent, deadline) = intent.into_parts();
        if deadline.expired_at(now) {
            self.remember_terminal(identity);
            return UiHostMeasurementOutcome::Expired(identity);
        }
        if self.pending.len() >= PENDING_REQUEST_LIMIT {
            return denied(UiHostMeasurementDenial::CapacityExceeded);
        }
        let family = request_intent.family();
        let basis = match current.basis_for(family, binding) {
            Ok(basis) => basis,
            Err(denial) => return denied(denial),
        };
        let request = match request_intent.issue(identity, capability_report) {
            Ok(request) => Rc::new(request),
            Err(denial) => {
                return denied(UiHostMeasurementDenial::RequestDenied(denial));
            }
        };
        let bytes = request.encoded_len();
        if self.pending_bytes.saturating_add(bytes) > PENDING_REQUEST_BYTE_LIMIT {
            return denied(UiHostMeasurementDenial::ByteCapacityExceeded);
        }
        self.pending_bytes += bytes;
        let requested = UiRequestedHostMeasurement::new(Rc::clone(&request));
        self.pending.insert(
            identity,
            UiPendingHostMeasurement {
                request,
                basis,
                deadline,
            },
        );
        UiHostMeasurementOutcome::Admitted(requested)
    }

    pub(crate) fn complete(
        &mut self,
        observation: UiHostMeasurementObservation,
        current: UiHostMeasurementCurrentTruth,
        now: u64,
    ) -> UiHostMeasurementOutcome {
        if self.shutdown {
            return denied(UiHostMeasurementDenial::Shutdown);
        }
        let identity = observation.request_identity();
        if self.terminal.contains(&identity) {
            return UiHostMeasurementOutcome::DuplicateSuppressed(identity);
        }
        let Some(pending) = self.pending.get(&identity) else {
            return denied(UiHostMeasurementDenial::UnknownRequest);
        };
        if pending.request.as_ref() != observation.request() {
            return denied(UiHostMeasurementDenial::StaleBasis);
        }
        let deadline_expired = pending.deadline.expired_at(now);
        let stale = !current.still_satisfies(pending.request.family(), pending.basis);
        let pending = self
            .pending
            .remove(&identity)
            .expect("validated pending request remains owned by the lifecycle");
        self.pending_bytes -= pending.request.encoded_len();
        self.remember_terminal(identity);
        if deadline_expired {
            return UiHostMeasurementOutcome::Expired(identity);
        }
        if stale {
            return denied(UiHostMeasurementDenial::StaleBasis);
        }
        UiHostMeasurementOutcome::Completed(UiSolicitedHostMeasurementResult::new(observation))
    }

    pub(crate) fn cancel(
        &mut self,
        identity: UiMeasurementRequestIdentity,
    ) -> UiHostMeasurementOutcome {
        if self.shutdown {
            return denied(UiHostMeasurementDenial::Shutdown);
        }
        if self.terminal.contains(&identity) {
            return UiHostMeasurementOutcome::DuplicateSuppressed(identity);
        }
        let Some(pending) = self.pending.remove(&identity) else {
            return denied(UiHostMeasurementDenial::UnknownRequest);
        };
        self.pending_bytes -= pending.request.encoded_len();
        self.remember_terminal(identity);
        UiHostMeasurementOutcome::Cancelled(identity)
    }

    pub(crate) fn expire(&mut self, now: u64) -> Box<[UiHostMeasurementOutcome]> {
        let expired = self
            .pending
            .iter()
            .filter_map(|(identity, pending)| pending.deadline.expired_at(now).then_some(*identity))
            .collect::<Vec<_>>();
        expired
            .into_iter()
            .map(|identity| {
                let pending = self
                    .pending
                    .remove(&identity)
                    .expect("expired identity was pending");
                self.pending_bytes -= pending.request.encoded_len();
                self.remember_terminal(identity);
                UiHostMeasurementOutcome::Expired(identity)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(crate) fn shutdown(&mut self) {
        self.ingress.shutdown();
        self.shutdown = true;
        self.pending.clear();
        self.pending_bytes = 0;
    }

    pub(crate) fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub(crate) fn pending_binding(
        &self,
        identity: UiMeasurementRequestIdentity,
    ) -> Option<Option<worth_ui_host_contract::UiSurfaceBindingGeneration>> {
        self.pending
            .get(&identity)
            .map(|pending| pending.basis.binding())
    }

    pub(crate) fn pending_bytes(&self) -> usize {
        self.pending_bytes
    }

    pub(crate) fn ingress(&self) -> super::WorthUiHostMeasurementIngress {
        self.ingress.clone()
    }

    pub(crate) fn drain_ingress(&self) -> Vec<super::UiHostMeasurementCompletion> {
        self.ingress.drain()
    }

    fn remember_terminal(&mut self, identity: UiMeasurementRequestIdentity) {
        if self.terminal.insert(identity) {
            self.terminal_order.push_back(identity);
        }
        while self.terminal_order.len() > TERMINAL_REQUEST_LIMIT {
            let forgotten = self
                .terminal_order
                .pop_front()
                .expect("over-limit terminal measurement queue is non-empty");
            self.terminal.remove(&forgotten);
        }
    }
}

fn denied(denial: UiHostMeasurementDenial) -> UiHostMeasurementOutcome {
    UiHostMeasurementOutcome::Denied(denial)
}

fn next_request_identity() -> Option<UiMeasurementRequestIdentity> {
    NEXT_MEASUREMENT_REQUEST
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .ok()
        .map(UiMeasurementRequestIdentity::new)
}
