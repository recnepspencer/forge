use worth_ui::facade::{ViewBindingId, WorthUiQueryRuntimeFactLoweringInput};

fn requires_runtime_lowering_input(_input: WorthUiQueryRuntimeFactLoweringInput) {}

fn main() {
    let id = ViewBindingId::new("validation.query.products").unwrap();
    requires_runtime_lowering_input(id);
}
