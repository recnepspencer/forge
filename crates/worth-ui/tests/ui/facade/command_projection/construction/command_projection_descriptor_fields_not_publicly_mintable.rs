use worth_ui::facade::{
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
};

fn main() {
    let _ = CommandProjectionDescriptor {
        id: CommandProjectionId::new("workspace.projection.palette").unwrap(),
        surface: CommandProjectionSurface::command_palette(),
        command_references: Vec::new(),
        eligible_categories: Vec::new(),
        groupings: Vec::new(),
        ordering: worth_ui::facade::CommandProjectionOrdering::Declaration,
        shortcut_visibility: worth_ui::facade::CommandProjectionShortcutVisibility::Hidden,
        readiness_display_policy: worth_ui::facade::CommandProjectionReadinessDisplayPolicy::HideReadiness,
        icon_label_policy: worth_ui::facade::CommandProjectionIconLabelPolicy::PreferCommandIconAndLabel,
        overflow_behavior: worth_ui::facade::CommandProjectionOverflowBehavior::NoOverflow,
        mosaic_scope: None,
        selection_mode: worth_ui::facade::CommandProjectionSelectionMode::SingleSelect,
        meaning_overrides: Vec::new(),
    };
}
