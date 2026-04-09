use forge_core::{ErrorContext, ErrorScope, KernelError, TopologyError};
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use crate::identity::OperationId;
use crate::operations::operator::EulerDelta;
use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::EulerDeltaCheck;

use super::{kernel_to_signal, stage_output_value};

#[derive(Debug)]
pub(crate) struct EulerSubscriber {
    before: ArenaCounts,
    op_name: Option<&'static str>,
    invocation_id: Option<OperationId>,
    declared: Option<EulerDelta>,
}

impl EulerSubscriber {
    pub(crate) fn new() -> Self {
        Self {
            before: ArenaCounts::default(),
            op_name: None,
            invocation_id: None,
            declared: None,
        }
    }

    fn reset(&mut self) {
        self.before = ArenaCounts::default();
        self.op_name = None;
        self.invocation_id = None;
        self.declared = None;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ArenaCounts {
    faces: usize,
    vertices: usize,
    half_edges: usize,
    loops: usize,
    edges: usize,
    shells: usize,
    solids: usize,
    lumps: usize,
    regions: usize,
}

impl ArenaCounts {
    fn from_runtime(runtime: &crate::transactions::logic::mutable_draft::MutableDraft) -> Self {
        Self {
            faces: runtime.arena().face_count(),
            vertices: runtime.arena().vertex_count(),
            half_edges: runtime.arena().half_edge_count(),
            loops: runtime.arena().loop_count(),
            edges: runtime.arena().edge_count(),
            shells: runtime.arena().shell_count(),
            solids: runtime.arena().body_count(),
            lumps: runtime.arena().lump_count(),
            regions: runtime.arena().region_count(),
        }
    }

    fn delta(self, after: Self) -> EulerDelta {
        EulerDelta {
            vertices: after.vertices as i32 - self.vertices as i32,
            half_edges: after.half_edges as i32 - self.half_edges as i32,
            faces: after.faces as i32 - self.faces as i32,
            loops: after.loops as i32 - self.loops as i32,
            edges: after.edges as i32 - self.edges as i32,
            shells: after.shells as i32 - self.shells as i32,
            solids: after.solids as i32 - self.solids as i32,
            lumps: after.lumps as i32 - self.lumps as i32,
            regions: after.regions as i32 - self.regions as i32,
        }
    }
}

impl EventSubscriber for EulerSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(140)
    }

    fn name(&self) -> &'static str {
        "euler_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::MutationCounts]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::EulerDeltaResult]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        runtime: &mut Self::RuntimeContext,
    ) {
        self.reset();
        self.before = ArenaCounts::from_runtime(runtime);
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let TopoOperationEvent::OperationStarted {
            op_name,
            invocation_id,
            ..
        } = event
        {
            self.op_name = Some(*op_name);
            self.invocation_id = Some(*invocation_id);
            return;
        }

        if let TopoOperationEvent::OperationCompleted { declared_delta, .. } = event {
            self.declared = Some(*declared_delta);
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

        let declared = self.declared.unwrap_or_default();
        let actual = self.before.delta(ArenaCounts::from_runtime(runtime));
        let matched = actual == declared;

        if !matched {
            let expected_vertices_after = self.before.vertices as i64 + declared.vertices as i64;
            let expected_edges_after = self.before.edges as i64 + declared.edges as i64;
            let expected_faces_after = self.before.faces as i64 + declared.faces as i64;
            let expected_chi =
                expected_vertices_after - expected_edges_after + expected_faces_after;
            let actual_chi = actual.vertices as i64 + self.before.vertices as i64
                - (actual.edges as i64 + self.before.edges as i64)
                + (actual.faces as i64 + self.before.faces as i64);

            let op_name = self.op_name.unwrap_or("<unknown-op>").to_string();
            let invocation = self.invocation_id.map(|v| v.get()).unwrap_or(0);
            return Err(kernel_to_signal(KernelError::TopologyViolation {
                err: TopologyError::EulerFormulaViolation {
                    vertices: runtime.arena().vertex_count(),
                    edges: runtime.arena().edge_count(),
                    faces: runtime.arena().face_count(),
                    expected_chi,
                    actual_chi,
                },
                context: Some(ErrorContext {
                    scope: ErrorScope::Operation {
                        op_name: op_name.clone(),
                        invocation_id: invocation,
                    },
                    suggested_fixes: vec![],
                    detail: format!(
                        "{} declared Euler delta V={} HE={} F={} L={} E={} S={} So={} but actual was V={} HE={} F={} L={} E={} S={} So={}",
                        op_name,
                        declared.vertices,
                        declared.half_edges,
                        declared.faces,
                        declared.loops,
                        declared.edges,
                        declared.shells,
                        declared.solids,
                        actual.vertices,
                        actual.half_edges,
                        actual.faces,
                        actual.loops,
                        actual.edges,
                        actual.shells,
                        actual.solids,
                    ),
                }),
            }));
        }

        let value = EulerDeltaCheck {
            declared,
            actual,
            matched,
        };
        stage_output_value(
            ctx,
            TopoSubscriberDataId::EulerDeltaResult,
            value,
            "EulerDeltaResult",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.reset();
    }
}
