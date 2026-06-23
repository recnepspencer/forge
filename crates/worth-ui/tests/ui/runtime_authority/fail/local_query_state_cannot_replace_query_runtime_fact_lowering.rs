use worth_ui::facade::WorthUiQueryRuntimeFactLoweringInput;

enum LocalQueryState {
    Loading,
}

fn requires_runtime_lowering_input(_input: WorthUiQueryRuntimeFactLoweringInput) {}

fn main() {
    requires_runtime_lowering_input(LocalQueryState::Loading);
}
