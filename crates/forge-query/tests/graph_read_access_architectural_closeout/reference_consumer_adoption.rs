use std::path::{Path, PathBuf};

use forge_query::facade::consumer_kit::{
    graph_read_bypass_adoption, graph_read_bypass_audit, query_boundary_source_inventory,
    ForgeQueryBoundaryAuditSourceInventory, ForgeQueryGraphReadBypassClass,
    ForgeQueryGraphReadBypassResidueManifest, ForgeQueryGraphReadBypassResidueRow,
};

#[test]
fn closeout_worth_kernel_reference_adoption_has_no_unclassified_bypass_findings() {
    assert_reference_consumer_adoption_is_classified(
        "worth-kernel-closeout",
        worth_kernel_source_inventory(),
        worth_kernel_residue_manifest(),
        &[(ForgeQueryGraphReadBypassClass::ManualRelationRowLoop, 4)],
    );
}

#[test]
fn closeout_worth_topo_reference_adoption_has_no_unclassified_bypass_findings() {
    assert_reference_consumer_adoption_is_classified(
        "worth-topo-closeout",
        worth_topo_split_support_inventory(),
        worth_topo_split_support_residue_manifest(),
        &[
            (ForgeQueryGraphReadBypassClass::ManualRelationRowLoop, 4),
            (ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup, 2),
            (ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap, 2),
            (ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal, 2),
            (ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan, 3),
        ],
    );
}

fn assert_reference_consumer_adoption_is_classified(
    audit_label: &str,
    inventory: ForgeQueryBoundaryAuditSourceInventory,
    manifest: ForgeQueryGraphReadBypassResidueManifest,
    expected_class_counts: &[(ForgeQueryGraphReadBypassClass, usize)],
) {
    let report = graph_read_bypass_audit(audit_label)
        .required_inventory(&inventory)
        .evaluate()
        .expect("reference-consumer bypass audit should evaluate real inventory");
    let evaluated_source_count = report.counters().evaluated_source_count();
    let finding_count = report.counters().finding_count();
    let expected_finding_count = expected_class_counts
        .iter()
        .map(|(_, count)| *count)
        .sum::<usize>();
    let report_identity = report.report_identity().clone();
    let residue_manifest_digest = manifest.manifest_digest().to_string();

    assert!(evaluated_source_count > 0);
    assert_eq!(finding_count, expected_finding_count);
    assert_eq!(report.audited_source_labels().len(), evaluated_source_count);
    assert!(!report.source_inventory_identities().is_empty());
    for (class, expected_count) in expected_class_counts {
        assert_eq!(
            report.finding_count_for_class(*class),
            *expected_count,
            "{} finding count should match closeout residue",
            class.as_str()
        );
    }

    let adoption = graph_read_bypass_adoption(audit_label)
        .audit_report(report)
        .residue_manifest(manifest)
        .certify()
        .expect("reference-consumer residue manifest should classify covered findings");

    assert!(adoption.has_no_unclassified_findings());
    assert_eq!(adoption.unclassified_finding_count(), 0);
    assert_eq!(adoption.report().report_identity(), &report_identity);
    assert_eq!(
        adoption.report().counters().evaluated_source_count(),
        evaluated_source_count
    );
    assert_eq!(adoption.report().counters().finding_count(), finding_count);
    assert_eq!(
        adoption.manifest().residue_manifest_digest(),
        residue_manifest_digest
    );
    assert_eq!(
        adoption.residue_manifest().manifest_digest(),
        residue_manifest_digest
    );
    assert_eq!(
        adoption.residue_certification().residue_manifest_digest(),
        residue_manifest_digest
    );
    assert_eq!(
        adoption.residue_certification().certified_finding_count(),
        finding_count
    );
    assert!(!adoption.manifest().manifest_digest().is_empty());
}

fn worth_kernel_source_inventory() -> ForgeQueryBoundaryAuditSourceInventory {
    query_boundary_source_inventory("worth-kernel-closeout")
        .required_root(workspace_crates_dir().join("worth-kernel/src"))
        .include_rs_files()
        .seal()
        .expect("worth-kernel source inventory should seal")
}

fn worth_topo_split_support_inventory() -> ForgeQueryBoundaryAuditSourceInventory {
    query_boundary_source_inventory("worth-topo-closeout")
        .required_root(workspace_crates_dir().join(
            "worth-topo/src/certification/projection_closeout/tests/topology_reads/declaration_entry/split",
        ))
        .include_rs_files()
        .seal()
        .expect("worth-topo split support source inventory should seal")
}

fn worth_topo_split_support_residue_manifest() -> ForgeQueryGraphReadBypassResidueManifest {
    ForgeQueryGraphReadBypassResidueManifest::capped([
        worth_topo_residue_row(ForgeQueryGraphReadBypassClass::ManualRelationRowLoop, 4),
        worth_topo_residue_row(ForgeQueryGraphReadBypassClass::PerNodeNeighborLookup, 2),
        worth_topo_residue_row(ForgeQueryGraphReadBypassClass::AdHocAdjacencyMap, 2),
        worth_topo_residue_row(ForgeQueryGraphReadBypassClass::ManualVisitedSetTraversal, 2),
        worth_topo_residue_row(ForgeQueryGraphReadBypassClass::BroadBooleanGraphScan, 3),
    ])
    .expect("worth-topo residue manifest should admit")
}

fn worth_topo_residue_row(
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
    .expect("worth-topo residue row should admit")
}

fn worth_kernel_residue_manifest() -> ForgeQueryGraphReadBypassResidueManifest {
    ForgeQueryGraphReadBypassResidueManifest::capped([
        ForgeQueryGraphReadBypassResidueRow::explicit(
            ForgeQueryGraphReadBypassClass::ManualRelationRowLoop,
            "worth-kernel Phase 19 graph-read closeout",
            "Milestone 9.10 Phase 19",
            4,
            4,
            "Phase 19 keeps remaining kernel relation assertions classified as capped residue",
            "covered runtime graph reads emit access-plan receipts",
        )
        .expect("worth-kernel residue row should admit"),
    ])
    .expect("worth-kernel residue manifest should admit")
}

fn workspace_crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("forge-query crate should live under crates")
        .to_path_buf()
}
