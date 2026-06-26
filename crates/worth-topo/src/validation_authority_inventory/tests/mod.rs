use std::fs;
use std::path::Path;

use super::discovery::{
    WorthValidationAuthorityDiscoveryReport, WorthValidationAuthorityReconciliation,
    WorthValidationAuthorityScanPattern,
};
use super::disposition::WorthValidationAuthorityDisposition;
use super::error::WorthValidationAuthorityInventoryError;
use super::inventory::{WorthValidationAuthorityInventory, WorthValidationAuthorityInventoryInput};
use super::milestone_eight_seed_summary::WorthValidationAuthorityMilestoneEightSeedSummary;
use super::source_authority::{
    current_validation_authority_rows, WorthValidationAuthoritySource,
    WorthValidationAuthoritySourceFirewallReport,
};

#[test]
fn phase_one_inventory_covers_current_old_authority_surfaces() {
    let inventory = WorthValidationAuthorityInventory::from_current_sources()
        .expect("phase one inventory should build");

    assert_eq!(inventory.unclassified_count(), 0);
    assert_eq!(inventory.keep_disposition_count(), 0);
    assert_eq!(inventory.rows().len(), 44);
    assert_eq!(inventory.counters().rule_registry_rows(), 5);
    assert_eq!(inventory.counters().cap_rows(), 21);
    assert_eq!(inventory.counters().query_access_gap_rows(), 1);
    assert_eq!(
        inventory
            .rows()
            .iter()
            .filter(|row| matches!(
                row.source(),
                WorthValidationAuthoritySource::MilestoneOneInvariantRegistration(_)
            ))
            .count(),
        14
    );
    assert!(
        inventory.cut_line().ready_for_parallel_catalog_lane(),
        "inventory must close with a ready cut line"
    );
}

#[test]
fn whole_view_validation_is_only_migration_or_capped_comparison_authority() {
    let inventory = WorthValidationAuthorityInventory::from_current_sources()
        .expect("phase one inventory should build");

    for source in [
        WorthValidationAuthoritySource::TopologyValidatorMaterializedReport,
        WorthValidationAuthoritySource::TopologyValidatorDerivedReport,
        WorthValidationAuthoritySource::ValidateInterpretedTopologyFacade,
    ] {
        let row = inventory
            .row_for_source(source)
            .expect("whole-view validation source should be inventoried");
        assert_eq!(
            row.disposition(),
            WorthValidationAuthorityDisposition::Migrate
        );
        assert!(
            row.certification_only_comparison_allowed(),
            "whole-view validation can survive only as comparison evidence"
        );
        assert!(
            row.removal_trigger().contains("Phase"),
            "old authority needs a concrete migration trigger"
        );
    }
}

#[test]
fn inventory_rejects_duplicate_sources() {
    let mut rows = current_validation_authority_rows();
    rows.push(rows[0].clone());

    let error = WorthValidationAuthorityInventory::from_rows_for_validation(rows)
        .expect_err("duplicate source should fail inventory validation");

    assert!(matches!(
        error,
        WorthValidationAuthorityInventoryError::DuplicateSource(_)
    ));
}

#[test]
fn inventory_rejects_missing_required_sources() {
    let mut rows = current_validation_authority_rows();
    rows.retain(|row| {
        row.source() != WorthValidationAuthoritySource::DerivedRuleRegistry("ownership")
    });

    let error = WorthValidationAuthorityInventory::from_rows_for_validation(rows)
        .expect_err("missing source should fail inventory validation");

    assert!(matches!(
        error,
        WorthValidationAuthorityInventoryError::MissingRequiredSource(_)
    ));
}

#[test]
fn milestone_eight_seed_is_not_validator_selection_authority() {
    let seed_summary =
        WorthValidationAuthorityMilestoneEightSeedSummary::from_parts("m8-seed", false, true, true);
    let input =
        WorthValidationAuthorityInventoryInput::from_milestone_eight_seed_summary(seed_summary);

    let inventory = WorthValidationAuthorityInventory::from_current_sources_with_input(input)
        .expect("seed summary that does not claim validator selection should be admitted");

    let admitted_seed = inventory
        .milestone_eight_seed_summary()
        .expect("seed summary should be retained");
    assert_eq!(admitted_seed.seed_digest(), "m8-seed");
    assert!(!admitted_seed.claims_validator_selection());
    assert!(admitted_seed.receipt_context_present());
    assert!(admitted_seed.posture_context_present());
}

#[test]
fn milestone_eight_seed_summary_rejects_validator_selection_claim() {
    let seed_summary = WorthValidationAuthorityMilestoneEightSeedSummary::from_parts(
        "dishonest-m8-seed",
        true,
        true,
        true,
    );
    let input =
        WorthValidationAuthorityInventoryInput::from_milestone_eight_seed_summary(seed_summary);

    let error = WorthValidationAuthorityInventory::from_current_sources_with_input(input)
        .expect_err("Milestone 8 seed cannot claim validator selection");

    assert!(matches!(
        error,
        WorthValidationAuthorityInventoryError::MilestoneEightSeedClaimsValidatorSelection(_)
    ));
}

#[test]
fn discovery_reconciliation_rejects_uninventoried_old_authority_source() {
    let root = temp_firewall_root("phase_one_uninventoried_source");
    write_file(
        &root
            .join("crates")
            .join("worth-topo")
            .join("src")
            .join("topology_operators")
            .join("fake_blueprint")
            .join("mod.rs"),
        "pub const validator_expectations: &[&str] = &[\"ownership\"];\n",
    );

    let inventory =
        WorthValidationAuthorityInventory::from_current_sources().expect("inventory should build");
    let discovery = WorthValidationAuthorityDiscoveryReport::scan_root(&root)
        .expect("discovery should scan temp root");

    let error = WorthValidationAuthorityReconciliation::from_inventory_and_discovery(
        &inventory, &discovery,
    )
    .expect_err("uninventoried source should fail reconciliation");

    assert!(matches!(
        error,
        WorthValidationAuthorityInventoryError::UnclassifiedDiscoveredSource(_)
    ));
}

#[test]
fn discovery_reconciliation_accepts_cataloged_old_authority_source() {
    let root = temp_firewall_root("phase_one_cataloged_source");
    write_file(
        &root
            .join("crates")
            .join("worth-topo")
            .join("src")
            .join("validation")
            .join("facade.rs"),
        "TopologyValidator::derived_validation_report\n",
    );

    let inventory =
        WorthValidationAuthorityInventory::from_current_sources().expect("inventory should build");
    let discovery = WorthValidationAuthorityDiscoveryReport::scan_root(&root)
        .expect("discovery should scan temp root");
    let reconciliation = WorthValidationAuthorityReconciliation::from_inventory_and_discovery(
        &inventory, &discovery,
    )
    .expect("cataloged source should reconcile");

    assert_eq!(reconciliation.discovered_source_count(), 1);
    assert_eq!(reconciliation.reconciled_source_count(), 1);
    assert_eq!(reconciliation.unclassified_discovered_source_count(), 0);
}

#[test]
fn source_firewall_rejects_every_old_authority_pattern() {
    for pattern in WorthValidationAuthorityScanPattern::all() {
        let root = temp_firewall_root(&format!("{:?}", pattern));
        let src_dir = root.join("topology_operators").join("fake_blueprint");
        fs::create_dir_all(&src_dir).expect("temp source dir");
        fs::write(src_dir.join("mod.rs"), format!("{}\n", pattern.pattern()))
            .expect("temp source file");

        let report =
            WorthValidationAuthoritySourceFirewallReport::scan_root(&root).expect("firewall scan");

        assert_eq!(report.violations().len(), 1);
        assert_eq!(report.violations()[0].pattern(), pattern.pattern());
    }
}

#[test]
fn inventory_backed_firewall_rejects_unreconciled_source() {
    let root = temp_firewall_root("phase_one_inventory_backed_firewall");
    write_file(
        &root
            .join("crates")
            .join("worth-topo")
            .join("src")
            .join("topology_operators")
            .join("fake_blueprint")
            .join("mod.rs"),
        "TopologyValidator::derived_validation_report\n",
    );
    let inventory =
        WorthValidationAuthorityInventory::from_current_sources().expect("inventory should build");

    let report = WorthValidationAuthoritySourceFirewallReport::scan_root_against_inventory(
        &root, &inventory,
    )
    .expect("inventory-backed firewall should scan");

    assert_eq!(report.violations().len(), 1);
    assert_eq!(
        report.violations()[0].pattern(),
        "TopologyValidator::derived_validation_report"
    );
}

#[test]
fn source_firewall_allows_inventory_and_old_authority_files() {
    let root = temp_firewall_root("phase_one_allowed_old_authority");
    write_file(
        &root.join("validation").join("facade.rs"),
        "TopologyValidator::derived_validation_report\n",
    );
    write_file(
        &root
            .join("validation_authority_inventory")
            .join("source_catalog.rs"),
        "DERIVED_TOPOLOGY_RULE_SPECS\n",
    );

    let report =
        WorthValidationAuthoritySourceFirewallReport::scan_root(&root).expect("firewall scan");

    assert!(report.violations().is_empty());
    assert_eq!(report.scanned_file_count(), 2);
}

fn temp_firewall_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "worth_validation_authority_inventory_{name}_{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("clear old temp source root");
    }
    fs::create_dir_all(&root).expect("create temp source root");
    root
}

fn write_file(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().expect("file has parent")).expect("create parent");
    fs::write(path, text).expect("write temp file");
}
