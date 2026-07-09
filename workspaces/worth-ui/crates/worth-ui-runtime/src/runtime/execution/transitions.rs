use crate::runtime::WorthUiAllocationPlanning;

/// Handle allocation entry proof: requires completed allocation planning.
#[derive(Debug, Clone)]
pub struct WorthUiExecutionLaneInput<'a>(pub(crate) &'a WorthUiAllocationPlanning);

impl<'a> WorthUiExecutionLaneInput<'a> {
    pub fn new(allocation_planning: &'a WorthUiAllocationPlanning) -> Self {
        Self(allocation_planning)
    }

    pub fn allocation_planning(&self) -> &WorthUiAllocationPlanning {
        self.0
    }
}
