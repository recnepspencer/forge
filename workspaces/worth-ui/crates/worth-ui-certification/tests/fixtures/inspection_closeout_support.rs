use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRef, UiEvidenceRichness, UiEvidenceSliceOmission,
    UiInspectionObligationFamily, UiInspectionObligationRelevanceDetail, UiInspectionQuery,
    UiInspectionRelevance, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui_certification::{
    WorthUiCertificationBuilderExt, WorthUiRustAuthoredDeclarationFixture,
};
use worth_ui_dsl::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily,
    UiDslSemanticKey, UiDslSourceProvenance, UiDslStructuralToken,
};

pub fn closeout_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.inspection-closeout",
            )
            .with_semantic_artifact_spec(
                UiDslSemanticArtifactSpec::new(
                    UiDslSemanticKey::new("ui.workflow.inspection_closeout"),
                    UiDslSemanticFamily::Control,
                    UiDslSourceProvenance::file_authored("app/inspection_closeout.wui", 0),
                )
                .with_structural_token(UiDslStructuralToken::new("control:inspection-closeout"))
                .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
                .with_published_aspect(UiDslAspectName::new("content.text"))
                .with_consumed_aspect(UiDslAspectName::new("content.text")),
            ),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

pub fn graph_node_digest(app: &WorthUiApp) -> u64 {
    app.graph()
        .lookup()
        .declaration_instances(app.declaration_artifacts()[0].identity())
        .value()[0]
        .digest()
}

pub fn declaration_identity_query(
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

pub fn authored_provenance_query(
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

pub fn graph_identity_query(graph_node_digest: u64) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::graph_node_identity(graph_node_digest),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

pub fn published_aspect_query() -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::published_aspect("content.text"),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Aspect,
    )))
    .with_richness(UiEvidenceRichness::refs_only())
}

pub fn obligation_query(graph_node_digest: u64, touch_identity_digest: u64) -> UiInspectionQuery {
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

pub fn family_counts(
    receipt: &worth_ui::facade::inspection::UiInspectionReceipt,
) -> Vec<(UiEvidenceFamily, usize)> {
    let slice = receipt
        .evidence_slice()
        .expect("receipt should retain a slice");
    slice
        .family_summaries()
        .iter()
        .map(|summary| (summary.family(), summary.ref_count()))
        .collect()
}

pub type InspectionReceiptSnapshot = (
    Option<u64>,
    Option<u64>,
    Vec<UiEvidenceRef>,
    Vec<(UiEvidenceFamily, usize)>,
    Option<UiEvidenceSliceOmission>,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    bool,
);

pub fn receipt_snapshot(
    receipt: &worth_ui::facade::inspection::UiInspectionReceipt,
) -> InspectionReceiptSnapshot {
    let slice = receipt
        .evidence_slice()
        .expect("receipt should retain a slice");
    let cost = receipt.cost().expect("receipt should expose cost");
    (
        receipt.authority_generation().map(|value| value.as_u64()),
        receipt.evidence_slice_ref().map(|value| value.digest()),
        slice.refs().to_vec(),
        family_counts(receipt),
        slice.omission(),
        cost.index_lookups(),
        cost.evidence_refs_considered(),
        cost.evidence_refs_returned(),
        cost.materialized_records(),
        cost.omitted_by_budget(),
        cost.traversals_denied(),
        cost.broad_scan_used(),
    )
}
