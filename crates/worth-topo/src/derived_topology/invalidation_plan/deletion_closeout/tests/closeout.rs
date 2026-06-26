use std::collections::BTreeSet;

use super::super::super::inventory::DerivedInvalidationProductCategory;
use super::super::super::inventory::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityInventoryErrorKind,
    DerivedInvalidationOrdinaryProofAdmission,
};
use super::super::super::migrated_products::CoveredDerivedProductMigrationError;
use super::super::{
    close_derived_invalidation_deletion,
    close_derived_invalidation_deletion_with_source_firewall_for_tests,
    DerivedInvalidationDeletionDisposition, DerivedInvalidationDeletionErrorKind,
};
use super::support::{
    clean_firewall, full_migration_sweep, inventory_closeout, mismatched_phase_eight_seed,
    partial_migration_sweep, phase_eight_seed, selected_plan,
};

#[test]
fn deletion_closeout_binds_phase_eight_seed_full_migration_sweep_and_inventory() {
    let selected_plan = selected_plan();
    let phase_eight_seed = phase_eight_seed(&selected_plan);
    let migration_sweep = full_migration_sweep(&selected_plan);
    let inventory_closeout = inventory_closeout();

    let closeout = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &phase_eight_seed,
        &migration_sweep,
        &inventory_closeout,
        clean_firewall(),
    )
    .expect("phase eight deletion closeout");

    assert_eq!(
        closeout.phase_eight_seed_digest(),
        phase_eight_seed.seed_digest()
    );
    assert_eq!(
        closeout.phase_nine_seed().phase_eight_seed_digest(),
        phase_eight_seed.seed_digest()
    );
    assert_eq!(
        closeout.phase_nine_seed().migration_sweep_digest(),
        migration_sweep.closeout_digest()
    );
    assert_eq!(closeout.counters().source_firewall_violation_count(), 0);
    assert_eq!(closeout.counters().ordinary_dirty_path_count(), 0);
    assert_eq!(closeout.counters().ordinary_whole_view_rebuild_count(), 0);
    assert_eq!(closeout.deletion_audit().scanned_source_count(), 0);
    assert_eq!(
        closeout
            .deletion_audit()
            .observed_old_authority_pattern_count(),
        0
    );
    assert_eq!(
        closeout
            .deletion_audit()
            .migrated_or_denied_old_authority_count(),
        closeout.deletion_ledger().rows().len()
    );
    assert_eq!(
        closeout.deletion_audit().certification_residue_count(),
        closeout.residue_audit().rows().len()
    );
    assert!(
        closeout
            .deletion_ledger()
            .rows()
            .iter()
            .all(|row| row.disposition()
                == DerivedInvalidationDeletionDisposition::OldAuthorityDenied)
    );
    assert_eq!(
        closeout.counters().deletion_row_count(),
        closeout.deletion_ledger().rows().len()
    );
}

#[test]
fn deletion_closeout_firewall_uses_inventory_source_scan_not_miniature_strings() {
    let selected_plan = selected_plan();
    let closeout = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &phase_eight_seed(&selected_plan),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout(),
        clean_firewall(),
    )
    .expect("phase eight deletion closeout");

    assert_eq!(closeout.source_firewall().scanned_source_count(), 0);
    assert_eq!(closeout.source_firewall().observed_pattern_count(), 0);
    assert_eq!(closeout.source_firewall().violations(), &[]);
}

#[test]
fn public_deletion_closeout_accepts_current_tree_after_old_ordinary_authority_cutover() {
    let selected_plan = selected_plan();
    let closeout = close_derived_invalidation_deletion(
        &phase_eight_seed(&selected_plan),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout(),
    )
    .expect("current tree should have no live old ordinary derived authority");

    assert_eq!(closeout.counters().source_firewall_violation_count(), 0);
    assert_eq!(closeout.source_firewall().violations(), &[]);
}

#[test]
fn deletion_ledger_covers_every_ordinary_derived_product_category() {
    let selected_plan = selected_plan();
    let closeout = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &phase_eight_seed(&selected_plan),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout(),
        clean_firewall(),
    )
    .expect("phase eight deletion closeout");
    let deleted_categories = closeout
        .deletion_ledger()
        .rows()
        .iter()
        .map(|row| row.product_category())
        .collect::<BTreeSet<_>>();

    for category in DerivedInvalidationProductCategory::COVERED_ORDINARY {
        assert!(
            deleted_categories.contains(&category),
            "missing deletion proof for {}",
            category.as_str()
        );
    }
}

#[test]
fn deletion_closeout_rejects_partial_migration_sweep() {
    let selected_plan = selected_plan();
    let error =
        partial_migration_sweep(&selected_plan).expect_err("partial migration sweep cannot close");

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable
    );
}

#[test]
fn deletion_closeout_rejects_mismatched_phase_eight_seed() {
    let selected_plan = selected_plan();
    let error = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &mismatched_phase_eight_seed(),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout(),
        clean_firewall(),
    )
    .expect_err("unrelated Phase 8 seed cannot close deletion");

    assert_eq!(
        error.kind(),
        DerivedInvalidationDeletionErrorKind::PhaseEightSeedDoesNotMatchMigrationSweep
    );
}

#[test]
fn deletion_closeout_residue_audit_is_capped_certification_only() {
    let selected_plan = selected_plan();
    let inventory_closeout = inventory_closeout();
    let closeout = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &phase_eight_seed(&selected_plan),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout,
        clean_firewall(),
    )
    .expect("phase eight deletion closeout");

    assert!(closeout.counters().residue_row_count() > 0);
    for row in closeout.residue_audit().rows() {
        assert!(row.capped_count() > 0);
        assert!(row.certification_or_bootstrap_only());
        assert!(!row.ordinary_invalidation_admissible());
        assert!(!row.blocker().is_empty());
        assert!(!row.removal_trigger().is_empty());
    }

    for row in inventory_closeout.inventory().rows().iter().filter(|row| {
        row.disposition() == DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
    }) {
        let error = DerivedInvalidationOrdinaryProofAdmission::admit_inventory_row(row)
            .expect_err("certification residue cannot satisfy ordinary invalidation proof");
        assert!(matches!(
            error.kind(),
            DerivedInvalidationAuthorityInventoryErrorKind::CertificationResidueCannotSatisfyOrdinaryInvalidation { .. }
        ));
    }
}
