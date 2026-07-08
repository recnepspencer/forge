use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmCompactionOrderingLaw;

impl S8LsmCompactionOrderingLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub fn verify_compaction(
        self,
        input_generations: &[u64],
        output_generation: u64,
        stale_runs_retired: bool,
    ) -> Result<(), S8StrategyDenial> {
        if input_generations.is_empty() || !stale_runs_retired {
            return Err(S8StrategyDenial::CompactionOrderingViolation);
        }

        let mut previous = 0;
        for generation in input_generations {
            if *generation <= previous {
                return Err(S8StrategyDenial::CompactionOrderingViolation);
            }
            previous = *generation;
        }

        if output_generation > previous {
            return Ok(());
        }
        Err(S8StrategyDenial::CompactionOrderingViolation)
    }
}
