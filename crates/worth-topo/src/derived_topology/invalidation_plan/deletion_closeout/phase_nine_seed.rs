use serde::Serialize;

use super::super::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use super::super::operator_cutover::DerivedInvalidationPhaseEightSeed;
use super::counters::DerivedInvalidationDeletionCounters;
use super::deletion_ledger::DerivedInvalidationDeletionLedger;
use super::residue_audit::DerivedInvalidationResidueAudit;
use super::source_firewall::DerivedInvalidationDeletionSourceFirewall;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPhaseNineSeed {
    phase_eight_seed_digest: String,
    migration_sweep_digest: String,
    deletion_ledger_digest: String,
    residue_audit_digest: String,
    source_firewall_digest: String,
    counters_digest: String,
    seed_digest: String,
}

impl DerivedInvalidationPhaseNineSeed {
    pub(crate) fn from_closeout_parts(
        phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        deletion_ledger: &DerivedInvalidationDeletionLedger,
        residue_audit: &DerivedInvalidationResidueAudit,
        source_firewall: &DerivedInvalidationDeletionSourceFirewall,
        counters: &DerivedInvalidationDeletionCounters,
    ) -> Self {
        let seed_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-phase-nine-seed:v1".to_string(),
            format!("phase-eight:{}", phase_eight_seed.seed_digest()),
            format!("migration-sweep:{}", migration_sweep.closeout_digest()),
            format!("deletion-ledger:{}", deletion_ledger.ledger_digest()),
            format!("residue-audit:{}", residue_audit.audit_digest()),
            format!("source-firewall:{}", source_firewall.report_digest()),
            format!("counters:{}", counters.counters_digest()),
        ]);
        Self {
            phase_eight_seed_digest: phase_eight_seed.seed_digest().to_string(),
            migration_sweep_digest: migration_sweep.closeout_digest().to_string(),
            deletion_ledger_digest: deletion_ledger.ledger_digest().to_string(),
            residue_audit_digest: residue_audit.audit_digest().to_string(),
            source_firewall_digest: source_firewall.report_digest().to_string(),
            counters_digest: counters.counters_digest().to_string(),
            seed_digest,
        }
    }

    pub fn phase_eight_seed_digest(&self) -> &str {
        &self.phase_eight_seed_digest
    }

    pub fn migration_sweep_digest(&self) -> &str {
        &self.migration_sweep_digest
    }

    pub fn deletion_ledger_digest(&self) -> &str {
        &self.deletion_ledger_digest
    }

    pub fn residue_audit_digest(&self) -> &str {
        &self.residue_audit_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
