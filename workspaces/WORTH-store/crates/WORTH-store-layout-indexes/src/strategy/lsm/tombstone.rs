use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmTombstoneLaw {
    newer_tombstone_shadows_older_value: bool,
}

impl S8LsmTombstoneLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            newer_tombstone_shadows_older_value: true,
        }
    }

    pub const fn verify_shadowing(
        self,
        newer_tombstone_sequence: u64,
        older_value_sequence: u64,
        tombstone_retained_during_compaction: bool,
    ) -> Result<(), S8StrategyDenial> {
        if self.newer_tombstone_shadows_older_value
            && newer_tombstone_sequence > older_value_sequence
            && tombstone_retained_during_compaction
        {
            return Ok(());
        }
        Err(S8StrategyDenial::TombstonePreservationViolation)
    }
}
