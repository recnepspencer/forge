use crate::classification::{
    build_consolidated_suite_inventory, validate, validate_proof_behavior_authority,
    ClassifiedInventory, ProofBehaviorAuthority,
};
use crate::discovery::{validate_executable_listing, CurrentExecutableListing};
use crate::evidence::read_json;
use crate::preservation::{
    historical_non_case_aggregate_ids, validate_current_reachability, validate_ledger,
    ProofPreservationLedger,
};

use super::{current_inventory, workspace_root};

#[test]
fn discovery_preservation_and_suite_topology_are_complete() {
    let root = workspace_root();
    let current = current_inventory(&root);
    assert!(current
        .inventory()
        .discovered
        .cases
        .iter()
        .all(|case| case.current_invocation != "unregistered"));
    let compile_fail_doctests = current
        .inventory()
        .discovered
        .cases
        .iter()
        .filter(|case| case.kind == crate::discovery::CaseKind::DoctestCompileFail)
        .count();
    let ignored_doctests = current
        .inventory()
        .discovered
        .cases
        .iter()
        .filter(|case| case.kind == crate::discovery::CaseKind::DoctestIgnored)
        .count();
    assert!(compile_fail_doctests > 0);
    assert!(ignored_doctests > 0);
    assert!(
        current
            .inventory()
            .discovered
            .cases
            .iter()
            .filter(|case| case.identity.package == "worth-store-physical-integrity")
            .filter(|case| case.source_path.ends_with("/src/lib.rs"))
            .filter(|case| case.kind == crate::discovery::CaseKind::DoctestCompileFail)
            .count()
            > 20
    );
    assert!(current.inventory().discovered.cases.iter().all(|case| {
        !matches!(
            case.kind,
            crate::discovery::CaseKind::DoctestSurface | crate::discovery::CaseKind::TestExecutable
        )
    }));

    let ledger: ProofPreservationLedger =
        read_json(&root.join("test-control/pre-cleanup/proof-preservation-ledger.json")).unwrap();
    let baseline: ClassifiedInventory =
        read_json(&root.join("test-control/pre-cleanup/classified-proof-inventory.json")).unwrap();
    let historical_non_cases = historical_non_case_aggregate_ids(&baseline);
    let baseline = validate(crate::ClassifiedProofInventory::from_discovered(baseline)).unwrap();
    validate_ledger(&baseline, &ledger).unwrap();
    validate_current_reachability(&ledger, &current, &historical_non_cases).unwrap();

    let suites = build_consolidated_suite_inventory(&root, current.inventory()).unwrap();
    assert_eq!(suites.pre_cleanup_scenario_executables, 93);
    assert_eq!(suites.consolidated_suite_executables, 6);
    assert!(suites
        .suites
        .iter()
        .all(|suite| !suite.scenarios.is_empty()));

    let executable_listing: CurrentExecutableListing =
        read_json(&root.join("test-control/current-executable-listing.json")).unwrap();
    validate_executable_listing(&current.inventory().discovered, &executable_listing).unwrap();
}

#[test]
fn executable_listing_cannot_cross_environment_identity() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut listing: CurrentExecutableListing =
        read_json(&root.join("test-control/current-executable-listing.json")).unwrap();
    listing.environment.operating_system.push_str("-foreign");
    let denials =
        validate_executable_listing(&current.inventory().discovered, &listing).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("different Cargo, Rust, OS, or architecture")));
}

#[test]
fn duplicate_libtest_case_cannot_mask_source_parity() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut listing: CurrentExecutableListing =
        read_json(&root.join("test-control/current-executable-listing.json")).unwrap();
    let target = listing
        .libtest_targets
        .iter_mut()
        .find(|target| !target.listed_cases.is_empty())
        .unwrap();
    target.listed_cases.push(target.listed_cases[0].clone());
    let denials =
        validate_executable_listing(&current.inventory().discovered, &listing).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("libtest/source multiplicity differs")));
}

#[test]
fn executable_and_behavior_authority_schemas_are_enforced() {
    let root = workspace_root();
    let current = current_inventory(&root);
    let mut listing: CurrentExecutableListing =
        read_json(&root.join("test-control/current-executable-listing.json")).unwrap();
    listing.schema_version = 99;
    listing
        .rustdoc_targets
        .first_mut()
        .unwrap()
        .listed_cases
        .push("opaque-rustdoc-case".to_owned());
    let denials =
        validate_executable_listing(&current.inventory().discovered, &listing).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("unsupported current executable listing schema")));
    assert!(denials
        .iter()
        .any(|denial| denial.contains("no parseable source location")));

    let mut authority: ProofBehaviorAuthority =
        read_json(&root.join("test-control/current-proof-behavior-authority.json")).unwrap();
    authority.schema_version = 99;
    let denials = validate_proof_behavior_authority(&authority, current.inventory()).unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("unsupported proof behavior authority schema")));
}

#[test]
fn preservation_denies_destination_and_product_drift() {
    let current = current_inventory(&workspace_root());
    let ledger: ProofPreservationLedger = read_json(
        &workspace_root().join("test-control/pre-cleanup/proof-preservation-ledger.json"),
    )
    .unwrap();
    let mut drifted = current.inventory().clone();
    let proof = drifted
        .proofs
        .iter_mut()
        .find(|proof| {
            proof.disposition == crate::classification::ProofDisposition::PreserveAndConsolidate
        })
        .unwrap();
    proof.case.target_identity = Some("worth-store-certification::test::wrong-suite".to_owned());
    proof.products.insert("store-ci:wrong-partition".to_owned());
    let drifted = crate::ValidatedProofInventory::from_classified(drifted);
    let baseline: ClassifiedInventory = read_json(
        &workspace_root().join("test-control/pre-cleanup/classified-proof-inventory.json"),
    )
    .unwrap();
    let denials = validate_current_reachability(
        &ledger,
        &drifted,
        &historical_non_case_aggregate_ids(&baseline),
    )
    .unwrap_err();
    assert!(denials
        .iter()
        .any(|denial| denial.contains("unadmitted destination")));
    assert!(denials
        .iter()
        .any(|denial| denial.contains("proof-product membership drifted")));
}
