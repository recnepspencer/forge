use super::enforcement::WorthQueryProhibitionEnforcementTier;
use super::seam::WorthQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryProhibitionRegistryRow {
    seam: WorthQueryProhibitedSeam,
    enforcement_tier: WorthQueryProhibitionEnforcementTier,
    replacement_lane: &'static str,
    rationale: &'static str,
}

impl WorthQueryProhibitionRegistryRow {
    pub(crate) const fn new(
        seam: WorthQueryProhibitedSeam,
        enforcement_tier: WorthQueryProhibitionEnforcementTier,
        replacement_lane: &'static str,
        rationale: &'static str,
    ) -> Self {
        Self {
            seam,
            enforcement_tier,
            replacement_lane,
            rationale,
        }
    }

    pub fn seam(&self) -> WorthQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn public_symbol(&self) -> &'static str {
        self.seam.public_symbol()
    }

    pub fn enforcement_tier(&self) -> WorthQueryProhibitionEnforcementTier {
        self.enforcement_tier
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn rationale(&self) -> &'static str {
        self.rationale
    }
}
