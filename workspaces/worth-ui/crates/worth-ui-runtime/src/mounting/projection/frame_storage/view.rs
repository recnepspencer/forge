use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedClipTable, UiMountedDiagnosticProjection,
    UiMountedLayerTable, UiMountedNodeProjectionView, UiMountedNodeProjectionViewInput,
    UiMountedOmissionReason, UiMountedPaintBatchReference, UiMountedPaintBatchTable,
    UiMountedPaintProjection, UiMountedParticipationStatus, UiMountedPreviewProjection,
    UiMountedProjectionAudience, UiMountedProjectionView, UiMountedProjectionViewInput,
    UiMountedRealtimeBatchTable, UiMountedResourceTable, UiMountedSpatialBatchTable,
    UiSurfaceBindingGeneration,
};

use super::super::{UiMountedNodeReceipt, UiMountedProjectionDenial};
use super::{UiMountedProjectionFrame, UiMountedProjectionNodeRecord};

#[derive(Clone)]
pub(super) enum UiMountedPaintSelector {
    Ordinary {
        receipt: crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
        batch: UiMountedPaintBatchReference,
    },
    PlanIndexes {
        indexes: Box<[u32]>,
        batch: UiMountedPaintBatchReference,
    },
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
        let nodes = self
            .semantic
            .order
            .iter()
            .filter_map(|instance| self.semantic.nodes.get(instance))
            .filter(|node| node.receipt.semantic_surface() == surface.surface)
            .map(|node| self.audience_node_view(node, surface.audience))
            .collect();
        Ok(UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame: self.frame,
            surface: surface.surface,
            binding,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(self.layers.clone()),
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
            paint: self.paint_for(node),
            accessibility,
            motion: receipt.motion(),
            diagnostic,
        })
    }

    fn paint_for(&self, node: &UiMountedProjectionNodeRecord) -> UiMountedPaintProjection {
        if node.receipt.participation().paint().status() != UiMountedParticipationStatus::Admitted {
            return UiMountedPaintProjection::Omitted(
                UiMountedOmissionReason::NotProducedByExecutedLane,
            );
        }
        self.paint_selectors
            .iter()
            .rev()
            .find_map(|selector| selector.batch_for(node.plan_index))
            .map_or_else(
                || {
                    UiMountedPaintProjection::Omitted(
                        UiMountedOmissionReason::NotProducedByExecutedLane,
                    )
                },
                UiMountedPaintProjection::Batch,
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

impl UiMountedPaintSelector {
    pub(super) fn batch_for(
        &self,
        plan_index: Option<u32>,
    ) -> Option<UiMountedPaintBatchReference> {
        let plan_index = plan_index?;
        match self {
            Self::Ordinary { receipt, batch } => receipt
                .touch()
                .names_plan_index(plan_index)
                .then_some(*batch),
            Self::PlanIndexes { indexes, batch } => indexes.contains(&plan_index).then_some(*batch),
        }
    }
}
