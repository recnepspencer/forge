use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8BTreeStableReadLaw {
    requires_published_root_generation: bool,
}

impl S8BTreeStableReadLaw {
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
    ) -> Result<(), S8StrategyDenial> {
        if !self.requires_published_root_generation {
            return Ok(());
        }
        if observed_root_generation == published_root_generation
            && replay_generation == published_root_generation
        {
            return Ok(());
        }
        Err(S8StrategyDenial::StableReadViolation)
    }
}
