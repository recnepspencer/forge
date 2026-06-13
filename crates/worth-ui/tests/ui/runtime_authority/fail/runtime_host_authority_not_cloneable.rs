use worth_ui::facade::WorthUiRuntimeHost;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<WorthUiRuntimeHost>();
}
