#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeAsyncSourceDeclarationCounters {
    async_source_declaration_count: usize,
    request_response_family_count: usize,
    subscription_backed_family_count: usize,
    signal_resource_descriptor_lowering_count: usize,
    signal_async_node_capability_lowering_count: usize,
    async_source_declaration_rejection_count: usize,
}

impl BridgeAsyncSourceDeclarationCounters {
    pub(crate) fn request_response_validated() -> Self {
        Self {
            async_source_declaration_count: 1,
            request_response_family_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn subscription_backed_validated() -> Self {
        Self {
            async_source_declaration_count: 1,
            subscription_backed_family_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn request_response_lowered() -> Self {
        Self {
            async_source_declaration_count: 1,
            request_response_family_count: 1,
            signal_resource_descriptor_lowering_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn subscription_backed_lowered() -> Self {
        Self {
            async_source_declaration_count: 1,
            subscription_backed_family_count: 1,
            signal_async_node_capability_lowering_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn rejected() -> Self {
        Self {
            async_source_declaration_rejection_count: 1,
            ..Self::default()
        }
    }

    pub fn async_source_declaration_count(&self) -> usize {
        self.async_source_declaration_count
    }

    pub fn request_response_family_count(&self) -> usize {
        self.request_response_family_count
    }

    pub fn subscription_backed_family_count(&self) -> usize {
        self.subscription_backed_family_count
    }

    pub fn signal_resource_descriptor_lowering_count(&self) -> usize {
        self.signal_resource_descriptor_lowering_count
    }

    pub fn signal_async_node_capability_lowering_count(&self) -> usize {
        self.signal_async_node_capability_lowering_count
    }

    pub fn async_source_declaration_rejection_count(&self) -> usize {
        self.async_source_declaration_rejection_count
    }
}
