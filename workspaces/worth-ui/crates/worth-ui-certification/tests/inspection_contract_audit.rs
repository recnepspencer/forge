use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceBudget, UiEvidenceLinkKind, UiEvidenceRichness, UiInspectionMilestoneExpectation,
    UiInspectionObligationRelevanceDetail, UiInspectionPosture, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionRelevanceOutcome, UiInspectionScope,
    UiInspectionSupportPosture, UiInspectionSupportReason, UiInspectionSupportStatus,
    UiInspectionSupportWorld, UiInspectionTarget, UiRelevanceFamily, UiRelevanceFilter,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};

#[test]
fn inspection_query_preserves_budget_richness_and_relevance_through_the_facade_receipt() {
    let app = empty_app();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::obligation_graph_node(17),
        UiInspectionScope::graph(),
    )
    .with_richness(UiEvidenceRichness::refs_only())
    .with_budget(UiEvidenceBudget::ordinary())
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Obligation,
    )));
    let receipt = app.inspect(query.clone());

    assert_eq!(receipt.query(), &query);
    assert_eq!(receipt.query().richness(), UiEvidenceRichness::RefsOnly);
    assert_eq!(receipt.query().budget(), UiEvidenceBudget::Ordinary);
    assert_eq!(
        receipt.query().relevance().filter().family_filter(),
        Some(UiRelevanceFamily::Obligation)
    );
    assert_eq!(
        receipt.relevance_admission().outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
}

#[test]
fn inspection_inventory_projects_typed_support_and_closure_reports() {
    let app = empty_app();
    let graph_report = app.inspection_support_report(UiInspectionScope::graph());
    let measurement_report = app.inspection_support_report(UiInspectionScope::measurement());
    let mounting_report = app.inspection_support_report(UiInspectionScope::mounting());
    let rebind_report = app.inspection_support_report(UiInspectionScope::rebind());
    let closure_report = app.inspection_closure_report();

    assert_eq!(graph_report.scope(), UiInspectionScope::Graph);
    assert_eq!(
        graph_report.status(),
        UiInspectionSupportStatus::Unsupported
    );
    assert_eq!(graph_report.posture(), UiInspectionSupportPosture::Deferred);
    assert_eq!(
        graph_report.reason(),
        Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted)
    );
    assert_eq!(
        graph_report.expected_in(),
        Some(UiInspectionMilestoneExpectation::Milestone31)
    );
    assert_eq!(measurement_report.scope(), UiInspectionScope::Measurement);
    assert_eq!(
        measurement_report.current_world(),
        UiInspectionSupportWorld::Authoritative
    );
    assert_eq!(
        mounting_report.current_world(),
        UiInspectionSupportWorld::Authoritative
    );
    assert_eq!(
        rebind_report.current_world(),
        UiInspectionSupportWorld::Authoritative
    );
    assert_eq!(closure_report.rows().len(), 20);
}

#[test]
fn supported_non_graph_receipts_remain_matched_on_the_real_path() {
    let app = empty_app();
    let support_report = app.inspection_support_report(UiInspectionScope::measurement());

    let receipt = app.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::measurement(),
    ));

    assert_eq!(receipt.query().scope(), UiInspectionScope::Measurement);
    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(receipt.support_report(), Some(support_report));
    assert_eq!(receipt.posture(), Some(UiInspectionPosture::available()));
}

#[test]
fn unsupported_scope_receipts_surface_typed_relevance_outcomes_on_the_real_path() {
    let app = empty_app();
    let support_report = app.inspection_support_report(UiInspectionScope::graph());

    let receipt = app.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));

    assert_eq!(receipt.query().scope(), UiInspectionScope::Graph);
    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::UnsupportedScope {
            scope: UiInspectionScope::Graph,
        }
    );
    assert_eq!(receipt.support_report(), Some(support_report));
    assert_eq!(
        receipt.posture(),
        Some(UiInspectionPosture::deferred(
            Some(UiInspectionMilestoneExpectation::Milestone31),
            UiInspectionSupportWorld::Authoritative,
        ))
    );
}

#[test]
fn contradictory_requests_surface_typed_relevance_outcomes_on_the_real_path() {
    let app = empty_app();

    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(17),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(
            UiRelevanceFilter::target_local().include_link(UiEvidenceLinkKind::Explains),
        )),
    );

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::ContradictoryRequest
    );
    assert_eq!(receipt.support_report(), None);
    assert_eq!(receipt.evidence_slice(), None);
}

#[test]
fn budget_exceeded_requests_surface_typed_relevance_outcomes_on_the_real_path() {
    let app = empty_app();

    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(17),
            UiInspectionScope::graph(),
        )
        .with_budget(UiEvidenceBudget::narrow())
        .with_relevance(UiInspectionRelevance::local(
            UiRelevanceFilter::family(UiRelevanceFamily::Obligation)
                .include_family(UiRelevanceFamily::Aspect),
        )),
    );

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::BudgetExceeded {
            budget: UiEvidenceBudget::Narrow,
        }
    );
    assert_eq!(receipt.support_report(), None);
    assert_eq!(receipt.evidence_slice(), None);
}

#[test]
fn not_applicable_requests_surface_typed_relevance_outcomes_on_the_real_path() {
    let app = empty_app();

    let receipt = app.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::product_root(),
            UiInspectionScope::graph(),
        )
        .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
            UiRelevanceFamily::Obligation,
        ))),
    );

    assert!(matches!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget { .. }
    ));
    assert_eq!(
        receipt.support_report(),
        Some(app.inspection_support_report(UiInspectionScope::graph()))
    );
    assert_eq!(receipt.evidence_slice(), None);
}

#[test]
fn declared_surface_supported_receipts_use_the_real_declared_surface_branch() {
    let app = declared_surface_app();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::declared_surface("app/inspection_relevance.wui", 1),
        UiInspectionScope::measurement(),
    );
    let support_report = app.inspection_support_report_for(&query);
    let receipt = app.inspect(query);

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(receipt.support_report(), Some(support_report));
    assert_eq!(receipt.posture(), Some(UiInspectionPosture::available()));
}

#[test]
fn declared_surface_unsupported_receipts_use_the_real_declared_surface_branch() {
    let app = declared_surface_app();
    let query = UiInspectionQuery::new(
        UiInspectionTarget::declared_surface("app/inspection_relevance.wui", 1),
        UiInspectionScope::graph(),
    );
    let support_report = app.inspection_support_report_for(&query);
    let receipt = app.inspect(query);

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::UnsupportedScope {
            scope: UiInspectionScope::Graph,
        }
    );
    assert_eq!(receipt.support_report(), Some(support_report));
    assert_eq!(
        receipt.posture(),
        Some(UiInspectionPosture::unsupported(
            UiInspectionSupportReason::TargetOutsideInspectionBoundary,
            None,
            UiInspectionSupportWorld::Authoritative,
        ))
    );
}

#[test]
fn inspection_relevance_has_one_explicit_layering_relationship() {
    let relevance = UiInspectionRelevance::local(
        UiRelevanceFilter::family(UiRelevanceFamily::Obligation)
            .include_family(UiRelevanceFamily::Aspect)
            .include_link(UiEvidenceLinkKind::CorrespondsTo),
    )
    .with_obligation_detail(UiInspectionObligationRelevanceDetail::new().with_family(
        worth_ui::facade::inspection::UiInspectionObligationFamily::QueryBindingRequirement,
    ));

    assert_eq!(
        relevance.filter().family_filter(),
        Some(UiRelevanceFamily::Obligation)
    );
    assert_eq!(
        relevance.filter().cross_family(),
        Some(UiRelevanceFamily::Aspect)
    );
    assert_eq!(
        relevance.filter().link_kind(),
        Some(UiEvidenceLinkKind::CorrespondsTo)
    );
    assert_eq!(
        relevance
            .obligation_detail()
            .and_then(|detail| detail.family()),
        Some(worth_ui::facade::inspection::UiInspectionObligationFamily::QueryBindingRequirement)
    );
}

#[test]
fn relevance_admission_distinguishes_typed_outcomes() {
    let matched = UiInspectionQuery::new(
        UiInspectionTarget::obligation_graph_node(17),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Obligation,
    )))
    .admit_relevance();
    let empty_local = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .admit_relevance();
    let supported_non_graph = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::measurement(),
    )
    .admit_relevance();
    let contradictory = UiInspectionQuery::new(
        UiInspectionTarget::obligation_graph_node(17),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local().include_link(UiEvidenceLinkKind::Explains),
    ))
    .admit_relevance();
    let budget_exceeded = UiInspectionQuery::new(
        UiInspectionTarget::obligation_graph_node(17),
        UiInspectionScope::graph(),
    )
    .with_budget(UiEvidenceBudget::narrow())
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::family(UiRelevanceFamily::Obligation)
            .include_family(UiRelevanceFamily::Aspect),
    ))
    .admit_relevance();
    let not_applicable = UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Obligation,
    )))
    .admit_relevance();

    assert_eq!(matched.outcome(), UiInspectionRelevanceOutcome::Matched);
    assert_eq!(
        empty_local.outcome(),
        UiInspectionRelevanceOutcome::EmptyLocal
    );
    assert_eq!(
        supported_non_graph.outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        contradictory.outcome(),
        UiInspectionRelevanceOutcome::ContradictoryRequest
    );
    assert_eq!(
        budget_exceeded.outcome(),
        UiInspectionRelevanceOutcome::BudgetExceeded {
            budget: UiEvidenceBudget::Narrow,
        }
    );
    assert!(matches!(
        not_applicable.outcome(),
        UiInspectionRelevanceOutcome::NotApplicableToTarget { .. }
    ));
}

#[test]
fn cross_family_expansion_requires_named_family_or_link_kind() {
    let local =
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Obligation));
    let cross_family = UiInspectionRelevance::local(
        UiRelevanceFilter::family(UiRelevanceFamily::Obligation)
            .include_family(UiRelevanceFamily::Aspect),
    );
    let cross_link = UiInspectionRelevance::local(
        UiRelevanceFilter::family(UiRelevanceFamily::Obligation)
            .include_link(UiEvidenceLinkKind::CorrespondsTo),
    );

    assert!(!local.filter().widens_beyond_local());
    assert!(cross_family.filter().widens_beyond_local());
    assert!(cross_link.filter().widens_beyond_local());
}

fn declared_surface_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.inspection.relevance.declared",
            )
            .with_semantic_artifact_spec(declared_surface_region_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn empty_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed")
}

fn declared_surface_region_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.region.root"),
        UiDslSemanticFamily::Region,
        UiDslSourceProvenance::file_authored("app/inspection_relevance.wui", 1),
    )
    .with_structural_token(UiDslStructuralToken::new("region:sidebar"))
}
