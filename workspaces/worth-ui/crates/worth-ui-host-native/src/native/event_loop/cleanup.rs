use std::cell::RefCell;
use std::rc::Rc;

use super::super::{UiNativeHostState, UiNativeResourceCensus};
use super::UiNativeEventLoopClientCleanup;

#[must_use]
pub struct UiNativeEventLoopCleanup {
    state: Rc<RefCell<UiNativeHostState>>,
    client: Option<Box<dyn UiNativeEventLoopClientCleanup>>,
}

impl UiNativeEventLoopCleanup {
    pub(super) fn retain(
        state: Rc<RefCell<UiNativeHostState>>,
        census: UiNativeResourceCensus,
        client: Option<Box<dyn UiNativeEventLoopClientCleanup>>,
    ) -> Option<Self> {
        (!census.is_zero() || client.is_some()).then_some(Self { state, client })
    }

    pub fn retry(mut self) -> Result<UiNativeResourceCensus, Self> {
        self.client = self
            .client
            .take()
            .and_then(|cleanup| cleanup.retry().into_cleanup());
        let census = self.state.borrow_mut().close();
        if census.is_zero() && self.client.is_none() {
            Ok(census)
        } else {
            Err(self)
        }
    }
}

impl std::fmt::Debug for UiNativeEventLoopCleanup {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("UiNativeEventLoopCleanup(..)")
    }
}
