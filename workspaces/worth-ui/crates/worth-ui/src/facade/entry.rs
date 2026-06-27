use super::WorthUiBuilder;

/// Public entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> WorthUiBuilder {
        WorthUiBuilder::new()
    }
}
