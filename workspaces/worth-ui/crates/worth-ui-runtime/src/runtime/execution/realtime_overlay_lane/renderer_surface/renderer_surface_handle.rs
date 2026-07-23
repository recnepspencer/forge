use crate::runtime::WorthUiRuntimeHandleLocator;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiRendererSurfaceHandle {
    locator: WorthUiRuntimeHandleLocator,
}

impl WorthUiRendererSurfaceHandle {
    pub(crate) fn new(locator: WorthUiRuntimeHandleLocator) -> Self {
        Self { locator }
    }

    pub fn plan_index(self) -> u32 {
        self.locator.plan_index()
    }

    pub fn locator(self) -> WorthUiRuntimeHandleLocator {
        self.locator
    }
}
