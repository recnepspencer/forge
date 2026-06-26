use serde::Serialize;

use super::CoveredDerivedProductMigrationCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CoveredDerivedProductPhaseSevenSeed {
    closeout_digest: String,
    counters_digest: String,
    seed_digest: String,
}

impl CoveredDerivedProductPhaseSevenSeed {
    pub(crate) fn from_closeout(
        closeout_digest: &str,
        counters: &CoveredDerivedProductMigrationCounters,
    ) -> Self {
        let counters_digest = counters.counters_digest().to_string();
        let seed_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:covered-derived-product-phase-seven-seed:v1".to_string(),
            format!("closeout:{closeout_digest}"),
            format!("counters:{counters_digest}"),
        ]);
        Self {
            closeout_digest: closeout_digest.to_string(),
            counters_digest,
            seed_digest,
        }
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
