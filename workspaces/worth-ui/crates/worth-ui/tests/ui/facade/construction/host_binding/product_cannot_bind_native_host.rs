use worth_ui::facade::app::WorthUi;

fn main() {
    let builder = WorthUi::app();
    let _ = builder.bind_native_platform_host((), ());
}
