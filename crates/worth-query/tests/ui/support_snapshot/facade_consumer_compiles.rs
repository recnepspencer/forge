use worth_query::facade::consumer_kit::{
    load_support_snapshot_terminal_json_document, project_support_snapshot, WorthQuerySupportSnapshotSchemaVersion,
};
use worth_query::facade::runtime::{
    WorthQueryRuntimeBackendPosture, WorthQueryRuntimePublicApiContract,
    WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimeSupportProfile,
};

fn main() {
    let profile = WorthQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_posture(WorthQueryRuntimeBackendPosture::Primary);
    let contract = WorthQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = WorthQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    let snapshot = project_support_snapshot(&matrix);
    let terminal_json_document = snapshot.to_canonical_terminal_json_document().unwrap();
    let loaded = load_support_snapshot_terminal_json_document(
        &terminal_json_document.to_external_terminal_json_document(),
        WorthQuerySupportSnapshotSchemaVersion::current(),
    )
    .unwrap();

    loaded.assert_equivalent_to_live_matrix(&matrix).unwrap();
}
