use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionAiHarness, UiInspectionObligationFamily,
    UiInspectionObligationRelevanceDetail, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionScope, UiInspectionTarget, UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

const AI_ARTIFACT_MODULE: &str = "app/ai_inspection_runtime.wui";

#[test]
fn ai_harness_matches_ordinary_receipts_for_declaration_source_graph_and_aspect_queries() {
    let app = ai_surface_app();
    let ai = UiInspectionAiHarness::new(&app);
    let artifact = &app.declaration_artifacts()[0];
    let graph_node_digest = graph_node_digest(&app);
    let cases = [
        AiSliceCase {
            query: declaration_identity_query(artifact),
            expected_families: &[
                (UiEvidenceFamily::Declaration, 1),
                (UiEvidenceFamily::Admission, 1),
            ],
            expected_returned: 2,
            max_considered: 2,
        },
        AiSliceCase {
            query: authored_provenance_query(artifact),
            expected_families: &[
                (UiEvidenceFamily::Declaration, 1),
                (UiEvidenceFamily::Admission, 1),
            ],
            expected_returned: 2,
            max_considered: 2,
        },
        AiSliceCase {
            query: graph_identity_query(graph_node_digest),
            expected_families: &[
                (UiEvidenceFamily::Declaration, 1),
                (UiEvidenceFamily::Admission, 1),
                (UiEvidenceFamily::Graph, 1),
                (UiEvidenceFamily::Obligation, 1),
            ],
            expected_returned: 4,
            max_considered: 4,
        },
        AiSliceCase {
            query: published_aspect_query(),
            expected_families: &[(UiEvidenceFamily::Aspect, 2)],
            expected_returned: 2,
            max_considered: 2,
        },
    ];

    for case in cases {
        let query = case.query.clone();
        let direct = app.inspect(query.clone());
        let harnessed = ai.inspect(query);
        let slice = harnessed
            .evidence_slice()
            .expect("supported AI-targeted query should retain an evidence slice");
        let cost = harnessed
            .cost()
            .expect("AI-targeted graph inspection should expose ordinary cost posture");

        assert_eq!(harnessed, direct);
        assert_eq!(harnessed.query().richness(), UiEvidenceRichness::RefsOnly);
        assert!(slice.materialized_detail().is_none());
        assert_eq!(family_counts(slice.refs()), case.expected_families);
        assert_eq!(family_summary_counts(slice), case.expected_families);
        assert_eq!(slice.refs().len(), case.expected_returned);
        assert_eq!(cost.evidence_refs_returned(), case.expected_returned);
        assert!(cost.evidence_refs_considered() <= case.max_considered);
        assert_eq!(cost.traversals_denied(), 0);
        assert_eq!(cost.omitted_by_budget(), 0);
        assert!(!cost.broad_scan_used());
    }
}

#[test]
fn ai_harness_uses_refs_first_followup_queries_for_declaration_source_graph_and_aspect_refs() {
    let app = ai_surface_app();
    let ai = UiInspectionAiHarness::new(&app);
    let artifact = &app.declaration_artifacts()[0];
    let graph_node_digest = graph_node_digest(&app);
    let cases = [
        AiFollowupCase {
            query: declaration_identity_query(artifact),
            source_family: UiEvidenceFamily::Declaration,
            expected_target: UiInspectionTarget::declaration_identity(
                artifact.identity().inspection_identity(),
            ),
            expected_relevance_family: UiRelevanceFamily::Declaration,
            expected_families: &[(UiEvidenceFamily::Declaration, 1)],
        },
        AiFollowupCase {
            query: authored_provenance_query(artifact),
            source_family: UiEvidenceFamily::Declaration,
            expected_target: UiInspectionTarget::declaration_identity(
                artifact.identity().inspection_identity(),
            ),
            expected_relevance_family: UiRelevanceFamily::Declaration,
            expected_families: &[(UiEvidenceFamily::Declaration, 1)],
        },
        AiFollowupCase {
            query: graph_identity_query(graph_node_digest),
            source_family: UiEvidenceFamily::Graph,
            expected_target: UiInspectionTarget::graph_node_identity(graph_node_digest),
            expected_relevance_family: UiRelevanceFamily::Graph,
            expected_families: &[(UiEvidenceFamily::Graph, 1)],
        },
        AiFollowupCase {
            query: published_aspect_query(),
            source_family: UiEvidenceFamily::Aspect,
            expected_target: UiInspectionTarget::published_aspect("content.text"),
            expected_relevance_family: UiRelevanceFamily::Aspect,
            expected_families: &[(UiEvidenceFamily::Aspect, 2)],
        },
    ];

    for case in cases {
        let receipt = ai.inspect(case.query);
        let evidence_ref = select_ref_for_family(
            receipt
                .evidence_slice()
                .expect("refs-first AI query should retain a slice")
                .refs(),
            case.source_family,
        );
        let expansion = ai.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
        let followup_query = expansion
            .followup_query()
            .expect("real refs-first family ref should lower to the next ordinary inspection query")
            .clone();
        let followup_receipt = ai.inspect(followup_query.clone());
        let followup_slice = followup_receipt
            .evidence_slice()
            .expect("followup ordinary query should keep a bounded evidence slice");

        assert!(expansion.outcome().is_available());
        assert!(expansion.materialized_detail().is_none());
        assert_eq!(followup_query.target(), &case.expected_target);
        assert_eq!(followup_query.scope(), UiInspectionScope::Graph);
        assert_eq!(
            followup_query.relevance().filter().family_filter(),
            Some(case.expected_relevance_family)
        );
        assert_eq!(followup_receipt, app.inspect(followup_query));
        assert_eq!(family_counts(followup_slice.refs()), case.expected_families);
    }
}

#[test]
fn ai_harness_matches_ordinary_receipts_and_expansion_for_obligation_neighborhoods() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let ai = UiInspectionAiHarness::new(&app);
    let query = UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        ),
        UiInspectionScope::graph(),
    )
    .with_relevance(
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
            .with_obligation_detail(
                UiInspectionObligationRelevanceDetail::new()
                    .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
            ),
    )
    .with_richness(UiEvidenceRichness::refs_only());

    let direct = app.inspect(query.clone());
    let harnessed = ai.inspect(query);
    let evidence_ref = harnessed
        .evidence_slice()
        .expect("obligation AI query should retain an evidence slice")
        .refs()[0];
    let direct_expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let harnessed_expansion = ai.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let slice = harnessed
        .evidence_slice()
        .expect("obligation AI query should retain an evidence slice");
    let cost = harnessed
        .cost()
        .expect("obligation AI query should expose ordinary cost posture");

    assert_eq!(harnessed, direct);
    assert_eq!(harnessed_expansion, direct_expansion);
    assert_eq!(
        family_counts(slice.refs()),
        vec![(UiEvidenceFamily::Obligation, slice.refs().len())]
    );
    assert_eq!(
        family_summary_counts(slice),
        vec![(UiEvidenceFamily::Obligation, slice.refs().len())]
    );
    assert!(slice.refs().len() <= 2);
    assert_eq!(cost.evidence_refs_returned(), slice.refs().len());
    assert_eq!(cost.traversals_denied(), 0);
    assert_eq!(cost.omitted_by_budget(), 0);
    assert!(!cost.broad_scan_used());
    assert!(harnessed_expansion.foreign_evidence_refs().is_empty());
}

#[test]
fn ai_harness_keeps_support_and_closure_reports_on_the_ordinary_surface() {
    let app = ai_surface_app();
    let ai = UiInspectionAiHarness::new(&app);

    assert_eq!(
        ai.support_report(UiInspectionScope::measurement()),
        app.inspection_support_report(UiInspectionScope::measurement())
    );
    assert_eq!(
        ai.support_report(UiInspectionScope::graph()),
        app.inspection_support_report(UiInspectionScope::graph())
    );
    assert_eq!(
        ai.closure_report(),
        app.inspection_closure_report(),
        "AI harness should not fork support-bearing inspection closure reporting",
    );
}

fn ai_surface_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.ai-inspection-runtime")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.ai_inspection"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(AI_ARTIFACT_MODULE, 0),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:workflow"))
                    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
                    .with_published_aspect(UiDslAspectName::new("content.text"))
                    .with_consumed_aspect(UiDslAspectName::new("content.text")),
                ),
        )
        .freeze()
        .expect("application preparation should succeed")
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

fn family_counts(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
) -> Vec<(UiEvidenceFamily, usize)> {
    let mut counts = std::collections::BTreeMap::new();
    for evidence_ref in refs {
        *counts.entry(evidence_ref.family()).or_insert(0usize) += 1;
    }
    counts.into_iter().collect()
}

fn family_summary_counts(
    slice: &worth_ui::facade::inspection::UiEvidenceSlice,
) -> Vec<(UiEvidenceFamily, usize)> {
    slice
        .family_summaries()
        .iter()
        .map(|summary| (summary.family(), summary.ref_count()))
        .collect()
}

fn select_ref_for_family(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
    family: UiEvidenceFamily,
) -> worth_ui::facade::inspection::UiEvidenceRef {
    refs.iter()
        .copied()
        .find(|evidence_ref| evidence_ref.family() == family)
        .expect("family-specific AI query should expose the requested evidence family")
}

struct AiSliceCase {
    query: UiInspectionQuery,
    expected_families: &'static [(UiEvidenceFamily, usize)],
    expected_returned: usize,
    max_considered: usize,
}

struct AiFollowupCase {
    query: UiInspectionQuery,
    source_family: UiEvidenceFamily,
    expected_target: UiInspectionTarget,
    expected_relevance_family: UiRelevanceFamily,
    expected_families: &'static [(UiEvidenceFamily, usize)],
}
