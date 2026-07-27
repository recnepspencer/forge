use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use worth_signal::facade::{ResourceAttemptId, ResourceRequestHandle};
use worth_store_physical_backend::{BackendQueueExecutionPlanBinding, MediaOperationIdentity};

use super::super::{
    PhysicalSignalAspectBindingDigest, PhysicalSignalSettlementOutcome,
    PhysicalWorkCounterSnapshot, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkRecoveryDisposition, SettledPhysicalWork,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkCausalRecord {
    identity: PhysicalWorkIdentity,
    operation: super::super::PhysicalWorkOperationFamily,
    signal_request: ResourceRequestHandle,
    signal_predecessor: Option<ResourceRequestHandle>,
    signal_attempt: ResourceAttemptId,
    signal_family: super::super::PhysicalWorkSignalFamily,
    signal_binding: PhysicalSignalAspectBindingDigest,
    scheduler_binding: BackendQueueExecutionPlanBinding,
    backend_operation: Option<MediaOperationIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
    derived_completion: Option<PhysicalSignalSettlementOutcome>,
    counters: PhysicalWorkCounterSnapshot,
}

pub(in crate::physical_runtime) struct PhysicalWorkCausalLedger {
    capacity: usize,
    records: Mutex<VecDeque<PhysicalWorkCausalRecord>>,
    overflow: AtomicU64,
}

#[derive(Clone)]
pub struct PhysicalWorkCausalObservation {
    ledger: Arc<PhysicalWorkCausalLedger>,
}

impl PhysicalWorkCausalLedger {
    pub(in crate::physical_runtime) fn bounded(capacity: usize) -> Arc<Self> {
        Arc::new(Self {
            capacity,
            records: Mutex::new(VecDeque::with_capacity(capacity)),
            overflow: AtomicU64::new(0),
        })
    }

    pub(in crate::physical_runtime) fn record_settlement(
        &self,
        settled: &SettledPhysicalWork,
        counters: PhysicalWorkCounterSnapshot,
    ) {
        let record = PhysicalWorkCausalRecord {
            identity: settled.intent().identity(),
            operation: settled.intent().operation(),
            signal_request: settled.signal_request(),
            signal_predecessor: settled.signal_evidence().replaces,
            signal_attempt: settled.request_attempt(),
            signal_family: settled.signal_family(),
            signal_binding: settled.signal_binding(),
            scheduler_binding: settled.scheduler_binding(),
            backend_operation: settled
                .effect_identity()
                .map(|identity| identity.backend_operation()),
            effect_fate: settled.evidence().fate(),
            recovery: settled.recovery_disposition(),
            derived_completion: None,
            counters,
        };
        let mut records = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if records.len() == self.capacity {
            records.pop_front();
            let _ = self
                .overflow
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                    Some(value.saturating_add(1))
                });
        }
        records.push_back(record);
    }

    pub(in crate::physical_runtime) fn record_derived_completion(
        &self,
        identity: PhysicalWorkIdentity,
        outcome: PhysicalSignalSettlementOutcome,
    ) {
        if let Some(record) = self
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter_mut()
            .rev()
            .find(|record| record.identity == identity)
        {
            record.derived_completion = Some(outcome);
        }
    }
}

impl PhysicalWorkCausalObservation {
    pub(super) fn new(ledger: Arc<PhysicalWorkCausalLedger>) -> Self {
        Self { ledger }
    }

    pub fn records(&self) -> Box<[PhysicalWorkCausalRecord]> {
        self.ledger
            .records
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .copied()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn overflow(&self) -> u64 {
        self.ledger.overflow.load(Ordering::Acquire)
    }
}

impl PhysicalWorkCausalRecord {
    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn operation(self) -> super::super::PhysicalWorkOperationFamily {
        self.operation
    }

    pub const fn signal_request(self) -> ResourceRequestHandle {
        self.signal_request
    }

    pub const fn signal_predecessor(self) -> Option<ResourceRequestHandle> {
        self.signal_predecessor
    }

    pub const fn signal_attempt(self) -> ResourceAttemptId {
        self.signal_attempt
    }

    pub const fn signal_family(self) -> super::super::PhysicalWorkSignalFamily {
        self.signal_family
    }

    pub const fn signal_binding(self) -> PhysicalSignalAspectBindingDigest {
        self.signal_binding
    }

    pub const fn scheduler_binding(self) -> BackendQueueExecutionPlanBinding {
        self.scheduler_binding
    }

    pub const fn backend_operation(self) -> Option<MediaOperationIdentity> {
        self.backend_operation
    }

    pub const fn effect_fate(self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }

    pub const fn derived_completion(self) -> Option<PhysicalSignalSettlementOutcome> {
        self.derived_completion
    }

    pub const fn counters(self) -> PhysicalWorkCounterSnapshot {
        self.counters
    }
}
