use super::super::{
    close_covered_derived_product_migration_sweep, CoveredDerivedProductMigrationError,
    CoveredDerivedProductMigrationStatus,
};
use super::support::{
    all_family_real_migration_sweep, loop_cycle_and_wire_view_bridge_rows, loop_cycle_bridge_rows,
    rows_without_family, selected_loop_cycle_plan,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;

#[test]
fn loop_cycle_bridge_marks_only_loop_cycles_as_ordinary_migrated() {
    let rows = loop_cycle_bridge_rows();

    assert_eq!(
        rows.len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len()
    );
    let loop_row = rows
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
        .unwrap();
    assert_eq!(
        loop_row.status(),
        CoveredDerivedProductMigrationStatus::Migrated
    );
    assert!(loop_row.ordinary_invalidation_consumable());

    let residue_rows: Vec<_> = rows
        .iter()
        .filter(|row| row.family_identity() != DerivedTopologyProductFamilyIdentity::LoopCycles)
        .collect();
    assert_eq!(
        residue_rows.len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len() - 1
    );
    assert!(residue_rows
        .iter()
        .all(|row| row.status() == CoveredDerivedProductMigrationStatus::CertificationResidueOnly));
    assert!(residue_rows
        .iter()
        .all(|row| !row.ordinary_invalidation_consumable()));
}

#[test]
fn loop_cycle_and_wire_view_bridge_marks_two_families_as_ordinary_migrated() {
    let rows = loop_cycle_and_wire_view_bridge_rows();
    let migrated: std::collections::BTreeSet<_> = rows
        .iter()
        .filter(|row| row.status() == CoveredDerivedProductMigrationStatus::Migrated)
        .map(|row| row.family_identity())
        .collect();

    assert_eq!(
        migrated,
        [
            DerivedTopologyProductFamilyIdentity::LoopCycles,
            DerivedTopologyProductFamilyIdentity::WireViews,
        ]
        .into_iter()
        .collect()
    );
    assert!(rows
        .iter()
        .filter(|row| migrated.contains(&row.family_identity()))
        .all(|row| row.ordinary_invalidation_consumable()));
}

#[test]
fn missing_covered_family_status_cannot_close_sweep() {
    let plan = selected_loop_cycle_plan();
    let error = close_covered_derived_product_migration_sweep(
        &plan,
        rows_without_family(DerivedTopologyProductFamilyIdentity::WireViews),
    )
    .unwrap_err();

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::MissingRequiredFamily
    );
}

#[test]
fn loop_cycle_only_bridge_cannot_close_selected_non_loop_products() {
    let plan = selected_loop_cycle_plan();
    let error =
        close_covered_derived_product_migration_sweep(&plan, loop_cycle_bridge_rows()).unwrap_err();

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable
    );
}

#[test]
fn two_family_bridge_still_cannot_close_remaining_selected_products() {
    let plan = selected_loop_cycle_plan();
    let error = close_covered_derived_product_migration_sweep(
        &plan,
        loop_cycle_and_wire_view_bridge_rows(),
    )
    .unwrap_err();

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::RequiredFamilyNotOrdinaryConsumable
    );
}

#[test]
fn duplicate_covered_family_status_cannot_close_sweep() {
    let plan = selected_loop_cycle_plan();
    let mut rows = loop_cycle_bridge_rows();
    let loop_cycle_row = rows
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
        .unwrap()
        .clone();
    rows.push(loop_cycle_row);

    let error = close_covered_derived_product_migration_sweep(&plan, rows).unwrap_err();

    assert_eq!(
        error,
        CoveredDerivedProductMigrationError::DuplicateFamilyStatus
    );
}

#[test]
fn all_required_families_close_from_real_family_migration_apis() {
    let sweep = all_family_real_migration_sweep();

    assert_eq!(
        sweep.status_rows().len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len()
    );
    assert_eq!(
        sweep.counters().ordinary_consumable_family_count(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len()
    );
    assert!(sweep.status_rows().iter().all(|row| {
        row.status() == CoveredDerivedProductMigrationStatus::Migrated
            && row.ordinary_invalidation_consumable()
            && row.selected_plan_digest().is_some()
            && row.execution_receipt_digest().is_some()
    }));
}
