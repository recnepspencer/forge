use crate::facade;

use super::support_snapshot_workspace;

#[test]
fn public_consumer_kit_facade_projects_exports_loads_and_compares_snapshot() {
    let workspace = support_snapshot_workspace();
    let matrix = workspace.public_support_matrix();

    let snapshot = facade::consumer_kit::project_workspace_support_snapshot(&workspace);
    let json = snapshot
        .to_canonical_json()
        .expect("facade snapshot export should encode");
    let loaded = facade::consumer_kit::load_support_snapshot_document(
        &json,
        facade::consumer_kit::ForgeQuerySupportSnapshotSchemaVersion::current(),
    )
    .expect("facade loader should accept current support snapshot schema");

    loaded
        .assert_equivalent_to_live_matrix(&matrix)
        .expect("facade-loaded snapshot should compare to live support matrix");
}
