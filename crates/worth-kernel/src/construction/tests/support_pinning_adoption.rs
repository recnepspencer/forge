const AUTHORING_RS: &str = include_str!("../authoring.rs");
const QUERY_SUPPORT_PINS_JSON: &str = include_str!("../query_support_pins.json");

use forge_query::facade::consumer_kit::{
    load_support_pin_contract_document, ForgeQueryPinnedSupportStatus,
    ForgeQueryPinnedTeachingPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQuerySupportPinContractSchemaVersion, ForgeQuerySupportPinningErrorKind,
};

use crate::construction::query_support_pins::{
    primitive_construction_query_support_pin_adoption_evidence,
    primitive_construction_query_support_pins,
};

#[test]
fn construction_authoring_loads_durable_query_owned_support_pins() {
    let adoption = primitive_construction_query_support_pin_adoption_evidence()
        .expect("support pin adoption evidence should load");
    let contract =
        primitive_construction_query_support_pins().expect("checked-in support pins load");

    assert_eq!(adoption.consumer_name(), "worth-kernel");
    assert_eq!(
        adoption.loaded_contract_digest(),
        contract.contract_digest()
    );
    assert_eq!(adoption.evaluated_requirement_count(), 2);
    assert_eq!(adoption.observed_row_count(), 0);
    assert_eq!(
        adoption.schema_version(),
        ForgeQuerySupportPinContractSchemaVersion::current()
    );
    assert!(AUTHORING_RS.contains("project_workspace_support_snapshot("));
    assert!(!AUTHORING_RS.contains("REQUIRED_QUERY_FAMILIES"));
    assert!(!AUTHORING_RS.contains("REPORTED_QUERY_FAMILIES"));
    assert!(!AUTHORING_RS.contains("PrimitiveConstructionQueryGapRow"));
    assert!(!AUTHORING_RS.contains("support_pinning_contract(\"worth-kernel\")"));
}

#[test]
fn checked_in_support_pin_contract_has_typed_worth_kernel_requirements() {
    let contract =
        primitive_construction_query_support_pins().expect("checked-in support pins load");

    assert_eq!(contract.consumer_name(), "worth-kernel");
    assert_eq!(contract.requirements().len(), 2);
    assert!(contract.observed_rows().is_empty());
    assert_required_family(&contract, ForgeQueryRuntimeFacadeFamily::Write);
    assert_required_family(&contract, ForgeQueryRuntimeFacadeFamily::Inspect);
}

#[test]
fn checked_in_support_pin_contract_tampering_fails_typed_at_load() {
    let tampered_json = QUERY_SUPPORT_PINS_JSON.replacen(
        "\"required_status\": \"supported\"",
        "\"required_status\": \"unsupported\"",
        1,
    );

    let error = load_support_pin_contract_document(
        &tampered_json,
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .expect_err("tampered checked-in support pin document must fail digest validation");

    assert_eq!(
        error.kind(),
        ForgeQuerySupportPinningErrorKind::ContractDigestMismatch
    );
}

fn assert_required_family(
    contract: &forge_query::facade::consumer_kit::ForgeQuerySupportPinContract,
    family: ForgeQueryRuntimeFacadeFamily,
) {
    let requirement = contract
        .requirements()
        .iter()
        .find(|requirement| requirement.family() == family)
        .expect("required family should be present");

    assert_eq!(
        requirement.required_status(),
        ForgeQueryPinnedSupportStatus::Supported
    );
    assert_eq!(
        requirement.required_teaching_posture(),
        ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx
    );
    assert!(!requirement.pinned_live_row_digest().is_empty());
    assert!(!requirement.pinned_snapshot_row_digest().is_empty());
}
