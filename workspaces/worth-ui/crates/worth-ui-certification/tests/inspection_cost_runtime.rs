use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceRichness, UiEvidenceSliceOmission, UiInspectionCostReceipt,
    UiInspectionQuery, UiInspectionRelevance, UiInspectionScope, UiInspectionTarget,
    UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui::facade::UiInspectionAspectRelevanceDetail;
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn declaration_and_graph_receipts_expose_bounded_indexed_costs() {
    let app = lookup_app();
    let artifact = authored_artifact(&app);
    let declaration_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::declaration_identity(artifact.identity().inspection_identity()),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Declaration,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let graph_node_identity = app
        .graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()[0];
    let graph_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::graph_node_identity(graph_node_identity.digest()),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Graph,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );

    assert_filtered_indexed_cost(
        declaration_receipt
            .cost()
            .expect("declaration receipt should expose cost"),
        2,
        declaration_receipt
            .evidence_slice()
            .expect("declaration receipt should carry a slice")
            .refs()
            .len(),
    );
    assert_filtered_indexed_cost(
        graph_receipt
            .cost()
            .expect("graph receipt should expose cost"),
        4,
        graph_receipt
            .evidence_slice()
            .expect("graph receipt should carry a slice")
            .refs()
            .len(),
    );
}

#[test]
fn obligation_receipts_expose_scope_omission_and_materialization_costs() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let refs_only = app.inspect(
        obligation_touch_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let rich = app.inspect(
        obligation_touch_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let refs_only_slice = refs_only
        .evidence_slice()
        .expect("refs-only obligation receipt should carry a slice");
    let rich_slice = rich
        .evidence_slice()
        .expect("rich obligation receipt should carry a slice");
    let refs_only_cost = refs_only
        .cost()
        .expect("refs-only obligation receipt should expose cost");
    let rich_cost = rich
        .cost()
        .expect("rich obligation receipt should expose cost");
    let materialized_records = rich_slice
        .materialized_detail()
        .and_then(obligation_projection_count)
        .expect("rich obligation receipt should materialize retained detail");

    assert_unfiltered_indexed_cost(refs_only_cost, refs_only_slice.refs().len());
    assert_eq!(refs_only_cost.materialized_records(), 0);
    assert_eq!(refs_only_cost.omitted_by_budget(), 0);
    assert_unfiltered_indexed_cost(rich_cost, rich_slice.refs().len());
    assert_eq!(rich_cost.materialized_records(), materialized_records);
    assert_eq!(rich_cost.omitted_by_budget(), 0);
}

#[test]
fn narrow_budget_rich_obligation_queries_omit_detail_by_budget_without_widening() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let receipt = app.inspect(
        obligation_touch_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_budget(UiEvidenceBudget::narrow())
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let slice = receipt
        .evidence_slice()
        .expect("narrow-budget obligation receipt should retain a slice");
    let cost = receipt
        .cost()
        .expect("narrow-budget obligation receipt should expose cost");

    assert!(slice.materialized_detail().is_none());
    assert_eq!(
        slice.omission(),
        Some(UiEvidenceSliceOmission::ByBudget {
            budget: UiEvidenceBudget::Narrow,
        })
    );
    assert_eq!(cost.evidence_refs_considered(), slice.refs().len());
    assert_eq!(cost.evidence_refs_returned(), slice.refs().len());
    assert_eq!(cost.materialized_records(), 0);
    assert_eq!(cost.omitted_by_budget(), 1);
    assert_eq!(cost.traversals_denied(), 0);
    assert!(!cost.broad_scan_used());
}

#[test]
fn aspect_provenance_widening_reports_honest_costs_on_the_real_path() {
    let app = aspect_cost_app();
    let plain_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::published_aspect("content.text"),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Aspect,
        )))
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let provenance_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::published_aspect("content.text"),
            UiInspectionScope::graph(),
        )
        .with_relevance(
            UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Aspect))
                .with_aspect_detail(
                    UiInspectionAspectRelevanceDetail::new().include_direct_provenance_refs(),
                ),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let plain_slice = plain_receipt
        .evidence_slice()
        .expect("plain aspect receipt should carry a slice");
    let provenance_slice = provenance_receipt
        .evidence_slice()
        .expect("provenance aspect receipt should carry a slice");
    let plain_cost = plain_receipt
        .cost()
        .expect("plain aspect receipt should expose cost");
    let provenance_cost = provenance_receipt
        .cost()
        .expect("provenance aspect receipt should expose cost");
    let plain_family_counts = family_counts(plain_slice.refs());
    let provenance_family_counts = family_counts(provenance_slice.refs());

    assert_eq!(plain_slice.refs().len(), 4);
    assert_eq!(provenance_slice.refs().len(), 6);
    assert_eq!(
        plain_family_counts,
        vec![(worth_ui::facade::inspection::UiEvidenceFamily::Aspect, 4)]
    );
    assert_eq!(
        provenance_family_counts,
        vec![
            (
                worth_ui::facade::inspection::UiEvidenceFamily::Declaration,
                2
            ),
            (worth_ui::facade::inspection::UiEvidenceFamily::Aspect, 4),
        ]
    );
    assert_unfiltered_indexed_cost(plain_cost, 4);
    assert_unfiltered_indexed_cost(provenance_cost, 6);
}

fn assert_filtered_indexed_cost(
    cost: UiInspectionCostReceipt,
    expected_considered: usize,
    expected_returned: usize,
) {
    assert_eq!(cost.index_lookups(), 1);
    assert_eq!(cost.evidence_refs_considered(), expected_considered);
    assert_eq!(cost.evidence_refs_returned(), expected_returned);
    assert!(cost.evidence_refs_considered() > cost.evidence_refs_returned());
    assert_eq!(cost.traversals_denied(), 0);
    assert_eq!(cost.omitted_by_budget(), 0);
    assert!(!cost.broad_scan_used());
}

fn assert_unfiltered_indexed_cost(cost: UiInspectionCostReceipt, expected_ref_count: usize) {
    assert_eq!(cost.index_lookups(), 1);
    assert_eq!(cost.evidence_refs_considered(), expected_ref_count);
    assert_eq!(cost.evidence_refs_returned(), expected_ref_count);
    assert_eq!(cost.traversals_denied(), 0);
    assert_eq!(cost.omitted_by_budget(), 0);
    assert!(!cost.broad_scan_used());
}

fn family_counts(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
) -> Vec<(worth_ui::facade::inspection::UiEvidenceFamily, usize)> {
    let mut counts = std::collections::BTreeMap::new();
    for evidence_ref in refs {
        *counts.entry(evidence_ref.family()).or_insert(0usize) += 1;
    }
    counts.into_iter().collect()
}

fn obligation_projection_count(
    detail: &worth_ui::facade::inspection::UiEvidenceMaterializedDetail,
) -> Option<usize> {
    match detail {
        worth_ui::facade::inspection::UiEvidenceMaterializedDetail::Obligation(receipt) => {
            Some(receipt.projections().len())
        }
        _ => None,
    }
}

fn lookup_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.inspection-cost")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.cost"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored("app/inspection_cost_runtime.wui", 0),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:workflow"))
                    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
                    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view")),
                ),
        )
        .freeze()
}

fn aspect_cost_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.inspection-cost.aspect")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.aspect.alpha"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/inspection_cost_aspect_alpha.wui",
                            0,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:alpha"))
                    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
                    .with_published_aspect(UiDslAspectName::new("content.text")),
                )
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.aspect.beta"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/inspection_cost_aspect_beta.wui",
                            0,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:beta"))
                    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
                    .with_published_aspect(UiDslAspectName::new("content.text")),
                ),
        )
        .freeze()
}

fn authored_artifact(
    app: &worth_ui::facade::app::WorthUiApp,
) -> &worth_ui::facade::declaration::UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/inspection_cost_runtime.wui"
        })
        .expect("inspection-cost app should contain the authored artifact")
}

fn obligation_touch_query(graph_node_digest: u64, touch_identity_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(graph_node_digest, touch_identity_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Obligation,
    )))
}
