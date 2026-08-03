use super::super::PhysicalResidencyWorkPort;
use super::{
    outcome::{classify_loaded_frame, prefetch_outcome},
    PhysicalPrefetchIntent, PhysicalPrefetchOutcome, PhysicalReadAheadBatch,
    PhysicalReadAheadFrameOutcome, PhysicalReadAheadIntent, PhysicalReadAheadOutcome,
    PhysicalSpeculativeReadDrop, PhysicalSpeculativeReadFailure,
};
use crate::physical_runtime::LifecycleGeneration;

impl PhysicalResidencyWorkPort {
    pub(in crate::physical_runtime::record_serving) fn prefetch(
        &self,
        intent: PhysicalPrefetchIntent,
        generation: LifecycleGeneration,
    ) -> PhysicalPrefetchOutcome {
        let grant = match self.admit_prefetch(intent.coordinate()) {
            Ok(grant) => grant,
            Err(denial) => {
                return PhysicalPrefetchOutcome::Dropped(PhysicalSpeculativeReadDrop::bind(
                    denial.into(),
                    generation,
                ))
            }
        };
        match self.load_prefetch(&grant) {
            Ok(frame) => match classify_loaded_frame(frame) {
                Ok(frame) => prefetch_outcome(frame),
                Err(failure) => PhysicalPrefetchOutcome::Failed(failure),
            },
            Err(failure) => PhysicalPrefetchOutcome::Failed(PhysicalSpeculativeReadFailure::Frame(
                failure.into(),
            )),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn read_ahead(
        &self,
        intent: PhysicalReadAheadIntent<'_>,
        generation: LifecycleGeneration,
    ) -> PhysicalReadAheadOutcome {
        let grant = match self.admit_read_ahead(intent.total_bytes(), intent.coordinates()) {
            Ok(grant) => grant,
            Err(denial) => {
                return PhysicalReadAheadOutcome::Dropped(PhysicalSpeculativeReadDrop::bind(
                    denial.into(),
                    generation,
                ))
            }
        };
        let mut outcomes = Vec::new();
        if outcomes
            .try_reserve_exact(grant.coordinates().len())
            .is_err()
        {
            return PhysicalReadAheadOutcome::FailedBeforeFrames(
                PhysicalSpeculativeReadFailure::OutcomeAllocationUnavailable,
            );
        }
        for index in 0..grant.coordinates().len() {
            let frame_grant = grant
                .frame(index)
                .expect("an index bounded by the grant coordinate count is admitted");
            let coordinate = frame_grant.coordinate();
            let outcome = match self.load_read_ahead(&frame_grant) {
                Ok(frame) => classify_loaded_frame(frame).unwrap_or_else(|failure| {
                    PhysicalReadAheadFrameOutcome::failed(coordinate, failure)
                }),
                Err(failure) => PhysicalReadAheadFrameOutcome::failed(
                    coordinate,
                    PhysicalSpeculativeReadFailure::Frame(failure.into()),
                ),
            };
            outcomes.push(outcome);
        }
        PhysicalReadAheadOutcome::from_batch(PhysicalReadAheadBatch::bind(outcomes))
    }
}
