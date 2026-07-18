use crate::runtime::source_ingress::debounce::{
    WorthUiDebouncedWatcherBatch, WorthUiReloadDebounce,
};
use crate::runtime::source_ingress::denial::{
    WorthUiSourceIngressDenial, WorthUiSourceIngressDenialReason,
};
use crate::runtime::source_ingress::event::WorthUiWatcherEvent;
use crate::runtime::source_ingress::provider::WorthUiSourceProvider;
use crate::runtime::source_ingress::source_ingress_hook::WorthUiSourceIngressHook;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceWatcher {
    provider: WorthUiSourceProvider,
    debounce: WorthUiReloadDebounce,
    hooks: Vec<WorthUiSourceIngressHook>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceIngressSession {
    provider: WorthUiSourceProvider,
    debounce: WorthUiReloadDebounce,
    hooks: Vec<WorthUiSourceIngressHook>,
    next_sequence: u64,
}

impl WorthUiSourceWatcher {
    pub fn new(provider: WorthUiSourceProvider) -> Self {
        Self {
            provider,
            debounce: WorthUiReloadDebounce::default(),
            hooks: Vec::new(),
        }
    }

    pub fn with_debounce(mut self, debounce: WorthUiReloadDebounce) -> Self {
        self.debounce = debounce;
        self
    }

    pub fn with_hook(mut self, hook: WorthUiSourceIngressHook) -> Self {
        self.hooks.push(hook);
        self
    }

    pub fn start(self) -> WorthUiSourceIngressSession {
        WorthUiSourceIngressSession {
            provider: self.provider,
            debounce: self.debounce,
            hooks: self.hooks,
            next_sequence: 1,
        }
    }
}

impl WorthUiSourceIngressSession {
    pub fn ingest(
        &mut self,
        events: impl IntoIterator<Item = WorthUiWatcherEvent>,
    ) -> Result<WorthUiDebouncedWatcherBatch, WorthUiSourceIngressDenial> {
        let events = events.into_iter().collect::<Vec<_>>();
        let provider = self.provider_with_hooks()?;
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.debounce.debounce(provider, &events, sequence)
    }

    fn provider_with_hooks(&self) -> Result<WorthUiSourceProvider, WorthUiSourceIngressDenial> {
        let mut provider = self.provider.clone();
        for hook in &self.hooks {
            if hook.provider().is_empty() {
                return Err(WorthUiSourceIngressDenial::new(
                    WorthUiSourceIngressDenialReason::UnsupportedHookOutput,
                ));
            }
            for module in hook.provider().source_modules() {
                provider = provider.with_file(module.relative_path(), module.source_text());
            }
            for input in hook.provider().rust_authored_inputs() {
                provider = provider.with_rust_authored_input(input.clone());
            }
        }
        Ok(provider)
    }
}
