use worth_ui_host_contract::{
    UiHostProtocolAgreement, UiHostSurfacePresentationMode, UiMountedAccessibilityProjection,
    UiMountedAllocationProjection, UiMountedCanonicalBox, UiMountedEffectFamily,
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedMechanicalRole,
    UiMountedMotionProjection, UiMountedOmissionReason, UiMountedPaintPrimitiveKind,
    UiMountedParticipation, UiMountedPresentationAttemptIdentity, UiMountedPreviewProjection,
    UiMountedResourceKind, UiSurfaceBindingGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHeadlessRecorderCapacity {
    surface_bindings: usize,
    retained_frames: usize,
    mechanics_per_frame: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiHeadlessResolvedClip {
    Unclipped,
    Clip(u16),
    Omitted(UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiHeadlessLayerMechanic {
    Ordered {
        semantic_order: u32,
        clip: UiHeadlessResolvedClip,
    },
    Omitted(UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHeadlessClipMechanic {
    bounds: UiMountedCanonicalBox,
    parent: Option<u16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHeadlessResourceContact {
    content_identity: u64,
    kind: UiMountedResourceKind,
    byte_len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHeadlessPaintBatchMechanic {
    batch_index: u16,
    primitive_kind: UiMountedPaintPrimitiveKind,
    primitive_count: u32,
    layer: UiHeadlessLayerMechanic,
    resource: Option<UiHeadlessResourceContact>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHeadlessNodePaintMechanic {
    Batch(u16),
    Omitted(UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHeadlessNodeMechanic {
    mounted_instance: UiMountedInstanceIdentity,
    role: UiMountedMechanicalRole,
    participation: UiMountedParticipation,
    allocation: UiMountedAllocationProjection,
    preview: UiMountedPreviewProjection,
    paint: UiHeadlessNodePaintMechanic,
    accessibility: UiMountedAccessibilityProjection,
    motion: UiMountedMotionProjection,
    diagnostic: worth_ui_host_contract::UiMountedDiagnosticProjection,
}

pub(crate) struct UiHeadlessNodeMechanicInput {
    pub mounted_instance: UiMountedInstanceIdentity,
    pub role: UiMountedMechanicalRole,
    pub participation: UiMountedParticipation,
    pub allocation: UiMountedAllocationProjection,
    pub preview: UiMountedPreviewProjection,
    pub paint: UiHeadlessNodePaintMechanic,
    pub accessibility: UiMountedAccessibilityProjection,
    pub motion: UiMountedMotionProjection,
    pub diagnostic: worth_ui_host_contract::UiMountedDiagnosticProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHeadlessUnperformedEffect {
    NativePaint {
        paint_batch_count: u32,
        preview_node_count: u32,
    },
    Accessibility {
        node_count: u32,
    },
    Focus {
        node_count: u32,
    },
    Motion {
        node_count: u32,
    },
    Diagnostic {
        node_count: u32,
    },
    CanvasSpatial {
        batch_index: u16,
        primitive_count: u32,
        hit_region_count: u32,
        overlay_row_count: u16,
        tool_state_row_count: u16,
    },
    Realtime {
        batch_index: u16,
        overlay_row_count: u16,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiHeadlessMountedFrameTranscript {
    host_session_identity: u64,
    protocol: UiHostProtocolAgreement,
    attempt: UiMountedPresentationAttemptIdentity,
    frame: UiMountedFrameIdentity,
    binding: UiSurfaceBindingGeneration,
    mode: UiHostSurfacePresentationMode,
    nodes: Box<[UiHeadlessNodeMechanic]>,
    clips: Box<[UiHeadlessClipMechanic]>,
    paint_batches: Box<[UiHeadlessPaintBatchMechanic]>,
    unperformed_effects: Box<[UiHeadlessUnperformedEffect]>,
}

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

pub(crate) struct UiHeadlessMountedFrameTranscriptInput {
    pub host_session_identity: u64,
    pub protocol: UiHostProtocolAgreement,
    pub attempt: UiMountedPresentationAttemptIdentity,
    pub frame: UiMountedFrameIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub nodes: Vec<UiHeadlessNodeMechanic>,
    pub clips: Vec<UiHeadlessClipMechanic>,
    pub paint_batches: Vec<UiHeadlessPaintBatchMechanic>,
    pub unperformed_effects: Vec<UiHeadlessUnperformedEffect>,
}

impl UiHeadlessMountedFrameTranscript {
    pub(crate) fn new(input: UiHeadlessMountedFrameTranscriptInput) -> Self {
        Self {
            host_session_identity: input.host_session_identity,
            protocol: input.protocol,
            attempt: input.attempt,
            frame: input.frame,
            binding: input.binding,
            mode: UiHostSurfacePresentationMode::RecordOnly,
            nodes: input.nodes.into_boxed_slice(),
            clips: input.clips.into_boxed_slice(),
            paint_batches: input.paint_batches.into_boxed_slice(),
            unperformed_effects: input.unperformed_effects.into_boxed_slice(),
        }
    }

    pub const fn host_session_identity(&self) -> u64 {
        self.host_session_identity
    }

    pub const fn protocol(&self) -> UiHostProtocolAgreement {
        self.protocol
    }

    pub const fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub const fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub const fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn mode(&self) -> UiHostSurfacePresentationMode {
        self.mode
    }

    pub fn nodes(&self) -> &[UiHeadlessNodeMechanic] {
        &self.nodes
    }

    pub fn clips(&self) -> &[UiHeadlessClipMechanic] {
        &self.clips
    }

    pub fn paint_batches(&self) -> &[UiHeadlessPaintBatchMechanic] {
        &self.paint_batches
    }

    pub fn unperformed_effects(&self) -> &[UiHeadlessUnperformedEffect] {
        &self.unperformed_effects
    }
}
