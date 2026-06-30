#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStateSlotKind {
    SplitterPosition,
    ActiveStackItem,
    RegionVisibility,
    CollapsedPosture,
    PinnedPosture,
    ScrollPosition,
    FocusedRegion,
    ActivePrimarySurface,
    ActiveAuxiliarySurface,
    SelectionToken,
    DraftInputState,
}

impl MosaicStateSlotKind {
    pub fn splitter_position() -> Self {
        Self::SplitterPosition
    }

    pub fn active_stack_item() -> Self {
        Self::ActiveStackItem
    }

    pub fn region_visibility() -> Self {
        Self::RegionVisibility
    }

    pub fn collapsed_posture() -> Self {
        Self::CollapsedPosture
    }

    pub fn pinned_posture() -> Self {
        Self::PinnedPosture
    }

    pub fn scroll_position() -> Self {
        Self::ScrollPosition
    }

    pub fn focused_region() -> Self {
        Self::FocusedRegion
    }

    pub fn active_primary_surface() -> Self {
        Self::ActivePrimarySurface
    }

    pub fn active_auxiliary_surface() -> Self {
        Self::ActiveAuxiliarySurface
    }

    pub fn selection_token() -> Self {
        Self::SelectionToken
    }

    pub fn draft_input_state() -> Self {
        Self::DraftInputState
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::SplitterPosition => "splitter_position",
            Self::ActiveStackItem => "active_stack_item",
            Self::RegionVisibility => "region_visibility",
            Self::CollapsedPosture => "collapsed_posture",
            Self::PinnedPosture => "pinned_posture",
            Self::ScrollPosition => "scroll_position",
            Self::FocusedRegion => "focused_region",
            Self::ActivePrimarySurface => "active_primary_surface",
            Self::ActiveAuxiliarySurface => "active_auxiliary_surface",
            Self::SelectionToken => "selection_token",
            Self::DraftInputState => "draft_input_state",
        }
    }
}
