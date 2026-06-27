use std::time::Duration;

use worth_ui::facade::{
    WorthUiReloadDebounce, WorthUiSourceIngressHook, WorthUiSourceProvider,
    WorthUiWatcherEvent,
};

fn main() {
    let provider = WorthUiSourceProvider::in_memory("editor-buffer").with_file("app/main.wui", "");
    let hook = WorthUiSourceIngressHook::generated_source(
        "generated-tokens",
        WorthUiSourceProvider::generated("generated-tokens").with_file("app/tokens.wui", ""),
    );
    let mut session = worth_ui::facade::WorthUiSourceWatcher::new(provider)
        .with_debounce(WorthUiReloadDebounce::stable_window(Duration::from_millis(35)))
        .with_hook(hook)
        .start();
    let batch = session
        .ingest([WorthUiWatcherEvent::provider_revision("editor-buffer")])
        .expect("facade types compile");

    let _revision = batch.source_revision();
    let _receipt = batch.ordering_receipt();
    let _counters = batch.counters();
}
