use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeSiblingLinkLaw {
    sibling_links_supported: bool,
}

impl S8BTreeSiblingLinkLaw {
    pub(crate) const fn baseline_absent() -> Self {
        Self {
            sibling_links_supported: false,
        }
    }

    pub const fn sibling_links_supported(self) -> bool {
        self.sibling_links_supported
    }

    pub const fn verify_sibling_link_posture(
        self,
        sibling_link_present: bool,
    ) -> Result<(), S8StrategyDenial> {
        if sibling_link_present == self.sibling_links_supported {
            return Ok(());
        }
        Err(S8StrategyDenial::SiblingLinkViolation)
    }
}
