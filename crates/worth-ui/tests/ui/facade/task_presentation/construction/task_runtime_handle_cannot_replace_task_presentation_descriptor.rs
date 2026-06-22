use worth_ui::facade::WorthUi;

struct TaskRuntimeHandle;

fn main() {
    let handle = TaskRuntimeHandle;
    let _app = WorthUi::app().register_task_presentation(handle).freeze();
}
