use std::collections::VecDeque;

use worth_relational::facade::history::CommitId;

use super::super::application_attempt::{
    WorthQueryAdmittedApplicationEmissionBatch, WorthQueryApplicationEmission,
};

const RETAINED_COMMIT_BATCH_CAPACITY: usize = 64;

#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryLiveDeliverySource {
    state: std::sync::Mutex<WorthQueryLiveDeliverySourceState>,
}

#[derive(Default)]
struct WorthQueryLiveDeliverySourceState {
    batches: VecDeque<WorthQueryLiveCommitBatch>,
    next_sequence: u64,
    last_commit_id: Option<u64>,
    published_commit_count: usize,
    retained_payload_bytes: u64,
    closed: bool,
}

#[derive(Clone)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryLiveCommitBatch {
    pub(super) sequence: u64,
    pub(super) commit_id: CommitId,
    pub(super) emissions: Vec<WorthQueryApplicationEmission>,
    retained_payload_bytes: u64,
}

pub(in crate::domain_computation::primary_graph) enum WorthQueryLiveSourcePoll {
    Batch(WorthQueryLiveCommitBatch),
    Pending,
    Overflow { missed: u64 },
    Closed,
}

impl WorthQueryLiveDeliverySource {
    pub(in crate::domain_computation::primary_graph) fn admit_publication(
        &self,
        commit_id: CommitId,
    ) -> Result<(), &'static str> {
        let state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state
            .last_commit_id
            .is_some_and(|last_commit_id| commit_id.0 <= last_commit_id)
        {
            Err("application commit causality is not strictly ordered")
        } else {
            Ok(())
        }
    }

    pub(in crate::domain_computation::primary_graph) fn publish(
        &self,
        commit_id: CommitId,
        emissions: WorthQueryAdmittedApplicationEmissionBatch,
    ) -> Result<usize, &'static str> {
        let (emissions, retained_payload_bytes) = emissions.into_parts();
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let emitted = emissions.len();
        if state
            .last_commit_id
            .is_some_and(|last_commit_id| commit_id.0 <= last_commit_id)
        {
            return Err("application commit causality is not strictly ordered");
        }
        state.last_commit_id = Some(commit_id.0);
        state.published_commit_count = state.published_commit_count.saturating_add(1);
        if state.closed {
            return Ok(emitted);
        }
        let sequence = state.next_sequence;
        state.next_sequence = state.next_sequence.saturating_add(1);
        state.batches.push_back(WorthQueryLiveCommitBatch {
            sequence,
            commit_id,
            emissions,
            retained_payload_bytes,
        });
        state.retained_payload_bytes = state
            .retained_payload_bytes
            .checked_add(retained_payload_bytes)
            .ok_or("live delivery retained-byte count overflowed")?;
        if let Some(evicted) = (state.batches.len() > RETAINED_COMMIT_BATCH_CAPACITY)
            .then(|| state.batches.pop_front())
            .flatten()
        {
            state.retained_payload_bytes = state
                .retained_payload_bytes
                .checked_sub(evicted.retained_payload_bytes)
                .ok_or("live delivery retained-byte count underflowed")?;
        }
        Ok(emitted)
    }

    pub(in crate::domain_computation::primary_graph) fn open_cursor(&self) -> u64 {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .next_sequence
    }

    pub(in crate::domain_computation::primary_graph) fn poll(
        &self,
        cursor: u64,
    ) -> WorthQueryLiveSourcePoll {
        let state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let first = state
            .batches
            .front()
            .map_or(state.next_sequence, |batch| batch.sequence);
        if cursor < first {
            return WorthQueryLiveSourcePoll::Overflow {
                missed: first - cursor,
            };
        }
        if let Some(batch) = state.batches.iter().find(|batch| batch.sequence == cursor) {
            return WorthQueryLiveSourcePoll::Batch(batch.clone());
        }
        if state.closed {
            WorthQueryLiveSourcePoll::Closed
        } else {
            WorthQueryLiveSourcePoll::Pending
        }
    }

    pub(in crate::domain_computation::primary_graph) fn close(&self) {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .closed = true;
    }

    #[cfg(test)]
    pub(in crate::domain_computation::primary_graph) fn emissions(
        &self,
        commit_id: CommitId,
    ) -> Vec<WorthQueryApplicationEmission> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .batches
            .iter()
            .find(|batch| batch.commit_id == commit_id)
            .map(|batch| batch.emissions.clone())
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(in crate::domain_computation::primary_graph) fn published_commit_count(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .published_commit_count
    }

    #[cfg(test)]
    pub(in crate::domain_computation::primary_graph) fn retained_payload_bytes(&self) -> u64 {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .retained_payload_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_commit_batches_report_exact_overflow_and_closure() {
        let source = WorthQueryLiveDeliverySource::default();
        let cursor = source.open_cursor();
        for ordinal in 1..=RETAINED_COMMIT_BATCH_CAPACITY + 1 {
            source
                .publish(
                    CommitId(ordinal as u64),
                    WorthQueryAdmittedApplicationEmissionBatch::admit(Vec::new(), 0).unwrap(),
                )
                .expect("unique commit should publish");
        }
        assert!(matches!(
            source.poll(cursor),
            WorthQueryLiveSourcePoll::Overflow { missed: 1 }
        ));

        let current = source.open_cursor();
        assert!(matches!(
            source.poll(current),
            WorthQueryLiveSourcePoll::Pending
        ));
        source.close();
        assert!(matches!(
            source.poll(current),
            WorthQueryLiveSourcePoll::Closed
        ));
        source
            .publish(
                CommitId(999),
                WorthQueryAdmittedApplicationEmissionBatch::admit(Vec::new(), 0).unwrap(),
            )
            .expect("delivery closure must not reject later authoritative commits");
        assert_eq!(
            source.published_commit_count(),
            RETAINED_COMMIT_BATCH_CAPACITY + 2
        );
        assert_eq!(source.retained_payload_bytes(), 0);
    }
}
