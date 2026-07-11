use crate::{
    certify_store_json_residue_inventory,
    courtroom::foundational::store_json_residue_inventory::StoreJsonResidueInventory,
    courtroom::foundational::store_json_residue_prelude_scan::scan_store_test_prelude_source,
    courtroom::foundational::store_json_residue_scan::scan_source_text, StoreJsonAuthorityRisk,
    StoreJsonResidueClassification, StoreJsonResidueDenial, StoreJsonResidueOccurrence,
    StoreJsonResidueTokenKind, StoreJsonResidueZone,
};

#[test]
fn store_json_residue_inventory_classifies_every_occurrence() {
    let inventory =
        certify_store_json_residue_inventory().expect("current Store residue is classified");

    assert!(inventory.contains_zone(StoreJsonResidueZone::LegacyCompatibilityResidue));
    assert!(inventory.contains_zone(StoreJsonResidueZone::LegacyHostileDenialTest));
    assert!(
        inventory.contains_zone(StoreJsonResidueZone::DedicatedWorkspaceCertificationEnforcement)
    );
    assert!(!inventory.classified().is_empty());

    for classification in inventory.classified() {
        assert!(!classification.owner().is_empty());
        assert!(!classification.quarantine_or_removal_condition().is_empty());
    }

    assert!(inventory
        .dedicated_workspace_classified()
        .all(|classification| {
            classification.is_quarantined_terminal_or_hostile_boundary()
                || classification.is_durable_serde_contract()
        }));
    assert!(inventory.classified().iter().any(|classification| {
        classification.zone() == StoreJsonResidueZone::DedicatedWorkspaceDurableSerdeContract
            && classification.owner() == "forge-store durable compatibility contract"
            && matches!(
                classification.occurrence().token(),
                StoreJsonResidueTokenKind::Serialize | StoreJsonResidueTokenKind::Deserialize
            )
    }));
}

#[test]
fn unclassified_store_json_residue_fails_the_gate() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store/src/native_authority.rs",
        7,
        StoreJsonResidueTokenKind::SerdeJson,
        "use serde_json::Value;",
    );

    let denial = StoreJsonResidueInventory::from_occurrences(vec![occurrence.clone()])
        .expect_err("dedicated workspace production JSON must fail");

    assert_eq!(
        denial,
        StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence)
    );
}

#[test]
fn scanned_dedicated_workspace_production_json_fails_the_gate() {
    let occurrences = scan_source_text(
        "workspaces/forge-store/crates/forge-store/src/native_authority.rs",
        r#"
        use serde_json::Value;
        fn accept_authority(value: Value) {}
        "#,
    );

    let denial = StoreJsonResidueInventory::from_occurrences(occurrences)
        .expect_err("scanned dedicated workspace production JSON must fail");

    assert!(matches!(
        denial,
        StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(_)
    ));
}

#[test]
fn terminal_projection_named_production_json_still_fails_until_exactly_readmitted() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store/src/terminal_json_projection_authority.rs",
        11,
        StoreJsonResidueTokenKind::SerdeJson,
        "let authority = serde_json::to_vec(record)?;",
    );

    let denial = StoreJsonResidueInventory::from_occurrences(vec![occurrence.clone()])
        .expect_err("terminal-looking production JSON must not pass by filename");

    assert_eq!(
        denial,
        StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence)
    );
}

#[test]
fn readmission_named_production_json_still_fails_until_exactly_readmitted() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store-authority/src/json_ingress_readmission.rs",
        19,
        StoreJsonResidueTokenKind::DeserializeOwned,
        "fn load<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> T;",
    );

    let denial = StoreJsonResidueInventory::from_occurrences(vec![occurrence.clone()])
        .expect_err("readmission-looking production JSON must not pass by filename");

    assert_eq!(
        denial,
        StoreJsonResidueDenial::ForbiddenDedicatedWorkspaceProduction(occurrence)
    );
}

#[test]
fn exact_terminal_projection_json_home_is_quarantined() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store-aspect-native/src/terminal_json_projection.rs",
        9,
        StoreJsonResidueTokenKind::SerdeJson,
        "use serde_json::Value;",
    );

    let inventory = StoreJsonResidueInventory::from_occurrences(vec![occurrence])
        .expect("exact terminal projection home is classified");
    let classification = inventory.classified().first().unwrap();

    assert_eq!(
        classification.zone(),
        StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary
    );
    assert_eq!(
        classification.authority_risk(),
        StoreJsonAuthorityRisk::TerminalProjectionOnly
    );
}

#[test]
fn exact_json_readmission_home_is_quarantined() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store-aspect-native/src/json_ingress_readmission.rs",
        11,
        StoreJsonResidueTokenKind::SerdeJson,
        "let input = compatibility().json().input(contract, source, value);",
    );

    let inventory = StoreJsonResidueInventory::from_occurrences(vec![occurrence])
        .expect("exact JSON ingress readmission home is classified");
    let classification = inventory.classified().first().unwrap();

    assert_eq!(
        classification.zone(),
        StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission
    );
    assert_eq!(
        classification.authority_risk(),
        StoreJsonAuthorityRisk::HostileReadmissionOnly
    );
}

#[test]
fn exact_hostile_readmission_json_certification_home_is_quarantined() {
    let occurrence = StoreJsonResidueOccurrence::new(
        "workspaces/forge-store/crates/forge-store-certification/src/scenario/foundational/hostile_readmission_json_fixture_boundary_tests.rs",
        19,
        StoreJsonResidueTokenKind::SerdeJson,
        "let attacker = serde_json::json!({});",
    );

    let inventory = StoreJsonResidueInventory::from_occurrences(vec![occurrence])
        .expect("hostile readmission JSON certification home is classified");
    let classification = inventory.classified().first().unwrap();

    assert_eq!(
        classification.zone(),
        StoreJsonResidueZone::DedicatedWorkspaceHostileReadmission
    );
    assert_eq!(
        classification.authority_risk(),
        StoreJsonAuthorityRisk::HostileReadmissionOnly
    );
}

#[test]
fn ordinary_store_tests_do_not_import_json_preludes() {
    certify_store_json_residue_inventory().expect("ordinary preludes are JSON-free");

    let denial = scan_store_test_prelude_source(
        "workspaces/forge-store/crates/forge-store-test-support/src/lib.rs",
        "pub use serde_json::{json, Value};",
    )
    .expect_err("ordinary prelude JSON exports must fail");

    assert!(matches!(
        denial,
        StoreJsonResidueDenial::OrdinaryPreludeJsonExport(_)
    ));
}

#[test]
fn scanner_ignores_byte_vector_and_display_noise() {
    let occurrences = scan_source_text(
        "workspaces/forge-store/crates/forge-store-certification/src/bytes.rs",
        r#"
        let bytes = body.to_vec();
        let label = digest.to_string();
        let posture = ChecksumCoverageEncoding::SerializedBytes;
        "#,
    );

    assert!(occurrences.is_empty());
}

#[test]
fn scanner_detects_serde_and_raw_json_helpers() {
    let occurrences = scan_source_text(
        "crates/forge-store/src/backend/sqlite/helpers.rs",
        r#"
        use serde::{Deserialize, Serialize};
        let payload_json = serde_json::to_string(record)?;
        fn deserialize_json<T: serde::de::DeserializeOwned>() {}
        "#,
    );

    assert!(occurrences
        .iter()
        .any(|occurrence| occurrence.token() == StoreJsonResidueTokenKind::Serialize));
    assert!(occurrences
        .iter()
        .any(|occurrence| occurrence.token() == StoreJsonResidueTokenKind::Deserialize));
    assert!(occurrences
        .iter()
        .any(|occurrence| occurrence.token() == StoreJsonResidueTokenKind::SerdeJson));
    assert!(occurrences
        .iter()
        .any(|occurrence| occurrence.token() == StoreJsonResidueTokenKind::RawJsonHelper));
}

#[test]
fn scanner_detects_legacy_canonical_json_helper_names() {
    let occurrences = scan_source_text(
        "crates/forge-store/src/evidence/helpers.rs",
        r#"
        pub fn canonical_json(&self) -> String;
        pub fn semantic_json(&self) -> String;
        pub(super) fn stable_json_digest<T>(value: &T) -> String;
        pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, Error>;
        pub fn validate_canonical_json_bytes(bytes: &[u8]) -> Result<Self, Error>;
        "#,
    );

    let raw_helper_count = occurrences
        .iter()
        .filter(|occurrence| occurrence.token() == StoreJsonResidueTokenKind::RawJsonHelper)
        .count();

    assert_eq!(raw_helper_count, 5);
}

#[test]
fn current_inventory_classifies_real_legacy_json_digest_surfaces() {
    let inventory =
        certify_store_json_residue_inventory().expect("current Store residue is classified");

    assert_real_legacy_json_digest_surface(
        &inventory,
        "crates/forge-store/src/authority/export.rs",
        "canonical_json",
        "legacy store semantic durability crate",
    );
    assert_real_legacy_json_digest_surface(
        &inventory,
        "crates/forge-store/src/bulk/planning/utils.rs",
        "stable_json_digest",
        "legacy store semantic durability crate",
    );
    assert_real_legacy_json_digest_surface(
        &inventory,
        "crates/forge-store/src/storage_foundation/s0/migration.rs",
        "to_canonical_json_bytes",
        "legacy S0 compatibility residue",
    );
}

fn assert_real_legacy_json_digest_surface(
    inventory: &StoreJsonResidueInventory,
    path: &str,
    excerpt_fragment: &str,
    owner: &str,
) {
    let classification = find_raw_json_helper_classification(inventory, path, excerpt_fragment);
    assert_eq!(
        classification.zone(),
        StoreJsonResidueZone::LegacyCompatibilityResidue
    );
    assert_eq!(classification.owner(), owner);
    assert_eq!(
        classification.authority_risk(),
        StoreJsonAuthorityRisk::LegacyDigestBasisResidue
    );
    assert_eq!(
        classification.quarantine_or_removal_condition(),
        "legacy root crate compatibility residue; remove or readmit through native Store before Roadmap 2 use"
    );
}

fn find_raw_json_helper_classification<'a>(
    inventory: &'a StoreJsonResidueInventory,
    path: &str,
    excerpt_fragment: &str,
) -> &'a StoreJsonResidueClassification {
    inventory
        .classified()
        .iter()
        .find(|classification| {
            let occurrence = classification.occurrence();
            occurrence.path() == path
                && occurrence.token() == StoreJsonResidueTokenKind::RawJsonHelper
                && occurrence.excerpt().contains(excerpt_fragment)
        })
        .expect("real legacy JSON helper surface should be inventoried")
}
