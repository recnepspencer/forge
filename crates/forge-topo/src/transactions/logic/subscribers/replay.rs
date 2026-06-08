use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use crate::identity::OperationId;
use crate::provenance::OpSignature;
use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::ReplayStats;

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct ReplaySubscriber {
    stats: ReplayStats,
    started: Option<StartedEvent>,
    cache_trace: Option<(OperationId, Vec<String>)>,
}

#[derive(Debug, Clone)]
struct StartedEvent {
    op_name: &'static str,
    invocation_id: OperationId,
    schema_version: u32,
    summary: String,
}

impl ReplaySubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.stats = ReplayStats::default();
        self.started = None;
        self.cache_trace = None;
    }
}

impl EventSubscriber for ReplaySubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(130)
    }

    fn name(&self) -> &'static str {
        "replay_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::TopologyHash]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::ReplayEntryFinalization]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.clear();
    }

    fn on_event(&mut self, event: &Self::Event) {
        match event {
            TopoOperationEvent::OperationStarted {
                op_name,
                invocation_id,
                schema_version,
                summary,
                ..
            } => {
                self.started = Some(StartedEvent {
                    op_name,
                    invocation_id: *invocation_id,
                    schema_version: *schema_version,
                    summary: summary.clone(),
                });
                self.stats.op_starts += 1;
            }
            TopoOperationEvent::ReplayCacheTraceApplied { op_id, trace } => {
                self.cache_trace = Some((*op_id, trace.clone()));
                self.stats.cache_trace_updates += 1;
            }
            _ => {}
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }

        if let Some(start) = self.started.take() {
            let mut signature = OpSignature::new(start.op_name);
            signature.set_invocation_id(start.invocation_id);
            runtime.log_operation_start(&signature, start.schema_version, start.summary);
            self.stats.entry_records += 1;
            self.stats.last_recorded_op = Some(start.invocation_id);
        }

        if let Some((_op_id, trace)) = self.cache_trace.take() {
            runtime.replay_log_mut().set_last_cache_refresh_trace(trace);
        }

        if runtime.config().per_op_hashing {
            if let Some(last_op) = self.stats.last_recorded_op {
                let post_hash = runtime.topology_hash();
                runtime.replay_log_mut().finalize_last(post_hash);
                self.stats.entry_finalizations += 1;
                self.stats.last_finalized_op = Some(last_op);
            }
        }

        stage_output_value(
            ctx,
            TopoSubscriberDataId::ReplayEntryFinalization,
            self.stats.clone(),
            "ReplayEntryFinalization",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.clear();
    }
}
