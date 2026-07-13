use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeStableReadLaw {
    requires_published_root_generation: bool,
}

impl BTreeStableReadLaw {
    pub(crate) const fn baseline() -> Self {
        Self {
            requires_published_root_generation: true,
        }
    }

    pub const fn verify_stable_read(
        self,
        observed_root_generation: u64,
        published_root_generation: u64,
        replay_generation: u64,
    ) -> Result<(), StrategyDenial> {
        if !self.requires_published_root_generation {
            return Ok(());
        }
        if observed_root_generation == published_root_generation
            && replay_generation == published_root_generation
        {
            return Ok(());
        }
        Err(StrategyDenial::StableReadViolation)
    }
}
