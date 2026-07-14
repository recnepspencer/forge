use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeSplitMergeLaw {
    minimum_occupancy: u16,
    maximum_occupancy: u16,
    sibling_links_required: bool,
}

impl BTreeSplitMergeLaw {
    pub(crate) const fn baseline(minimum_occupancy: u16, maximum_occupancy: u16) -> Self {
        Self {
            minimum_occupancy,
            maximum_occupancy,
            sibling_links_required: false,
        }
    }

    pub const fn sibling_links_required(self) -> bool {
        self.sibling_links_required
    }

    pub const fn verify_split(
        self,
        left_occupancy: u16,
        right_occupancy: u16,
        promoted_separator_is_between_halves: bool,
    ) -> Result<(), StrategyDenial> {
        let left_valid =
            left_occupancy >= self.minimum_occupancy && left_occupancy <= self.maximum_occupancy;
        let right_valid =
            right_occupancy >= self.minimum_occupancy && right_occupancy <= self.maximum_occupancy;

        if left_valid && right_valid && promoted_separator_is_between_halves {
            return Ok(());
        }
        Err(StrategyDenial::OccupancyViolation)
    }
}
