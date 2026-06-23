use forge_query::facade::consumer_kit::{
    load_support_snapshot_terminal_json_document, project_support_snapshot, ForgeQuerySupportSnapshotSchemaVersion,
};
use forge_query::facade::runtime::{
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicSupportMatrix, ForgeQueryRuntimeSupportProfile,
};

fn main() {
    let profile = ForgeQueryRuntimeSupportProfile::scaffold_backend_profile()
        .with_posture(ForgeQueryRuntimeBackendPosture::Primary);
    let contract = ForgeQueryRuntimePublicApiContract::from_support_profile(&profile);
    let matrix = ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&contract);
    let snapshot = project_support_snapshot(&matrix);
    let terminal_json_document = snapshot.to_canonical_terminal_json_document().unwrap();
    let loaded = load_support_snapshot_terminal_json_document(
        &terminal_json_document.to_external_terminal_json_document(),
        ForgeQuerySupportSnapshotSchemaVersion::current(),
    )
    .unwrap();

    loaded.assert_equivalent_to_live_matrix(&matrix).unwrap();
}
