use super::UiTextOriginalRange;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextVisualEdge {
    Leading,
    Trailing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiTextCaretAffinity {
    Upstream,
    Downstream,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiTextCaretPosition {
    original_boundary: UiTextOriginalRange,
    visual_edge: UiTextVisualEdge,
    affinity: UiTextCaretAffinity,
}

impl UiTextCaretPosition {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        original_boundary: UiTextOriginalRange,
        visual_edge: UiTextVisualEdge,
        affinity: UiTextCaretAffinity,
    ) -> Option<Self> {
        if original_boundary.is_empty() {
            Some(Self {
                original_boundary,
                visual_edge,
                affinity,
            })
        } else {
            None
        }
    }

    pub const fn original_boundary(self) -> UiTextOriginalRange {
        self.original_boundary
    }

    pub const fn visual_edge(self) -> UiTextVisualEdge {
        self.visual_edge
    }

    pub const fn affinity(self) -> UiTextCaretAffinity {
        self.affinity
    }
}
