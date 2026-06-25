use super::super::{
    close_derived_invalidation_deletion_with_source_firewall_for_tests,
    current_deletion_source_firewall, DerivedInvalidationDeletionErrorKind,
};
use super::support::{
    dirty_firewall, full_migration_sweep, inventory_closeout, phase_eight_seed, selected_plan,
};

#[test]
fn deletion_source_firewall_reports_no_current_old_ordinary_authority_after_cutover() {
    let firewall = current_deletion_source_firewall();

    assert!(
        firewall.violations().is_empty(),
        "Milestone 10 final cutover should leave no live old ordinary authority"
    );
    assert!(
        firewall
            .violations()
            .iter()
            .all(|violation| !violation.source_path().contains("certification/topology_operator_closeout/derived_fallout")),
        "historical operator fallout rows must stay capped certification residue, not ordinary deletion blockers"
    );
    assert!(firewall.scanned_source_count() >= 13);
    assert!(firewall.observed_pattern_count() >= 9);
}

#[test]
fn deletion_source_firewall_rejects_old_dirty_product_authority() {
    let selected_plan = selected_plan();
    let dirty_firewall = dirty_firewall();

    assert_eq!(dirty_firewall.violations().len(), 1);
    assert_eq!(
        dirty_firewall.violations()[0].forbidden_surface(),
        "operator_dirty_products"
    );

    let error = close_derived_invalidation_deletion_with_source_firewall_for_tests(
        &phase_eight_seed(&selected_plan),
        &full_migration_sweep(&selected_plan),
        &inventory_closeout(),
        dirty_firewall,
    )
    .expect_err("dirty product authority cannot survive Phase 8 deletion");

    assert_eq!(
        error.kind(),
        DerivedInvalidationDeletionErrorKind::SourceFirewallViolation
    );
}
