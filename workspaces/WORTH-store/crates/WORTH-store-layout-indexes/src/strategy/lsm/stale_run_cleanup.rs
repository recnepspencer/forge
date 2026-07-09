use crate::strategy::S8StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LsmStaleRunCleanupLaw;

impl S8LsmStaleRunCleanupLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub const fn verify_cleanup(
        self,
        stale_runs_retired: u16,
        shadowed_runs_identified: u16,
        live_manifest_runs: u16,
    ) -> Result<(), S8StrategyDenial> {
        if stale_runs_retired > 0
            && shadowed_runs_identified >= stale_runs_retired
            && live_manifest_runs > 0
        {
            return Ok(());
        }
        Err(S8StrategyDenial::StaleRunCleanupViolation)
    }
}
