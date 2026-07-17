use worth_ui::facade::query_binding::{
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity, WorthUiQueryViewLifecycle,
    WorthUiQueryViewShape,
};

fn main() {
    let _definition = WorthUiQueryViewDefinition {
        identity: WorthUiQueryViewIdentity::new("workspace.measurements").unwrap(),
        lifecycle: WorthUiQueryViewLifecycle::Snapshot,
        shape: WorthUiQueryViewShape::Collection,
        required_facts: Box::new([]),
    };
}
