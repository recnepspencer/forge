use crate::planning::SelectedAccessPlanBasis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AccessLoweringRequest {
    selected: SelectedAccessPlanBasis,
}

impl AccessLoweringRequest {
    pub(crate) const fn new(selected: SelectedAccessPlanBasis) -> Self {
        Self { selected }
    }

    pub(crate) const fn selected(self) -> SelectedAccessPlanBasis {
        self.selected
    }
}
