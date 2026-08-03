use crate::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedNodeProjectionView {
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    role: super::UiMountedMechanicalRole,
    participation: super::UiMountedParticipation,
    allocation: super::UiMountedAllocationProjection,
    preview: super::UiMountedPreviewProjection,
    paint: super::UiMountedPaintProjection,
    hit_test: super::UiMountedHitTestProjection,
    accessibility: super::UiMountedAccessibilityProjection,
    motion: super::UiMountedMotionProjection,
    diagnostic: super::UiMountedDiagnosticProjection,
    semantic_text: Box<[super::UiMountedSemanticTextReference]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedProjectionView {
    frame: UiMountedFrameIdentity,
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    content_generation: crate::UiMountedContentGeneration,
    nodes: Box<[UiMountedNodeProjectionView]>,
    clips: super::UiMountedClipTable,
    layers: super::UiMountedLayerTable,
    filled_rects: super::UiMountedFilledRectTable,
    semantic_text: super::UiMountedSemanticTextTable,
    hit_tests: super::UiMountedHitTestTable,
    paint_batches: super::UiMountedPaintBatchTable,
    spatial_batches: super::UiMountedSpatialBatchTable,
    realtime_batches: super::UiMountedRealtimeBatchTable,
    resources: super::UiMountedResourceTable,
}

pub struct UiMountedNodeProjectionViewInput {
    pub mounted_instance: UiMountedInstanceIdentity,
    pub node_receipt: UiMountedNodeReceiptIdentity,
    pub role: super::UiMountedMechanicalRole,
    pub participation: super::UiMountedParticipation,
    pub allocation: super::UiMountedAllocationProjection,
    pub preview: super::UiMountedPreviewProjection,
    pub paint: super::UiMountedPaintProjection,
    pub hit_test: super::UiMountedHitTestProjection,
    pub accessibility: super::UiMountedAccessibilityProjection,
    pub motion: super::UiMountedMotionProjection,
    pub diagnostic: super::UiMountedDiagnosticProjection,
    pub semantic_text: Vec<super::UiMountedSemanticTextReference>,
}

pub struct UiMountedProjectionViewInput {
    pub frame: UiMountedFrameIdentity,
    pub surface: UiSemanticSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub content_generation: crate::UiMountedContentGeneration,
    pub nodes: Vec<UiMountedNodeProjectionView>,
    pub clips: super::UiMountedClipTable,
    pub layers: super::UiMountedLayerTable,
    pub filled_rects: super::UiMountedFilledRectTable,
    pub semantic_text: super::UiMountedSemanticTextTable,
    pub hit_tests: super::UiMountedHitTestTable,
    pub paint_batches: super::UiMountedPaintBatchTable,
    pub spatial_batches: super::UiMountedSpatialBatchTable,
    pub realtime_batches: super::UiMountedRealtimeBatchTable,
    pub resources: super::UiMountedResourceTable,
}

impl UiMountedNodeProjectionView {
    pub fn new(input: UiMountedNodeProjectionViewInput) -> Self {
        Self {
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            role: input.role,
            participation: input.participation,
            allocation: input.allocation,
            preview: input.preview,
            paint: input.paint,
            hit_test: input.hit_test,
            accessibility: input.accessibility,
            motion: input.motion,
            diagnostic: input.diagnostic,
            semantic_text: input.semantic_text.into_boxed_slice(),
        }
    }
    pub fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub fn node_receipt(&self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub fn role(&self) -> super::UiMountedMechanicalRole {
        self.role
    }
    pub fn participation(&self) -> super::UiMountedParticipation {
        self.participation
    }
    pub fn allocation(&self) -> super::UiMountedAllocationProjection {
        self.allocation
    }
    pub fn preview(&self) -> super::UiMountedPreviewProjection {
        self.preview
    }
    pub fn paint(&self) -> super::UiMountedPaintProjection {
        self.paint
    }
    pub fn hit_test(&self) -> super::UiMountedHitTestProjection {
        self.hit_test
    }
    pub fn accessibility(&self) -> super::UiMountedAccessibilityProjection {
        self.accessibility
    }
    pub fn motion(&self) -> super::UiMountedMotionProjection {
        self.motion
    }
    pub fn diagnostic(&self) -> super::UiMountedDiagnosticProjection {
        self.diagnostic
    }
    pub fn semantic_text(&self) -> &[super::UiMountedSemanticTextReference] {
        &self.semantic_text
    }
}

impl UiMountedProjectionView {
    pub fn new(input: UiMountedProjectionViewInput) -> Self {
        Self {
            frame: input.frame,
            surface: input.surface,
            binding: input.binding,
            content_generation: input.content_generation,
            nodes: input.nodes.into_boxed_slice(),
            clips: input.clips,
            layers: input.layers,
            filled_rects: input.filled_rects,
            semantic_text: input.semantic_text,
            hit_tests: input.hit_tests,
            paint_batches: input.paint_batches,
            spatial_batches: input.spatial_batches,
            realtime_batches: input.realtime_batches,
            resources: input.resources,
        }
    }
    pub fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }
    pub fn surface(&self) -> UiSemanticSurfaceIdentity {
        self.surface
    }
    pub fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }
    pub fn content_generation(&self) -> crate::UiMountedContentGeneration {
        self.content_generation
    }
    pub fn nodes(&self) -> &[UiMountedNodeProjectionView] {
        &self.nodes
    }
    pub fn clips(&self) -> &super::UiMountedClipTable {
        &self.clips
    }
    pub fn layers(&self) -> &super::UiMountedLayerTable {
        &self.layers
    }
    pub fn filled_rects(&self) -> &super::UiMountedFilledRectTable {
        &self.filled_rects
    }
    pub fn semantic_text(&self) -> &super::UiMountedSemanticTextTable {
        &self.semantic_text
    }
    pub fn hit_tests(&self) -> &super::UiMountedHitTestTable {
        &self.hit_tests
    }
    pub fn paint_batches(&self) -> &super::UiMountedPaintBatchTable {
        &self.paint_batches
    }
    pub fn spatial_batches(&self) -> &super::UiMountedSpatialBatchTable {
        &self.spatial_batches
    }
    pub fn realtime_batches(&self) -> &super::UiMountedRealtimeBatchTable {
        &self.realtime_batches
    }
    pub fn resources(&self) -> &super::UiMountedResourceTable {
        &self.resources
    }
}
