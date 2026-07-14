use crate::strategy::StrategyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmStaleRunCleanupLaw;

impl LsmStaleRunCleanupLaw {
    pub(crate) const fn baseline() -> Self {
        Self
    }

    pub const fn verify_cleanup(
        self,
        stale_runs_retired: u16,
        shadowed_runs_identified: u16,
        live_manifest_runs: u16,
    ) -> Result<(), StrategyDenial> {
        if stale_runs_retired > 0
            && shadowed_runs_identified >= stale_runs_retired
            && live_manifest_runs > 0
        {
            return Ok(());
        }
        Err(StrategyDenial::StaleRunCleanupViolation)
    }
}
