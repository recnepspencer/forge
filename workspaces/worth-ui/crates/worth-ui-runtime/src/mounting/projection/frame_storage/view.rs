use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedClipTable, UiMountedDiagnosticProjection,
    UiMountedFilledRectMechanic, UiMountedFilledRectReference, UiMountedFilledRectTable,
    UiMountedHitTestMechanic, UiMountedHitTestProjection, UiMountedHitTestReference,
    UiMountedHitTestTable, UiMountedInstanceIdentity, UiMountedLayerTable,
    UiMountedNodeProjectionView, UiMountedNodeProjectionViewInput, UiMountedOmissionReason,
    UiMountedPaintBatchReference, UiMountedPaintBatchTable, UiMountedPaintProjection,
    UiMountedParticipationStatus, UiMountedPreviewProjection, UiMountedProjectionAudience,
    UiMountedProjectionView, UiMountedProjectionViewInput, UiMountedRealtimeBatchTable,
    UiMountedResourceTable, UiMountedSemanticTextMechanic, UiMountedSemanticTextReference,
    UiMountedSemanticTextTable, UiMountedSpatialBatchTable, UiSurfaceBindingGeneration,
};

use super::super::{UiMountedNodeReceipt, UiMountedProjectionDenial};
use super::{UiMountedProjectionFrame, UiMountedProjectionNodeRecord, UiMountedProjectionSurface};

type UiMountedFilledRectReferenceIndex =
    std::collections::BTreeMap<UiMountedInstanceIdentity, UiMountedFilledRectReference>;
type UiMountedHitTestReferenceIndex =
    std::collections::BTreeMap<UiMountedInstanceIdentity, UiMountedHitTestReference>;
type UiMountedSemanticTextReferenceIndex =
    std::collections::BTreeMap<UiMountedInstanceIdentity, Vec<UiMountedSemanticTextReference>>;
use super::drawable_order::{
    drawable_reference_index, validate_drawable_coverage, UiMountedDrawableReferenceIndex,
};

struct UiMountedFilledRectViewRows {
    rows: Vec<UiMountedFilledRectMechanic>,
    references: UiMountedFilledRectReferenceIndex,
}

struct UiMountedHitTestViewRows {
    rows: Vec<UiMountedHitTestMechanic>,
    references: UiMountedHitTestReferenceIndex,
}

struct UiMountedSemanticTextViewRows {
    rows: Vec<UiMountedSemanticTextMechanic>,
    references: UiMountedSemanticTextReferenceIndex,
}

struct UiMountedNodeViewContext<'a> {
    surface: UiMountedProjectionSurface,
    filled_rect_by_instance: &'a UiMountedFilledRectReferenceIndex,
    hit_test_by_instance: &'a UiMountedHitTestReferenceIndex,
    semantic_text_by_instance: &'a UiMountedSemanticTextReferenceIndex,
    drawables_by_instance: &'a UiMountedDrawableReferenceIndex,
}

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
        let filled_rects = self.filled_rect_view_rows(surface)?;
        let hit_tests = self.hit_test_view_rows(surface)?;
        let semantic_text = self.semantic_text_view_rows(surface)?;
        let drawables = drawable_reference_index(&filled_rects.rows, &semantic_text.rows)?;
        let node_view_context = UiMountedNodeViewContext {
            surface,
            filled_rect_by_instance: &filled_rects.references,
            hit_test_by_instance: &hit_tests.references,
            semantic_text_by_instance: &semantic_text.references,
            drawables_by_instance: &drawables,
        };
        let nodes = self
            .semantic
            .order
            .iter()
            .filter_map(|instance| self.semantic.nodes.get(instance))
            .filter(|node| node.receipt.semantic_surface() == surface.surface)
            .map(|node| self.audience_node_view(node, &node_view_context))
            .collect::<Vec<_>>();
        validate_drawable_coverage(
            &drawables,
            &nodes,
            filled_rects.rows.len() + semantic_text.rows.len(),
        )?;
        let materialized_rows = [
            nodes.len(),
            filled_rects.rows.len(),
            semantic_text.rows.len(),
            hit_tests.rows.len(),
            self.layers.len(),
            self.paint_batches.len(),
            self.spatial_batches.len(),
            self.realtime_batches.len(),
            self.resources.len(),
        ]
        .into_iter()
        .try_fold(0usize, usize::checked_add)
        .and_then(|rows| u64::try_from(rows).ok())
        .ok_or(UiMountedProjectionDenial::CostCounterOverflow)?;
        let (authored_paint_commands, authored_paint_order) =
            super::presentation_sources::compile(&nodes, &filled_rects.rows, &semantic_text.rows);
        let filled_rects = UiMountedFilledRectTable::from_runtime_mounting(filled_rects.rows)
            .map_err(|_| UiMountedProjectionDenial::StaticPaintCapacityExceeded)?;
        let hit_tests = UiMountedHitTestTable::from_runtime_mounting(hit_tests.rows)
            .ok_or(UiMountedProjectionDenial::HitTestCapacityExceeded)?;
        let semantic_text = UiMountedSemanticTextTable::from_runtime_mounting(semantic_text.rows)
            .map_err(|_| UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
        let projection = UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame: self.frame,
            surface: surface.surface,
            binding,
            content_generation: self.content_generation,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(self.layers.clone()),
            filled_rects,
            semantic_text,
            hit_tests,
            paint_batches: UiMountedPaintBatchTable::new(self.paint_batches.clone()),
            spatial_batches: UiMountedSpatialBatchTable::new(self.spatial_batches.clone()),
            realtime_batches: UiMountedRealtimeBatchTable::new(self.realtime_batches.clone()),
            resources: UiMountedResourceTable::new(self.resources.clone()),
            authored_paint_commands,
            authored_paint_order,
        });
        self.materialized_projection_rows.set(
            self.materialized_projection_rows
                .get()
                .checked_add(materialized_rows)
                .ok_or(UiMountedProjectionDenial::CostCounterOverflow)?,
        );
        Ok(projection)
    }

    fn semantic_text_view_rows(
        &self,
        surface: UiMountedProjectionSurface,
    ) -> Result<UiMountedSemanticTextViewRows, UiMountedProjectionDenial> {
        let rows = self.mechanics.semantic_text_for(
            &self.semantic,
            surface.surface,
            surface.binding,
            self.content_generation,
            self.frame,
            &self.receipt_basis,
        )?;
        let mut references = UiMountedSemanticTextReferenceIndex::new();
        for (index, row) in rows.iter().enumerate() {
            let reference = u16::try_from(index)
                .map(UiMountedSemanticTextReference::from_runtime_mounting)
                .map_err(|_| UiMountedProjectionDenial::SemanticTextCapacityExceeded)?;
            references
                .entry(row.mounted_instance())
                .or_default()
                .push(reference);
        }
        Ok(UiMountedSemanticTextViewRows { rows, references })
    }

    fn filled_rect_view_rows(
        &self,
        surface: UiMountedProjectionSurface,
    ) -> Result<UiMountedFilledRectViewRows, UiMountedProjectionDenial> {
        let rows = self.mechanics.filled_rects_for(
            &self.semantic,
            surface.surface,
            surface.binding,
            self.frame,
            &self.receipt_basis,
        )?;
        let references = rows
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
            .collect::<Result<UiMountedFilledRectReferenceIndex, _>>()?;
        Ok(UiMountedFilledRectViewRows { rows, references })
    }

    fn hit_test_view_rows(
        &self,
        surface: UiMountedProjectionSurface,
    ) -> Result<UiMountedHitTestViewRows, UiMountedProjectionDenial> {
        let rows = self.mechanics.hit_tests_for(
            &self.semantic,
            surface.surface,
            surface.binding,
            self.frame,
            &self.receipt_basis,
        )?;
        let references = rows
            .iter()
            .enumerate()
            .map(|(index, row)| {
                u32::try_from(index)
                    .map(|index| {
                        (
                            row.mounted_instance(),
                            UiMountedHitTestReference::from_runtime_mounting(index),
                        )
                    })
                    .map_err(|_| UiMountedProjectionDenial::HitTestCapacityExceeded)
            })
            .collect::<Result<UiMountedHitTestReferenceIndex, _>>()?;
        Ok(UiMountedHitTestViewRows { rows, references })
    }

    fn audience_node_view(
        &self,
        node: &UiMountedProjectionNodeRecord,
        context: &UiMountedNodeViewContext<'_>,
    ) -> UiMountedNodeProjectionView {
        let receipt = &node.receipt;
        let audience = context.surface.audience;
        let accessibility = if audience.accessibility_disclosed() {
            receipt.accessibility()
        } else {
            UiMountedAccessibilityProjection::Omitted(
                UiMountedOmissionReason::SurfacePolicyWithheld,
            )
        };
        let diagnostic = self.diagnostic_for(receipt, context.surface, audience);
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
            paint: self.paint_for(node, context.filled_rect_by_instance),
            hit_test: context
                .hit_test_by_instance
                .get(&receipt.mounted_instance())
                .copied()
                .map_or_else(
                    || {
                        UiMountedHitTestProjection::Omitted(
                            UiMountedOmissionReason::NotProducedByExecutedLane,
                        )
                    },
                    UiMountedHitTestProjection::Region,
                ),
            accessibility,
            motion: receipt.motion(),
            diagnostic,
            drawables: context
                .drawables_by_instance
                .get(&receipt.mounted_instance())
                .map(|drawables| drawables.to_vec())
                .unwrap_or_default(),
            semantic_text: context
                .semantic_text_by_instance
                .get(&receipt.mounted_instance())
                .cloned()
                .unwrap_or_default(),
        })
    }

    fn diagnostic_for(
        &self,
        receipt: &UiMountedNodeReceipt,
        surface: UiMountedProjectionSurface,
        audience: UiMountedProjectionAudience,
    ) -> UiMountedDiagnosticProjection {
        if !audience.diagnostics_disclosed() {
            return UiMountedDiagnosticProjection::Omitted(
                UiMountedOmissionReason::SurfacePolicyWithheld,
            );
        }
        self.visual_overlay
            .filter(|overlay| {
                overlay.target_receipt.mounted_instance() == receipt.mounted_instance()
            })
            .and_then(|overlay| overlay.mechanic_for(self.frame, surface.surface, surface.binding))
            .map_or_else(
                || receipt.diagnostic(),
                UiMountedDiagnosticProjection::IdentityOverlay,
            )
    }

    fn paint_for(
        &self,
        node: &UiMountedProjectionNodeRecord,
        filled_rect_by_instance: &UiMountedFilledRectReferenceIndex,
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
