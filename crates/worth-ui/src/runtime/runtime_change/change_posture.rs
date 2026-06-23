#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeChangeActivationPosture {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied,
    Mixed(WorthUiRuntimeChangeMixedPosture),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeChangeMixedPosture {
    equivalent_family_count: usize,
    ready_family_count: usize,
    activated_family_count: usize,
    denied_family_count: usize,
}

impl WorthUiRuntimeChangeMixedPosture {
    pub(crate) fn new(
        equivalent_family_count: usize,
        ready_family_count: usize,
        activated_family_count: usize,
        denied_family_count: usize,
    ) -> Self {
        Self {
            equivalent_family_count,
            ready_family_count,
            activated_family_count,
            denied_family_count,
        }
    }

    pub fn activated_family_count(self) -> usize {
        self.activated_family_count
    }

    pub fn denied_family_count(self) -> usize {
        self.denied_family_count
    }
}
