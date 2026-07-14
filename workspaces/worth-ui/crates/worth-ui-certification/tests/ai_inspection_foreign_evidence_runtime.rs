use std::collections::BTreeSet;

use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRichness, UiInspectionAiHarness,
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef,
    UiInspectionObligationFamily, UiInspectionObligationRelevanceDetail, UiInspectionQuery,
    UiInspectionQueryForeignEvidenceArtifactKind, UiInspectionQueryForeignEvidenceKind,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui_query_binding::{
    WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryProjectionConsumptionLane,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

#[test]
fn ai_harness_cites_query_owned_foreign_evidence_without_copying_it_into_ui_truth() {
    let app = obligation_dispatch_prerequisite_support::apps::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::touches::query_touch(&app);
    let ai = UiInspectionAiHarness::new(&app);
    let receipt = ai.inspect(
        UiInspectionQuery::new(
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
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let evidence_ref = receipt
        .evidence_slice()
        .expect("obligation AI query should retain an evidence slice")
        .refs()[0];
    let expansion = ai.expand_evidence_ref(evidence_ref, UiEvidenceRichness::materialized_detail());
    let foreign_refs = expansion.foreign_evidence_refs();
    let expected_prerequisite =
        obligation_dispatch_prerequisite_support::targets::graph_aligned_query_target(&touch)
            .query_prerequisites()
            .expect(
                "graph-aligned query target should retain one Query-owned prerequisite artifact",
            )
            .clone();
    let routes = foreign_refs
        .iter()
        .map(|foreign_ref| match foreign_ref {
            UiInspectionForeignEvidenceRef::Query(query_ref) => {
                (query_ref.kind(), query_ref.artifact_identity_digest())
            }
        })
        .collect::<BTreeSet<_>>();
    let artifact_identity_digests = foreign_refs
        .iter()
        .map(|foreign_ref| foreign_ref_artifact_identity_digest(*foreign_ref))
        .collect::<BTreeSet<_>>();

    assert!(matches!(
        expansion.materialized_detail(),
        Some(UiEvidenceMaterializedDetail::Obligation(_))
    ));
    assert_eq!(
        routes,
        BTreeSet::from([
            (
                UiInspectionQueryForeignEvidenceKind::ProjectionConsumption,
                artifact_identity_digests
                    .iter()
                    .copied()
                    .next()
                    .expect("foreign refs should expose one retained Query artifact identity"),
            ),
            (
                UiInspectionQueryForeignEvidenceKind::Inspection,
                artifact_identity_digests
                    .iter()
                    .copied()
                    .next()
                    .expect("foreign refs should expose one retained Query artifact identity"),
            ),
            (
                UiInspectionQueryForeignEvidenceKind::CausalExplanation,
                artifact_identity_digests
                    .iter()
                    .copied()
                    .next()
                    .expect("foreign refs should expose one retained Query artifact identity"),
            ),
        ])
    );
    assert_eq!(foreign_refs.len(), 3);
    assert_eq!(artifact_identity_digests.len(), 1);

    for foreign_ref in foreign_refs {
        let citation = ai.cite_foreign_evidence(*foreign_ref);
        let replayed_citation = ai.cite_foreign_evidence(*foreign_ref);
        match (*foreign_ref, citation) {
            (
                UiInspectionForeignEvidenceRef::Query(query_ref),
                UiInspectionForeignEvidenceCitation::Query(query_citation),
            ) => {
                assert_eq!(
                    UiInspectionForeignEvidenceCitation::Query(query_citation.clone()),
                    replayed_citation
                );
                assert_eq!(query_citation.foreign_ref(), query_ref);
                assert_eq!(query_citation.kind(), query_ref.kind());
                assert_eq!(
                    query_citation.artifact_kind(),
                    UiInspectionQueryForeignEvidenceArtifactKind::PrerequisiteEvidence
                );
                assert_eq!(
                    query_citation.artifact_identity_digest(),
                    query_ref.artifact_identity_digest()
                );
                assert_eq!(
                    query_citation.obligation_handle_digest(),
                    query_ref.obligation_handle_digest()
                );
                assert_eq!(
                    query_citation.graph_node_digest(),
                    touch.target().graph_node_identity().digest()
                );
                assert_eq!(
                    query_citation.touch_identity_digest(),
                    Some(touch.identity_digest())
                );
                assert!(query_citation.is_available());
                assert_eq!(
                    query_citation.prerequisite_evidence(),
                    Some(&expected_prerequisite)
                );
                assert_route_supported_by_prerequisite(
                    query_citation.kind(),
                    query_citation
                        .prerequisite_evidence()
                        .expect("available citation should retain the Query-owned prerequisite"),
                );
            }
        }
    }
}

fn foreign_ref_artifact_identity_digest(foreign_ref: UiInspectionForeignEvidenceRef) -> u64 {
    match foreign_ref {
        UiInspectionForeignEvidenceRef::Query(query_ref) => query_ref.artifact_identity_digest(),
    }
}

fn assert_route_supported_by_prerequisite(
    route: UiInspectionQueryForeignEvidenceKind,
    prerequisite: &worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
) {
    match route {
        UiInspectionQueryForeignEvidenceKind::ProjectionConsumption => assert_eq!(
            prerequisite.projection_consumption_lane(),
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts
        ),
        UiInspectionQueryForeignEvidenceKind::Inspection => assert_eq!(
            prerequisite.inspection_lane(),
            WorthUiQueryInspectionLane::WorkspaceInspect
        ),
        UiInspectionQueryForeignEvidenceKind::CausalExplanation => assert_eq!(
            prerequisite.causal_explanation_lane(),
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
        ),
    }
}
