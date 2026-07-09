use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceRichness, UiInspectionAiHarness, UiInspectionForeignEvidenceCitation,
    UiInspectionObligationFamily, UiInspectionObligationRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

#[test]
fn ordinary_35_covered_queries_stay_narrow_index_backed_and_log_free() {
    let app = boundary_app();
    let artifact = &app.declaration_artifacts()[0];
    let graph_node_digest = graph_node_digest(&app);
    let observation_before = app.inspection_observation();
    let receipts = [
        app.inspect(declaration_identity_query(artifact)),
        app.inspect(authored_provenance_query(artifact)),
        app.inspect(graph_identity_query(graph_node_digest)),
        app.inspect(published_aspect_query()),
    ];

    for receipt in receipts {
        let slice = receipt
            .evidence_slice()
            .expect("35-covered query should retain a public evidence slice");
        let cost = receipt
            .cost()
            .expect("35-covered query should expose ordinary cost posture");

        assert!(!slice.refs().is_empty());
        assert!(slice.materialized_detail().is_none());
        assert!(!cost.broad_scan_used());
        assert_eq!(cost.index_lookups(), 1);
        assert_eq!(cost.traversals_denied(), 0);
    }

    let observation_after = app.inspection_observation();
    assert_eq!(
        observation_after.log_emission_count() - observation_before.log_emission_count(),
        0
    );
    assert_eq!(
        observation_after.rich_artifact_materialization_count()
            - observation_before.rich_artifact_materialization_count(),
        0
    );
    assert_eq!(
        observation_after.graph_node_evidence_index_rebuild_count()
            - observation_before.graph_node_evidence_index_rebuild_count(),
        0
    );
    assert_eq!(
        observation_after.graph_aspect_evidence_index_rebuild_count()
            - observation_before.graph_aspect_evidence_index_rebuild_count(),
        0
    );
}

#[test]
fn ai_harness_and_query_citation_stay_on_the_ordinary_boundary() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let ai = UiInspectionAiHarness::new(&app);
    let query = obligation_query(
        touch.target().graph_node_identity().digest(),
        touch.identity_digest(),
    )
    .with_richness(UiEvidenceRichness::refs_only());
    let direct = app.inspect(query.clone());
    let harnessed = ai.inspect(query);
    let expansion = ai.expand_evidence_ref(
        harnessed
            .evidence_slice()
            .expect("obligation query should retain a slice")
            .refs()[0],
        UiEvidenceRichness::materialized_detail(),
    );

    assert_eq!(harnessed, direct);
    assert!(harnessed
        .evidence_slice()
        .expect("obligation query should retain a slice")
        .materialized_detail()
        .is_none());
    assert_eq!(expansion.foreign_evidence_refs().len(), 3);

    match ai.cite_foreign_evidence(expansion.foreign_evidence_refs()[0]) {
        UiInspectionForeignEvidenceCitation::Query(citation) => {
            assert!(citation.is_available());
            assert!(citation.prerequisite_evidence().is_some());
        }
    }
}

#[test]
fn unsupported_queries_do_not_fall_back_to_renderer_or_log_local_explanation() {
    let app = WorthUi::app()
        .with_dsl_package(WorthUiDslPackage::empty())
        .freeze();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    );
    let observation_before = app.inspection_observation();
    let first = app.inspect(query.clone());
    let second = app.inspect(query);
    let observation_after = app.inspection_observation();

    assert_eq!(first, second);
    assert!(first.evidence_slice().is_none());
    assert_eq!(
        observation_after.log_emission_count() - observation_before.log_emission_count(),
        0
    );
    assert_eq!(
        observation_after.unsupported_query_count() - observation_before.unsupported_query_count(),
        2
    );
}

fn boundary_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.inspection-boundary")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.boundary"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored("app/inspection_boundary.wui", 0),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:boundary"))
                    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
                    .with_published_aspect(UiDslAspectName::new("content.text"))
                    .with_consumed_aspect(UiDslAspectName::new("content.text")),
                ),
        )
        .freeze()
}

fn graph_node_digest(app: &WorthUiApp) -> u64 {
    app.graph()
        .lookup()
        .declaration_instances(app.declaration_artifacts()[0].identity())
        .value()[0]
        .digest()
}

fn declaration_identity_query(
    artifact: &worth_ui::facade::declaration::UiDeclarationArtifact,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::declaration_identity(artifact.identity().inspection_identity()),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn authored_provenance_query(
    artifact: &worth_ui::facade::declaration::UiDeclarationArtifact,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::authored_source_provenance(
            artifact
                .provenance()
                .inspection_authored_source_provenance_ref(),
        ),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn graph_identity_query(graph_node_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn published_aspect_query() -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::published_aspect("content.text"),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Aspect,
    )))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn obligation_query(graph_node_digest: u64, touch_identity_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(graph_node_digest, touch_identity_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
            .with_obligation_detail(
                UiInspectionObligationRelevanceDetail::new()
                    .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
            ),
    )
}
