use forge_query::facade::consumer_kit::{
    load_support_snapshot_document, project_support_snapshot, ForgeQuerySupportSnapshotSchemaVersion,
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
    let json = snapshot.to_canonical_json().unwrap();
    let loaded =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .unwrap();

    loaded.assert_equivalent_to_live_matrix(&matrix).unwrap();
}
