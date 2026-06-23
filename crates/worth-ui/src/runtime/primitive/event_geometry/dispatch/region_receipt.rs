use super::super::super::{
    WorthUiBoxEdges, WorthUiPrimitiveDrawPlan, WorthUiPrimitiveFrame, WorthUiPrimitiveProofReceipt,
    WorthUiPrimitiveResolvedCursorPosture,
};
use super::super::digest::event_region_digest;
use super::super::receipt::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveHitArea, WorthUiPrimitivePointerCapture,
};
use super::dispatch_receipt::{
    WorthUiPrimitiveHitFrameDerivationBasis, WorthUiPrimitiveHitFrameDerivationReceipt,
};

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
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    can_activate: bool,
    containment: WorthUiPrimitiveEventContainment,
    capture: WorthUiPrimitivePointerCapture,
    receipt_digest: u64,
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
            hit_frame_for_primitive_event_contract(visual_frame, primitive)
        else {
            return None;
        };
        let cursor = affordance.cursor();
        let receipt_digest = event_region_digest(
            primitive.surface_id(),
            primitive.interaction().interaction_id(),
            parent_surface_id.as_deref(),
            order,
            visual_frame,
            hit_frame,
            cursor,
            affordance.can_activate(),
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
            cursor,
            can_activate: affordance.can_activate(),
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

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn can_activate(&self) -> bool {
        self.can_activate
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
) -> Option<(
    WorthUiPrimitiveFrame,
    WorthUiPrimitiveHitFrameDerivationReceipt,
)> {
    let event_geometry = primitive.event_geometry();
    let affordance = primitive.interaction().affordance();
    if affordance.disabled_posture()
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
            let edges = primitive.flow_layout().padding_edges();
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
