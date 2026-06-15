use worth_ui::facade::{
    CommandCategory, CommandProjectionDescriptor, CommandProjectionGrouping, CommandProjectionId,
    CommandProjectionOrdering, CommandProjectionOverflowBehavior, CommandProjectionSurface,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessCommandProjectionVisualRole {
    CommandPalette,
    MenuBar,
    ContextMenu,
    Toolbar,
    StatusAction,
}

impl HarnessCommandProjectionVisualRole {
    pub const REQUIRED: [Self; 5] = [
        Self::CommandPalette,
        Self::MenuBar,
        Self::ContextMenu,
        Self::Toolbar,
        Self::StatusAction,
    ];

    pub fn projection_id_text(self) -> &'static str {
        match self {
            Self::CommandPalette => "harness.command_projection.command_palette",
            Self::MenuBar => "harness.command_projection.menu_bar",
            Self::ContextMenu => "harness.command_projection.context_menu",
            Self::Toolbar => "harness.command_projection.toolbar",
            Self::StatusAction => "harness.command_projection.status_action",
        }
    }
}

pub(crate) fn harness_command_visual_projections() -> Vec<CommandProjectionDescriptor> {
    HarnessCommandProjectionVisualRole::REQUIRED
        .into_iter()
        .map(command_projection)
        .collect()
}

fn command_projection(role: HarnessCommandProjectionVisualRole) -> CommandProjectionDescriptor {
    let projection = CommandProjectionDescriptor::new(projection_id(role), surface(role))
        .with_grouping(CommandProjectionGrouping::optional("harness.workbench"))
        .with_ordering(CommandProjectionOrdering::ByCategoryThenCommandId)
        .prefer_command_icon_and_label();

    match role {
        HarnessCommandProjectionVisualRole::CommandPalette => projection
            .with_eligible_category(CommandCategory::Workspace)
            .with_eligible_category(CommandCategory::File)
            .with_eligible_category(CommandCategory::Edit)
            .with_eligible_category(CommandCategory::View)
            .with_eligible_category(CommandCategory::Navigate)
            .with_eligible_category(CommandCategory::Tools)
            .show_shortcuts()
            .show_readiness()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::scroll_within_surface()),
        HarnessCommandProjectionVisualRole::MenuBar => projection
            .with_eligible_category(CommandCategory::Application)
            .with_eligible_category(CommandCategory::File)
            .with_eligible_category(CommandCategory::Edit)
            .with_eligible_category(CommandCategory::View)
            .with_eligible_category(CommandCategory::Help)
            .show_shortcuts()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::collapse_to_more()),
        HarnessCommandProjectionVisualRole::ContextMenu => projection
            .with_eligible_category(CommandCategory::Workspace)
            .with_eligible_category(CommandCategory::Edit)
            .with_eligible_category(CommandCategory::Tools)
            .show_readiness()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::collapse_to_more()),
        HarnessCommandProjectionVisualRole::Toolbar => projection
            .with_eligible_category(CommandCategory::Workspace)
            .with_eligible_category(CommandCategory::View)
            .show_shortcuts()
            .disable_unavailable_commands()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::collapse_to_more()),
        HarnessCommandProjectionVisualRole::StatusAction => projection
            .with_eligible_category(CommandCategory::Workspace)
            .with_eligible_category(CommandCategory::Tools)
            .disable_unavailable_commands()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::no_overflow()),
    }
}

fn projection_id(role: HarnessCommandProjectionVisualRole) -> CommandProjectionId {
    CommandProjectionId::new(role.projection_id_text())
        .expect("valid harness command projection id")
}

fn surface(role: HarnessCommandProjectionVisualRole) -> CommandProjectionSurface {
    match role {
        HarnessCommandProjectionVisualRole::CommandPalette => {
            CommandProjectionSurface::command_palette()
        }
        HarnessCommandProjectionVisualRole::MenuBar => CommandProjectionSurface::menu_bar(),
        HarnessCommandProjectionVisualRole::ContextMenu => CommandProjectionSurface::context_menu(),
        HarnessCommandProjectionVisualRole::Toolbar => CommandProjectionSurface::toolbar(),
        HarnessCommandProjectionVisualRole::StatusAction => {
            CommandProjectionSurface::status_action()
        }
    }
}
