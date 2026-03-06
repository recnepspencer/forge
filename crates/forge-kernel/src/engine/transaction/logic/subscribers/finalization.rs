//! Subscriber that assembles finalization artifacts from drained context data.

use crate::engine::transaction::data::feature_event::{FeatureInvocationId, KernelFeatureEvent};
use crate::engine::transaction::data::operation_outputs::{DecisionDrainOutput, FinalizationOutput};
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::FeatureEventRuntimeContext;
use forge_core::tracing::compute_trace_fingerprint;
use forge_core::KernelError;
use forge_signal::facade::{CheckpointBarrier, EventSubscriber, SubscriberContext, SubscriberId};

use super::stage_output_value;

pub(crate) struct FinalizationSubscriber {
    started: Option<(FeatureInvocationId, u128)>,
    completed: Option<(FeatureInvocationId, u64, u128)>,
}

impl FinalizationSubscriber {
    pub(crate) fn new() -> Self {
        Self {
            started: None,
            completed: None,
        }
    }
}

impl EventSubscriber for FinalizationSubscriber {
    type Event = KernelFeatureEvent;
    type DataId = KernelSubscriberDataId;
    type RuntimeContext = FeatureEventRuntimeContext;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(20)
    }

    fn name(&self) -> &'static str {
        "FinalizationSubscriber"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[KernelSubscriberDataId::DecisionDrain]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[KernelSubscriberDataId::Finalization]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.started = None;
        self.completed = None;
    }

    fn on_event(&mut self, event: &Self::Event) {
        match event {
            KernelFeatureEvent::OperationStarted {
                invocation_id,
                state_hash_before,
                ..
            } => {
                self.started = Some((*invocation_id, *state_hash_before));
            }
            KernelFeatureEvent::OperationCompleted {
                invocation_id,
                duration_micros,
                state_hash_after,
            } => {
                self.completed = Some((*invocation_id, *duration_micros, *state_hash_after));
            }
            KernelFeatureEvent::OperationFailed { .. } => {}
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), KernelError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }
        let drained = ctx
            .staged::<DecisionDrainOutput>(KernelSubscriberDataId::DecisionDrain)
            .ok_or_else(|| KernelError::InternalError {
                message: "DecisionDrain output missing in FinalizationSubscriber".to_string(),
                context: None,
            })?;
        let (started_id, hash_before) = self.started.ok_or_else(|| KernelError::InternalError {
            message: "OperationStarted event missing in FinalizationSubscriber".to_string(),
            context: None,
        })?;
        let (completed_id, duration_micros, hash_after) =
            self.completed.ok_or_else(|| KernelError::InternalError {
                message: "OperationCompleted event missing in FinalizationSubscriber".to_string(),
                context: None,
            })?;
        if started_id != completed_id {
            return Err(KernelError::InternalError {
                message: format!(
                    "FinalizationSubscriber saw mismatched invocation IDs: started={} completed={}",
                    started_id.get(),
                    completed_id.get()
                ),
                context: None,
            });
        }

        let decision_log = drained.decision_log.clone();
        let trace_fingerprint = compute_trace_fingerprint(&decision_log);
        let output = FinalizationOutput {
            decision_log,
            warnings: drained.sub_metadata.warnings.clone(),
            metrics: drained.sub_metadata.metrics.clone(),
            lineage_delta: drained.sub_metadata.lineage_delta.clone(),
            accumulated_error_budget: drained.sub_metadata.accumulated_error_budget,
            state_hash_before: hash_before,
            state_hash_after: hash_after,
            trace_fingerprint,
            adjunct_count: drained.trace_adjuncts.records().len(),
            duration_micros,
        };
        stage_output_value(
            ctx,
            KernelSubscriberDataId::Finalization,
            output,
            "finalization",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.started = None;
        self.completed = None;
    }
}
