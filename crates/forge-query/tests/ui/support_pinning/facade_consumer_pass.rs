use forge_query::facade::consumer_kit::{
    load_support_pin_contract_terminal_json_document, project_support_snapshot, support_pinning_contract,
    ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture, ForgeQueryRuntimeFacadeFamily,
    ForgeQuerySupportPinContractSchemaVersion, ForgeQuerySupportPinFindingKind,
};
use forge_query::facade::runtime::{
    ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile,
};

fn main() {
    let profile = ForgeQueryRuntimeSupportProfile::scaffold_backend_profile();
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    let snapshot = project_support_snapshot(&matrix);

    let pins = support_pinning_contract("external-consumer")
        .against_snapshot(&snapshot)
        .unwrap()
        .require_family(ForgeQueryRuntimeFacadeFamily::Write, |row| {
            row.status(ForgeQueryPinnedSupportStatus::Supported)
                .teaching_posture(ForgeQueryPinnedTeachingPosture::OrdinaryRuntimeDx)
                .bind_live_row_digest()
        })
        .unwrap()
        .observe_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
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
        ForgeQuerySupportPinContractSchemaVersion::current(),
    )
    .unwrap();
    let drifted_profile = ForgeQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_family_support(ForgeQueryRuntimeFamilySupport::deferred(
            ForgeQueryRuntimeFacadeFamily::Write,
            "test drift",
        ));
    let drifted_contract =
        ForgeQueryRuntimePublicApiContract::from_support_profile(&drifted_profile);
    let drifted_matrix =
        ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&drifted_contract);
    let drifted_snapshot = project_support_snapshot(&drifted_matrix);
    let report = durable_pins.evaluate_snapshot(&drifted_snapshot).unwrap();

    assert!(!report.satisfied());
    assert!(report.findings().iter().any(|finding| {
        finding.kind() == ForgeQuerySupportPinFindingKind::StatusMismatch
            && finding.family() == Some(ForgeQueryRuntimeFacadeFamily::Write)
            && finding.expected() == Some("supported")
            && finding.found() == Some("deferred-debt")
    }));
}
