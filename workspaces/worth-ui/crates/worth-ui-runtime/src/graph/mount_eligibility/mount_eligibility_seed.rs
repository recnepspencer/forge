#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphMountEligibilitySeed {
    graph_eligibility_reserved: bool,
}

impl UiGraphMountEligibilitySeed {
    pub(crate) const fn reserved() -> Self {
        Self {
            graph_eligibility_reserved: true,
        }
    }

    pub fn graph_eligibility_reserved(self) -> bool {
        self.graph_eligibility_reserved
    }
}
