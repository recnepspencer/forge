use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeRootPublicationLaw {
    stable_reads_require_monotonic_root_generation: bool,
    replay_requires_manifest_monotonicity: bool,
}

impl S8BTreeRootPublicationLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            stable_reads_require_monotonic_root_generation: true,
            replay_requires_manifest_monotonicity: true,
        }
    }

    pub const fn verify_root_publication(
        self,
        previous_generation: u64,
        next_generation: u64,
        checksum_scope_matches: bool,
    ) -> Result<(), S8StrategyDenial> {
        if next_generation > previous_generation && checksum_scope_matches {
            return Ok(());
        }
        Err(S8StrategyDenial::RootPublicationViolation)
    }

    pub const fn verify_root_publication_progress(
        self,
        root_generation_advanced: bool,
        checksum_scope_matches: bool,
    ) -> Result<(), S8StrategyDenial> {
        if root_generation_advanced && checksum_scope_matches {
            return Ok(());
        }
        Err(S8StrategyDenial::RootPublicationViolation)
    }

    pub const fn verify_recovery_replay(
        self,
        last_published_generation: u64,
        replay_visible_generation: u64,
        manifest_advanced: bool,
    ) -> Result<(), S8StrategyDenial> {
        if self.replay_requires_manifest_monotonicity
            && ((manifest_advanced && replay_visible_generation >= last_published_generation)
                || (!manifest_advanced && replay_visible_generation == last_published_generation))
        {
            return Ok(());
        }
        Err(S8StrategyDenial::RecoveryReplayViolation)
    }

    pub const fn verify_recovery_replay_progress(
        self,
        replay_generation_monotonic: bool,
        manifest_advanced: bool,
    ) -> Result<(), S8StrategyDenial> {
        if self.replay_requires_manifest_monotonicity
            && (!manifest_advanced || replay_generation_monotonic)
        {
            return Ok(());
        }
        Err(S8StrategyDenial::RecoveryReplayViolation)
    }
}
