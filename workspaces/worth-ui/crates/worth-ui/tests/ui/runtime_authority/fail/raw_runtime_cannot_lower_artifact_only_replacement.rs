use worth_ui::facade::runtime::WorthUiRuntime;

fn lower_without_prepared_application(runtime: &WorthUiRuntime) {
    let _ = runtime.prepare_replacement_lowering((), ());
}

fn main() {}
