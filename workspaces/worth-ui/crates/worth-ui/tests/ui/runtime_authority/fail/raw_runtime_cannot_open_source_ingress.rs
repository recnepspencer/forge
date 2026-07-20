use worth_ui::facade::runtime::WorthUiRuntime;
use worth_ui::facade::source::WorthUiSourceProvider;

fn open_ingress_without_active_application(
    runtime: &WorthUiRuntime,
    provider: WorthUiSourceProvider,
) {
    let _ = runtime.source_event_ingress(provider);
}

fn main() {}
