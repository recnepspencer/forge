use crate::access::planning::S8SelectedAccessPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8AccessLoweringRequest {
    selected: S8SelectedAccessPlan,
}

impl S8AccessLoweringRequest {
    pub(crate) const fn new(selected: S8SelectedAccessPlan) -> Self {
        Self { selected }
    }

    pub(crate) const fn selected(self) -> S8SelectedAccessPlan {
        self.selected
    }
}
