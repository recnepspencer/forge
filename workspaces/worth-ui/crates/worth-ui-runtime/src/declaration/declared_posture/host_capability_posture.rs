use worth_ui_host_contract::WorthUiHostCapability;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredHostCapabilityPosture {
    required_capabilities: Vec<WorthUiHostCapability>,
}

impl UiDeclaredHostCapabilityPosture {
    pub(crate) fn new(mut required_capabilities: Vec<WorthUiHostCapability>) -> Self {
        required_capabilities.sort_by_key(|capability| capability.as_str());
        required_capabilities.dedup();

        Self {
            required_capabilities,
        }
    }

    pub fn required_capabilities(&self) -> &[WorthUiHostCapability] {
        &self.required_capabilities
    }

    pub fn requires(&self, capability: WorthUiHostCapability) -> bool {
        self.required_capabilities.contains(&capability)
    }
}
