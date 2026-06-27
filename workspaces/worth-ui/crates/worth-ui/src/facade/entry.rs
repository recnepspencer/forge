/// Public entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> worth_ui_runtime::facade::WorthUiBuilder {
        worth_ui_runtime::facade::WorthUi::app()
    }
}
