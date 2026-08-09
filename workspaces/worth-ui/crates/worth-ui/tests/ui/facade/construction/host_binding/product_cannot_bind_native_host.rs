use worth_ui::facade::app::WorthUiHostNeutralApp;

fn bind_from_product_code(application: WorthUiHostNeutralApp) {
    let _ = application.bind_qualified_native(());
}

fn main() {}
