use worth_ui::facade::runtime::WorthUiRuntime;

fn attempt(host: WorthUiRuntime) {
    let _ = host.prepare_execution_plan_input("component Button {}");
}

fn main() {}
