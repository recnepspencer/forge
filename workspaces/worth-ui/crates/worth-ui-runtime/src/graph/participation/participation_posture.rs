use crate::declaration::stable_text_digest;
use crate::graph::UiGraphParticipationAxis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphParticipationStatus {
    Admitted,
    Deferred,
    Withheld,
}

impl UiGraphParticipationStatus {
    pub const fn admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphParticipationReasonSource {
    GraphInstantiation,
    MountEligibility,
    ReservedRuntimeMutation,
    ParticipationMutation,
    AttachmentPosture,
    ContainmentClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphParticipationReasonCode {
    InstantiatedNodeExists,
    MountedAxisAwaitsRuntimeMutation,
    VisibleAxisAwaitsRuntimeMutation,
    LayoutAxisAwaitsRuntimeMutation,
    HitTestAxisAwaitsRuntimeMutation,
    FocusAxisAwaitsRuntimeMutation,
    AccessibilityAxisAwaitsRuntimeMutation,
    PaintAxisAwaitsRuntimeMutation,
    InputAxisAwaitsRuntimeMutation,
    RuntimeMutationApplied,
    QueryBindingAttached,
    QueryBindingAbsent,
    ServiceUsageAttached,
    ServiceUsageAbsent,
    DiagnosticSurfaceOwned,
    DiagnosticSurfaceAbsent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphParticipationEvidenceHandle {
    InstantiationPlan,
    MountEligibilitySeed,
    ReservedRuntimeMutationLane,
    ParticipationMutation,
    QueryBindingAttachment,
    ServiceUsageAttachment,
    DiagnosticContainmentClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphAxisParticipation {
    status: UiGraphParticipationStatus,
    source: UiGraphParticipationReasonSource,
    reason: UiGraphParticipationReasonCode,
    evidence_handle: UiGraphParticipationEvidenceHandle,
}

impl UiGraphAxisParticipation {
    pub const fn new(
        status: UiGraphParticipationStatus,
        source: UiGraphParticipationReasonSource,
        reason: UiGraphParticipationReasonCode,
        evidence_handle: UiGraphParticipationEvidenceHandle,
    ) -> Self {
        Self {
            status,
            source,
            reason,
            evidence_handle,
        }
    }

    pub fn status(self) -> UiGraphParticipationStatus {
        self.status
    }

    pub fn source(self) -> UiGraphParticipationReasonSource {
        self.source
    }

    pub fn reason(self) -> UiGraphParticipationReasonCode {
        self.reason
    }

    pub fn evidence_handle(self) -> UiGraphParticipationEvidenceHandle {
        self.evidence_handle
    }

    pub const fn runtime_mutation(status: UiGraphParticipationStatus) -> Self {
        Self::new(
            status,
            UiGraphParticipationReasonSource::ParticipationMutation,
            UiGraphParticipationReasonCode::RuntimeMutationApplied,
            UiGraphParticipationEvidenceHandle::ParticipationMutation,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphParticipationPosture {
    axes: [UiGraphAxisParticipation; UiGraphParticipationAxis::COUNT],
}

impl UiGraphParticipationPosture {
    pub const fn new(axes: [UiGraphAxisParticipation; UiGraphParticipationAxis::COUNT]) -> Self {
        Self { axes }
    }

    pub fn axis(self, axis: UiGraphParticipationAxis) -> UiGraphAxisParticipation {
        self.axes[axis.as_index()]
    }

    pub const fn with_axis(
        self,
        axis: UiGraphParticipationAxis,
        next_axis_participation: UiGraphAxisParticipation,
    ) -> Self {
        let mut axes = self.axes;
        axes[axis.as_index()] = next_axis_participation;
        Self { axes }
    }

    pub fn axes(&self) -> &[UiGraphAxisParticipation; UiGraphParticipationAxis::COUNT] {
        &self.axes
    }

    pub(crate) fn identity_digest(self) -> u64 {
        UiGraphParticipationAxis::ALL.iter().fold(
            stable_text_digest("graph-participation-posture"),
            |digest, axis| {
                let participation = self.axis(*axis);
                digest.rotate_left(7)
                    ^ stable_text_digest(axis_label(*axis))
                    ^ stable_text_digest(status_label(participation.status())).rotate_left(11)
                    ^ stable_text_digest(source_label(participation.source())).rotate_left(17)
                    ^ stable_text_digest(reason_label(participation.reason())).rotate_left(23)
                    ^ stable_text_digest(evidence_label(participation.evidence_handle()))
                        .rotate_left(29)
            },
        )
    }
}

fn axis_label(axis: UiGraphParticipationAxis) -> &'static str {
    match axis {
        UiGraphParticipationAxis::Exists => "exists",
        UiGraphParticipationAxis::Mounted => "mounted",
        UiGraphParticipationAxis::Visible => "visible",
        UiGraphParticipationAxis::Layout => "layout",
        UiGraphParticipationAxis::HitTest => "hit-test",
        UiGraphParticipationAxis::Focus => "focus",
        UiGraphParticipationAxis::Accessibility => "accessibility",
        UiGraphParticipationAxis::Paint => "paint",
        UiGraphParticipationAxis::Input => "input",
        UiGraphParticipationAxis::QueryBound => "query-bound",
        UiGraphParticipationAxis::ServiceBound => "service-bound",
        UiGraphParticipationAxis::Diagnostic => "diagnostic",
    }
}

fn status_label(status: UiGraphParticipationStatus) -> &'static str {
    match status {
        UiGraphParticipationStatus::Admitted => "admitted",
        UiGraphParticipationStatus::Deferred => "deferred",
        UiGraphParticipationStatus::Withheld => "withheld",
    }
}

fn source_label(source: UiGraphParticipationReasonSource) -> &'static str {
    match source {
        UiGraphParticipationReasonSource::GraphInstantiation => "graph-instantiation",
        UiGraphParticipationReasonSource::MountEligibility => "mount-eligibility",
        UiGraphParticipationReasonSource::ReservedRuntimeMutation => "reserved-runtime-mutation",
        UiGraphParticipationReasonSource::ParticipationMutation => "participation-mutation",
        UiGraphParticipationReasonSource::AttachmentPosture => "attachment-posture",
        UiGraphParticipationReasonSource::ContainmentClaim => "containment-claim",
    }
}

fn reason_label(reason: UiGraphParticipationReasonCode) -> &'static str {
    match reason {
        UiGraphParticipationReasonCode::InstantiatedNodeExists => "instantiated-node-exists",
        UiGraphParticipationReasonCode::MountedAxisAwaitsRuntimeMutation => {
            "mounted-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::VisibleAxisAwaitsRuntimeMutation => {
            "visible-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::LayoutAxisAwaitsRuntimeMutation => {
            "layout-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::HitTestAxisAwaitsRuntimeMutation => {
            "hit-test-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::FocusAxisAwaitsRuntimeMutation => {
            "focus-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::AccessibilityAxisAwaitsRuntimeMutation => {
            "accessibility-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::PaintAxisAwaitsRuntimeMutation => {
            "paint-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::InputAxisAwaitsRuntimeMutation => {
            "input-axis-awaits-runtime-mutation"
        }
        UiGraphParticipationReasonCode::RuntimeMutationApplied => "runtime-mutation-applied",
        UiGraphParticipationReasonCode::QueryBindingAttached => "query-binding-attached",
        UiGraphParticipationReasonCode::QueryBindingAbsent => "query-binding-absent",
        UiGraphParticipationReasonCode::ServiceUsageAttached => "service-usage-attached",
        UiGraphParticipationReasonCode::ServiceUsageAbsent => "service-usage-absent",
        UiGraphParticipationReasonCode::DiagnosticSurfaceOwned => "diagnostic-surface-owned",
        UiGraphParticipationReasonCode::DiagnosticSurfaceAbsent => "diagnostic-surface-absent",
    }
}

fn evidence_label(handle: UiGraphParticipationEvidenceHandle) -> &'static str {
    match handle {
        UiGraphParticipationEvidenceHandle::InstantiationPlan => "instantiation-plan",
        UiGraphParticipationEvidenceHandle::MountEligibilitySeed => "mount-eligibility-seed",
        UiGraphParticipationEvidenceHandle::ReservedRuntimeMutationLane => {
            "reserved-runtime-mutation-lane"
        }
        UiGraphParticipationEvidenceHandle::ParticipationMutation => "participation-mutation",
        UiGraphParticipationEvidenceHandle::QueryBindingAttachment => "query-binding-attachment",
        UiGraphParticipationEvidenceHandle::ServiceUsageAttachment => "service-usage-attachment",
        UiGraphParticipationEvidenceHandle::DiagnosticContainmentClaim => {
            "diagnostic-containment-claim"
        }
    }
}
