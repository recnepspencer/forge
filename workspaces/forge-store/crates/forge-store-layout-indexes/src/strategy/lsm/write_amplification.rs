use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmWriteAmplificationLaw;

impl S8LsmWriteAmplificationLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub const fn verify_accounting(
        self,
        bytes_in: u64,
        bytes_out: u64,
        rewritten_runs: u16,
    ) -> Result<(), S8StrategyDenial> {
        // Compaction may legitimately shrink output after tombstone/value
        // collapse. Exact non-zero input/output work and rewritten-run count
        // are the accounting invariant; amplification is their measured ratio.
        if bytes_in > 0 && bytes_out > 0 && rewritten_runs > 0 {
            return Ok(());
        }
        Err(S8StrategyDenial::WriteAmplificationViolation)
    }
}
