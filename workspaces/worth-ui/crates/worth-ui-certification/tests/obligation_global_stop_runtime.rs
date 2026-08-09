use worth_ui::facade::admission::WorthUiAdmissionExt;
use worth_ui::facade::graph::{UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchTiming};
use worth_ui::facade::inspection::{
    UiEvidenceMaterializedDetail, UiEvidenceRichness, UiInspectionObligationVerdictClass,
    UiInspectionObligationVerdictPosture, UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};
use worth_ui::facade::{app::WorthUi, declaration::UiDeclarationArtifact};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};
use worth_ui_runtime::facade::admission::UiAdmissionReport;

#[test]
fn distinct_global_stop_reports_keep_distinct_public_refs_for_the_same_stop_posture() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let foreign = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.foreign-global-stop",
            )
            .with_semantic_artifact_spec(foreign_spec("foreign/one.wui", "foreign.one"))
            .with_semantic_artifact_spec(foreign_spec("foreign/two.wui", "foreign.two")),
        )
        .freeze()
        .expect("application preparation should succeed");

    let left_touch =
        foreign_declaration_touch(&foreign, foreign_artifact(&foreign, "foreign/one.wui"));
    let right_touch =
        foreign_declaration_touch(&foreign, foreign_artifact(&foreign, "foreign/two.wui"));

    let left_report = app
        .admission()
        .admit_selected_obligations(&app.admission().select_obligations(&left_touch));
    let right_report = app
        .admission()
        .admit_selected_obligations(&app.admission().select_obligations(&right_touch));

    let left = public_global_stop(&left_report, &left_touch);
    let right = public_global_stop(&right_report, &right_touch);

    assert_eq!(left.1.denial_posture(), None);
    assert_eq!(
        left.1.verdict_class(),
        Some(UiInspectionObligationVerdictClass::Violation)
    );
    assert_eq!(
        left.1.verdict_posture(),
        Some(UiInspectionObligationVerdictPosture::Unsupported)
    );
    assert_eq!(left.1.dispatch_posture(), None);
    assert_eq!(left.1.verdict_posture(), right.1.verdict_posture());
    assert_ne!(left.0.identity(), right.0.identity());
    assert_ne!(left.0.handle(), right.0.handle());
    assert_ne!(
        left.0.authority_binding().artifact_identity().digest(),
        right.0.authority_binding().artifact_identity().digest()
    );
}

fn public_global_stop(
    report: &UiAdmissionReport,
    touch: &worth_ui::facade::graph::UiGraphTouchDescriptor,
) -> (
    worth_ui::facade::inspection::UiEvidenceRef,
    worth_ui::facade::inspection::UiInspectionObligationReasonProjection,
) {
    let receipt = report.inspect(
        UiInspectionQuery::new(
            UiInspectionTarget::obligation_graph_node(
                touch.target().graph_node_identity().digest(),
            ),
            UiInspectionScope::graph(),
        )
        .with_richness(UiEvidenceRichness::materialized_detail()),
    );
    let slice = receipt
        .evidence_slice()
        .expect("global-stop report should expose an evidence slice");
    let rows = slice
        .materialized_detail()
        .and_then(|detail| match detail {
            UiEvidenceMaterializedDetail::Obligation(receipt) => Some(receipt.projections()),
            _ => None,
        })
        .expect("global-stop report should materialize obligation detail");
    let row = rows
        .iter()
        .find(|projection| {
            projection.decision()
                == worth_ui::facade::inspection::UiInspectionObligationDecision::Verdict
        })
        .expect("global-stop inspection should retain a verdict row");
    let evidence_ref = slice
        .refs()
        .iter()
        .copied()
        .find(|reference| reference.handle().handle_digest() == row.handle_digest())
        .expect("global-stop verdict row should match a public evidence ref");

    (evidence_ref, row.clone())
}

fn foreign_spec(module_path: &str, semantic_key: &str) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(module_path, 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:foreign"))
}

fn foreign_artifact<'a>(
    app: &'a worth_ui::facade::app::WorthUiApp,
    module_path: &str,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| artifact.provenance().source_provenance().module_path() == module_path)
        .expect("foreign artifact should exist")
}

fn foreign_declaration_touch(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphTouchDescriptor {
    let graph = app.graph();
    let node = graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("foreign declaration should admit one graph node");
    graph
        .touches()
        .from_node(
            graph
                .touches()
                .declaration_change_receipt(artifact)
                .expect("foreign declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            node,
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("foreign declaration touch should admit")
}
