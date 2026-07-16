use crate::capability::{ViewBindingDescriptor, ViewBindingFamily, ViewBindingId};

pub(super) fn standard_query_owned_view_binding_descriptor() -> ViewBindingDescriptor {
    let definition = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
        "workspace.view_binding.selection",
    )
    .expect("test Query definition should admit");
    ViewBindingDescriptor::from_definition(
        ViewBindingId::new("workspace.view_binding.selection").unwrap(),
        ViewBindingFamily::collection(),
        definition,
    )
}
