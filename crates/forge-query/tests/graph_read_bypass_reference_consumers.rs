use std::path::{Path, PathBuf};

use forge_query::facade::consumer_kit::{
    graph_read_bypass_adoption, graph_read_bypass_audit, query_boundary_source_inventory,
    ForgeQueryBoundaryAuditSourceInventory, ForgeQueryGraphReadBypassClass,
    ForgeQueryGraphReadBypassResidueManifest, ForgeQueryGraphReadBypassResidueRow,
};

#[test]
fn worth_topo_real_inventory_classifies_graph_read_bypass_residue() {
    let inventory = real_worth_topo_split_support_inventory();
    let report = graph_read_bypass_audit("worth-topo")
        .required_inventory(&inventory)
        .evaluate()
        .expect("worth-topo graph-read bypass audit should evaluate");

    assert_eq!(report.source_inventory_identities().len(), 1);
    assert_eq!(
        report.counters().evaluated_source_count(),
        inventory.source_count()
    );
    assert_eq!(report.findings().len(), 13);

    let adoption = graph_read_bypass_adoption("worth-topo")
        .audit_report(report)
        .residue_manifest(worth_topo_split_support_residue_manifest())
        .certify()
        .expect("worth-topo graph-read bypass residue must classify every covered finding");

    assert!(adoption.has_no_unclassified_findings());
    assert_eq!(adoption.unclassified_finding_count(), 0);
    assert_eq!(
        adoption.manifest().residue_manifest_digest(),
        adoption.residue_manifest().manifest_digest()
    );
}

#[test]
fn worth_kernel_real_inventory_classifies_graph_read_bypass_residue() {
    let inventory = real_worth_kernel_source_inventory();
    let report = graph_read_bypass_audit("worth-kernel")
        .required_inventory(&inventory)
        .evaluate()
        .expect("worth-kernel graph-read bypass audit should evaluate");

    assert_eq!(report.source_inventory_identities().len(), 1);
    assert_eq!(
        report.counters().evaluated_source_count(),
        inventory.source_count()
    );
    assert_eq!(report.findings().len(), 4);
    assert_eq!(
        report.finding_count_for_class(ForgeQueryGraphReadBypassClass::ManualRelationRowLoop),
        4
    );

    let adoption = graph_read_bypass_adoption("worth-kernel")
        .audit_report(report)
        .residue_manifest(worth_kernel_residue_manifest())
        .certify()
        .expect("worth-kernel graph-read bypass residue must classify every covered finding");

    assert!(adoption.has_no_unclassified_findings());
}

fn worth_topo_split_support_residue_manifest() -> ForgeQueryGraphReadBypassResidueManifest {
    ForgeQueryGraphReadBypassResidueManifest::capped([
        residue_row(ForgeQueryGraphReadBypassClass::ManualRelationRowLoop, 4),
        residue_row(ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup, 2),
        residue_row(ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap, 2),
        residue_row(ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal, 2),
        residue_row(ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan, 3),
    ])
    .expect("worth-topo split support residue manifest should be explicit and capped")
}

fn residue_row(
    class: ForgeQueryGraphReadBypassClass,
    count: usize,
) -> ForgeQueryGraphReadBypassResidueRow {
    ForgeQueryGraphReadBypassResidueRow::explicit(
        class,
        "worth-topo Phase 16 graph-read adoption",
        "Milestone 9.10 Phase 15",
        count,
        count,
        "Phase 16 migrates declaration-entry split support graph reads into Query access planning",
        "covered split support reads emit graph-read access receipts",
    )
    .expect("residue row should be complete")
}

fn worth_kernel_residue_manifest() -> ForgeQueryGraphReadBypassResidueManifest {
    ForgeQueryGraphReadBypassResidueManifest::capped([
        ForgeQueryGraphReadBypassResidueRow::explicit(
            ForgeQueryGraphReadBypassClass::ManualRelationRowLoop,
            "worth-kernel Phase 16 graph-read adoption",
            "Milestone 9.10 Phase 15",
            4,
            4,
            "Phase 16 migrates public contract relation assertions into Query access planning",
            "covered worth-kernel relation assertions emit graph-read access receipts",
        )
        .expect("worth-kernel residue row should be complete"),
    ])
    .expect("worth-kernel residue manifest should be explicit and capped")
}

fn real_worth_topo_split_support_inventory() -> ForgeQueryBoundaryAuditSourceInventory {
    query_boundary_source_inventory("worth-topo")
        .required_root(workspace_crates_dir().join(
            "worth-topo/src/certification/projection_closeout/tests/topology_reads/declaration_entry/split",
        ))
        .include_rs_files()
        .seal()
        .expect("worth-topo split support source inventory should be discoverable")
}

fn real_worth_kernel_source_inventory() -> ForgeQueryBoundaryAuditSourceInventory {
    query_boundary_source_inventory("worth-kernel")
        .required_root(workspace_crates_dir().join("worth-kernel/src"))
        .include_rs_files()
        .seal()
        .expect("worth-kernel source inventory should be discoverable")
}

fn workspace_crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("forge-query crate should live under crates")
        .to_path_buf()
}
