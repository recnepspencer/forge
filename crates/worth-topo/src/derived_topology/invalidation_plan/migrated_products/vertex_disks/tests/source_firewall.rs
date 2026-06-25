use std::path::Path;

use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::VertexDiskOldAuthorityResidue;

#[test]
fn old_vertex_disk_module_is_deleted_and_not_ordinary_authority() {
    let old_path = Path::new("crates/worth-topo/src/derived_topology/vertex_disks/mod.rs");

    assert!(!old_path.exists());
    assert!(VertexDiskOldAuthorityResidue::current_source_scan()
        .capped_rows()
        .is_empty());
}

#[test]
fn unknown_old_authority_residue_cannot_satisfy_closeout_caps() {
    let residue = VertexDiskOldAuthorityResidue::unknown_old_authority_for_tests();

    assert_eq!(residue.capped_direct_interpreter_count(), 1);
    assert_ne!(
        residue.capped_direct_interpreter_count(),
        VertexDiskOldAuthorityResidue::required_capped_callers().len()
    );
}
