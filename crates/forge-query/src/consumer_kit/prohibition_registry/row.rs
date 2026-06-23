use super::enforcement::ForgeQueryProhibitionEnforcementTier;
use super::seam::ForgeQueryProhibitedSeam;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryProhibitionRegistryRow {
    seam: ForgeQueryProhibitedSeam,
    enforcement_tier: ForgeQueryProhibitionEnforcementTier,
    replacement_lane: &'static str,
    rationale: &'static str,
}

impl ForgeQueryProhibitionRegistryRow {
    pub(crate) const fn new(
        seam: ForgeQueryProhibitedSeam,
        enforcement_tier: ForgeQueryProhibitionEnforcementTier,
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

    pub fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub fn seam_key(&self) -> &'static str {
        self.seam.key()
    }

    pub fn public_symbol(&self) -> &'static str {
        self.seam.public_symbol()
    }

    pub fn enforcement_tier(&self) -> ForgeQueryProhibitionEnforcementTier {
        self.enforcement_tier
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn rationale(&self) -> &'static str {
        self.rationale
    }
}
