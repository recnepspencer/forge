use worth_ui::facade::{WorthUiRuntime, WorthUiSourceProvider};

fn open_ingress_without_active_application(
    runtime: &WorthUiRuntime,
    provider: WorthUiSourceProvider,
) {
    let _ = runtime.source_ingress(provider);
}

fn main() {}
