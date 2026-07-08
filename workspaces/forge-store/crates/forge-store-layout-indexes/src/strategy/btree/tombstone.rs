use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeTombstoneLaw {
    tombstones_supported: bool,
}

impl S8BTreeTombstoneLaw {
    pub(crate) const fn baseline_absent() -> Self {
        Self {
            tombstones_supported: false,
        }
    }

    pub const fn tombstones_supported(self) -> bool {
        self.tombstones_supported
    }

    pub const fn verify_tombstone_posture(
        self,
        tombstone_seen: bool,
    ) -> Result<(), S8StrategyDenial> {
        if tombstone_seen == self.tombstones_supported {
            return Ok(());
        }
        Err(S8StrategyDenial::TombstonePostureViolation)
    }
}
