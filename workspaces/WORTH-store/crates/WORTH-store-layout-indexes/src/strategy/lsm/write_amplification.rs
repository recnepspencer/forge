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
        if bytes_in > 0 && bytes_out >= bytes_in && rewritten_runs > 0 {
            return Ok(());
        }
        Err(S8StrategyDenial::WriteAmplificationViolation)
    }
}
