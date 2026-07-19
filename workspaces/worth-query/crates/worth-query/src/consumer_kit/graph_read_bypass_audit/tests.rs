use crate::facade::consumer_kit::{
    graph_read_bypass_audit, WorthQueryBoundaryAuditSourceSet, WorthQueryGraphReadBypassClass,
    WorthQueryGraphReadBypassResidueErrorKind, WorthQueryGraphReadBypassResidueManifest,
    WorthQueryGraphReadBypassResidueRow,
};

#[test]
fn seeded_graph_read_bypass_patterns_are_classified() {
    let source = r#"
fn bypass(relation_rows: &[Row]) {
    for relation in relation_rows.iter() {}
    let mut adjacency = BTreeMap::new();
    adjacency.insert(source, target);
    while let Some(next) = frontier.pop() {}
    if visited.contains(&next) {}
    let target = relation_rows.iter().find(|neighbor| true);
    let broad = relation_rows.iter().filter(|row| row_matches_source_kind(row));
    let graph_cache = build_cache();
    fallback_graph_read();
    prepare_read_execution_binding();
    claim_production_graph_read_proof_for_test();
}
"#;
    let report = graph_read_bypass_audit("seeded")
        .required_sources(WorthQueryBoundaryAuditSourceSet::new("seeded").source("lib", source))
        .evaluate()
        .expect("seeded graph-read bypass audit should evaluate");

    for class in [
        WorthQueryGraphReadBypassClass::ManualRelationRowLoop,
        WorthQueryGraphReadBypassClass::AdHocAdjacencyMap,
        WorthQueryGraphReadBypassClass::ManualFrontierScan,
        WorthQueryGraphReadBypassClass::ManualVisitedSetTraversal,
        WorthQueryGraphReadBypassClass::PerNodeNeighborLookup,
        WorthQueryGraphReadBypassClass::BroadBooleanGraphScan,
        WorthQueryGraphReadBypassClass::SurfaceLocalGraphCache,
        WorthQueryGraphReadBypassClass::HiddenGraphReadFallback,
        WorthQueryGraphReadBypassClass::RuntimeReadLoweringBypass,
        WorthQueryGraphReadBypassClass::TestSupportClaimingProductionProof,
    ] {
        assert!(
            report.finding_count_for_class(class) > 0,
            "expected finding for {}",
            class.as_str()
        );
    }
}

#[test]
fn multiline_graph_read_bypass_patterns_are_classified() {
    let source = r#"
fn bypass(workspace: &Workspace, surfaces: &Surfaces) {
    let rows =
        workspace.read(surfaces.relations());
    for relation in rows.iter() {}

    let mut local_edges: BTreeMap<Id, Vec<Id>> =
        BTreeMap::new();
    local_edges.insert(source, vec![target]);

    let mut pending_frontier = VecDeque::from_iter([source]);
    while let Some(current) = pending_frontier.pop_front() {}

    let mut already_visited = BTreeSet::new();
    if already_visited.insert(source) {}

    let next = local_edges[&source].iter().find(|candidate| true);
    let broad = rows.iter().filter(|row| row_matches_source_kind(row));
}
"#;
    let report = graph_read_bypass_audit("multiline")
        .required_sources(WorthQueryBoundaryAuditSourceSet::new("multiline").source("lib", source))
        .evaluate()
        .expect("multiline graph-read bypass audit should evaluate");

    for class in [
        WorthQueryGraphReadBypassClass::ManualRelationRowLoop,
        WorthQueryGraphReadBypassClass::AdHocAdjacencyMap,
        WorthQueryGraphReadBypassClass::ManualFrontierScan,
        WorthQueryGraphReadBypassClass::ManualVisitedSetTraversal,
        WorthQueryGraphReadBypassClass::PerNodeNeighborLookup,
        WorthQueryGraphReadBypassClass::BroadBooleanGraphScan,
    ] {
        assert!(
            report.finding_count_for_class(class) > 0,
            "expected multiline finding for {}",
            class.as_str()
        );
    }
}

#[test]
fn comments_strings_and_unrelated_iteration_do_not_trigger_findings() {
    let source = r####"
// for relation in relation_rows.iter() {}
/* let mut adjacency = BTreeMap::new(); */
/// while let Some(next) = frontier.pop_front() {}
//! if visited.contains(&next) {}
#[doc = "fallback_graph_read()"]
fn clean(values: &[usize]) {
    let text = "fallback_graph_read relation_rows adjacency frontier visited";
    let escaped = r###"claim_production_graph_read_proof_for_test"###;
    let bytes = br#"relation_rows.iter().filter(|row| row_matches(row))"#;
    macro_rules! docs { () => { "frontier.pop()" } }
    for value in values.iter() {
        if value % 2 == 0 {}
    }
}
"####;
    let report = graph_read_bypass_audit("clean")
        .required_sources(WorthQueryBoundaryAuditSourceSet::new("clean").source("lib", source))
        .evaluate()
        .expect("clean graph-read bypass audit should evaluate");

    assert!(report.is_clean());
}

#[test]
fn multi_source_counters_explain_audit_breadth() {
    let report = graph_read_bypass_audit("counter-proof")
        .required_sources(
            WorthQueryBoundaryAuditSourceSet::new("counter-proof")
                .source_file(
                    "dirty",
                    "src/dirty.rs",
                    "fn dirty(relation_rows: &[Row]) { for row in relation_rows.iter() {} }",
                )
                .source_file(
                    "clean",
                    "src/clean.rs",
                    "fn clean(values: &[usize]) { for value in values.iter() {} }",
                )
                .source_file("empty", "src/empty.rs", "  "),
        )
        .evaluate()
        .expect("counter audit should evaluate");

    assert_eq!(report.counters().evaluated_source_count(), 2);
    assert_eq!(report.counters().skipped_empty_source_count(), 1);
    assert_eq!(report.counters().finding_count(), report.findings().len());
    assert_eq!(
        report.audited_source_labels(),
        &["dirty".to_string(), "clean".to_string()]
    );
}

#[test]
fn residue_rows_require_metadata_and_reject_growth() {
    let row = WorthQueryGraphReadBypassResidueRow::explicit(
        WorthQueryGraphReadBypassClass::ManualFrontierScan,
        "worth-topo adoption",
        "Milestone 9.10 Phase 15",
        1,
        1,
        "Phase 16 migrates the read",
        "graph-read access receipt exists",
    )
    .expect("complete residue row should admit");
    let missing_owner = WorthQueryGraphReadBypassResidueRow::explicit(
        WorthQueryGraphReadBypassClass::ManualFrontierScan,
        "",
        "Milestone 9.10 Phase 15",
        1,
        1,
        "blocker",
        "trigger",
    )
    .expect_err("missing owner should reject with typed residue error");
    assert_eq!(
        missing_owner.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::MissingRequiredField
    );
    assert_eq!(missing_owner.field_name(), Some("owner"));

    let cap_error = WorthQueryGraphReadBypassResidueRow::explicit(
        WorthQueryGraphReadBypassClass::ManualFrontierScan,
        "worth-topo adoption",
        "Milestone 9.10 Phase 15",
        2,
        1,
        "blocker",
        "trigger",
    )
    .expect_err("count above cap should reject");
    assert_eq!(
        cap_error.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::CountExceedsCap
    );

    let previous = WorthQueryGraphReadBypassResidueManifest::capped([row.clone()])
        .expect("previous residue should admit");
    let duplicate_error =
        WorthQueryGraphReadBypassResidueManifest::capped([row.clone(), row.clone()])
            .expect_err("duplicate residue classes should reject");
    assert_eq!(
        duplicate_error.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::DuplicateClass
    );
    let grown = WorthQueryGraphReadBypassResidueManifest::capped([
        WorthQueryGraphReadBypassResidueRow::explicit(
            WorthQueryGraphReadBypassClass::ManualFrontierScan,
            row.owner(),
            row.introduced_in(),
            2,
            2,
            row.blocker(),
            row.removal_trigger(),
        )
        .expect("candidate row shape should admit"),
    ])
    .expect("candidate manifest should admit");

    let growth_error =
        WorthQueryGraphReadBypassResidueManifest::certify_candidate_against_previous(
            &previous, &grown,
        )
        .expect_err("growing residue class must fail certification");
    assert_eq!(
        growth_error.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::ResidueGrowth
    );

    let changed_contract = WorthQueryGraphReadBypassResidueManifest::capped([
        WorthQueryGraphReadBypassResidueRow::explicit(
            WorthQueryGraphReadBypassClass::ManualFrontierScan,
            "different owner",
            row.introduced_in(),
            1,
            1,
            row.blocker(),
            row.removal_trigger(),
        )
        .expect("candidate row shape should admit"),
    ])
    .expect("candidate manifest should admit");
    let contract_error =
        WorthQueryGraphReadBypassResidueManifest::certify_candidate_against_previous(
            &previous,
            &changed_contract,
        )
        .expect_err("changed residue contract should fail certification");
    assert_eq!(
        contract_error.kind(),
        &WorthQueryGraphReadBypassResidueErrorKind::ResidueContractChanged
    );
}
