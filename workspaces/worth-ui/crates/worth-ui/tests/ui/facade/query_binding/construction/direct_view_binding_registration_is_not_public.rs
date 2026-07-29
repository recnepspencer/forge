use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::ViewBindingDescriptor;

fn bypass(descriptor: ViewBindingDescriptor) {
    let _ = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse()).register_view_binding(descriptor);
}

fn main() {}
