use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmMemtableWalLaw {
    wal_must_precede_memtable_visibility: bool,
    replay_requires_monotonic_sequence: bool,
}

impl S8LsmMemtableWalLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            wal_must_precede_memtable_visibility: true,
            replay_requires_monotonic_sequence: true,
        }
    }

    pub const fn verify_memtable_visibility(
        self,
        last_durable_wal_sequence: u64,
        visible_memtable_floor: u64,
    ) -> Result<(), S8StrategyDenial> {
        if !self.wal_must_precede_memtable_visibility
            || last_durable_wal_sequence >= visible_memtable_floor
        {
            return Ok(());
        }
        Err(S8StrategyDenial::RecoveryReplayViolation)
    }

    pub const fn verify_recovery_replay(
        self,
        last_flushed_sequence: u64,
        replayed_sequence: u64,
    ) -> Result<(), S8StrategyDenial> {
        if !self.replay_requires_monotonic_sequence || replayed_sequence >= last_flushed_sequence {
            return Ok(());
        }
        Err(S8StrategyDenial::RecoveryReplayViolation)
    }

    pub const fn verify_recovery_replay_progress(
        self,
        replay_monotonic: bool,
    ) -> Result<(), S8StrategyDenial> {
        if !self.replay_requires_monotonic_sequence || replay_monotonic {
            return Ok(());
        }
        Err(S8StrategyDenial::RecoveryReplayViolation)
    }
}
