use worth_ui::facade::WorthUiRuntime;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<WorthUiRuntime>();
}
