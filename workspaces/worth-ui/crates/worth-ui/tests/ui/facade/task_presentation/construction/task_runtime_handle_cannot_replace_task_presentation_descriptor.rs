use worth_ui::facade::app::WorthUi;

struct TaskRuntimeHandle;

fn main() {
    let handle = TaskRuntimeHandle;
    let _app = WorthUi::app().register_task_presentation(handle).freeze().expect("application preparation should succeed");
}
