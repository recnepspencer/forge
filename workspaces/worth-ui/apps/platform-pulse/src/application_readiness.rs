use std::sync::Arc;

/// Product-internal wake delivery shared by the binary and its library-owned
/// input worker. It carries no semantic or presentation authority.
#[doc(hidden)]
#[derive(Clone)]
pub struct PlatformPulseApplicationReadinessSignal {
    signal: Arc<dyn Fn() + Send + Sync>,
}

impl PlatformPulseApplicationReadinessSignal {
    #[doc(hidden)]
    pub fn from_callback(signal: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            signal: Arc::new(signal),
        }
    }

    #[doc(hidden)]
    pub fn from_native(port: worth_ui_native_platform::UiNativeApplicationReadinessPort) -> Self {
        Self::from_callback(move || {
            let _ = port.signal();
        })
    }

    #[doc(hidden)]
    pub fn signal(&self) {
        (self.signal)();
    }
}
