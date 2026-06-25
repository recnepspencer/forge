use topology::derived_invalidation_authority_inventory::{
    DerivedInvalidationAuthorityInventoryCloseout as DeletionContractAuthorityInventoryCloseout,
    DerivedInvalidationAuthorityOwner as DeletionContractAuthorityOwner,
    DerivedInvalidationOldAuthorityKind as DeletionContractOldAuthorityKind,
    DerivedInvalidationProductCategory as DeletionContractProductCategory,
};
use topology::derived_invalidation_deletion_closeout::{
    close_derived_invalidation_deletion, current_deletion_source_firewall,
    DerivedInvalidationDeletionAudit, DerivedInvalidationDeletionCloseout,
    DerivedInvalidationDeletionCounters, DerivedInvalidationDeletionDisposition,
    DerivedInvalidationDeletionError, DerivedInvalidationDeletionErrorKind,
    DerivedInvalidationDeletionLedger, DerivedInvalidationDeletionRow,
    DerivedInvalidationDeletionSourceFirewall, DerivedInvalidationDeletionSourceFirewallViolation,
    DerivedInvalidationPhaseNineSeed, DerivedInvalidationResidueAudit,
    DerivedInvalidationResidueAuditRow,
};
use topology::derived_invalidation_migrated_products::CoveredDerivedProductMigrationSweepCloseout as DeletionContractMigrationSweepCloseout;
use topology::derived_invalidation_operator_cutover::DerivedInvalidationPhaseEightSeed as DeletionContractPhaseEightSeed;

fn _derived_invalidation_deletion_closeout_contract() {
    let _: fn(
        &DeletionContractPhaseEightSeed,
        &DeletionContractMigrationSweepCloseout,
        &DeletionContractAuthorityInventoryCloseout,
    ) -> Result<DerivedInvalidationDeletionCloseout, DerivedInvalidationDeletionError> =
        close_derived_invalidation_deletion;

    let _: fn(&DerivedInvalidationDeletionError) -> DerivedInvalidationDeletionErrorKind =
        DerivedInvalidationDeletionError::kind;
    let _: fn(&DerivedInvalidationDeletionError) -> &str =
        DerivedInvalidationDeletionError::reason;

    let _: fn(&DerivedInvalidationDeletionCloseout) -> &str =
        DerivedInvalidationDeletionCloseout::phase_eight_seed_digest;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &str =
        DerivedInvalidationDeletionCloseout::migration_sweep_digest;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &str =
        DerivedInvalidationDeletionCloseout::inventory_digest;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &DerivedInvalidationDeletionLedger =
        DerivedInvalidationDeletionCloseout::deletion_ledger;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &DerivedInvalidationResidueAudit =
        DerivedInvalidationDeletionCloseout::residue_audit;
    let _: fn(
        &DerivedInvalidationDeletionCloseout,
    ) -> &DerivedInvalidationDeletionSourceFirewall =
        DerivedInvalidationDeletionCloseout::source_firewall;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &DerivedInvalidationDeletionAudit =
        DerivedInvalidationDeletionCloseout::deletion_audit;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &DerivedInvalidationDeletionCounters =
        DerivedInvalidationDeletionCloseout::counters;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &DerivedInvalidationPhaseNineSeed =
        DerivedInvalidationDeletionCloseout::phase_nine_seed;
    let _: fn(&DerivedInvalidationDeletionCloseout) -> &str =
        DerivedInvalidationDeletionCloseout::closeout_digest;

    let _: fn(&DerivedInvalidationDeletionLedger) -> &[DerivedInvalidationDeletionRow] =
        DerivedInvalidationDeletionLedger::rows;
    let _: fn(&DerivedInvalidationDeletionLedger) -> &str =
        DerivedInvalidationDeletionLedger::ledger_digest;
    let _: fn(&DerivedInvalidationDeletionRow) -> &str =
        DerivedInvalidationDeletionRow::source_path;
    let _: fn(&DerivedInvalidationDeletionRow) -> &str = DerivedInvalidationDeletionRow::surface;
    let _: fn(&DerivedInvalidationDeletionRow) -> DeletionContractProductCategory =
        DerivedInvalidationDeletionRow::product_category;
    let _: fn(&DerivedInvalidationDeletionRow) -> DeletionContractOldAuthorityKind =
        DerivedInvalidationDeletionRow::authority_kind;
    let _: fn(&DerivedInvalidationDeletionRow) -> DeletionContractAuthorityOwner =
        DerivedInvalidationDeletionRow::owner;
    let _: fn(&DerivedInvalidationDeletionRow) -> DerivedInvalidationDeletionDisposition =
        DerivedInvalidationDeletionRow::disposition;
    let _: fn(&DerivedInvalidationDeletionRow) -> &str =
        DerivedInvalidationDeletionRow::inventory_row_digest;
    let _: fn(&DerivedInvalidationDeletionRow) -> &str =
        DerivedInvalidationDeletionRow::row_digest;
    let _: fn(DerivedInvalidationDeletionDisposition) -> &'static str =
        DerivedInvalidationDeletionDisposition::as_str;

    let _: fn(&DerivedInvalidationResidueAudit) -> &[DerivedInvalidationResidueAuditRow] =
        DerivedInvalidationResidueAudit::rows;
    let _: fn(&DerivedInvalidationResidueAudit) -> &str =
        DerivedInvalidationResidueAudit::audit_digest;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::source_path;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::surface;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> DeletionContractProductCategory =
        DerivedInvalidationResidueAuditRow::product_category;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> DeletionContractOldAuthorityKind =
        DerivedInvalidationResidueAuditRow::authority_kind;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> DeletionContractAuthorityOwner =
        DerivedInvalidationResidueAuditRow::owner;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> usize =
        DerivedInvalidationResidueAuditRow::capped_count;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::blocker;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::removal_trigger;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> bool =
        DerivedInvalidationResidueAuditRow::certification_or_bootstrap_only;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> bool =
        DerivedInvalidationResidueAuditRow::ordinary_invalidation_admissible;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::inventory_row_digest;
    let _: fn(&DerivedInvalidationResidueAuditRow) -> &str =
        DerivedInvalidationResidueAuditRow::row_digest;

    let _: fn() -> DerivedInvalidationDeletionSourceFirewall = current_deletion_source_firewall;
    let _: fn(
        &DerivedInvalidationDeletionSourceFirewall,
    ) -> &[DerivedInvalidationDeletionSourceFirewallViolation] =
        DerivedInvalidationDeletionSourceFirewall::violations;
    let _: fn(&DerivedInvalidationDeletionSourceFirewall) -> &str =
        DerivedInvalidationDeletionSourceFirewall::report_digest;
    let _: fn(&DerivedInvalidationDeletionSourceFirewall) -> usize =
        DerivedInvalidationDeletionSourceFirewall::scanned_source_count;
    let _: fn(&DerivedInvalidationDeletionSourceFirewall) -> usize =
        DerivedInvalidationDeletionSourceFirewall::observed_pattern_count;
    let _: fn(&DerivedInvalidationDeletionSourceFirewallViolation) -> &str =
        DerivedInvalidationDeletionSourceFirewallViolation::source_path;
    let _: fn(&DerivedInvalidationDeletionSourceFirewallViolation) -> &str =
        DerivedInvalidationDeletionSourceFirewallViolation::forbidden_surface;
    let _: fn(&DerivedInvalidationDeletionSourceFirewallViolation) -> &str =
        DerivedInvalidationDeletionSourceFirewallViolation::owner;
    let _: fn(&DerivedInvalidationDeletionSourceFirewallViolation) -> &str =
        DerivedInvalidationDeletionSourceFirewallViolation::removal_trigger;

    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::scanned_source_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::observed_old_authority_pattern_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::ordinary_dirty_path_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::ordinary_whole_view_rebuild_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::migrated_or_denied_old_authority_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> usize =
        DerivedInvalidationDeletionAudit::certification_residue_count;
    let _: fn(&DerivedInvalidationDeletionAudit) -> &str =
        DerivedInvalidationDeletionAudit::audit_digest;

    let _: fn(&DerivedInvalidationDeletionCounters) -> usize =
        DerivedInvalidationDeletionCounters::deletion_row_count;
    let _: fn(&DerivedInvalidationDeletionCounters) -> usize =
        DerivedInvalidationDeletionCounters::residue_row_count;
    let _: fn(&DerivedInvalidationDeletionCounters) -> usize =
        DerivedInvalidationDeletionCounters::source_firewall_violation_count;
    let _: fn(&DerivedInvalidationDeletionCounters) -> usize =
        DerivedInvalidationDeletionCounters::ordinary_dirty_path_count;
    let _: fn(&DerivedInvalidationDeletionCounters) -> usize =
        DerivedInvalidationDeletionCounters::ordinary_whole_view_rebuild_count;
    let _: fn(&DerivedInvalidationDeletionCounters) -> &str =
        DerivedInvalidationDeletionCounters::counters_digest;

    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::phase_eight_seed_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::migration_sweep_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::deletion_ledger_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::residue_audit_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::source_firewall_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::counters_digest;
    let _: fn(&DerivedInvalidationPhaseNineSeed) -> &str =
        DerivedInvalidationPhaseNineSeed::seed_digest;
}
