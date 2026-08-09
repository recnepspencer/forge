use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationMode, UiMountedAllocationBasis,
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedClipTable,
    UiMountedContentGeneration, UiMountedCoordinateSpace, UiMountedFilledRectCompletionInput,
    UiMountedFilledRectMechanic, UiMountedFilledRectTable, UiMountedFrameIdentity,
    UiMountedHitTestTable, UiMountedInstanceIdentity, UiMountedLayerTable,
    UiMountedNodeReceiptIssuer, UiMountedPaintBatchTable, UiMountedProjectionView,
    UiMountedProjectionViewInput, UiMountedResourceTable, UiMountedRgba8,
    UiMountedSemanticTextTable, UiMountedSpatialBatchTable, UiMountedSurfaceBindingRequirement,
    UiMountedTransformProjection, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityObservationGeneration,
};

use super::rect_node::rect_node;

pub(super) struct MountedPresentationWorld {
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    content: UiMountedContentGeneration,
    pub(super) first_instance: UiMountedInstanceIdentity,
    pub(super) second_instance: UiMountedInstanceIdentity,
    pub(super) requirement: UiMountedSurfaceBindingRequirement,
}

#[derive(Clone, Copy)]
pub(super) struct RectSpec {
    instance: UiMountedInstanceIdentity,
    pub(super) x: f32,
    pub(super) color: UiMountedRgba8,
    pub(super) clip_x: f32,
    clip_width: f32,
}

pub(super) fn rect_spec(instance: UiMountedInstanceIdentity, x: f32) -> RectSpec {
    RectSpec {
        instance,
        x,
        color: UiMountedRgba8::new(47, 129, 247, 255),
        clip_x: x,
        clip_width: 32.0,
    }
}

pub(super) fn rect_spec_with_clip(
    instance: UiMountedInstanceIdentity,
    x: f32,
    clip_x: f32,
    clip_width: f32,
) -> RectSpec {
    RectSpec {
        instance,
        x,
        color: UiMountedRgba8::new(47, 129, 247, 255),
        clip_x,
        clip_width,
    }
}

impl MountedPresentationWorld {
    pub(super) fn new() -> Self {
        let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let generation = WorthUiHostCapabilityObservationGeneration::new(7);
        let requirement = UiMountedSurfaceBindingRequirement::new(
            surface,
            UiHostSurfaceIdentity::mint_unbound().unwrap(),
            binding,
            generation,
            11,
            UiHostSurfacePresentationMode::RecordOnly,
        );
        Self {
            surface,
            binding,
            content: UiMountedContentGeneration::mint_unbound().unwrap(),
            first_instance: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            second_instance: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            requirement,
        }
    }

    pub(super) fn projection(
        &self,
        frame: UiMountedFrameIdentity,
        specs: impl IntoIterator<Item = RectSpec>,
    ) -> UiMountedProjectionView {
        let rows = specs
            .into_iter()
            .map(|spec| self.rect(frame, spec))
            .collect::<Vec<_>>();
        let nodes = rows
            .iter()
            .enumerate()
            .map(|(index, row)| rect_node(index, row))
            .collect();
        UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame,
            surface: self.surface,
            binding: self.binding,
            content_generation: self.content,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(Vec::new()),
            filled_rects: UiMountedFilledRectTable::from_runtime_mounting(rows).unwrap(),
            semantic_text: UiMountedSemanticTextTable::empty(),
            hit_tests: UiMountedHitTestTable::empty(),
            paint_batches: UiMountedPaintBatchTable::new(Vec::new()),
            spatial_batches: UiMountedSpatialBatchTable::new(Vec::new()),
            realtime_batches: worth_ui_host_contract::UiMountedRealtimeBatchTable::new(Vec::new()),
            resources: UiMountedResourceTable::new(Vec::new()),
        })
    }

    fn rect(&self, frame: UiMountedFrameIdentity, spec: RectSpec) -> UiMountedFilledRectMechanic {
        let bounds = canonical_box(spec.x, 0.0, 32.0, 24.0);
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(
            UiMountedFilledRectCompletionInput {
                frame,
                surface: self.surface,
                binding: self.binding,
                mounted_instance: spec.instance,
                node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
                    .unwrap()
                    .receipt_for(spec.instance),
                allocation_basis: UiMountedAllocationBasis::new(
                    1,
                    2,
                    3,
                    UiMountedTransformProjection::Identity,
                ),
                bounds,
                color: spec.color,
                layer_semantic_order: (spec.x as u32) / 40,
                clip_bounds: canonical_box(spec.clip_x, 0.0, spec.clip_width, 24.0),
            },
        )
        .unwrap()
    }
}

fn canonical_box(x: f32, y: f32, width: f32, height: f32) -> UiMountedCanonicalBox {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width,
        height,
        coordinate_space: UiMountedCoordinateSpace::HostSurface,
    })
    .unwrap()
}
