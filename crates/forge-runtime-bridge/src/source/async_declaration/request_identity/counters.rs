#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeAsyncRequestIdentityCounters {
    async_request_identity_count: usize,
    request_response_request_count: usize,
    subscription_backed_request_count: usize,
    signal_resource_request_admission_count: usize,
    signal_async_request_admission_count: usize,
    async_request_identity_rejection_count: usize,
}

impl BridgeAsyncRequestIdentityCounters {
    pub(crate) fn request_response_admitted() -> Self {
        Self {
            async_request_identity_count: 1,
            request_response_request_count: 1,
            signal_resource_request_admission_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn subscription_backed_admitted() -> Self {
        Self {
            async_request_identity_count: 1,
            subscription_backed_request_count: 1,
            signal_async_request_admission_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn rejected() -> Self {
        Self {
            async_request_identity_rejection_count: 1,
            ..Self::default()
        }
    }

    pub fn async_request_identity_count(&self) -> usize {
        self.async_request_identity_count
    }

    pub fn request_response_request_count(&self) -> usize {
        self.request_response_request_count
    }

    pub fn subscription_backed_request_count(&self) -> usize {
        self.subscription_backed_request_count
    }

    pub fn signal_resource_request_admission_count(&self) -> usize {
        self.signal_resource_request_admission_count
    }

    pub fn signal_async_request_admission_count(&self) -> usize {
        self.signal_async_request_admission_count
    }

    pub fn async_request_identity_rejection_count(&self) -> usize {
        self.async_request_identity_rejection_count
    }
}
