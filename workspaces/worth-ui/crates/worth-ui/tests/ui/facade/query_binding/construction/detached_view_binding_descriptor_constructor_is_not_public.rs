use worth_ui::facade::query_binding::WorthUiQueryViewDefinition;
use worth_ui::facade::registry::{ViewBindingDescriptor, ViewBindingFamily, ViewBindingId};

fn main() {
    let definition = WorthUiQueryViewDefinition::measurement_snapshot("workspace.measurements")
        .expect("semantic definition should admit");
    let _descriptor = ViewBindingDescriptor::from_definition(
        ViewBindingId::new("workspace.measurements").unwrap(),
        ViewBindingFamily::collection(),
        definition,
    );
}
