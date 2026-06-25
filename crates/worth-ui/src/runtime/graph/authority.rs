use crate::runtime::{WorthUiGraphInvalidationReceipt, WorthUiGraphInvalidationRequest};

#[derive(Clone, Copy, Debug, Default)]
pub struct WorthUiRuntimeGraphAuthority;

impl WorthUiRuntimeGraphAuthority {
    pub fn new() -> Self {
        Self
    }

    pub fn plan_invalidation(
        &self,
        request: WorthUiGraphInvalidationRequest,
    ) -> WorthUiGraphInvalidationReceipt {
        WorthUiGraphInvalidationReceipt::plan(request)
    }
}
