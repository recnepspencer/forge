use std::collections::BTreeSet;

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceExpansionOutcome, UiEvidenceFamily,
    UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture, UiEvidenceRichness,
    UiInspectionAiHarness, UiInspectionAiHarnessLane, UiInspectionClosedSemanticLane,
    UiInspectionCloseoutGuarantee, UiInspectionCloseoutNonGoal, UiInspectionCostLane,
    UiInspectionDerivedIndexLane, UiInspectionForeignEvidenceRef,
    UiInspectionQueryForeignEvidenceKind, UiInspectionRefLifecycleLane,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionSliceLane,
    UiInspectionTargetClass,
};

#[path = "fixtures/inspection_closeout_support.rs"]
mod inspection_closeout_support;
use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

use inspection_closeout_support::{
    authored_provenance_query, closeout_app, declaration_identity_query, family_counts,
    graph_identity_query, graph_node_digest, obligation_query, published_aspect_query,
    receipt_snapshot,
};

#[test]
fn inspection_closeout_report_enumerates_milestone35_lanes_guarantees_and_non_goals() {
    let report = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed")
        .inspection_closeout_report();

    assert_eq!(
        report.evidence_families(),
        &[
            UiEvidenceFamily::Declaration,
            UiEvidenceFamily::Admission,
            UiEvidenceFamily::Graph,
            UiEvidenceFamily::Planning,
            UiEvidenceFamily::Aspect,
            UiEvidenceFamily::Obligation,
        ],
    );
    assert_eq!(
        report.relevance_outcomes(),
        &[
            UiInspectionRelevanceOutcome::Matched,
            UiInspectionRelevanceOutcome::EmptyLocal,
            UiInspectionRelevanceOutcome::UnsupportedScope {
                scope: UiInspectionScope::Graph,
            },
            UiInspectionRelevanceOutcome::ContradictoryRequest,
            UiInspectionRelevanceOutcome::BudgetExceeded {
                budget: UiEvidenceBudget::Narrow,
            },
            UiInspectionRelevanceOutcome::NotApplicableToTarget {
                target: UiInspectionTargetClass::ProductRoot,
            },
        ],
    );
    assert_eq!(
        report.ref_lifecycle_lanes(),
        &[
            UiInspectionRefLifecycleLane::MaterializationPostureBoundRef,
            UiInspectionRefLifecycleLane::FollowupQueryExpansion,
            UiInspectionRefLifecycleLane::RetainedDetailExpansion,
            UiInspectionRefLifecycleLane::NotMaterializedExpansion,
            UiInspectionRefLifecycleLane::WrongGenerationExpansion,
            UiInspectionRefLifecycleLane::DiscardedTombstoneExpansion,
        ],
    );
    assert_eq!(
        report.materialization_postures(),
        &[
            UiEvidenceMaterializationPosture::RefsOnly,
            UiEvidenceMaterializationPosture::SummaryAvailable,
            UiEvidenceMaterializationPosture::DetailAvailable,
        ],
    );
    assert_eq!(
        report.retention_postures(),
        &[
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            UiEvidenceRetentionPosture::DiscardedWithTombstone,
        ],
    );
    assert_eq!(
        report.query_citation_kinds(),
        &[
            UiInspectionQueryForeignEvidenceKind::ProjectionConsumption,
            UiInspectionQueryForeignEvidenceKind::Inspection,
            UiInspectionQueryForeignEvidenceKind::CausalExplanation,
        ],
    );
    assert_eq!(
        report.derived_index_lanes(),
        &[
            UiInspectionDerivedIndexLane::DeclarationAuthoredEvidence,
            UiInspectionDerivedIndexLane::GraphNodeEvidence,
            UiInspectionDerivedIndexLane::GraphAspectEvidence,
        ],
    );
    assert_eq!(
        report.slice_lanes(),
        &[
            UiInspectionSliceLane::DeclarationIdentity,
            UiInspectionSliceLane::AuthoredSourceProvenance,
            UiInspectionSliceLane::GraphNodeIdentity,
            UiInspectionSliceLane::AspectNeighborhood,
            UiInspectionSliceLane::ObligationNeighborhood,
            UiInspectionSliceLane::FamilySummaries,
            UiInspectionSliceLane::OmissionByScope,
            UiInspectionSliceLane::OmissionByBudget,
        ],
    );
    assert_eq!(
        report.cost_lanes(),
        &[
            UiInspectionCostLane::IndexedLookup,
            UiInspectionCostLane::NoBroadScan,
            UiInspectionCostLane::BudgetOmissionTracked,
            UiInspectionCostLane::MaterializationTracked,
            UiInspectionCostLane::TraversalDenialsExplicit,
        ],
    );
    assert_eq!(
        report.ai_harness_lanes(),
        &[
            UiInspectionAiHarnessLane::Inspect,
            UiInspectionAiHarnessLane::ExpandEvidenceRef,
            UiInspectionAiHarnessLane::CiteForeignEvidence,
            UiInspectionAiHarnessLane::SupportReport,
            UiInspectionAiHarnessLane::ClosureReport,
        ],
    );
    assert_eq!(
        report.closed_semantic_lanes(),
        &[
            UiInspectionClosedSemanticLane::EvidenceFamilies,
            UiInspectionClosedSemanticLane::RelevanceNarrowing,
            UiInspectionClosedSemanticLane::StableEvidenceRefs,
            UiInspectionClosedSemanticLane::RefExpansionLifecycle,
            UiInspectionClosedSemanticLane::RetentionPosture,
            UiInspectionClosedSemanticLane::QueryForeignEvidenceCitation,
            UiInspectionClosedSemanticLane::DerivedIndexLookup,
            UiInspectionClosedSemanticLane::SliceProjection,
            UiInspectionClosedSemanticLane::CostPosture,
            UiInspectionClosedSemanticLane::AiHarnessParity,
            UiInspectionClosedSemanticLane::SupportAndClosureReports,
        ],
    );
    assert_eq!(
        report.guarantees(),
        &[
            UiInspectionCloseoutGuarantee::CallerBypassDiesAtCompileAndFacadeBoundary,
            UiInspectionCloseoutGuarantee::EquivalentQueriesConvergeUnderStableAuthorityGeneration,
            UiInspectionCloseoutGuarantee::OrdinaryInspectionStaysNarrowAndIndexBacked,
            UiInspectionCloseoutGuarantee::QueryOwnedTruthRemainsForeignOwned,
            UiInspectionCloseoutGuarantee::FutureFamiliesExtendOneSubstrate,
        ],
    );
    assert_eq!(
        report.non_goals(),
        &[
            UiInspectionCloseoutNonGoal::MeasurementEvidence,
            UiInspectionCloseoutNonGoal::MountedReceiptEvidence,
            UiInspectionCloseoutNonGoal::VisualSnapshotEvidence,
            UiInspectionCloseoutNonGoal::ReplayEvidence,
            UiInspectionCloseoutNonGoal::RendererLocalExplanation,
            UiInspectionCloseoutNonGoal::HostLocalExplanation,
            UiInspectionCloseoutNonGoal::LogLocalExplanation,
        ],
    );
}

#[test]
fn closeout_runtime_covers_exact_family_relevance_and_query_citation_lanes() {
    let app = closeout_app();
    let artifact = &app.declaration_artifacts()[0];
    let graph_node_digest = graph_node_digest(&app);
    let declaration = app.inspect(declaration_identity_query(artifact));
    let provenance = app.inspect(authored_provenance_query(artifact));
    let graph = app.inspect(graph_identity_query(graph_node_digest));
    let aspect = app.inspect(published_aspect_query());
    let touch_app =
        obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&touch_app);
    let obligation = touch_app.inspect(
        obligation_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let obligation_ref = obligation
        .evidence_slice()
        .expect("obligation query should retain a slice")
        .refs()[0];
    let expansion = touch_app.expand_evidence_ref(obligation_ref, UiEvidenceRichness::summary());

    assert_eq!(
        family_counts(&declaration),
        vec![
            (UiEvidenceFamily::Declaration, 1),
            (UiEvidenceFamily::Admission, 1)
        ]
    );
    assert_eq!(
        family_counts(&provenance),
        vec![
            (UiEvidenceFamily::Declaration, 1),
            (UiEvidenceFamily::Admission, 1)
        ]
    );
    assert_eq!(
        family_counts(&graph),
        vec![
            (UiEvidenceFamily::Declaration, 1),
            (UiEvidenceFamily::Admission, 1),
            (UiEvidenceFamily::Graph, 1),
            (UiEvidenceFamily::Obligation, 1),
        ]
    );
    assert_eq!(family_counts(&aspect), vec![(UiEvidenceFamily::Aspect, 2)]);
    assert_eq!(
        family_counts(&obligation),
        vec![(
            UiEvidenceFamily::Obligation,
            obligation.evidence_slice().unwrap().refs().len()
        )]
    );
    assert_eq!(expansion.outcome(), UiEvidenceExpansionOutcome::Available);
    assert_eq!(
        expansion
            .foreign_evidence_refs()
            .iter()
            .map(|foreign_ref| match foreign_ref {
                UiInspectionForeignEvidenceRef::Query(query_ref) => query_ref.kind(),
            })
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            UiInspectionQueryForeignEvidenceKind::ProjectionConsumption,
            UiInspectionQueryForeignEvidenceKind::Inspection,
            UiInspectionQueryForeignEvidenceKind::CausalExplanation,
        ])
    );
}

#[test]
fn equivalent_declaration_queries_converge_on_refs_slice_shape_and_full_cost_posture() {
    let app = closeout_app();
    let artifact = &app.declaration_artifacts()[0];
    let identity = declaration_identity_query(artifact);
    let provenance = authored_provenance_query(artifact);
    // These two public queries target the same declaration-owned truth through
    // different caller-visible identities and must converge under one generation.
    assert_eq!(
        receipt_snapshot(&app.inspect(identity)),
        receipt_snapshot(&app.inspect(provenance))
    );
}

#[test]
fn graph_query_converges_between_ordinary_and_ai_harness_paths() {
    let app = closeout_app();
    let ai = UiInspectionAiHarness::new(&app);
    let graph = graph_identity_query(graph_node_digest(&app));

    assert_eq!(
        receipt_snapshot(&app.inspect(graph.clone())),
        receipt_snapshot(&ai.inspect(graph))
    );
}

#[test]
fn ref_lifecycle_and_retention_posture_stay_explicit_on_the_closeout_path() {
    let app = obligation_dispatch_prerequisite_support::application_authority::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::graph_touches::query_touch(&app);
    let receipt = app.inspect(
        obligation_query(
            touch.target().graph_node_identity().digest(),
            touch.identity_digest(),
        )
        .with_richness(UiEvidenceRichness::refs_only()),
    );
    let slice = receipt
        .evidence_slice()
        .expect("obligation query should retain a slice");
    let evidence_ref = slice.refs()[0];

    assert_eq!(
        evidence_ref.materialization_posture(),
        UiEvidenceMaterializationPosture::DetailAvailable
    );
    assert_eq!(
        evidence_ref.retention_posture(),
        UiEvidenceRetentionPosture::CurrentGenerationOnly
    );
    assert!(app.discard_evidence_slice(slice.slice_ref()));
    assert_eq!(
        app.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary())
            .outcome(),
        UiEvidenceExpansionOutcome::Discarded {
            retention: UiEvidenceRetentionPosture::DiscardedWithTombstone,
        }
    );
}
