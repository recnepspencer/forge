#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiMountedPresentationNodeState {
    mounted_instance: crate::UiMountedInstanceIdentity,
    authored_position: u64,
    role: crate::UiMountedMechanicalRole,
    participation: crate::UiMountedParticipation,
    allocation: crate::UiMountedAllocationProjection,
    preview: crate::UiMountedPreviewProjection,
    paint: UiMountedPresentationNodePaint,
    accessibility: crate::UiMountedAccessibilityProjection,
    motion: crate::UiMountedMotionProjection,
    diagnostic: crate::UiMountedDiagnosticProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationNodePaint {
    Command(crate::UiMountedPaintCommandIdentity),
    CountOnlyBatch(u16),
    Omitted(crate::UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiMountedPresentationNodeChange {
    Upsert(UiMountedPresentationNodeState),
    Remove(crate::UiMountedInstanceIdentity),
}

#[doc(hidden)]
pub struct UiMountedPresentationNodeStateInput {
    pub mounted_instance: crate::UiMountedInstanceIdentity,
    pub authored_position: u64,
    pub role: crate::UiMountedMechanicalRole,
    pub participation: crate::UiMountedParticipation,
    pub allocation: crate::UiMountedAllocationProjection,
    pub preview: crate::UiMountedPreviewProjection,
    pub paint: UiMountedPresentationNodePaint,
    pub accessibility: crate::UiMountedAccessibilityProjection,
    pub motion: crate::UiMountedMotionProjection,
    pub diagnostic: crate::UiMountedDiagnosticProjection,
}

impl UiMountedPresentationNodeState {
    #[doc(hidden)]
    pub fn from_runtime_mounting(input: UiMountedPresentationNodeStateInput) -> Self {
        Self {
            mounted_instance: input.mounted_instance,
            authored_position: input.authored_position,
            role: input.role,
            participation: input.participation,
            allocation: input.allocation,
            preview: input.preview,
            paint: input.paint,
            accessibility: input.accessibility,
            motion: input.motion,
            diagnostic: input.diagnostic,
        }
    }

    pub const fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub const fn authored_position(self) -> u64 {
        self.authored_position
    }

    pub const fn role(self) -> crate::UiMountedMechanicalRole {
        self.role
    }

    pub const fn participation(self) -> crate::UiMountedParticipation {
        self.participation
    }

    pub const fn allocation(self) -> crate::UiMountedAllocationProjection {
        self.allocation
    }

    pub const fn preview(self) -> crate::UiMountedPreviewProjection {
        self.preview
    }

    pub const fn paint(self) -> UiMountedPresentationNodePaint {
        self.paint
    }

    pub const fn accessibility(self) -> crate::UiMountedAccessibilityProjection {
        self.accessibility
    }

    pub const fn motion(self) -> crate::UiMountedMotionProjection {
        self.motion
    }

    pub const fn diagnostic(self) -> crate::UiMountedDiagnosticProjection {
        self.diagnostic
    }
}

impl UiMountedPresentationNodeChange {
    pub const fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        match self {
            Self::Upsert(state) => state.mounted_instance(),
            Self::Remove(instance) => instance,
        }
    }
}
