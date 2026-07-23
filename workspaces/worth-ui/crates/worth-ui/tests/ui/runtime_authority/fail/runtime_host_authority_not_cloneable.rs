use worth_ui::facade::runtime::WorthUiRuntime;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<WorthUiRuntime>();
}
