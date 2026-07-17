use worth_ui::facade::app::WorthUi;
use worth_ui::facade::query_binding::WorthUiInstalledQueryView;

fn register(view: WorthUiInstalledQueryView) {
    let _app = WorthUi::app()
        .register_query_view(view)
        .expect("installed view registration")
        .freeze();
}

fn main() {}
