use serde::Serialize;

use super::{TraversalViewsMigrationCounters, TraversalViewsOldAuthorityResidue};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsPhaseElevenSeed {
    migrated_family: &'static str,
    closeout_digest: String,
    counters_digest: String,
    old_authority_residue_digest: String,
    seed_digest: String,
}

impl TraversalViewsPhaseElevenSeed {
    pub(crate) fn from_closeout_parts(
        closeout_digest: &str,
        counters: &TraversalViewsMigrationCounters,
        residue: &TraversalViewsOldAuthorityResidue,
    ) -> Self {
        let migrated_family = "traversal_views";
        let counters_digest = counters.counters_digest().to_string();
        let old_authority_residue_digest = residue.residue_digest().to_string();
        let seed_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-phase-eleven-seed:v1".to_string(),
            format!("family:{migrated_family}"),
            format!("closeout:{closeout_digest}"),
            format!("counters:{counters_digest}"),
            format!("old-authority-residue:{old_authority_residue_digest}"),
        ]);
        Self {
            migrated_family,
            closeout_digest: closeout_digest.to_string(),
            counters_digest,
            old_authority_residue_digest,
            seed_digest,
        }
    }

    pub const fn migrated_family(&self) -> &'static str {
        self.migrated_family
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn old_authority_residue_digest(&self) -> &str {
        &self.old_authority_residue_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
