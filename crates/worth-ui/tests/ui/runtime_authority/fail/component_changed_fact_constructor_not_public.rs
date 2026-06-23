use worth_ui::facade::{
    ComponentId, WorthUiCapabilityChangedFacts, WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
};

fn main() {
    let component_id = ComponentId::new("validation.component.header.dropdown").unwrap();
    let _ = WorthUiCapabilityChangedFacts::from_admitted_capability_reload(
        WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::component(&component_id)),
        10,
        11,
    );
}
