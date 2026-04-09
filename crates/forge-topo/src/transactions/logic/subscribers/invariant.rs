use forge_core::KernelError;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::ValidationSummary;
use crate::validators::invariant_id::{validator_for, InvariantId, InvariantRelation};
use forge_core::ValidationCheckpoint;

use super::{kernel_to_signal, stage_output_value};

#[derive(Debug, Default)]
pub(crate) struct InvariantSubscriber {
    summary: ValidationSummary,
    op_name: Option<&'static str>,
    invocation_id: Option<crate::identity::OperationId>,
    relation: Option<fn(InvariantId) -> InvariantRelation>,
}

impl InvariantSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl EventSubscriber for InvariantSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(150)
    }

    fn name(&self) -> &'static str {
        "invariant_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::EulerDeltaResult]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::ValidationResult]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.summary = ValidationSummary::default();
        self.op_name = None;
        self.invocation_id = None;
        self.relation = None;
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let TopoOperationEvent::OperationStarted {
            op_name,
            invocation_id,
            invariant_relation,
            ..
        } = event
        {
            self.op_name = Some(*op_name);
            self.invocation_id = Some(*invocation_id);
            self.relation = Some(*invariant_relation);
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

        if runtime.config().suppress_per_op_validation {
            return stage_output_value(
                ctx,
                TopoSubscriberDataId::ValidationResult,
                self.summary.clone(),
                "ValidationResult",
            );
        }

        // Relation is static per operator type; carry it from the event stream.
        // If unavailable, treat as unrelated (run none).
        let relation = self
            .relation
            .unwrap_or(|_id: InvariantId| InvariantRelation::Unrelated);
        let checkpoint = ValidationCheckpoint::PerOp;
        let max_cost = runtime.config().group_policy.max_cost_at(checkpoint);

        for &id in InvariantId::ALL {
            let should_run = if runtime.config().validate_all_invariants_per_op {
                true
            } else {
                relation(id) == InvariantRelation::MayBreak
                    && runtime
                        .config()
                        .group_policy
                        .should_run(id.group(), checkpoint)
            };

            if !should_run {
                continue;
            }

            let entry = validator_for(id);
            if entry.cost > max_cost {
                continue;
            }

            self.summary.checks_run += 1;
            let check_result = (entry.check)(runtime.arena());
            let passed = check_result.is_ok();
            if !passed {
                self.summary.checks_failed += 1;
            }

            tracing::info!(
                invariant = ?id,
                operator = self.op_name.unwrap_or("<unknown-op>"),
                invocation = self.invocation_id.map(|v| v.get()).unwrap_or(0),
                cost = ?entry.cost,
                passed = passed,
                "invariant_check"
            );

            if let Err(e) = check_result {
                let op_name = self.op_name.unwrap_or("<unknown-op>");
                let invocation = self.invocation_id.map(|v| v.get()).unwrap_or(0);
                return Err(kernel_to_signal(
                    e.ensure_operation_context(
                        op_name,
                        invocation,
                        &format!("Invariant {:?} violated after {}", id, op_name),
                    )
                    .with_phase(&format!("invariant_check({:?})", id)),
                ));
            }
        }
        stage_output_value(
            ctx,
            TopoSubscriberDataId::ValidationResult,
            self.summary.clone(),
            "ValidationResult",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.summary = ValidationSummary::default();
        self.op_name = None;
        self.invocation_id = None;
        self.relation = None;
    }
}
