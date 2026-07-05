use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceMaterializedDetail, UiEvidenceRichness, UiEvidenceSliceOmission,
    UiInspectionCostReceipt, UiInspectionQuery, UiInspectionReceipt, UiInspectionRelevance,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget, UiRelevanceFamily,
    UiRelevanceFilter,
};
use worth_ui::facade::{
    UiAuthoredSourceProvenanceRef, UiEvidenceRef, UiInspectionDeclarationIdentity,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, WorthUiDslPackage,
};

#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
mod obligation_dispatch_prerequisite_support;

#[test]
fn equivalent_declaration_slice_routes_return_the_same_canonical_slice_shape() {
    let app = declaration_lookup_app();
    let artifact = authored_artifact(&app);
    let identity_receipt = app.inspect(declaration_identity_query(
        artifact.identity().inspection_identity(),
    ));
    let provenance_receipt = app.inspect(authored_provenance_query(
        artifact
            .provenance()
            .inspection_authored_source_provenance_ref(),
    ));
    let identity_slice = identity_receipt
        .evidence_slice()
        .expect("declaration identity query should return a slice");
    let provenance_slice = provenance_receipt
        .evidence_slice()
        .expect("authored provenance query should return a slice");

    assert_eq!(
        identity_receipt.query(),
        &declaration_identity_query(artifact.identity().inspection_identity())
    );
    assert_eq!(
        identity_receipt.authority_generation(),
        Some(identity_slice.authority_generation())
    );
    assert_eq!(
        provenance_receipt.authority_generation(),
        Some(provenance_slice.authority_generation())
    );
    assert_eq!(
        identity_receipt.evidence_slice_ref(),
        Some(identity_slice.slice_ref())
    );
    assert_eq!(
        provenance_receipt.evidence_slice_ref(),
        Some(provenance_slice.slice_ref())
    );
    assert_eq!(identity_slice.slice_ref(), provenance_slice.slice_ref());
    assert_eq!(identity_slice.refs(), provenance_slice.refs());
    assert_eq!(
        identity_slice.family_summaries(),
        provenance_slice.family_summaries()
    );
    assert_eq!(identity_slice.omission(), None);
    assert_eq!(identity_slice.materialized_detail(), None);
    assert_eq!(
        identity_slice
            .family_summaries()
            .iter()
            .map(|summary| (summary.family(), summary.ref_count()))
            .collect::<Vec<_>>(),
        vec![
            (UiEvidenceFamily::Declaration, 1),
            (UiEvidenceFamily::Admission, 1)
        ]
    );
    assert_eq!(
        ordered_ref_keys(identity_slice.refs()),
        sorted_ref_keys(identity_slice.refs())
    );
}

#[test]
fn obligation_slice_keeps_refs_and_detail_separate_with_explicit_scope_omission() {
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
        .expect("refs-only obligation query should return a slice");
    let rich_slice = rich
        .evidence_slice()
        .expect("rich obligation query should return a slice");

    assert_eq!(
        refs_only.evidence_slice_ref(),
        Some(refs_only_slice.slice_ref())
    );
    assert_eq!(rich.evidence_slice_ref(), Some(rich_slice.slice_ref()));
    assert_eq!(refs_only_slice.refs(), rich_slice.refs());
    assert_eq!(refs_only_slice.slice_ref(), rich_slice.slice_ref());
    assert_eq!(
        refs_only_slice.omission(),
        Some(UiEvidenceSliceOmission::ByScope {
            scope: UiInspectionScope::Graph,
        })
    );
    assert_eq!(rich_slice.omission(), None);
    assert!(refs_only_slice.materialized_detail().is_none());
    assert!(matches!(
        rich_slice.materialized_detail(),
        Some(UiEvidenceMaterializedDetail::Obligation(_))
    ));
    assert_eq!(
        refs_only_slice
            .family_summaries()
            .iter()
            .map(|summary| (summary.family(), summary.ref_count()))
            .collect::<Vec<_>>(),
        vec![(UiEvidenceFamily::Obligation, refs_only_slice.refs().len())]
    );
}

#[test]
fn declaration_slice_replay_and_rebuild_stay_stable_after_assembly() {
    let app = declaration_lookup_app();
    let query =
        declaration_identity_query(authored_artifact(&app).identity().inspection_identity());
    let first = receipt_slice_facts(&app.inspect(query.clone()));
    let second = receipt_slice_facts(&app.inspect(query.clone()));

    let rebuilt = declaration_lookup_app();
    let rebuilt_query =
        declaration_identity_query(authored_artifact(&rebuilt).identity().inspection_identity());
    let rebuilt_facts = receipt_slice_facts(&rebuilt.inspect(rebuilt_query));

    assert_eq!(first, second);
    assert_eq!(first, rebuilt_facts);
}

#[test]
fn obligation_slice_replay_preserves_omission_detail_and_cost_posture() {
    let app = obligation_dispatch_prerequisite_support::query_touch_app();
    let touch = obligation_dispatch_prerequisite_support::query_touch(&app);
    let refs_query = obligation_touch_query(
        touch.target().graph_node_identity().digest(),
        touch.identity_digest(),
    )
    .with_richness(UiEvidenceRichness::refs_only());
    let rich_query = obligation_touch_query(
        touch.target().graph_node_identity().digest(),
        touch.identity_digest(),
    )
    .with_richness(UiEvidenceRichness::materialized_detail());

    let refs_first = receipt_slice_facts(&app.inspect(refs_query.clone()));
    let rich_first = receipt_slice_facts(&app.inspect(rich_query.clone()));
    let refs_second = receipt_slice_facts(&app.inspect(refs_query));
    let rich_second = receipt_slice_facts(&app.inspect(rich_query));

    assert_eq!(refs_first, refs_second);
    assert_eq!(rich_first, rich_second);
}

fn declaration_lookup_app() -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.evidence-slice")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.editor"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored("app/evidence_slice_runtime.wui", 0),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:workflow")),
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
                == "app/evidence_slice_runtime.wui"
        })
        .expect("slice app should contain the authored artifact")
}

fn declaration_identity_query(identity: UiInspectionDeclarationIdentity) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::declaration_identity(identity),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn authored_provenance_query(provenance: UiAuthoredSourceProvenanceRef) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::authored_source_provenance(provenance),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
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

fn ordered_ref_keys(refs: &[UiEvidenceRef]) -> Vec<(UiEvidenceFamily, u64, u64, u64)> {
    refs.iter().map(ref_key).collect()
}

fn sorted_ref_keys(refs: &[UiEvidenceRef]) -> Vec<(UiEvidenceFamily, u64, u64, u64)> {
    let mut keys = ordered_ref_keys(refs);
    keys.sort_unstable();
    keys
}

fn ref_key(evidence_ref: &UiEvidenceRef) -> (UiEvidenceFamily, u64, u64, u64) {
    (
        evidence_ref.family(),
        evidence_ref.authority_generation().as_u64(),
        evidence_ref.identity().digest(),
        evidence_ref.handle().handle_digest(),
    )
}

#[derive(Debug, Eq, PartialEq)]
struct ReceiptSliceFacts {
    outcome: UiInspectionRelevanceOutcome,
    authority_generation: Option<u64>,
    slice_ref: Option<u64>,
    ref_keys: Vec<(UiEvidenceFamily, u64, u64, u64)>,
    family_summaries: Option<Vec<(UiEvidenceFamily, usize)>>,
    omission: Option<UiEvidenceSliceOmission>,
    detail_kind: Option<DetailKind>,
    cost: Option<CostFacts>,
}

#[derive(Debug, Eq, PartialEq)]
enum DetailKind {
    Obligation(usize),
    Other,
}

#[derive(Debug, Eq, PartialEq)]
struct CostFacts {
    index_lookups: usize,
    considered: usize,
    returned: usize,
    materialized_records: usize,
    omitted_by_budget: usize,
    traversals_denied: usize,
    broad_scan_used: bool,
}

fn receipt_slice_facts(receipt: &UiInspectionReceipt) -> ReceiptSliceFacts {
    let slice = receipt.evidence_slice();

    ReceiptSliceFacts {
        outcome: receipt.relevance_outcome(),
        authority_generation: receipt
            .authority_generation()
            .map(|generation| generation.as_u64()),
        slice_ref: receipt
            .evidence_slice_ref()
            .map(|slice_ref| slice_ref.digest()),
        ref_keys: slice
            .map(|value| ordered_ref_keys(value.refs()))
            .unwrap_or_default(),
        family_summaries: slice.map(|value| {
            value
                .family_summaries()
                .iter()
                .map(|summary| (summary.family(), summary.ref_count()))
                .collect()
        }),
        omission: slice.and_then(|value| value.omission()),
        detail_kind: slice.and_then(|value| detail_kind(value.materialized_detail())),
        cost: receipt.cost().map(cost_facts),
    }
}

fn detail_kind(detail: Option<&UiEvidenceMaterializedDetail>) -> Option<DetailKind> {
    match detail {
        Some(UiEvidenceMaterializedDetail::Obligation(receipt)) => {
            Some(DetailKind::Obligation(receipt.projections().len()))
        }
        Some(_) => Some(DetailKind::Other),
        None => None,
    }
}

fn cost_facts(cost: UiInspectionCostReceipt) -> CostFacts {
    CostFacts {
        index_lookups: cost.index_lookups(),
        considered: cost.evidence_refs_considered(),
        returned: cost.evidence_refs_returned(),
        materialized_records: cost.materialized_records(),
        omitted_by_budget: cost.omitted_by_budget(),
        traversals_denied: cost.traversals_denied(),
        broad_scan_used: cost.broad_scan_used(),
    }
}
