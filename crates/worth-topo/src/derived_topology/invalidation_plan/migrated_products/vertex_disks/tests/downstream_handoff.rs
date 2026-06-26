use super::support::close_vertex_disk_slice_from_topology;
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::covered_sweep::{
    status_rows_from_migrated_family_closeouts, CoveredDerivedProductMigrationStatus,
};

#[test]
fn covered_product_status_consumes_vertex_disk_family_closeout() {
    let closeout = close_vertex_disk_slice_from_topology("vertex-disk-downstream-handoff");
    let rows = status_rows_from_migrated_family_closeouts(
        &[closeout.migrated_family_closeout()],
        closeout.old_authority_residue_digest(),
    );
    let vertex_disk_row = rows
        .iter()
        .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks)
        .expect("covered status should include vertex disks");

    assert_eq!(
        vertex_disk_row.status(),
        CoveredDerivedProductMigrationStatus::Migrated
    );
    assert!(vertex_disk_row.ordinary_invalidation_consumable());
    assert_eq!(
        vertex_disk_row.proof_digest(),
        closeout.migrated_family_closeout().proof_digest()
    );
}
