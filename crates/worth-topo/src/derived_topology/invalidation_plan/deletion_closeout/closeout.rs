use serde::Serialize;

use super::super::inventory::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityInventoryCloseout,
};
use super::super::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use super::super::operator_cutover::DerivedInvalidationPhaseEightSeed;
use super::counters::DerivedInvalidationDeletionCounters;
use super::deletion_audit::DerivedInvalidationDeletionAudit;
use super::deletion_ledger::{
    DerivedInvalidationDeletionDisposition, DerivedInvalidationDeletionLedger,
    DerivedInvalidationDeletionRow,
};
use super::error::{DerivedInvalidationDeletionError, DerivedInvalidationDeletionErrorKind};
use super::phase_nine_seed::DerivedInvalidationPhaseNineSeed;
use super::residue_audit::{DerivedInvalidationResidueAudit, DerivedInvalidationResidueAuditRow};
use super::source_firewall::DerivedInvalidationDeletionSourceFirewall;

pub fn close_derived_invalidation_deletion(
    phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
) -> Result<DerivedInvalidationDeletionCloseout, DerivedInvalidationDeletionError> {
    DerivedInvalidationDeletionCloseout::close(
        phase_eight_seed,
        migration_sweep,
        inventory_closeout,
        DerivedInvalidationDeletionSourceFirewall::from_inventory_closeout(inventory_closeout),
    )
}

#[cfg(test)]
pub(crate) fn close_derived_invalidation_deletion_with_source_firewall_for_tests(
    phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
    source_firewall: DerivedInvalidationDeletionSourceFirewall,
) -> Result<DerivedInvalidationDeletionCloseout, DerivedInvalidationDeletionError> {
    DerivedInvalidationDeletionCloseout::close(
        phase_eight_seed,
        migration_sweep,
        inventory_closeout,
        source_firewall,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionCloseout {
    phase_eight_seed_digest: String,
    migration_sweep_digest: String,
    inventory_digest: String,
    deletion_ledger: DerivedInvalidationDeletionLedger,
    residue_audit: DerivedInvalidationResidueAudit,
    source_firewall: DerivedInvalidationDeletionSourceFirewall,
    deletion_audit: DerivedInvalidationDeletionAudit,
    counters: DerivedInvalidationDeletionCounters,
    phase_nine_seed: DerivedInvalidationPhaseNineSeed,
    closeout_digest: String,
}

impl DerivedInvalidationDeletionCloseout {
    fn close(
        phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
        migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
        inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
        source_firewall: DerivedInvalidationDeletionSourceFirewall,
    ) -> Result<Self, DerivedInvalidationDeletionError> {
        require_phase_eight_seed(phase_eight_seed)?;
        require_complete_migration_sweep(migration_sweep)?;
        require_matching_selected_plan(phase_eight_seed, migration_sweep)?;
        require_clean_source_firewall(&source_firewall)?;

        let deletion_ledger = deletion_ledger_from_inventory(inventory_closeout)?;
        let residue_audit = residue_audit_from_inventory(inventory_closeout)?;
        let deletion_audit = DerivedInvalidationDeletionAudit::from_products(
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
        );
        let counters = DerivedInvalidationDeletionCounters::from_products(
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &deletion_audit,
        );
        let phase_nine_seed = DerivedInvalidationPhaseNineSeed::from_closeout_parts(
            phase_eight_seed,
            migration_sweep,
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &counters,
        );
        let closeout_digest = closeout_digest(
            phase_eight_seed,
            migration_sweep,
            inventory_closeout,
            &deletion_ledger,
            &residue_audit,
            &source_firewall,
            &deletion_audit,
            &counters,
        );
        Ok(Self {
            phase_eight_seed_digest: phase_eight_seed.seed_digest().to_string(),
            migration_sweep_digest: migration_sweep.closeout_digest().to_string(),
            inventory_digest: inventory_closeout.inventory().report_digest().to_string(),
            deletion_ledger,
            residue_audit,
            source_firewall,
            deletion_audit,
            counters,
            phase_nine_seed,
            closeout_digest,
        })
    }

    pub fn phase_eight_seed_digest(&self) -> &str {
        &self.phase_eight_seed_digest
    }

    pub fn migration_sweep_digest(&self) -> &str {
        &self.migration_sweep_digest
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }

    pub const fn deletion_ledger(&self) -> &DerivedInvalidationDeletionLedger {
        &self.deletion_ledger
    }

    pub const fn residue_audit(&self) -> &DerivedInvalidationResidueAudit {
        &self.residue_audit
    }

    pub const fn source_firewall(&self) -> &DerivedInvalidationDeletionSourceFirewall {
        &self.source_firewall
    }

    pub const fn deletion_audit(&self) -> &DerivedInvalidationDeletionAudit {
        &self.deletion_audit
    }

    pub const fn counters(&self) -> &DerivedInvalidationDeletionCounters {
        &self.counters
    }

    pub const fn phase_nine_seed(&self) -> &DerivedInvalidationPhaseNineSeed {
        &self.phase_nine_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn require_phase_eight_seed(
    phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
) -> Result<(), DerivedInvalidationDeletionError> {
    if phase_eight_seed.seed_digest().trim().is_empty() {
        return Err(DerivedInvalidationDeletionError::new(
            DerivedInvalidationDeletionErrorKind::MissingPhaseEightSeed,
            "Phase 8 deletion closeout requires the operator/projection cutover seed",
        ));
    }
    Ok(())
}

fn require_complete_migration_sweep(
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
) -> Result<(), DerivedInvalidationDeletionError> {
    let counters = migration_sweep.counters();
    if counters.ordinary_consumable_family_count() != counters.required_family_count() {
        return Err(DerivedInvalidationDeletionError::new(
            DerivedInvalidationDeletionErrorKind::IncompleteMigrationSweep,
            "Phase 8 requires every covered derived product family to be ordinary-consumable",
        ));
    }
    Ok(())
}

fn require_matching_selected_plan(
    phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
) -> Result<(), DerivedInvalidationDeletionError> {
    if phase_eight_seed.selected_plan_digest() != migration_sweep.selected_plan_digest() {
        return Err(DerivedInvalidationDeletionError::new(
            DerivedInvalidationDeletionErrorKind::PhaseEightSeedDoesNotMatchMigrationSweep,
            "Phase 8 deletion closeout cannot bind unrelated operator cutover and migration sweep proofs",
        ));
    }
    Ok(())
}

fn require_clean_source_firewall(
    source_firewall: &DerivedInvalidationDeletionSourceFirewall,
) -> Result<(), DerivedInvalidationDeletionError> {
    if !source_firewall.violations().is_empty() {
        return Err(DerivedInvalidationDeletionError::new(
            DerivedInvalidationDeletionErrorKind::SourceFirewallViolation,
            "Phase 8 source firewall found forbidden old derived-maintenance authority",
        ));
    }
    Ok(())
}

fn deletion_ledger_from_inventory(
    inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
) -> Result<DerivedInvalidationDeletionLedger, DerivedInvalidationDeletionError> {
    let mut rows = Vec::new();
    for row in inventory_closeout.inventory().rows() {
        match row.disposition() {
            DerivedInvalidationAuthorityDisposition::Migrate => {
                rows.push(DerivedInvalidationDeletionRow::from_inventory_row(
                    row,
                    DerivedInvalidationDeletionDisposition::MigratedAuthorityDeleted,
                ));
            }
            DerivedInvalidationAuthorityDisposition::Delete => {
                rows.push(DerivedInvalidationDeletionRow::from_inventory_row(
                    row,
                    DerivedInvalidationDeletionDisposition::OldAuthorityDenied,
                ));
            }
            DerivedInvalidationAuthorityDisposition::TrueQueryCapabilityGap => {
                return Err(DerivedInvalidationDeletionError::new(
                    DerivedInvalidationDeletionErrorKind::TrueQueryGapCannotClose,
                    format!("true Query gap `{}` cannot close Phase 8", row.surface()),
                ));
            }
            DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue => {}
        }
    }
    Ok(DerivedInvalidationDeletionLedger::from_rows(rows))
}

fn residue_audit_from_inventory(
    inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
) -> Result<DerivedInvalidationResidueAudit, DerivedInvalidationDeletionError> {
    let mut rows = Vec::new();
    for row in inventory_closeout.inventory().rows() {
        if row.disposition()
            != DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
        {
            continue;
        }
        if row.ordinary_path() || !row.certification_or_bootstrap_only() || row.cap().is_none() {
            return Err(DerivedInvalidationDeletionError::new(
                DerivedInvalidationDeletionErrorKind::OrdinaryResidueCannotClose,
                format!(
                    "residue `{}` can still behave as ordinary authority",
                    row.surface()
                ),
            ));
        }
        rows.push(DerivedInvalidationResidueAuditRow::from_inventory_row(row));
    }
    Ok(DerivedInvalidationResidueAudit::from_rows(rows))
}

fn closeout_digest(
    phase_eight_seed: &DerivedInvalidationPhaseEightSeed,
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    inventory_closeout: &DerivedInvalidationAuthorityInventoryCloseout,
    deletion_ledger: &DerivedInvalidationDeletionLedger,
    residue_audit: &DerivedInvalidationResidueAudit,
    source_firewall: &DerivedInvalidationDeletionSourceFirewall,
    deletion_audit: &DerivedInvalidationDeletionAudit,
    counters: &DerivedInvalidationDeletionCounters,
) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-deletion-closeout:v1".to_string(),
        format!("phase-eight:{}", phase_eight_seed.seed_digest()),
        format!("migration-sweep:{}", migration_sweep.closeout_digest()),
        format!(
            "inventory:{}",
            inventory_closeout.inventory().report_digest()
        ),
        format!("deletion-ledger:{}", deletion_ledger.ledger_digest()),
        format!("residue-audit:{}", residue_audit.audit_digest()),
        format!("source-firewall:{}", source_firewall.report_digest()),
        format!("deletion-audit:{}", deletion_audit.audit_digest()),
        format!("counters:{}", counters.counters_digest()),
    ])
}
