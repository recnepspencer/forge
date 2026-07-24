use worth_ui_host_contract::{
    UiMountIncarnation, UiMountedAccessibilityProjection, UiMountedAllocationProjection,
    UiMountedDiagnosticProjection, UiMountedInstanceIdentity, UiMountedMechanicalRole,
    UiMountedMotionProjection, UiMountedNodeReceiptIdentity, UiMountedPaintProjection,
    UiMountedParticipation, UiMountedPreviewProjection, UiSemanticSurfaceIdentity,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedNodeReceipt {
    identity: UiMountedNodeReceiptIdentity,
    mounted_instance: UiMountedInstanceIdentity,
    graph_node: crate::graph::UiGraphNodeIdentity,
    semantic_surface: UiSemanticSurfaceIdentity,
    incarnation: UiMountIncarnation,
    plan_digest: u64,
    role: UiMountedMechanicalRole,
    participation: UiMountedParticipation,
    allocation: UiMountedAllocationProjection,
    preview: UiMountedPreviewProjection,
    paint: UiMountedPaintProjection,
    accessibility: UiMountedAccessibilityProjection,
    motion: UiMountedMotionProjection,
    diagnostic: UiMountedDiagnosticProjection,
}

pub(super) struct UiMountedNodeReceiptInput {
    pub identity: UiMountedNodeReceiptIdentity,
    pub mounted_instance: UiMountedInstanceIdentity,
    pub graph_node: crate::graph::UiGraphNodeIdentity,
    pub semantic_surface: UiSemanticSurfaceIdentity,
    pub incarnation: UiMountIncarnation,
    pub plan_digest: u64,
    pub role: UiMountedMechanicalRole,
    pub participation: UiMountedParticipation,
    pub allocation: UiMountedAllocationProjection,
}

impl UiMountedNodeReceipt {
    pub(super) fn from_input(input: UiMountedNodeReceiptInput) -> Self {
        let accessibility = projection_from_participation(
            input.participation.accessibility().status(),
            UiMountedAccessibilityProjection::Admitted(input.role),
        );
        let diagnostic = diagnostic_from_participation(input.participation.diagnostic().status());
        Self {
            identity: input.identity,
            mounted_instance: input.mounted_instance,
            graph_node: input.graph_node,
            semantic_surface: input.semantic_surface,
            incarnation: input.incarnation,
            plan_digest: input.plan_digest,
            role: input.role,
            participation: input.participation,
            allocation: input.allocation,
            preview: UiMountedPreviewProjection::Omitted(
                worth_ui_host_contract::UiMountedOmissionReason::NotProducedByExecutedLane,
            ),
            paint: UiMountedPaintProjection::Omitted(
                worth_ui_host_contract::UiMountedOmissionReason::NotProducedByExecutedLane,
            ),
            accessibility,
            motion: UiMountedMotionProjection::Omitted(
                worth_ui_host_contract::UiMountedOmissionReason::NotDefinedByCurrentRuntime,
            ),
            diagnostic,
        }
    }

    pub fn identity(&self) -> UiMountedNodeReceiptIdentity {
        self.identity
    }
    pub fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub fn graph_node(&self) -> crate::graph::UiGraphNodeIdentity {
        self.graph_node
    }
    pub fn semantic_surface(&self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub fn incarnation(&self) -> UiMountIncarnation {
        self.incarnation
    }
    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
    pub fn participation(&self) -> UiMountedParticipation {
        self.participation
    }
    pub fn role(&self) -> UiMountedMechanicalRole {
        self.role
    }
    pub fn allocation(&self) -> UiMountedAllocationProjection {
        self.allocation
    }
    pub fn preview(&self) -> UiMountedPreviewProjection {
        self.preview
    }
    pub fn paint(&self) -> UiMountedPaintProjection {
        self.paint
    }
    pub fn accessibility(&self) -> UiMountedAccessibilityProjection {
        self.accessibility
    }
    pub fn motion(&self) -> UiMountedMotionProjection {
        self.motion
    }
    pub fn diagnostic(&self) -> UiMountedDiagnosticProjection {
        self.diagnostic
    }

    pub(super) fn attach_paint(
        &mut self,
        reference: worth_ui_host_contract::UiMountedPaintBatchReference,
    ) {
        self.paint = UiMountedPaintProjection::Batch(reference);
    }

    pub(super) fn attach_preview(&mut self, preview: UiMountedPreviewProjection) {
        self.preview = preview;
    }
}

fn projection_from_participation(
    status: worth_ui_host_contract::UiMountedParticipationStatus,
    admitted: UiMountedAccessibilityProjection,
) -> UiMountedAccessibilityProjection {
    match status {
        worth_ui_host_contract::UiMountedParticipationStatus::Admitted => admitted,
        worth_ui_host_contract::UiMountedParticipationStatus::Deferred
        | worth_ui_host_contract::UiMountedParticipationStatus::Withheld => {
            UiMountedAccessibilityProjection::Omitted(
                worth_ui_host_contract::UiMountedOmissionReason::AwaitingRuntimeMutation,
            )
        }
    }
}

fn diagnostic_from_participation(
    status: worth_ui_host_contract::UiMountedParticipationStatus,
) -> UiMountedDiagnosticProjection {
    let reason = match status {
        worth_ui_host_contract::UiMountedParticipationStatus::Admitted => {
            worth_ui_host_contract::UiMountedOmissionReason::NotProducedByExecutedLane
        }
        worth_ui_host_contract::UiMountedParticipationStatus::Deferred
        | worth_ui_host_contract::UiMountedParticipationStatus::Withheld => {
            worth_ui_host_contract::UiMountedOmissionReason::AwaitingRuntimeMutation
        }
    };
    UiMountedDiagnosticProjection::Omitted(reason)
}
