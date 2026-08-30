use worth_ui::facade::interaction::UiCommandRouteReceipt;

fn requires_clone<T: Clone>() {}

fn main() {
    requires_clone::<UiCommandRouteReceipt>();
}
