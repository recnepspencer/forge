use super::{ExpectedProducerDelta, ExpectedTrace};
use crate::tests::domains::fintech::world::FinancialLocalityOutput;

impl ExpectedTrace {
    pub(super) fn refresh_subscriber_index(&mut self, outputs: &[FinancialLocalityOutput]) {
        self.subscribers_by_producer.clear();
        for output in outputs {
            for dependency in &output.subscriptions {
                self.subscribers_by_producer
                    .entry(dependency.upstream)
                    .or_default()
                    .push(output.id);
            }
        }
        for subscribers in self.subscribers_by_producer.values_mut() {
            subscribers.sort_unstable();
            subscribers.dedup();
        }
    }

    pub(super) fn record_delta(&mut self, delta: ExpectedProducerDelta) {
        self.commit_ordinals_by_wave_producer
            .entry((delta.admission_wave, delta.producer))
            .or_default()
            .push(delta.output_commit_ordinal);
        self.deltas.push(delta);
    }
}
