use std::{collections::BTreeMap, num::NonZeroUsize, sync::Arc};

use worth_signal::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, SignalGraph, SignalRuntime, TemporalCondition,
    TemporalWakeId, TemporalWakeRetirementReason,
};

use super::contract::{
    BridgeManagedClockLease, BridgeManagedDueWake, BridgeManagedDueWakeBatch,
    BridgeManagedTemporalDenial, BridgeManagedTemporalDenialKind,
    BridgeManagedTemporalIntentIdentity, BridgeManagedTemporalIntentLifecycle,
    BridgeManagedTemporalIntentReconciliation,
};

pub(super) struct BridgeManagedTemporalIntentRecord {
    revision: u64,
    due_coordinate: u64,
    idempotency_identity: Arc<str>,
    wake_id: TemporalWakeId,
}

pub(in crate::conditional_execution) struct BridgeManagedClockLane {
    pub(super) source_identity: Arc<str>,
    pub(super) timeline_identity: Arc<str>,
    pub(super) lease: Arc<BridgeManagedClockLease>,
    maximum_active_intents: usize,
    maximum_due_wakes_per_observation: NonZeroUsize,
    last_observation: Option<(u64, u64)>,
    signal: SignalRuntime<(), (), (), (), ()>,
    intents: BTreeMap<BridgeManagedTemporalIntentIdentity, BridgeManagedTemporalIntentRecord>,
    wake_to_intent: BTreeMap<TemporalWakeId, BridgeManagedTemporalIntentIdentity>,
}

impl BridgeManagedClockLane {
    pub(super) fn new(
        source_identity: Arc<str>,
        timeline_identity: Arc<str>,
        lease: Arc<BridgeManagedClockLease>,
        maximum_active_intents: usize,
        maximum_due_wakes_per_observation: NonZeroUsize,
    ) -> Self {
        Self {
            source_identity,
            timeline_identity,
            lease,
            maximum_active_intents,
            maximum_due_wakes_per_observation,
            last_observation: None,
            signal: SignalRuntime::builder(SignalGraph::new())
                .with_kernel_defaults()
                .build(),
            intents: BTreeMap::new(),
            wake_to_intent: BTreeMap::new(),
        }
    }

    pub(super) fn last_observation(&self) -> Option<(u64, u64)> {
        self.last_observation
    }

    pub(super) fn record_observation(&mut self, sequence: u64, coordinate: u64) {
        self.last_observation = Some((sequence, coordinate));
    }

    pub(super) fn advance_signal_clock(
        &mut self,
        coordinate: u64,
    ) -> Result<u64, BridgeManagedTemporalDenial> {
        self.signal
            .advance_clock(ClockAdvanceRequest::new(
                ClockDomain::MonotonicExecution,
                ClockTick::new(coordinate),
            ))
            .map(|advance| advance.ordinal().get())
            .map_err(signal_denial)
    }

    pub(super) fn reconcile_active_intent(
        &mut self,
        identity: BridgeManagedTemporalIntentIdentity,
        revision: u64,
        due_coordinate: u64,
        idempotency_identity: Arc<str>,
    ) -> Result<BridgeManagedTemporalIntentReconciliation, BridgeManagedTemporalDenial> {
        let Some(existing) = self.intents.get(&identity) else {
            return self.install_new_intent(
                identity,
                revision,
                due_coordinate,
                idempotency_identity,
            );
        };
        if revision < existing.revision {
            return Ok(BridgeManagedTemporalIntentReconciliation::Stale);
        }
        if revision == existing.revision {
            return if existing.due_coordinate == due_coordinate
                && existing.idempotency_identity == idempotency_identity
            {
                Ok(BridgeManagedTemporalIntentReconciliation::Duplicate)
            } else {
                Err(BridgeManagedTemporalDenial::new(
                    BridgeManagedTemporalDenialKind::IntentRevisionConflict,
                    "one temporal-intent revision changed its due or idempotency meaning",
                ))
            };
        }

        let old_wake = existing.wake_id;
        let replacement = self
            .signal
            .supersede_temporal_wake(
                old_wake,
                TemporalCondition::at_or_after(ClockTick::new(due_coordinate)),
                ClockTick::new(due_coordinate),
            )
            .map_err(signal_denial)?;
        self.wake_to_intent.remove(&old_wake);
        self.wake_to_intent
            .insert(replacement.scheduled().id(), identity.clone());
        self.intents.insert(
            identity,
            BridgeManagedTemporalIntentRecord {
                revision,
                due_coordinate,
                idempotency_identity,
                wake_id: replacement.scheduled().id(),
            },
        );
        Ok(BridgeManagedTemporalIntentReconciliation::Superseded)
    }

    fn install_new_intent(
        &mut self,
        identity: BridgeManagedTemporalIntentIdentity,
        revision: u64,
        due_coordinate: u64,
        idempotency_identity: Arc<str>,
    ) -> Result<BridgeManagedTemporalIntentReconciliation, BridgeManagedTemporalDenial> {
        if self.intents.len() >= self.maximum_active_intents {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::IntentCapacityExhausted,
                "managed temporal-intent capacity was exhausted before Signal admission",
            ));
        }
        let wake = self
            .signal
            .schedule_temporal_wake(
                TemporalCondition::at_or_after(ClockTick::new(due_coordinate)),
                ClockTick::new(due_coordinate),
            )
            .map_err(signal_denial)?;
        self.wake_to_intent.insert(wake.id(), identity.clone());
        self.intents.insert(
            identity,
            BridgeManagedTemporalIntentRecord {
                revision,
                due_coordinate,
                idempotency_identity,
                wake_id: wake.id(),
            },
        );
        Ok(BridgeManagedTemporalIntentReconciliation::Installed)
    }

    pub(super) fn reconcile_terminal_intent(
        &mut self,
        identity: &BridgeManagedTemporalIntentIdentity,
        revision: u64,
        lifecycle: BridgeManagedTemporalIntentLifecycle,
    ) -> Result<BridgeManagedTemporalIntentReconciliation, BridgeManagedTemporalDenial> {
        let Some(existing) = self.intents.get(identity) else {
            return Ok(BridgeManagedTemporalIntentReconciliation::TerminalNoop);
        };
        if revision < existing.revision {
            return Ok(BridgeManagedTemporalIntentReconciliation::Stale);
        }
        if revision == existing.revision {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::IntentRevisionConflict,
                "one temporal-intent revision cannot change lifecycle meaning",
            ));
        }
        let reason = match lifecycle {
            BridgeManagedTemporalIntentLifecycle::Cancelled => {
                TemporalWakeRetirementReason::Cancelled
            }
            BridgeManagedTemporalIntentLifecycle::Completed => {
                TemporalWakeRetirementReason::Consumed
            }
            BridgeManagedTemporalIntentLifecycle::Active => {
                return Err(BridgeManagedTemporalDenial::new(
                    BridgeManagedTemporalDenialKind::InvalidContract,
                    "active intent reached terminal reconciliation",
                ));
            }
        };
        let wake_id = existing.wake_id;
        self.signal
            .retire_temporal_wake(wake_id, reason)
            .map_err(signal_denial)?;
        self.intents.remove(identity);
        self.wake_to_intent.remove(&wake_id);
        Ok(BridgeManagedTemporalIntentReconciliation::Retired)
    }

    pub(super) fn promote_due(
        &mut self,
        binding_identity: &Arc<str>,
    ) -> Result<BridgeManagedDueWakeBatch, BridgeManagedTemporalDenial> {
        let bounded = self
            .signal
            .promote_due_temporal_wakes_ready_bounded(self.maximum_due_wakes_per_observation)
            .map_err(signal_denial)?;
        let frontier_before = bounded.promotion().frontier_before();
        let frontier_after = bounded.promotion().frontier_after();
        let wakes = bounded
            .promotion()
            .ready_wakes()
            .iter()
            .map(|wake| self.join_due_wake(binding_identity, wake))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BridgeManagedDueWakeBatch {
            wakes,
            due_work_remaining: bounded.due_work_remaining(),
            frontier_width_before: frontier_before.scheduled_frontier_width(),
            frontier_width_after: frontier_after.scheduled_frontier_width(),
        })
    }

    fn join_due_wake(
        &self,
        binding_identity: &Arc<str>,
        wake: &worth_signal::facade::ReadyTemporalWake,
    ) -> Result<BridgeManagedDueWake, BridgeManagedTemporalDenial> {
        let identity = self.wake_to_intent.get(&wake.id()).ok_or_else(|| {
            BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::MissingIntentAssociation,
                "Signal promoted a wake without its Bridge temporal-intent association",
            )
        })?;
        let intent = self.intents.get(identity).ok_or_else(|| {
            BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::MissingIntentAssociation,
                "Bridge wake association lost its active temporal intent",
            )
        })?;
        Ok(BridgeManagedDueWake {
            binding_identity: Arc::clone(binding_identity),
            intent_identity: identity.clone(),
            revision: intent.revision,
            idempotency_identity: Arc::clone(&intent.idempotency_identity),
            due_coordinate: intent.due_coordinate,
            ready_coordinate: wake.ready_tick().get(),
            signal_wake_id: wake.id(),
            scheduled_ordinal: wake.scheduled_ordinal(),
            ready_ordinal: wake.ready_ordinal(),
        })
    }

    pub(super) fn closure_counts(&self) -> (usize, usize, usize) {
        let wake_summary = self.signal.temporal_wake_summary();
        (
            self.intents.len(),
            wake_summary.scheduled_count() as usize,
            wake_summary.ready_count() as usize,
        )
    }
}

fn signal_denial(error: worth_signal::facade::SignalError) -> BridgeManagedTemporalDenial {
    BridgeManagedTemporalDenial::new(
        BridgeManagedTemporalDenialKind::SignalTemporalFailure,
        format!("Signal temporal authority denied managed time: {error:?}"),
    )
}
