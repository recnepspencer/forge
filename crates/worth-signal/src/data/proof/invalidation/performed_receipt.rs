use worth_proof::{ActionMarker, Performed};

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::telemetry::{InvalidationPerformedCounter, SignalInvalidationRealizedCounters};
use crate::logic::transaction::SignalRuntime;

worth_proof::authority_marker!(InvalidationPerformedReceiptAuthority);

struct CompleteInvalidationExecutionObservation;
impl ActionMarker for CompleteInvalidationExecutionObservation {}

type PerformedInvalidationObservation = Performed<
    CompleteInvalidationExecutionObservation,
    InvalidationPerformedReceiptAuthority,
    SignalInvalidationRealizedCounters,
>;

/// Proof that Signal observed these counters after performed execution.
#[derive(Debug)]
pub struct SignalInvalidationExecutionReceipt {
    performed: PerformedInvalidationObservation,
    graph_instance: u64,
    executed_targets: Vec<crate::data::handle::NodeId>,
}

/// Linear token delimiting one runtime-owned performed observation.
#[derive(Debug)]
pub struct SignalInvalidationExecutionObservation {
    graph_instance: u64,
    generation: u64,
    liveness: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl Drop for SignalInvalidationExecutionObservation {
    fn drop(&mut self) {
        let _ = self.liveness.compare_exchange(
            self.generation,
            0,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
        );
    }
}

impl SignalInvalidationExecutionReceipt {
    fn after_execution(
        graph_instance: u64,
        counters: SignalInvalidationRealizedCounters,
        executed_targets: Vec<crate::data::handle::NodeId>,
    ) -> Self {
        Self {
            performed: Performed::record(
                &InvalidationPerformedReceiptAuthority::witness(),
                counters,
            ),
            graph_instance,
            executed_targets,
        }
    }

    pub fn realized_counters(&self) -> &SignalInvalidationRealizedCounters {
        self.performed.outcome()
    }

    /// Produce a descriptive summary from this performed observation.
    pub fn summary(&self) -> InvalidationExecutionSummary {
        InvalidationExecutionSummary {
            realized_counters: *self.realized_counters(),
        }
    }

    pub fn retains_executed_target(
        &self,
        graph_instance: u64,
        target: crate::data::handle::NodeId,
    ) -> bool {
        self.graph_instance == graph_instance
            && self.executed_targets.binary_search(&target).is_ok()
    }
}

/// Read-only summary derived from performed invalidation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidationExecutionSummary {
    realized_counters: SignalInvalidationRealizedCounters,
}

impl InvalidationExecutionSummary {
    pub const fn realized_counters(&self) -> &SignalInvalidationRealizedCounters {
        &self.realized_counters
    }
}

impl crate::data::proof::SummaryForm for InvalidationExecutionSummary {}

impl SignalGraph {
    pub fn begin_invalidation_execution_observation(
        &mut self,
    ) -> SignalInvalidationExecutionObservation {
        let generation = self.begin_invalidation_performed_observation();
        SignalInvalidationExecutionObservation {
            graph_instance: self.runtime_instance_id(),
            generation,
            liveness: self
                .invalidation_performed_counter_state()
                .observation_liveness(),
        }
    }

    pub fn finish_invalidation_execution_observation(
        &self,
        observation: SignalInvalidationExecutionObservation,
    ) -> Result<SignalInvalidationExecutionReceipt, SignalError> {
        self.finish_optional_invalidation_execution_observation(observation)?
            .ok_or_else(|| {
                SignalError::invalid_input(
                    "invalidation execution observation contains no executed invalidation batch",
                )
            })
    }

    pub fn finish_optional_invalidation_execution_observation(
        &self,
        observation: SignalInvalidationExecutionObservation,
    ) -> Result<Option<SignalInvalidationExecutionReceipt>, SignalError> {
        if observation.graph_instance != self.runtime_instance_id() {
            return Err(SignalError::invalid_input(
                "invalidation execution observation belongs to another runtime",
            ));
        }
        if observation.generation
            != self
                .invalidation_performed_counter_state()
                .observation_generation()
        {
            return Err(SignalError::invalid_input(
                "invalidation execution observation was superseded",
            ));
        }
        let counters = self.invalidation_performed_counters();
        let mut executed_targets = self
            .invalidation_performed_work()
            .into_iter()
            .map(|binding| binding.target)
            .collect::<Vec<_>>();
        executed_targets.sort_unstable();
        executed_targets.dedup();
        if !self
            .invalidation_performed_counter_state()
            .finish_observation(observation.generation)
        {
            return Err(SignalError::invalid_input(
                "invalidation execution observation is no longer active",
            ));
        }
        if counters.value(InvalidationPerformedCounter::NodesEvaluated) == 0 {
            return Ok(None);
        }
        Ok(Some(SignalInvalidationExecutionReceipt::after_execution(
            self.runtime_instance_id(),
            counters,
            executed_targets,
        )))
    }

    pub fn observe_invalidation_execution<Outcome>(
        &mut self,
        execute: impl FnOnce(&mut Self) -> Result<Outcome, SignalError>,
    ) -> Result<(Outcome, SignalInvalidationExecutionReceipt), SignalError> {
        let observation = self.begin_invalidation_execution_observation();
        let outcome = execute(self)?;
        let receipt = self.finish_invalidation_execution_observation(observation)?;
        Ok((outcome, receipt))
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn begin_invalidation_execution_observation(
        &mut self,
    ) -> SignalInvalidationExecutionObservation {
        self.graph_mut().begin_invalidation_execution_observation()
    }

    pub fn finish_invalidation_execution_observation(
        &self,
        observation: SignalInvalidationExecutionObservation,
    ) -> Result<SignalInvalidationExecutionReceipt, SignalError> {
        self.graph()
            .finish_invalidation_execution_observation(observation)
    }

    /// Observe one bounded runtime execution and seal only counters it performed.
    pub fn observe_invalidation_execution<Outcome>(
        &mut self,
        execute: impl FnOnce(&mut Self) -> Result<Outcome, SignalError>,
    ) -> Result<(Outcome, SignalInvalidationExecutionReceipt), SignalError> {
        let observation = self.begin_invalidation_execution_observation();
        let outcome = execute(self)?;
        let receipt = self.finish_invalidation_execution_observation(observation)?;
        Ok((outcome, receipt))
    }
}
