use super::admission::RefreshFallback;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatchWidthResolution {
    Deliver,
    Coalesce,
    Refresh(RefreshFallback),
    Reject,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatchWidthAssessment {
    pub(in crate::live) measured_width: usize,
    pub(in crate::live) budget_limit: usize,
    pub(in crate::live) resolution: PatchWidthResolution,
}

impl PatchWidthAssessment {
    pub fn measured_width(&self) -> usize {
        self.measured_width
    }

    pub fn budget_limit(&self) -> usize {
        self.budget_limit
    }

    pub fn resolution(&self) -> &PatchWidthResolution {
        &self.resolution
    }
}
