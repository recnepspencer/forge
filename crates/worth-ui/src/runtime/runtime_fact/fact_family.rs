#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiRuntimeFactFamily {
    ActiveArtifact,
    ExecutionPlan,
    ThemeToken,
    Command,
    CommandProjection,
    QueryBinding,
    LayoutTopology,
    ContentMount,
    Component,
    Appearance,
    InteractionPolicy,
}

impl WorthUiRuntimeFactFamily {
    pub const fn token(self) -> &'static str {
        match self {
            Self::ActiveArtifact => "active_artifact",
            Self::ExecutionPlan => "execution_plan",
            Self::ThemeToken => "theme_token",
            Self::Command => "command",
            Self::CommandProjection => "command_projection",
            Self::QueryBinding => "query_binding",
            Self::LayoutTopology => "layout_topology",
            Self::ContentMount => "content_mount",
            Self::Component => "component",
            Self::Appearance => "appearance",
            Self::InteractionPolicy => "interaction_policy",
        }
    }
}
