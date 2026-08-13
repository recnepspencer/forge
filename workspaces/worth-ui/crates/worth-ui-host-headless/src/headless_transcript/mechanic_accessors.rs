use super::*;

impl UiHeadlessRecorderCapacity {
    pub const fn new(
        surface_bindings: usize,
        retained_frames: usize,
        mechanics_per_frame: usize,
    ) -> Self {
        Self {
            surface_bindings,
            retained_frames,
            mechanics_per_frame,
        }
    }

    pub const fn production_default() -> Self {
        Self::new(256, 64, 4_096)
    }

    pub const fn surface_bindings(self) -> usize {
        self.surface_bindings
    }

    pub const fn retained_frames(self) -> usize {
        self.retained_frames
    }

    pub const fn mechanics_per_frame(self) -> usize {
        self.mechanics_per_frame
    }
}

impl UiHeadlessResourceContact {
    pub(crate) fn new(content_identity: u64, kind: UiMountedResourceKind, byte_len: u32) -> Self {
        Self {
            content_identity,
            kind,
            byte_len,
        }
    }

    pub const fn content_identity(self) -> u64 {
        self.content_identity
    }

    pub const fn kind(self) -> UiMountedResourceKind {
        self.kind
    }

    pub const fn byte_len(self) -> u32 {
        self.byte_len
    }
}

impl UiHeadlessClipMechanic {
    pub(crate) const fn new(bounds: UiMountedCanonicalBox, parent: Option<u16>) -> Self {
        Self { bounds, parent }
    }

    pub const fn bounds(self) -> UiMountedCanonicalBox {
        self.bounds
    }

    pub const fn parent(self) -> Option<u16> {
        self.parent
    }
}

impl UiHeadlessPaintBatchMechanic {
    pub(crate) fn new(
        batch_index: u16,
        primitive_kind: UiMountedPaintPrimitiveKind,
        primitive_count: u32,
        layer: UiHeadlessLayerMechanic,
        resource: Option<UiHeadlessResourceContact>,
    ) -> Self {
        Self {
            batch_index,
            primitive_kind,
            primitive_count,
            layer,
            resource,
        }
    }

    pub const fn batch_index(self) -> u16 {
        self.batch_index
    }

    pub const fn primitive_kind(self) -> UiMountedPaintPrimitiveKind {
        self.primitive_kind
    }

    pub const fn primitive_count(self) -> u32 {
        self.primitive_count
    }

    pub const fn layer(self) -> UiHeadlessLayerMechanic {
        self.layer
    }

    pub const fn resource(self) -> Option<UiHeadlessResourceContact> {
        self.resource
    }
}

impl UiHeadlessNodeMechanic {
    pub(crate) fn new(input: UiHeadlessNodeMechanicInput) -> Self {
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

    pub const fn mounted_instance(self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub(crate) const fn authored_position(self) -> u64 {
        self.authored_position
    }

    pub const fn role(self) -> UiMountedMechanicalRole {
        self.role
    }

    pub const fn participation(self) -> UiMountedParticipation {
        self.participation
    }

    pub const fn allocation(self) -> UiMountedAllocationProjection {
        self.allocation
    }

    pub const fn preview(self) -> UiMountedPreviewProjection {
        self.preview
    }

    pub const fn paint(self) -> UiHeadlessNodePaintMechanic {
        self.paint
    }

    pub const fn accessibility(self) -> UiMountedAccessibilityProjection {
        self.accessibility
    }

    pub const fn motion(self) -> UiMountedMotionProjection {
        self.motion
    }

    pub const fn diagnostic(self) -> worth_ui_host_contract::UiMountedDiagnosticProjection {
        self.diagnostic
    }
}

impl UiHeadlessUnperformedEffect {
    pub const fn family(self) -> UiMountedEffectFamily {
        match self {
            Self::NativePaint { .. } => UiMountedEffectFamily::NativePaint,
            Self::Accessibility { .. } => UiMountedEffectFamily::Accessibility,
            Self::Focus { .. } => UiMountedEffectFamily::Focus,
            Self::Motion { .. } => UiMountedEffectFamily::Motion,
            Self::Diagnostic { .. } => UiMountedEffectFamily::Diagnostic,
            Self::CanvasSpatial { .. } => UiMountedEffectFamily::CanvasSpatial,
            Self::Realtime { .. } => UiMountedEffectFamily::Realtime,
        }
    }
}
