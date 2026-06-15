#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SurfaceAtlasFamily {
    ActivityNavigation,
    ScenarioList,
    CommandProjectionSurface,
    TabbedEditor,
    PinnedSidebar,
    StackedScrollPane,
    EvidenceInspector,
    BottomTimeline,
    OverlayPreview,
    StatusSurface,
    ThemeControls,
    DensityControls,
    WorkbenchCanvas,
}

impl SurfaceAtlasFamily {
    pub const REQUIRED: [Self; 13] = [
        Self::ActivityNavigation,
        Self::ScenarioList,
        Self::CommandProjectionSurface,
        Self::TabbedEditor,
        Self::PinnedSidebar,
        Self::StackedScrollPane,
        Self::EvidenceInspector,
        Self::BottomTimeline,
        Self::OverlayPreview,
        Self::StatusSurface,
        Self::ThemeControls,
        Self::DensityControls,
        Self::WorkbenchCanvas,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::ActivityNavigation => "Activity/navigation",
            Self::ScenarioList => "Scenario list",
            Self::CommandProjectionSurface => "Command projections",
            Self::TabbedEditor => "Tabbed editor",
            Self::PinnedSidebar => "Pinned sidebar",
            Self::StackedScrollPane => "Stacked scroll panes",
            Self::EvidenceInspector => "Evidence inspector",
            Self::BottomTimeline => "Run timeline",
            Self::OverlayPreview => "Overlay preview",
            Self::StatusSurface => "Status surface",
            Self::ThemeControls => "Theme controls",
            Self::DensityControls => "Density controls",
            Self::WorkbenchCanvas => "Workbench canvas",
        }
    }
}
