use super::super::WorthUiBoxEdges;
use super::super::WorthUiPrimitiveResolvedCursorPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveEventCursor {
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    Resize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveHitArea {
    VisualBounds,
    PaddedBounds,
    ExplicitHitSlop,
    DisabledNone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveEventContainment {
    Contain,
    Bubble,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitivePointerCapture {
    None,
    PressDrag,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventGeometryReceipt {
    cursor: WorthUiPrimitiveEventCursor,
    hit_area: WorthUiPrimitiveHitArea,
    hit_slop_token: String,
    hit_slop_edges: WorthUiBoxEdges,
    containment: WorthUiPrimitiveEventContainment,
    capture: WorthUiPrimitivePointerCapture,
    receipt_digest: u64,
}

impl WorthUiPrimitiveEventGeometryReceipt {
    pub(crate) fn new(
        cursor: WorthUiPrimitiveEventCursor,
        hit_area: WorthUiPrimitiveHitArea,
        hit_slop_token: impl Into<String>,
        hit_slop_edges: WorthUiBoxEdges,
        containment: WorthUiPrimitiveEventContainment,
        capture: WorthUiPrimitivePointerCapture,
        receipt_digest: u64,
    ) -> Self {
        Self {
            cursor,
            hit_area,
            hit_slop_token: hit_slop_token.into(),
            hit_slop_edges,
            containment,
            capture,
            receipt_digest,
        }
    }

    pub fn cursor(&self) -> WorthUiPrimitiveEventCursor {
        self.cursor
    }

    pub fn resolved_cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor.resolved_cursor()
    }

    pub fn hit_area(&self) -> WorthUiPrimitiveHitArea {
        self.hit_area
    }

    pub fn hit_slop_token(&self) -> &str {
        &self.hit_slop_token
    }

    pub fn hit_slop_edges(&self) -> WorthUiBoxEdges {
        self.hit_slop_edges
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.containment
    }

    pub fn capture(&self) -> WorthUiPrimitivePointerCapture {
        self.capture
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiPrimitiveEventCursor {
    pub fn resolved_cursor(self) -> WorthUiPrimitiveResolvedCursorPosture {
        match self {
            Self::Default => WorthUiPrimitiveResolvedCursorPosture::Default,
            Self::Pointer => WorthUiPrimitiveResolvedCursorPosture::Pointer,
            Self::Text => WorthUiPrimitiveResolvedCursorPosture::Text,
            Self::Grab => WorthUiPrimitiveResolvedCursorPosture::Grab,
            Self::Grabbing => WorthUiPrimitiveResolvedCursorPosture::Grabbing,
            Self::Resize => WorthUiPrimitiveResolvedCursorPosture::Resize,
        }
    }
}
