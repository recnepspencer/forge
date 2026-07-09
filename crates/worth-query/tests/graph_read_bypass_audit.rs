use worth_query::facade::consumer_kit::{
    graph_read_bypass_adoption, graph_read_bypass_audit, WorthQueryBoundaryAuditSourceSet,
    WorthQueryGraphReadBypassAdoptionErrorKind, WorthQueryGraphReadBypassClass,
    WorthQueryGraphReadBypassResidueErrorKind, WorthQueryGraphReadBypassResidueManifest,
    WorthQueryGraphReadBypassResidueRow,
};

#[test]
fn facade_graph_read_bypass_audit_detects_and_accounts_for_findings() {
    let source = r#"
fn topology_walk(relation_rows: &[Row]) {
    for relation in relation_rows.iter() {}
    let mut adjacency = BTreeMap::new();
    adjacency.insert(source, target);
}
"#;
    let report = graph_read_bypass_audit("facade-consumer")
        .required_sources(
            WorthQueryBoundaryAuditSourceSet::new("facade-consumer").source_file(
                "topology.walk",
                "src/topology.rs",
                source,
            ),
        )
        .evaluate()
        .expect("graph-read bypass audit should evaluate");

    assert_eq!(report.counters().evaluated_source_count(), 1);
    assert!(report.source_inventory_identities().is_empty());
    assert!(
        report.finding_count_for_class(WorthQueryGraphReadBypassClass::ManualRelationRowLoop) > 0
    );
    assert!(report.finding_count_for_class(WorthQueryGraphReadBypassClass::AdHocAdjacencyMap) > 0);
    assert!(report
        .findings()
        .iter()
        .all(|finding| finding.authority_violation().is_graph_read_access_bypass()));

    let residue = WorthQueryGraphReadBypassResidueManifest::capped([
        WorthQueryGraphReadBypassResidueRow::explicit(
            WorthQueryGraphReadBypassClass::ManualRelationRowLoop,
            "worth-topo Phase 16 adoption",
            "Milestone 9.10 Phase 15",
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::ManualRelationRowLoop),
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::ManualRelationRowLoop),
            "Phase 16 migrates relation row loop to graph-read access planning",
            "graph-read access receipt replaces local relation loop",
        )
        .expect("relation-loop residue row should admit"),
        WorthQueryGraphReadBypassResidueRow::explicit(
            WorthQueryGraphReadBypassClass::AdHocAdjacencyMap,
            "worth-topo Phase 16 adoption",
            "Milestone 9.10 Phase 15",
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::AdHocAdjacencyMap),
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::AdHocAdjacencyMap),
            "Phase 16 migrates adjacency map to graph-read access planning",
            "graph-read access receipt replaces local adjacency map",
        )
        .expect("adjacency-map residue row should admit"),
    ])
    .expect("residue manifest should admit");

    let certification = report
        .certify_with_residue(&residue)
        .expect("complete residue manifest should certify report findings");
    assert_eq!(
        certification.certified_finding_count(),
        report.findings().len()
    );
    assert_eq!(
        certification.residue_manifest_digest(),
        residue.manifest_digest()
    );
    assert_eq!(certification.report_identity(), report.report_identity());
    assert_eq!(
        certification.certification_identity().scope().as_str(),
        "consumer-graph-read-bypass-residue"
    );

    let incomplete_residue = WorthQueryGraphReadBypassResidueManifest::capped([
        WorthQueryGraphReadBypassResidueRow::explicit(
            WorthQueryGraphReadBypassClass::ManualRelationRowLoop,
            "worth-topo Phase 16 adoption",
            "Milestone 9.10 Phase 15",
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::ManualRelationRowLoop),
            report.finding_count_for_class(WorthQueryGraphReadBypassClass::ManualRelationRowLoop),
            "Phase 16 migrates relation row loop to graph-read access planning",
            "graph-read access receipt replaces local relation loop",
        )
        .expect("relation-loop residue row should admit"),
    ])
    .expect("incomplete residue shape should admit");
    let coverage_error = report
        .certify_with_residue(&incomplete_residue)
        .expect_err("missing adjacency-map residue must fail report certification");
    assert_eq!(
        coverage_error.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::ResidueCoverageShortfall
    );

    let synthetic_adoption_error = graph_read_bypass_adoption("facade-consumer")
        .audit_report(report)
        .residue_manifest(residue)
        .certify()
        .expect_err("synthetic source-set report must not certify as reference adoption");
    assert_eq!(
        synthetic_adoption_error.kind(),
        WorthQueryGraphReadBypassAdoptionErrorKind::MissingSourceInventoryProof
    );
}

#[test]
fn graph_read_bypass_audit_rejects_missing_source_sets() {
    let error = graph_read_bypass_audit("facade-consumer")
        .evaluate()
        .expect_err("audit without sources must fail closed");

    assert_eq!(
        error.kind(),
        worth_query::facade::consumer_kit::WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot
    );
}
