use worth_ui::facade::{
    declaration::{CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface},
};

fn main() {
    let _ = CommandProjectionDescriptor {
        id: CommandProjectionId::new("workspace.projection.palette").unwrap(),
        surface: CommandProjectionSurface::command_palette(),
        command_references: Vec::new(),
        eligible_categories: Vec::new(),
        groupings: Vec::new(),
        ordering: worth_ui::facade::declaration::CommandProjectionOrdering::Declaration,
        shortcut_visibility: worth_ui::facade::declaration::CommandProjectionShortcutVisibility::Hidden,
        readiness_display_policy: worth_ui::facade::declaration::CommandProjectionReadinessDisplayPolicy::HideReadiness,
        icon_label_policy: worth_ui::facade::declaration::CommandProjectionIconLabelPolicy::PreferCommandIconAndLabel,
        overflow_behavior: worth_ui::facade::declaration::CommandProjectionOverflowBehavior::NoOverflow,
        mosaic_scope: None,
        meaning_overrides: Vec::new(),
    };
}
