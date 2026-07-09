use worth_query::facade::consumer_kit::{
    load_support_pin_contract_terminal_json_document, project_support_snapshot, support_pinning_contract,
    WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture, WorthQueryRuntimeFacadeFamily,
    WorthQuerySupportPinContractSchemaVersion, WorthQuerySupportPinFindingKind,
};
use worth_query::facade::runtime::{
    WorthQueryRuntimeFamilySupport, WorthQueryRuntimePublicApiContract,
    WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimeSupportProfile,
};

fn main() {
    let profile = WorthQueryRuntimeSupportProfile::scaffold_backend_profile();
    let contract = WorthQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = WorthQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    let snapshot = project_support_snapshot(&matrix);

    let pins = support_pinning_contract("external-consumer")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(WorthQueryRuntimeFacadeFamily::Write, |row| {
            row.status(WorthQueryPinnedSupportStatus::Supported)
                .teaching_posture(WorthQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .observe_family(WorthQueryRuntimeFacadeFamily::BranchPreview)
        .unwrap()
        .seal()
        .unwrap();

    pins.evaluate_snapshot(&snapshot)
        .unwrap()
        .assert_satisfied()
        .unwrap();

    let terminal_json_document = pins.to_canonical_terminal_json_document().unwrap();
    let durable_pins = load_support_pin_contract_terminal_json_document(
        &terminal_json_document.to_external_terminal_json_document(),
        WorthQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap();
    let drifted_profile = WorthQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_family_support(WorthQueryRuntimeFamilySupport::deferred(
            WorthQueryRuntimeFacadeFamily::Write,
            "test drift",
        ));
    let drifted_contract =
        WorthQueryRuntimePublicApiContract::from_support_profile(&drifted_profile);
    let drifted_matrix =
        WorthQueryRuntimePublicSupportMatrix::from_public_api_contract(&drifted_contract);
    let drifted_snapshot = project_support_snapshot(&drifted_matrix);
    let report = durable_pins.evaluate_snapshot(&drifted_snapshot).unwrap();

    assert!(!report.satisfied());
    assert!(report.findings().iter().any(|finding| {
        finding.kind() == WorthQuerySupportPinFindingKind::StatusMismatch
            && finding.family() == Some(WorthQueryRuntimeFacadeFamily::Write)
            && finding.expected() == Some("supported")
            && finding.found() == Some("deferred-debt")
    }));
}
