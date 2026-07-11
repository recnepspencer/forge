use super::S8SelectedAccessPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessLoweringRequest {
    selected: S8SelectedAccessPlan,
}

impl S8AccessLoweringRequest {
    pub(crate) const fn new(selected: S8SelectedAccessPlan) -> Self {
        Self { selected }
    }

    pub const fn selected(self) -> S8SelectedAccessPlan {
        self.selected
    }
}
