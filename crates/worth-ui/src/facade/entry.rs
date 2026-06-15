use super::{WorthUiAppBuilder, WorthUiRuntimeLaunchBuilder};

/// Public entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> WorthUiAppBuilder {
        WorthUiAppBuilder::new()
    }

    /// Start preparing a runtime launch through public Worth UI source contracts.
    pub fn runtime_launch() -> WorthUiRuntimeLaunchBuilder {
        WorthUiRuntimeLaunchBuilder::default()
    }
}
