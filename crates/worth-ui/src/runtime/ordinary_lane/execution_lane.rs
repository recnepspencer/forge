#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiOrdinaryExecutionLane {
    WidgetShell,
    ShellRegion,
    ChildRangeTraversal,
    CommandSurface,
    TokenStyleSupport,
    EguiBoundarySupport,
}

impl WorthUiOrdinaryExecutionLane {
    pub(crate) fn canonical_tag(self) -> u64 {
        match self {
            Self::WidgetShell => 1,
            Self::ShellRegion => 2,
            Self::ChildRangeTraversal => 3,
            Self::CommandSurface => 4,
            Self::TokenStyleSupport => 5,
            Self::EguiBoundarySupport => 6,
        }
    }
}
