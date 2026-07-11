use forge_store_physical_certification::layout_harness::scenario::{
    S8LayoutTransitionState, canonical_s8_layout_required_transitions,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutFixtures {
    required_transitions: &'static [S8LayoutTransitionState],
}

pub fn s8_layout_fixtures() -> S8LayoutFixtures {
    S8LayoutFixtures {
        required_transitions: canonical_s8_layout_required_transitions(),
    }
}

impl S8LayoutFixtures {
    pub const fn required_transitions(&self) -> &'static [S8LayoutTransitionState] {
        self.required_transitions
    }
}
