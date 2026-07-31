use std::collections::VecDeque;

use worth_query_declaration::facade::application_schema::{
    ApplicationEffectPayload, ApplicationEffectRef,
};
use worth_runtime_bridge::facade::BridgeManagedQueueOccupancy;

use super::{WorthQueryLiveCommitBatch, WorthQueryLiveDeliverySource, WorthQueryLiveSourcePoll};
use crate::domain_computation::managed_run::WorthQueryManagedLowerExecutionBasis;

pub(in crate::domain_computation::primary_graph) struct WorthQueryLiveCauseQueue<Payload> {
    cursor: u64,
    active_batch: Option<WorthQueryActiveLiveBatch>,
    pending: VecDeque<WorthQueryBufferedLiveCause<Payload>>,
}

struct WorthQueryActiveLiveBatch {
    batch: WorthQueryLiveCommitBatch,
    next_emission: usize,
}

struct WorthQueryBufferedLiveCause<Payload> {
    commit_id: worth_relational::facade::history::CommitId,
    payload: Payload,
    occupancy: BridgeManagedQueueOccupancy,
}

pub(in crate::domain_computation::primary_graph) enum WorthQueryLiveCauseFillPosture {
    Pending,
    Overflow(u64),
    Closed,
    Unavailable,
}

impl<Payload> WorthQueryLiveCauseQueue<Payload> {
    pub(in crate::domain_computation::primary_graph) fn open(
        source: &WorthQueryLiveDeliverySource,
    ) -> Self {
        Self {
            cursor: source.open_cursor(),
            active_batch: None,
            pending: VecDeque::new(),
        }
    }

    pub(in crate::domain_computation::primary_graph) fn buffered_cause_count(&self) -> usize {
        self.pending.len()
    }

    pub(in crate::domain_computation::primary_graph) fn front(
        &self,
    ) -> Option<(worth_relational::facade::history::CommitId, &Payload)> {
        self.pending
            .front()
            .map(|cause| (cause.commit_id, &cause.payload))
    }

    pub(in crate::domain_computation::primary_graph) fn acknowledge_front(
        &mut self,
        basis: &mut WorthQueryManagedLowerExecutionBasis,
    ) -> Result<(), ()> {
        let Some(mut cause) = self.pending.pop_front() else {
            return Ok(());
        };
        if let Err(failure) = basis
            .bridge
            .release_managed_queue_occupancy(cause.occupancy)
        {
            cause.occupancy = failure.into_occupancy();
            self.pending.push_front(cause);
            return Err(());
        }
        Ok(())
    }

    pub(in crate::domain_computation::primary_graph) fn release_all(
        &mut self,
        basis: &mut WorthQueryManagedLowerExecutionBasis,
    ) -> Result<(), ()> {
        while let Some(mut cause) = self.pending.pop_front() {
            if let Err(failure) = basis
                .bridge
                .release_managed_queue_occupancy(cause.occupancy)
            {
                cause.occupancy = failure.into_occupancy();
                self.pending.push_front(cause);
                return Err(());
            }
        }
        Ok(())
    }
}

impl<Payload> WorthQueryLiveCauseQueue<Payload>
where
    Payload: ApplicationEffectPayload + Clone,
{
    pub(in crate::domain_computation::primary_graph) fn fill<Schema, Effect>(
        &mut self,
        source: &WorthQueryLiveDeliverySource,
        basis: &mut WorthQueryManagedLowerExecutionBasis,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
        capacity: usize,
        mut admits_payload: impl FnMut(&Payload) -> bool,
    ) -> WorthQueryLiveCauseFillPosture {
        let mut terminal = WorthQueryLiveCauseFillPosture::Pending;
        while self.pending.len() < capacity {
            if self.active_batch.is_none() {
                let batch = match source.poll(self.cursor) {
                    WorthQueryLiveSourcePoll::Batch(batch) => batch,
                    WorthQueryLiveSourcePoll::Pending => break,
                    WorthQueryLiveSourcePoll::Overflow { missed } => {
                        terminal = WorthQueryLiveCauseFillPosture::Overflow(missed);
                        break;
                    }
                    WorthQueryLiveSourcePoll::Closed => {
                        terminal = WorthQueryLiveCauseFillPosture::Closed;
                        break;
                    }
                };
                self.active_batch = Some(WorthQueryActiveLiveBatch {
                    batch,
                    next_emission: 0,
                });
            }
            let active = self
                .active_batch
                .as_mut()
                .expect("a source batch was installed above");
            let Some(emission) = active.batch.emissions.get(active.next_emission) else {
                self.cursor = active.batch.sequence.saturating_add(1);
                self.active_batch = None;
                continue;
            };
            active.next_emission = active.next_emission.saturating_add(1);
            let Some(payload) = emission
                .cloned_payload(&effect)
                .filter(|payload| admits_payload(payload))
            else {
                continue;
            };
            let admission = match basis.bridge.enqueue_managed_queue(1) {
                Ok(admission) => admission,
                Err(_) => return WorthQueryLiveCauseFillPosture::Unavailable,
            };
            let (_, occupancy) = admission.into_parts();
            self.pending.push_back(WorthQueryBufferedLiveCause {
                commit_id: active.batch.commit_id,
                payload,
                occupancy,
            });
        }
        terminal
    }
}
