use super::{UiTextCaretPosition, UiTextOriginalRange, UiTextRect, UiTextVisualEdge};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextLineRecord {
    original_range: UiTextOriginalRange,
    visual_run_start: u32,
    visual_run_end: u32,
    logical_bounds: UiTextRect,
    ink_bounds: UiTextRect,
    baseline_millipoints: i64,
    hard_break: bool,
    overflowed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextVisualRunRecord {
    original_range: UiTextOriginalRange,
    line_index: u32,
    logical_run_start: u32,
    logical_run_end: u32,
    bidi_level: u8,
    bounds: UiTextRect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextCaretRecord {
    position: UiTextCaretPosition,
    line_index: u32,
    visual_run_index: u32,
    x_millipoints: i64,
    top_millipoints: i64,
    bottom_millipoints: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiTextHitResult {
    caret: UiQualifiedTextCaretRecord,
    cluster_range: UiTextOriginalRange,
    visual_edge: UiTextVisualEdge,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQualifiedTextSelectionRect {
    selected_range: UiTextOriginalRange,
    line_index: u32,
    visual_run_index: u32,
    bounds: UiTextRect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPositionedTextGlyphRecord {
    source_glyph_index: u32,
    line_index: u32,
    visual_run_index: u32,
    origin_x_millipoints: i64,
    origin_y_millipoints: i64,
    advance_x_millipoints: i64,
    ink_bounds: UiTextRect,
}

#[doc(hidden)]
pub struct UiQualifiedTextLineInput {
    pub original_range: UiTextOriginalRange,
    pub visual_run_start: u32,
    pub visual_run_end: u32,
    pub logical_bounds: UiTextRect,
    pub ink_bounds: UiTextRect,
    pub baseline_millipoints: i64,
    pub hard_break: bool,
    pub overflowed: bool,
}

#[doc(hidden)]
pub struct UiPositionedTextGlyphInput {
    pub source_glyph_index: u32,
    pub line_index: u32,
    pub visual_run_index: u32,
    pub origin_x_millipoints: i64,
    pub origin_y_millipoints: i64,
    pub advance_x_millipoints: i64,
    pub ink_bounds: UiTextRect,
}

#[doc(hidden)]
pub struct UiQualifiedTextVisualRunInput {
    pub original_range: UiTextOriginalRange,
    pub line_index: u32,
    pub logical_run_start: u32,
    pub logical_run_end: u32,
    pub bidi_level: u8,
    pub bounds: UiTextRect,
}

impl UiQualifiedTextLineRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiQualifiedTextLineInput) -> Self {
        Self {
            original_range: input.original_range,
            visual_run_start: input.visual_run_start,
            visual_run_end: input.visual_run_end,
            logical_bounds: input.logical_bounds,
            ink_bounds: input.ink_bounds,
            baseline_millipoints: input.baseline_millipoints,
            hard_break: input.hard_break,
            overflowed: input.overflowed,
        }
    }
    pub const fn original_range(self) -> UiTextOriginalRange {
        self.original_range
    }
    pub const fn visual_run_range(self) -> core::ops::Range<u32> {
        self.visual_run_start..self.visual_run_end
    }
    pub const fn bounds(self) -> UiTextRect {
        self.logical_bounds
    }
    pub const fn logical_bounds(self) -> UiTextRect {
        self.logical_bounds
    }
    pub const fn ink_bounds(self) -> UiTextRect {
        self.ink_bounds
    }
    pub const fn baseline_millipoints(self) -> i64 {
        self.baseline_millipoints
    }
    pub const fn hard_break(self) -> bool {
        self.hard_break
    }
    pub const fn overflowed(self) -> bool {
        self.overflowed
    }
}

impl UiQualifiedTextVisualRunRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiQualifiedTextVisualRunInput) -> Self {
        Self {
            original_range: input.original_range,
            line_index: input.line_index,
            logical_run_start: input.logical_run_start,
            logical_run_end: input.logical_run_end,
            bidi_level: input.bidi_level,
            bounds: input.bounds,
        }
    }
    pub const fn original_range(self) -> UiTextOriginalRange {
        self.original_range
    }
    pub const fn line_index(self) -> u32 {
        self.line_index
    }
    pub const fn logical_run_range(self) -> core::ops::Range<u32> {
        self.logical_run_start..self.logical_run_end
    }
    pub const fn bidi_level(self) -> u8 {
        self.bidi_level
    }
    pub const fn bounds(self) -> UiTextRect {
        self.bounds
    }
}

impl UiQualifiedTextCaretRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        position: UiTextCaretPosition,
        line_index: u32,
        visual_run_index: u32,
        x_millipoints: i64,
        top_millipoints: i64,
        bottom_millipoints: i64,
    ) -> Self {
        Self {
            position,
            line_index,
            visual_run_index,
            x_millipoints,
            top_millipoints,
            bottom_millipoints,
        }
    }
    pub const fn position(self) -> UiTextCaretPosition {
        self.position
    }
    pub const fn line_index(self) -> u32 {
        self.line_index
    }
    pub const fn visual_run_index(self) -> u32 {
        self.visual_run_index
    }
    pub const fn x_millipoints(self) -> i64 {
        self.x_millipoints
    }
    pub const fn top_millipoints(self) -> i64 {
        self.top_millipoints
    }
    pub const fn bottom_millipoints(self) -> i64 {
        self.bottom_millipoints
    }
}

impl UiTextHitResult {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        caret: UiQualifiedTextCaretRecord,
        cluster_range: UiTextOriginalRange,
        visual_edge: UiTextVisualEdge,
    ) -> Self {
        Self {
            caret,
            cluster_range,
            visual_edge,
        }
    }
    pub const fn caret(self) -> UiQualifiedTextCaretRecord {
        self.caret
    }
    pub const fn cluster_range(self) -> UiTextOriginalRange {
        self.cluster_range
    }
    pub const fn visual_edge(self) -> UiTextVisualEdge {
        self.visual_edge
    }
}

impl UiQualifiedTextSelectionRect {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        selected_range: UiTextOriginalRange,
        line_index: u32,
        visual_run_index: u32,
        bounds: UiTextRect,
    ) -> Self {
        Self {
            selected_range,
            line_index,
            visual_run_index,
            bounds,
        }
    }
    pub const fn selected_range(self) -> UiTextOriginalRange {
        self.selected_range
    }
    pub const fn line_index(self) -> u32 {
        self.line_index
    }
    pub const fn visual_run_index(self) -> u32 {
        self.visual_run_index
    }
    pub const fn bounds(self) -> UiTextRect {
        self.bounds
    }
}

impl UiPositionedTextGlyphRecord {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiPositionedTextGlyphInput) -> Self {
        Self {
            source_glyph_index: input.source_glyph_index,
            line_index: input.line_index,
            visual_run_index: input.visual_run_index,
            origin_x_millipoints: input.origin_x_millipoints,
            origin_y_millipoints: input.origin_y_millipoints,
            advance_x_millipoints: input.advance_x_millipoints,
            ink_bounds: input.ink_bounds,
        }
    }
    pub const fn source_glyph_index(self) -> u32 {
        self.source_glyph_index
    }
    pub const fn line_index(self) -> u32 {
        self.line_index
    }
    pub const fn visual_run_index(self) -> u32 {
        self.visual_run_index
    }
    pub const fn origin_x_millipoints(self) -> i64 {
        self.origin_x_millipoints
    }
    pub const fn origin_y_millipoints(self) -> i64 {
        self.origin_y_millipoints
    }
    pub const fn advance_x_millipoints(self) -> i64 {
        self.advance_x_millipoints
    }
    pub const fn ink_bounds(self) -> UiTextRect {
        self.ink_bounds
    }
}
