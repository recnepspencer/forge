use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedClipTable, UiMountedDiagnosticProjection,
    UiMountedFilledRectReference, UiMountedFilledRectTable, UiMountedLayerTable,
    UiMountedNodeProjectionView, UiMountedNodeProjectionViewInput, UiMountedOmissionReason,
    UiMountedPaintBatchReference, UiMountedPaintBatchTable, UiMountedPaintProjection,
    UiMountedParticipationStatus, UiMountedPreviewProjection, UiMountedProjectionAudience,
    UiMountedProjectionView, UiMountedProjectionViewInput, UiMountedRealtimeBatchTable,
    UiMountedResourceTable, UiMountedSpatialBatchTable, UiSurfaceBindingGeneration,
};

use super::super::{UiMountedNodeReceipt, UiMountedProjectionDenial};
use super::{UiMountedProjectionFrame, UiMountedProjectionNodeRecord};

#[derive(Clone)]
pub(super) struct UiMountedOrdinaryPaintSelector {
    receipt: crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    batch: UiMountedPaintBatchReference,
}

#[derive(Clone)]
pub(super) struct UiMountedPlanIndexPaintSelector {
    indexes: Box<[u32]>,
    batch: UiMountedPaintBatchReference,
}

impl UiMountedProjectionFrame {
    pub fn view_for(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<UiMountedProjectionView, UiMountedProjectionDenial> {
        let surface = self
            .semantic
            .surfaces
            .get(&binding)
            .copied()
            .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
        let filled_rects = self
            .filled_rects
            .iter()
            .copied()
            .filter(|row| row.surface() == surface.surface && row.binding() == binding)
            .collect::<Vec<_>>();
        let filled_rect_by_instance = filled_rects
            .iter()
            .enumerate()
            .map(|(index, row)| {
                u16::try_from(index)
                    .map(|index| {
                        (
                            row.mounted_instance(),
                            UiMountedFilledRectReference::from_runtime_mounting(index),
                        )
                    })
                    .map_err(|_| UiMountedProjectionDenial::StaticPaintCapacityExceeded)
            })
            .collect::<Result<std::collections::BTreeMap<_, _>, _>>()?;
        let nodes = self
            .semantic
            .order
            .iter()
            .filter_map(|instance| self.semantic.nodes.get(instance))
            .filter(|node| node.receipt.semantic_surface() == surface.surface)
            .map(|node| self.audience_node_view(node, surface.audience, &filled_rect_by_instance))
            .collect();
        let filled_rects = UiMountedFilledRectTable::from_runtime_mounting(filled_rects)
            .map_err(|_| UiMountedProjectionDenial::StaticPaintCapacityExceeded)?;
        Ok(UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame: self.frame,
            surface: surface.surface,
            binding,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(self.layers.clone()),
            filled_rects,
            paint_batches: UiMountedPaintBatchTable::new(self.paint_batches.clone()),
            spatial_batches: UiMountedSpatialBatchTable::new(self.spatial_batches.clone()),
            realtime_batches: UiMountedRealtimeBatchTable::new(self.realtime_batches.clone()),
            resources: UiMountedResourceTable::new(self.resources.clone()),
        }))
    }

    fn audience_node_view(
        &self,
        node: &UiMountedProjectionNodeRecord,
        audience: UiMountedProjectionAudience,
        filled_rect_by_instance: &std::collections::BTreeMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            UiMountedFilledRectReference,
        >,
    ) -> UiMountedNodeProjectionView {
        let receipt = &node.receipt;
        let accessibility = if audience.accessibility_disclosed() {
            receipt.accessibility()
        } else {
            UiMountedAccessibilityProjection::Omitted(
                UiMountedOmissionReason::SurfacePolicyWithheld,
            )
        };
        let diagnostic = if audience.diagnostics_disclosed() {
            receipt.diagnostic()
        } else {
            UiMountedDiagnosticProjection::Omitted(UiMountedOmissionReason::SurfacePolicyWithheld)
        };
        UiMountedNodeProjectionView::new(UiMountedNodeProjectionViewInput {
            mounted_instance: receipt.mounted_instance(),
            node_receipt: self
                .receipt_basis
                .receipt_for(receipt.mounted_instance())
                .expect("projected semantic nodes belong to the frame receipt basis"),
            role: receipt.role(),
            participation: receipt.participation(),
            allocation: receipt.allocation(),
            preview: self.preview_for(receipt),
            paint: self.paint_for(node, filled_rect_by_instance),
            accessibility,
            motion: receipt.motion(),
            diagnostic,
        })
    }

    fn paint_for(
        &self,
        node: &UiMountedProjectionNodeRecord,
        filled_rect_by_instance: &std::collections::BTreeMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            UiMountedFilledRectReference,
        >,
    ) -> UiMountedPaintProjection {
        if node.receipt.participation().paint().status() != UiMountedParticipationStatus::Admitted {
            return UiMountedPaintProjection::Omitted(
                UiMountedOmissionReason::NotProducedByExecutedLane,
            );
        }
        if let Some(reference) = filled_rect_by_instance.get(&node.receipt.mounted_instance()) {
            return UiMountedPaintProjection::FilledRect(*reference);
        }
        self.plan_index_paint_selectors
            .iter()
            .rev()
            .find_map(|selector| selector.batch_for(node.plan_index))
            .or_else(|| {
                self.ordinary_paint_selector
                    .as_ref()
                    .and_then(|selector| selector.batch_for(node.plan_index))
            })
            .map_or_else(
                || {
                    UiMountedPaintProjection::Omitted(
                        UiMountedOmissionReason::NotProducedByExecutedLane,
                    )
                },
                UiMountedPaintProjection::CountOnlyBatch,
            )
    }

    fn preview_for(&self, receipt: &UiMountedNodeReceipt) -> UiMountedPreviewProjection {
        self.preview
            .filter(|preview| preview.mounted_instance == receipt.mounted_instance())
            .map_or_else(
                || {
                    UiMountedPreviewProjection::Omitted(
                        UiMountedOmissionReason::NotProducedByExecutedLane,
                    )
                },
                |preview| {
                    UiMountedPreviewProjection::resize(
                        preview.frame_epoch,
                        preview.extent_subpixels,
                        preview.candidate_count,
                        preview.all_candidates_admitted,
                    )
                },
            )
    }
}

impl UiMountedOrdinaryPaintSelector {
    pub(super) fn new(
        receipt: crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
        batch: UiMountedPaintBatchReference,
    ) -> Self {
        Self { receipt, batch }
    }

    pub(super) fn batch_for(
        &self,
        plan_index: Option<u32>,
    ) -> Option<UiMountedPaintBatchReference> {
        let plan_index = plan_index?;
        self.receipt
            .touch()
            .names_plan_index(plan_index)
            .then_some(self.batch)
    }
}

impl UiMountedPlanIndexPaintSelector {
    pub(super) fn new(indexes: Box<[u32]>, batch: UiMountedPaintBatchReference) -> Self {
        Self { indexes, batch }
    }

    pub(super) fn batch_for(
        &self,
        plan_index: Option<u32>,
    ) -> Option<UiMountedPaintBatchReference> {
        let plan_index = plan_index?;
        self.indexes.contains(&plan_index).then_some(self.batch)
    }
}
