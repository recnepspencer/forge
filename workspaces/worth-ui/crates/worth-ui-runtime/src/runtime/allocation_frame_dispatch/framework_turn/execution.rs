use crate::runtime::launch::runtime_instance::WorthUiRuntime;

/// Borrowed proof that the ordinary framework turn closed and pumped without
/// producing a sealed source frame that must first be consumed by Phase 5.
#[derive(Debug)]
pub struct WorthUiFrameworkTurnExecution<'runtime> {
    pub(crate) _runtime: &'runtime WorthUiRuntime,
    pub(super) boundary: crate::runtime::WorthUiFrameBoundary,
}

impl WorthUiFrameworkTurnExecution<'_> {
    pub fn activation_boundary(&self) -> &crate::runtime::WorthUiFrameBoundary {
        &self.boundary
    }

    pub fn into_activation_boundary(self) -> crate::runtime::WorthUiFrameBoundary {
        self.boundary
    }
}
