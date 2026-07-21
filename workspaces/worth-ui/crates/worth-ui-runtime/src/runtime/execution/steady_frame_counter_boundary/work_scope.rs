/// Breadth admitted before lane execution and breadth independently observed
/// from the resulting execution receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFrameWorkScope {
    requested: u64,
    executed: u64,
}

impl WorthUiFrameWorkScope {
    pub(crate) fn new(requested: u64, executed: u64) -> Self {
        Self {
            requested,
            executed,
        }
    }

    pub fn requested(self) -> u64 {
        self.requested
    }

    pub fn executed(self) -> u64 {
        self.executed
    }

    pub fn is_within_request(self) -> bool {
        self.executed <= self.requested
    }
}
