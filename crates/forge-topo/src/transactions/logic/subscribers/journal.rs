use forge_core::EntityKind;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use crate::transactions::data::mutation_journal::EntityKindCounts;
use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::MutationCounts;

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct JournalSubscriber {
    created: EntityKindCounts,
    destroyed: EntityKindCounts,
}

impl JournalSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.created = EntityKindCounts::default();
        self.destroyed = EntityKindCounts::default();
    }

    fn bump(counts: &mut EntityKindCounts, kind: EntityKind) {
        match kind {
            EntityKind::Face => counts.faces += 1,
            EntityKind::HalfEdge => counts.half_edges += 1,
            EntityKind::Vertex => counts.vertices += 1,
            EntityKind::Loop => counts.loops += 1,
            EntityKind::Edge => counts.edges += 1,
            EntityKind::Shell => counts.shells += 1,
            EntityKind::Body => counts.bodies += 1,
            EntityKind::Lump => counts.lumps += 1,
            EntityKind::Region => counts.regions += 1,
        }
    }
}

impl EventSubscriber for JournalSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(100)
    }

    fn name(&self) -> &'static str {
        "journal_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::MutationCounts]
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
            TopoOperationEvent::EntityCreated(entity) => {
                Self::bump(&mut self.created, entity.kind());
            }
            TopoOperationEvent::EntityDestroyed(entity) => {
                Self::bump(&mut self.destroyed, entity.kind());
            }
            _ => {}
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }

        stage_output_value(
            ctx,
            TopoSubscriberDataId::MutationCounts,
            MutationCounts {
                created: self.created.clone(),
                destroyed: self.destroyed.clone(),
            },
            "MutationCounts",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.clear();
    }
}
