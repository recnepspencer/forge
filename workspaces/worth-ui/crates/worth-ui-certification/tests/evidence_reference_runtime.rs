use worth_ui::facade::inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceAuthorityKind, UiEvidenceExpansionOutcome,
    UiEvidenceFamily, UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
    UiEvidenceRichness, UiInspectionObligationFamily, UiInspectionObligationRelevanceDetail,
    UiInspectionQuery, UiInspectionRelevance, UiInspectionScope, UiInspectionTarget,
    UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui::facade::{
    admission::UiAdmissionTarget, admission::UiAdmissionWorld, app::WorthUi,
    graph::UiGraphInstantiationPlan,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, WorthUiDslPackage,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn refs_only_receipts_preserve_selected_relevance_and_slice_reference() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let query = obligation_query(
        touch.target().graph_node_identity().digest(),
        touch.identity_digest(),
    )
    .with_richness(UiEvidenceRichness::refs_only());

    let receipt = selected.inspect(query.clone());
    let slice = receipt
        .evidence_slice()
        .expect("matched obligation inspection should retain an evidence slice");

    assert_eq!(receipt.query(), &query);
    assert_eq!(receipt.selected_relevance(), &query.relevance());
    assert_eq!(
        receipt.authority_generation(),
        Some(UiEvidenceAuthorityGeneration::new(
            app.graph().generation().as_u64(),
        ))
    );
    assert_eq!(receipt.evidence_slice_ref(), Some(slice.slice_ref()));
    assert_eq!(slice.authority_generation(), app_generation(&app));
    assert!(slice.materialized_detail().is_none());
    assert!(!slice.refs().is_empty());
    assert!(slice.refs().iter().all(|evidence_ref| {
        evidence_ref.family() == UiEvidenceFamily::Obligation
            && evidence_ref.authority_generation() == app_generation(&app)
            && evidence_ref.materialization_posture()
                == UiEvidenceMaterializationPosture::DetailAvailable
            && evidence_ref.retention_posture() == UiEvidenceRetentionPosture::CurrentGenerationOnly
    }));
}

#[test]
fn equivalent_queries_over_one_generation_converge_on_equivalent_refs() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let target = obligation_dispatch_prerequisite_support::graph_aligned_query_target(&touch);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target);
    let query = obligation_query(
        touch.target().graph_node_identity().digest(),
        touch.identity_digest(),
    )
    .with_richness(UiEvidenceRichness::refs_only());

    let first = selected.inspect(query.clone());
    let second = selected.inspect(query);

    assert_eq!(first.authority_generation(), second.authority_generation());
    assert_eq!(first.evidence_slice_ref(), second.evidence_slice_ref());
    assert_eq!(
        first.evidence_slice().map(|slice| slice.refs().to_vec()),
        second.evidence_slice().map(|slice| slice.refs().to_vec())
    );
}

#[test]
fn expanding_a_same_generation_ref_keeps_identity_and_returns_typed_availability() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let refs_only_receipt = app.inspect(
        obligation_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let evidence_ref = refs_only_receipt
        .evidence_slice()
        .expect("matched obligation inspection should retain an evidence slice")
        .refs()[0];

    let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let direct_receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_evidence_handle(evidence_ref.handle().handle_digest()),
            UiInspectionScope::graph(),
        )
        .with_relevance(obligation_relevance())
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let expanded_detail = obligation_detail(
        expansion
            .materialized_detail()
            .expect("public expansion should materialize one obligation receipt"),
    );
    let direct_detail = obligation_detail(
        direct_receipt
            .evidence_slice()
            .expect("direct public handle inspection should retain a slice")
            .materialized_detail()
            .expect("direct public handle inspection should materialize obligation detail"),
    );

    assert_eq!(expansion.evidence_ref(), evidence_ref);
    assert_eq!(expansion.requested_richness(), UiEvidenceRichness::Summary);
    assert!(expansion.outcome().is_available());
    assert_eq!(expanded_detail.refs(), direct_detail.refs());
    assert_eq!(expanded_detail.projections(), direct_detail.projections());
}

#[test]
fn graph_family_refs_bind_real_snapshot_generation_on_certification_boundary() {
    let app = graph_evidence_app();
    let graph_node_identity = first_graph_node_identity(&app);

    let evidence_ref = app
        .graph()
        .evidence_ref_for_node(graph_node_identity)
        .expect("graph authority should derive a shared evidence ref for the node");

    assert_eq!(evidence_ref.family(), UiEvidenceFamily::Graph);
    assert_eq!(
        evidence_ref.authority_binding().artifact_identity().kind(),
        UiEvidenceAuthorityKind::GraphSnapshot
    );
    assert_eq!(evidence_ref.authority_generation(), app_generation(&app));
    assert_eq!(
        evidence_ref
            .authority_binding()
            .artifact_identity()
            .digest(),
        app.graph().generation().as_u64()
    );
    assert_eq!(
        evidence_ref.materialization_posture(),
        UiEvidenceMaterializationPosture::SummaryAvailable
    );
}

#[test]
fn admission_family_refs_expand_as_not_materialized_on_public_path() {
    let app = graph_evidence_app();
    let graph_node_identity = first_graph_node_identity(&app);
    let report = app.admission().report(UiAdmissionTarget::graph_node(
        graph_node_identity,
        UiAdmissionWorld::from_graph_world_profile(app.graph().world_profile().clone()),
    ));
    let evidence_ref = report.evidence_ref();

    let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

    assert_eq!(expansion.evidence_ref(), evidence_ref);
    assert_eq!(
        expansion.outcome(),
        UiEvidenceExpansionOutcome::NotMaterialized {
            posture: UiEvidenceMaterializationPosture::RefsOnly,
        }
    );
    assert!(expansion.materialized_detail().is_none());
}

#[test]
fn graph_family_refs_expand_to_followup_queries_on_public_path() {
    let app = graph_evidence_app();
    let graph_node_identity = first_graph_node_identity(&app);
    let evidence_ref = app
        .graph()
        .evidence_ref_for_node(graph_node_identity)
        .expect("graph authority should derive a shared evidence ref for the node");

    let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let followup_query = expansion
        .followup_query()
        .expect("graph evidence ref should lower to a real ordinary followup query");
    let followup_receipt = app.inspect(followup_query.clone());

    assert_eq!(expansion.evidence_ref(), evidence_ref);
    assert!(expansion.outcome().is_available());
    assert!(expansion.materialized_detail().is_none());
    assert_eq!(
        followup_query.target(),
        &UiInspectionTarget::graph_node_identity(graph_node_identity.digest())
    );
    assert!(followup_receipt.evidence_slice().is_some());
}

#[test]
fn stale_generation_refs_expand_as_wrong_generation_after_real_graph_successor_commit() {
    let app = graph_evidence_app();
    let stale_ref = app
        .graph()
        .evidence_ref_for_node(first_graph_node_identity(&app))
        .expect("graph authority should derive a shared evidence ref for the node");
    let successor = successor_graph_commit(&app);
    let current_generation =
        UiEvidenceAuthorityGeneration::new(successor.committed_generation().as_u64());

    let expansion = successor
        .graph()
        .expand_evidence_ref(stale_ref, UiEvidenceRichness::summary());

    assert_eq!(expansion.evidence_ref(), stale_ref);
    assert_eq!(
        expansion.outcome(),
        UiEvidenceExpansionOutcome::WrongGeneration {
            requested_generation: app_generation(&app),
            current_generation,
        }
    );
    assert!(expansion.materialized_detail().is_none());
}

#[test]
fn public_refs_only_inspection_stays_cheap_until_explicit_expansion() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let observation_before = app.inspection_observation();
    let receipt = app.inspect(
        obligation_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let observation_after_refs_only = app.inspection_observation();
    let evidence_ref = receipt
        .evidence_slice()
        .expect("public refs-only inspection should retain an evidence slice")
        .refs()[0];

    assert_eq!(
        observation_after_refs_only.rich_artifact_materialization_count()
            - observation_before.rich_artifact_materialization_count(),
        0
    );
    assert!(receipt
        .evidence_slice()
        .expect("public refs-only inspection should retain an evidence slice")
        .materialized_detail()
        .is_none());

    let _ = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());
    let observation_after_expand = app.inspection_observation();

    assert_eq!(
        observation_after_expand.rich_artifact_materialization_count()
            - observation_after_refs_only.rich_artifact_materialization_count(),
        1
    );
}

#[test]
fn discarded_slice_closeout_makes_public_ref_expansion_return_tombstone_discarded() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let receipt = app.inspect(
        obligation_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let slice = receipt
        .evidence_slice()
        .expect("public refs-only inspection should retain an evidence slice");
    let evidence_ref = slice.refs()[0];

    assert!(
        app.discard_evidence_slice(slice.slice_ref()),
        "retained public evidence slice should admit lifecycle closeout"
    );
    let expansion = app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

    assert_eq!(expansion.evidence_ref().handle(), evidence_ref.handle());
    assert_eq!(
        expansion.outcome(),
        UiEvidenceExpansionOutcome::Discarded {
            retention: UiEvidenceRetentionPosture::DiscardedWithTombstone,
        }
    );
    assert!(expansion.materialized_detail().is_none());
}

fn app_generation(app: &worth_ui::facade::app::WorthUiApp) -> UiEvidenceAuthorityGeneration {
    UiEvidenceAuthorityGeneration::new(app.graph().generation().as_u64())
}

fn obligation_query(graph_node_digest: u64, touch_identity_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::obligation_touch(graph_node_digest, touch_identity_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(obligation_relevance())
}

fn graph_evidence_app() -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.evidence.references")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.graph.evidence.reference"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/evidence_reference_runtime.wui",
                            0,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:test")),
                ),
        )
        .freeze()
}

type WorthUiApp = worth_ui::facade::app::WorthUiApp;

fn first_graph_node_identity(app: &WorthUiApp) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    let declaration_identity = app.declaration_artifacts()[0].identity();
    app.graph()
        .lookup()
        .declaration_instances(declaration_identity)
        .value()[0]
}

fn successor_graph_commit(
    app: &WorthUiApp,
) -> worth_ui::facade::graph::UiGraphMutationCommitResult {
    let handoffs = app
        .declaration_artifacts()
        .iter()
        .map(|artifact| {
            artifact
                .graph_handoff()
                .expect("graph evidence app declarations should lower to graph handoffs")
        })
        .collect::<Vec<_>>();

    UiGraphInstantiationPlan::admit_handoffs(&handoffs, &[])
        .expect("graph evidence app declarations should admit successor graph planning")
        .commit_successor_generation(app.graph())
        .expect("admitted graph plan should commit one real successor generation")
}

fn obligation_relevance() -> UiInspectionRelevance {
    UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation))
        .with_obligation_detail(
            UiInspectionObligationRelevanceDetail::new()
                .with_family(UiInspectionObligationFamily::QueryBindingRequirement),
        )
}

fn obligation_detail(
    detail: &worth_ui::facade::inspection::UiEvidenceMaterializedDetail,
) -> &worth_ui::facade::inspection::UiInspectionObligationEvidenceReceipt {
    match detail {
        worth_ui::facade::inspection::UiEvidenceMaterializedDetail::Obligation(receipt) => receipt,
        _ => panic!("expected obligation materialized detail"),
    }
}
