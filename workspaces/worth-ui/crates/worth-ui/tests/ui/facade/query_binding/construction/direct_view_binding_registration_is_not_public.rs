use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::ViewBindingDescriptor;

fn bypass(descriptor: ViewBindingDescriptor) {
    let _ = WorthUi::app().register_view_binding(descriptor);
}

fn main() {}
