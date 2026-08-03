#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualQueryBudget {
    maximum_results: u16,
    maximum_candidates: u16,
}

impl UiVisualQueryBudget {
    #[doc(hidden)]
    pub const fn from_runtime_projection(maximum_results: u16, maximum_candidates: u16) -> Self {
        Self {
            maximum_results,
            maximum_candidates,
        }
    }

    pub const fn maximum_results(self) -> u16 {
        self.maximum_results
    }

    pub const fn maximum_candidates(self) -> u16 {
        self.maximum_candidates
    }
}
