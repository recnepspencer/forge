use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedClipProjection, UiMountedClipTable,
    UiMountedDiagnosticProjection, UiMountedLayerProjection, UiMountedLayerReference,
    UiMountedLayerRow, UiMountedLayerTable, UiMountedNodeProjectionView,
    UiMountedNodeProjectionViewInput, UiMountedOmissionReason, UiMountedPaintBatchReference,
    UiMountedPaintBatchRow, UiMountedPaintBatchTable, UiMountedPaintPrimitiveKind,
    UiMountedPaintProjection, UiMountedParticipationStatus, UiMountedPreviewProjection,
    UiMountedProjectionAudience, UiMountedProjectionView, UiMountedProjectionViewInput,
    UiMountedRealtimeBatchRow, UiMountedRealtimeBatchTable, UiMountedResourceEntry,
    UiMountedResourceKind, UiMountedResourceReference, UiMountedResourceTable,
    UiMountedSpatialBatchRow, UiMountedSpatialBatchTable, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration,
};

use super::{UiMountedNodeReceipt, UiMountedProjectionDenial};

const TABLE_LIMIT: usize = 2_048;
const RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone)]
pub(super) struct UiMountedProjectionNodeRecord {
    pub receipt: UiMountedNodeReceipt,
    pub plan_index: Option<u32>,
}

#[derive(Clone, Copy)]
pub(super) struct UiMountedProjectionSurface {
    pub surface: UiSemanticSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub audience: UiMountedProjectionAudience,
}

#[derive(Clone)]
pub(super) struct UiMountedSemanticProjection {
    nodes: crate::runtime::persistent_index::UiPersistentOrdMap<
        worth_ui_host_contract::UiMountedInstanceIdentity,
        UiMountedProjectionNodeRecord,
    >,
    order: std::rc::Rc<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
    membership: crate::runtime::persistent_index::UiPersistentOrdSet<
        worth_ui_host_contract::UiMountedInstanceIdentity,
    >,
    semantic_surfaces:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiSemanticSurfaceIdentity>,
    binding_by_surface: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiSemanticSurfaceIdentity,
        UiSurfaceBindingGeneration,
    >,
    surfaces: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiSurfaceBindingGeneration,
        UiMountedProjectionSurface,
    >,
}

#[derive(Clone)]
pub struct UiMountedProjectionFrame {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    receipt_basis: super::super::UiMountedNodeReceiptBasis,
    plan_digest: u64,
    semantic: UiMountedSemanticProjection,
    paint_batches: Vec<UiMountedPaintBatchRow>,
    layers: Vec<UiMountedLayerRow>,
    spatial_batches: Vec<UiMountedSpatialBatchRow>,
    realtime_batches: Vec<UiMountedRealtimeBatchRow>,
    resources: Vec<UiMountedResourceEntry>,
    ordinary_recorded: bool,
    virtualized_recorded: bool,
    canvas_recorded: bool,
    realtime_recorded: bool,
    paint_selectors: Vec<UiMountedPaintSelector>,
    preview: Option<super::lowering::UiMountedPreviewProjectionInput>,
    counters: super::super::UiMountStageCounters,
}

#[derive(Clone)]
enum UiMountedPaintSelector {
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
    pub(super) fn new(
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        receipt_basis: super::super::UiMountedNodeReceiptBasis,
        plan_digest: u64,
        semantic: UiMountedSemanticProjection,
        counters: super::super::UiMountStageCounters,
    ) -> Self {
        Self {
            frame,
            receipt_basis,
            plan_digest,
            semantic,
            paint_batches: Vec::new(),
            layers: Vec::new(),
            spatial_batches: Vec::new(),
            realtime_batches: Vec::new(),
            resources: Vec::new(),
            ordinary_recorded: false,
            virtualized_recorded: false,
            canvas_recorded: false,
            realtime_recorded: false,
            paint_selectors: Vec::new(),
            preview: None,
            counters,
        }
    }

    pub fn frame_identity(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }
    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
    pub(crate) fn mounted_instances(
        &self,
    ) -> impl ExactSizeIterator<Item = worth_ui_host_contract::UiMountedInstanceIdentity> + '_ {
        self.semantic.order.iter().copied()
    }

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.counters.finish()
    }

    pub(super) fn record_ordinary(
        &mut self,
        receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.ordinary_recorded)?;
        let batch = self.push_lane_batch(
            receipt.touch().row_count() as u32,
            0,
            None,
            UiMountedPaintPrimitiveKind::FilledRect,
        )?;
        self.paint_selectors.push(UiMountedPaintSelector::Ordinary {
            receipt: receipt.clone(),
            batch,
        });
        Ok(())
    }

    pub(super) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.virtualized_recorded)?;
        let range = receipt.visible_range();
        let count = range
            .row_count()
            .checked_mul(range.column_count())
            .ok_or(UiMountedProjectionDenial::TableCapacityExceeded)?;
        let batch =
            self.push_lane_batch(count, 1, None, UiMountedPaintPrimitiveKind::FilledRect)?;
        self.push_plan_index_selector([receipt.touched_plan_index()], batch);
        Ok(())
    }

    pub(super) fn record_canvas(
        &mut self,
        receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        resource_content_identity: u64,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.canvas_recorded)?;
        if self.spatial_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedSpatialBatchRow>(1)?;
        self.spatial_batches.push(UiMountedSpatialBatchRow::new(
            receipt.visible_primitive_count(),
            receipt.queried_hit_test_region_count(),
            receipt.touched_overlay_row_count(),
            receipt.touched_tool_state_row_count(),
        ));
        let resource = self.intern_canvas_resource(resource_content_identity)?;
        let batch = self.push_lane_batch(
            receipt.visible_primitive_count(),
            2,
            Some(resource),
            UiMountedPaintPrimitiveKind::CanvasSpatialBatch,
        )?;
        self.push_plan_index_selector(receipt.touched_plan_indexes().iter().copied(), batch);
        Ok(())
    }

    pub(super) fn record_realtime(
        &mut self,
        receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.realtime_recorded)?;
        if self.realtime_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedRealtimeBatchRow>(1)?;
        self.realtime_batches.push(UiMountedRealtimeBatchRow::new(
            receipt.touched_overlay_row_count(),
        ));
        let batch = self.push_lane_batch(
            u32::from(receipt.touched_overlay_row_count()),
            3,
            None,
            UiMountedPaintPrimitiveKind::RealtimeBatch,
        )?;
        self.push_plan_index_selector(receipt.touched_plan_indexes().iter().copied(), batch);
        Ok(())
    }

    pub(super) fn record_preview(
        &mut self,
        preview: super::lowering::UiMountedPreviewProjectionInput,
    ) -> Result<(), UiMountedProjectionDenial> {
        let node = self
            .semantic
            .nodes
            .get(&preview.mounted_instance)
            .ok_or(UiMountedProjectionDenial::PreviewInstanceMismatch)?;
        if node.receipt.graph_node() != preview.graph_node {
            return Err(UiMountedProjectionDenial::PreviewInstanceMismatch);
        }
        self.preview = Some(preview);
        Ok(())
    }

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

    pub(crate) fn rebound(
        &self,
        replacements: &[(
            UiSurfaceBindingGeneration,
            super::super::UiSurfaceBindingIdentityView,
        )],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut rebound = self.clone();
        for (affected, replacement) in replacements {
            let mut surface = rebound
                .semantic
                .surfaces
                .get(affected)
                .copied()
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            if surface.surface != replacement.semantic_surface_identity() {
                return Err(UiMountedProjectionDenial::MissingSurfaceBinding);
            }
            rebound.semantic.surfaces.remove(affected);
            surface.binding = replacement.binding_generation();
            rebound.semantic.surfaces.insert(surface.binding, surface);
        }
        Ok(rebound)
    }

    pub(super) fn semantic_projection(&self) -> &UiMountedSemanticProjection {
        &self.semantic
    }

    fn push_paint_batch(
        &mut self,
        primitive_count: u32,
        layer: UiMountedLayerReference,
        resource: Option<UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Result<UiMountedPaintBatchReference, UiMountedProjectionDenial> {
        if self.paint_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedPaintBatchRow>(1)?;
        let batch_index = u16::try_from(self.paint_batches.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.paint_batches.push(UiMountedPaintBatchRow::new(
            primitive_count,
            UiMountedLayerProjection::Layer(layer),
            resource,
            primitive_kind,
        ));
        Ok(UiMountedPaintBatchReference::new(batch_index))
    }

    fn push_lane_batch(
        &mut self,
        primitive_count: u32,
        semantic_order: u32,
        resource: Option<UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Result<UiMountedPaintBatchReference, UiMountedProjectionDenial> {
        let layer = self.push_layer(semantic_order)?;
        self.push_paint_batch(primitive_count, layer, resource, primitive_kind)
    }

    fn push_plan_index_selector(
        &mut self,
        indexes: impl IntoIterator<Item = u32>,
        batch: UiMountedPaintBatchReference,
    ) {
        self.paint_selectors
            .push(UiMountedPaintSelector::PlanIndexes {
                indexes: indexes.into_iter().collect(),
                batch,
            });
    }

    fn push_layer(
        &mut self,
        semantic_order: u32,
    ) -> Result<UiMountedLayerReference, UiMountedProjectionDenial> {
        if self.layers.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedLayerRow>(1)?;
        let index = u16::try_from(self.layers.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.layers.push(UiMountedLayerRow::new(
            semantic_order,
            UiMountedClipProjection::Unclipped,
        ));
        Ok(UiMountedLayerReference::new(index))
    }

    fn intern_canvas_resource(
        &mut self,
        content_identity: u64,
    ) -> Result<UiMountedResourceReference, UiMountedProjectionDenial> {
        if let Some(index) = self
            .resources
            .iter()
            .position(|entry| entry.content_identity() == content_identity)
        {
            return u16::try_from(index)
                .map(UiMountedResourceReference::new)
                .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded);
        }
        if self.resources.len() >= RESOURCE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedResourceEntry>(1)?;
        let index = u16::try_from(self.resources.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.resources.push(UiMountedResourceEntry::new(
            content_identity,
            UiMountedResourceKind::CanvasContract,
            0,
        ));
        Ok(UiMountedResourceReference::new(index))
    }

    fn record_rows<Row>(&mut self, count: usize) -> Result<(), UiMountedProjectionDenial> {
        self.counters
            .replace_rows::<Row>(count)
            .map_err(|_| UiMountedProjectionDenial::CostCounterOverflow)
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

impl UiMountedSemanticProjection {
    pub(super) fn initial(
        nodes: Vec<UiMountedProjectionNodeRecord>,
        surfaces: Vec<UiMountedProjectionSurface>,
    ) -> Self {
        let order = nodes
            .iter()
            .map(|record| record.receipt.mounted_instance())
            .collect::<Vec<_>>();
        let mut node_index = crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut membership = crate::runtime::persistent_index::UiPersistentOrdSet::default();
        for node in nodes {
            let instance = node.receipt.mounted_instance();
            node_index.insert(instance, node);
            membership.insert(instance);
        }
        let mut surface_index = crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut binding_by_surface =
            crate::runtime::persistent_index::UiPersistentOrdMap::default();
        let mut semantic_surfaces = crate::runtime::persistent_index::UiPersistentOrdSet::default();
        for surface in surfaces {
            semantic_surfaces.insert(surface.surface);
            binding_by_surface.insert(surface.surface, surface.binding);
            surface_index.insert(surface.binding, surface);
        }
        Self {
            nodes: node_index,
            order: order.into(),
            membership,
            semantic_surfaces,
            binding_by_surface,
            surfaces: surface_index,
        }
    }

    pub(super) fn membership(
        &self,
    ) -> crate::runtime::persistent_index::UiPersistentOrdSet<
        worth_ui_host_contract::UiMountedInstanceIdentity,
    > {
        self.membership.clone()
    }

    pub(super) fn supports_surfaces(&self, surfaces: &[UiSemanticSurfaceIdentity]) -> bool {
        surfaces.len() == self.semantic_surfaces.len()
            && surfaces
                .iter()
                .all(|surface| self.semantic_surfaces.contains_with_probes(surface).0)
    }

    pub(super) fn contains(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> bool {
        self.membership.contains_with_probes(&instance).0
    }

    pub(super) fn insert_node(
        &mut self,
        node: UiMountedProjectionNodeRecord,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        let instance = node.receipt.mounted_instance();
        self.membership.insert(instance);
        self.nodes.insert_with_work(instance, node)
    }

    pub(super) fn remove_node(
        &mut self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        self.membership.remove_with_work(&instance);
        self.nodes.remove_with_work(&instance).1
    }

    pub(super) fn replace_order(
        &mut self,
        order: Vec<worth_ui_host_contract::UiMountedInstanceIdentity>,
    ) {
        self.order = order.into();
    }

    pub(super) fn replace_surface(
        &mut self,
        surface: UiMountedProjectionSurface,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        if let Some(previous) = self.binding_by_surface.get(&surface.surface).copied() {
            self.surfaces.remove(&previous);
        }
        self.semantic_surfaces.insert(surface.surface);
        self.binding_by_surface
            .insert(surface.surface, surface.binding);
        self.surfaces.insert_with_work(surface.binding, surface)
    }

    pub(super) fn remove_surface(
        &mut self,
        surface: UiSemanticSurfaceIdentity,
    ) -> crate::runtime::persistent_index::UiPersistentIndexMutationWork {
        self.semantic_surfaces.remove_with_work(&surface);
        let binding = self.binding_by_surface.get(&surface).copied();
        self.binding_by_surface.remove(&surface);
        binding.map_or_else(Default::default, |binding| {
            self.surfaces.remove_with_work(&binding).1
        })
    }

    pub(super) fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub(super) fn surface_instance_count(&self, surfaces: &[UiSemanticSurfaceIdentity]) -> usize {
        self.order
            .iter()
            .filter(|instance| {
                self.nodes
                    .get(instance)
                    .is_some_and(|node| surfaces.contains(&node.receipt.semantic_surface()))
            })
            .count()
    }
}

fn require_once(recorded: &mut bool) -> Result<(), UiMountedProjectionDenial> {
    if *recorded {
        return Err(UiMountedProjectionDenial::DuplicateLaneContribution);
    }
    *recorded = true;
    Ok(())
}

impl UiMountedPaintSelector {
    fn batch_for(&self, plan_index: Option<u32>) -> Option<UiMountedPaintBatchReference> {
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
