use crate::workload_platform::spatial_compiled_product_consumer_cutover::{
    current_displaced_evidence_index_helper_surface_inventory,
    current_spatial_consumer_residue_manifest, require_exact_spatial_consumer_closeout,
};

use super::super::{
    helper_surface_inventory::DisplacedEvidenceIndexHelperSurfaceDisposition,
    residue_manifest::SpatialConsumerResidueDisposition,
};

#[test]
fn spatial_residue_rows_are_exact_and_non_authoritative() {
    require_exact_spatial_consumer_closeout();
    let residue = current_spatial_consumer_residue_manifest();

    assert_eq!(residue.len(), 1);
    assert_eq!(
        residue[0].source_path(),
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/current.rs"
    );
    assert_eq!(
        residue[0].current_surface(),
        "current_evidence_lookup_public_closeout_assembly_input"
    );
    assert_eq!(
        residue[0].disposition(),
        SpatialConsumerResidueDisposition::CertificationOnly
    );
    assert!(residue.iter().all(|row| !row.blocker().is_empty()));
    assert!(residue.iter().all(|row| !row.removal_trigger().is_empty()));

    assert!(
        !std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/workload_platform/evidence_lookup_public_closeout/current_source.rs"
        ))
        .exists(),
        "phase 8 removes the displaced public-closeout current_source entrypoint",
    );
    let public_closeout_route_source =
        include_str!("../../planner_owned_routing/public_closeout_route/current.rs");
    assert!(public_closeout_route_source.contains("current_evidence_lookup_public_closeout"));
    assert!(public_closeout_route_source
        .contains("current_evidence_lookup_public_closeout_assembly_input"));
    assert!(
        !include_str!("../../../facade/evidence_lookup_index_product/mod.rs")
            .contains("reuse_evidence_lookup_index_product"),
        "displaced evidence-index reuse helper must not survive on the ordinary public facade",
    );
    assert!(
        !include_str!("../../evidence_lookup_index_product/mod.rs")
            .contains("reuse_evidence_lookup_index_product"),
        "displaced evidence-index helper module must not re-export ordinary reuse authority",
    );

    let helper_inventory = current_displaced_evidence_index_helper_surface_inventory()
        .expect("phase-12 helper inventory scan should succeed");
    assert!(
        helper_inventory.ordinary_caller_violations().is_empty(),
        "phase-12 residue proof requires the full ordinary caller scope to stay free of displaced evidence-index helper authority",
    );

    let surviving_helper_surfaces: Vec<_> = helper_inventory
        .rows()
        .iter()
        .map(|row| (row.source_path(), row.mention_count(), row.disposition()))
        .collect();
    assert_eq!(
        surviving_helper_surfaces,
        vec![
            (
                "crates/worth-kernel/src/workload_composition/compiled_product_consumer_cutover/vertical_slice/lookup_consumed/reuse_resolution.rs",
                3,
                DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority,
            ),
            (
                "crates/worth-kernel/src/workload_composition/compiled_product_reuse_inventory/catalog_spatial.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::InventorySupport,
            ),
            (
                "crates/worth-kernel/src/workload_composition/compiled_product_reuse_inventory/source_scan/matcher.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::InventorySupport,
            ),
            (
                "crates/worth-spatial/src/facade/spatial_compiled_product_consumer_cutover/mod.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority,
            ),
            (
                "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/mod.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority,
            ),
            (
                "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/spatial_consumer_cluster/evidence_index_lowering.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority,
            ),
            (
                "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/spatial_consumer_cluster/mod.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::CutoverAuthority,
            ),
            (
                "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/tests.rs",
                5,
                DisplacedEvidenceIndexHelperSurfaceDisposition::TestSupport,
            ),
            (
                "crates/worth-spatial/src/workload_platform/spatial_compiled_product_consumer_cutover/tests/residue_tests.rs",
                1,
                DisplacedEvidenceIndexHelperSurfaceDisposition::TestSupport,
            ),
        ],
    );
}
