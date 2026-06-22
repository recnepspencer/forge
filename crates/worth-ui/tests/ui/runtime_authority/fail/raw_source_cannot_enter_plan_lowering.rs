use worth_ui::facade::WorthUiRuntimeHost;

fn attempt(host: WorthUiRuntimeHost) {
    let _ = host.prepare_execution_plan_input("component Button {}");
}

fn main() {}
