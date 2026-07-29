use worth_ui::facade::app::WorthUi;

struct TaskRuntimeHandle;

fn main() {
    let handle = TaskRuntimeHandle;
    let _app = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse()).register_task_presentation(handle).freeze().expect("application preparation should succeed");
}
