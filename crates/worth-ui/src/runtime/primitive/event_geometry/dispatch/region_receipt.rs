use super::super::super::{
    WorthUiBoxEdges, WorthUiPrimitiveActivationPosture, WorthUiPrimitiveDrawPlan,
    WorthUiPrimitiveFrame, WorthUiPrimitiveOperabilityPosture, WorthUiPrimitiveProofReceipt,
    WorthUiPrimitiveResolvedCursorPosture,
};
use super::super::digest::event_region_digest;
use super::super::receipt::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveHitArea, WorthUiPrimitivePointerCapture,
};
use super::hit_frame_receipt::{
    WorthUiPrimitiveHitFrameDerivationBasis, WorthUiPrimitiveHitFrameDerivationReceipt,
};
use crate::runtime::WorthUiRuntimeFactId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventHitTestPoint {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiPrimitiveEventRegionOrder {
    depth: u16,
    order: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventRegionReceipt {
    surface_id: String,
    interaction_id: String,
    parent_surface_id: Option<String>,
    order: WorthUiPrimitiveEventRegionOrder,
    visual_frame: WorthUiPrimitiveFrame,
    hit_frame: WorthUiPrimitiveFrame,
    hit_frame_derivation: WorthUiPrimitiveHitFrameDerivationReceipt,
    graph_basis: WorthUiPrimitiveEventRegionGraphBasis,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    activation_posture: WorthUiPrimitiveActivationPosture,
    containment: WorthUiPrimitiveEventContainment,
    capture: WorthUiPrimitivePointerCapture,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveEventRegionGraphBasis {
    surface_id: String,
    produced_fact: WorthUiRuntimeFactId,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    source_parse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiPrimitiveEventHitTestPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl WorthUiPrimitiveEventRegionOrder {
    pub fn new(depth: u16, order: u16) -> Self {
        Self { depth, order }
    }

    pub fn depth(self) -> u16 {
        self.depth
    }

    pub fn order(self) -> u16 {
        self.order
    }
}

impl WorthUiPrimitiveEventRegionGraphBasis {
    fn from_draw_plan(
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
    ) -> Self {
        let mut consumed_facts = draw_plan.graph_basis().consumed_facts().to_vec();
        consumed_facts.push(draw_plan.graph_basis().produced_fact().clone());
        consumed_facts.push(WorthUiRuntimeFactId::primitive_event_geometry(
            primitive.surface_id(),
        ));
        Self {
            surface_id: primitive.surface_id().to_owned(),
            produced_fact: WorthUiRuntimeFactId::primitive_event_region(primitive.surface_id()),
            consumed_facts,
            source_parse_count: draw_plan.graph_basis().source_parse_count(),
            artifact_scan_count: draw_plan.graph_basis().artifact_scan_count(),
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn produced_fact(&self) -> &WorthUiRuntimeFactId {
        &self.produced_fact
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn artifact_scan_count(&self) -> usize {
        self.artifact_scan_count
    }
}

impl WorthUiPrimitiveEventRegionReceipt {
    pub fn from_primitive_draw_plan(
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
        order: WorthUiPrimitiveEventRegionOrder,
    ) -> Option<Self> {
        Self::from_primitive_draw_plan_at(primitive, draw_plan, order, 0.0, 0.0)
    }

    pub fn from_primitive_draw_plan_at(
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
        order: WorthUiPrimitiveEventRegionOrder,
        offset_x: f32,
        offset_y: f32,
    ) -> Option<Self> {
        Self::from_nested_primitive_draw_plan_at(
            primitive, draw_plan, order, None, offset_x, offset_y,
        )
    }

    pub fn from_child_primitive_draw_plan_at(
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
        order: WorthUiPrimitiveEventRegionOrder,
        parent_surface_id: impl Into<String>,
        offset_x: f32,
        offset_y: f32,
    ) -> Option<Self> {
        Self::from_nested_primitive_draw_plan_at(
            primitive,
            draw_plan,
            order,
            Some(parent_surface_id.into()),
            offset_x,
            offset_y,
        )
    }

    fn from_nested_primitive_draw_plan_at(
        primitive: &WorthUiPrimitiveProofReceipt,
        draw_plan: &WorthUiPrimitiveDrawPlan,
        order: WorthUiPrimitiveEventRegionOrder,
        parent_surface_id: Option<String>,
        offset_x: f32,
        offset_y: f32,
    ) -> Option<Self> {
        let event_geometry = primitive.event_geometry();
        let affordance = primitive.interaction().affordance();
        let visual_frame = translated_frame(draw_plan.frame(), offset_x, offset_y);
        let Some((hit_frame, hit_frame_derivation)) =
            hit_frame_for_primitive_event_contract(visual_frame, primitive, draw_plan)
        else {
            return None;
        };
        let graph_basis =
            WorthUiPrimitiveEventRegionGraphBasis::from_draw_plan(primitive, draw_plan);
        let cursor = affordance.cursor();
        let receipt_digest = event_region_digest(
            primitive.surface_id(),
            primitive.interaction().interaction_id(),
            parent_surface_id.as_deref(),
            order,
            visual_frame,
            hit_frame,
            cursor,
            affordance.activation_posture(),
            event_geometry.containment(),
            event_geometry.capture(),
        );
        Some(Self {
            surface_id: primitive.surface_id().to_owned(),
            interaction_id: primitive.interaction().interaction_id().to_owned(),
            parent_surface_id,
            order,
            visual_frame,
            hit_frame,
            hit_frame_derivation,
            graph_basis,
            cursor,
            activation_posture: affordance.activation_posture(),
            containment: event_geometry.containment(),
            capture: event_geometry.capture(),
            receipt_digest,
        })
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn parent_surface_id(&self) -> Option<&str> {
        self.parent_surface_id.as_deref()
    }

    pub fn order(&self) -> WorthUiPrimitiveEventRegionOrder {
        self.order
    }

    pub fn visual_frame(&self) -> WorthUiPrimitiveFrame {
        self.visual_frame
    }

    pub fn hit_frame(&self) -> WorthUiPrimitiveFrame {
        self.hit_frame
    }

    pub fn hit_frame_derivation(&self) -> WorthUiPrimitiveHitFrameDerivationReceipt {
        self.hit_frame_derivation
    }

    pub fn graph_basis(&self) -> &WorthUiPrimitiveEventRegionGraphBasis {
        &self.graph_basis
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn activation_posture(&self) -> WorthUiPrimitiveActivationPosture {
        self.activation_posture
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.containment
    }

    pub fn capture(&self) -> WorthUiPrimitivePointerCapture {
        self.capture
    }

    pub fn contains(&self, point: WorthUiPrimitiveEventHitTestPoint) -> bool {
        frame_contains(self.hit_frame, point)
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn translated_frame(
    frame: WorthUiPrimitiveFrame,
    offset_x: f32,
    offset_y: f32,
) -> WorthUiPrimitiveFrame {
    WorthUiPrimitiveFrame::new(
        frame.x() + offset_x,
        frame.y() + offset_y,
        frame.width(),
        frame.height(),
    )
}

fn hit_frame_for_primitive_event_contract(
    visual_frame: WorthUiPrimitiveFrame,
    primitive: &WorthUiPrimitiveProofReceipt,
    draw_plan: &WorthUiPrimitiveDrawPlan,
) -> Option<(
    WorthUiPrimitiveFrame,
    WorthUiPrimitiveHitFrameDerivationReceipt,
)> {
    let event_geometry = primitive.event_geometry();
    let affordance = primitive.interaction().affordance();
    if affordance.operability().posture() == WorthUiPrimitiveOperabilityPosture::Disabled
        && event_geometry.hit_area() == WorthUiPrimitiveHitArea::DisabledNone
    {
        return None;
    }
    match event_geometry.hit_area() {
        WorthUiPrimitiveHitArea::VisualBounds | WorthUiPrimitiveHitArea::DisabledNone => Some((
            visual_frame,
            WorthUiPrimitiveHitFrameDerivationReceipt::new(
                if event_geometry.hit_area() == WorthUiPrimitiveHitArea::DisabledNone {
                    WorthUiPrimitiveHitFrameDerivationBasis::DisabledNone
                } else {
                    WorthUiPrimitiveHitFrameDerivationBasis::VisualBounds
                },
                WorthUiBoxEdges::uniform(0.0),
            ),
        )),
        WorthUiPrimitiveHitArea::PaddedBounds => {
            let edges = draw_plan.flow_padding_edges();
            Some((
                expand_frame_by_edges(visual_frame, edges),
                WorthUiPrimitiveHitFrameDerivationReceipt::new(
                    WorthUiPrimitiveHitFrameDerivationBasis::FlowPadding,
                    edges,
                ),
            ))
        }
        WorthUiPrimitiveHitArea::ExplicitHitSlop => {
            let edges = event_geometry.hit_slop_edges();
            Some((
                expand_frame_by_edges(visual_frame, edges),
                WorthUiPrimitiveHitFrameDerivationReceipt::new(
                    WorthUiPrimitiveHitFrameDerivationBasis::ExplicitHitSlop,
                    edges,
                ),
            ))
        }
    }
}

fn expand_frame_by_edges(
    visual_frame: WorthUiPrimitiveFrame,
    edges: WorthUiBoxEdges,
) -> WorthUiPrimitiveFrame {
    WorthUiPrimitiveFrame::new(
        visual_frame.x() - edges.left(),
        visual_frame.y() - edges.top(),
        visual_frame.width() + edges.horizontal(),
        visual_frame.height() + edges.vertical(),
    )
}

fn frame_contains(frame: WorthUiPrimitiveFrame, point: WorthUiPrimitiveEventHitTestPoint) -> bool {
    point.x >= frame.x()
        && point.x <= frame.x() + frame.width()
        && point.y >= frame.y()
        && point.y <= frame.y() + frame.height()
}
