use worth_ui::facade::{IconDescriptor, IconFamily, IconId, IconSourceDescriptor};

pub(crate) fn harness_icon_descriptors() -> Vec<IconDescriptor> {
    vec![
        command_icon("harness.icon.command.palette"),
        command_icon("harness.icon.command.run"),
        surface_icon("harness.icon.surface.sidebar"),
        surface_icon("harness.icon.surface.panel"),
        surface_icon("harness.icon.surface.overlay"),
        runtime_icon("harness.icon.runtime.success"),
        runtime_icon("harness.icon.runtime.warning"),
        runtime_icon("harness.icon.runtime.danger"),
        runtime_icon("harness.icon.runtime.disabled"),
        runtime_icon("harness.icon.runtime.active"),
    ]
}

fn command_icon(id: &str) -> IconDescriptor {
    IconDescriptor::new(
        icon_id(id),
        IconFamily::command(),
        IconSourceDescriptor::symbol(id),
    )
}

fn surface_icon(id: &str) -> IconDescriptor {
    IconDescriptor::new(
        icon_id(id),
        IconFamily::surface(),
        IconSourceDescriptor::symbol(id),
    )
}

fn runtime_icon(id: &str) -> IconDescriptor {
    IconDescriptor::new(
        icon_id(id),
        IconFamily::runtime_outcome(),
        IconSourceDescriptor::symbol(id),
    )
}

fn icon_id(raw_text: &str) -> IconId {
    IconId::new(raw_text).expect("valid harness icon id")
}
