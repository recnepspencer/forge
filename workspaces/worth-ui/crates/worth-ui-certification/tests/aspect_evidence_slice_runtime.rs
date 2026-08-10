use worth_ui::facade::app::{WorthUi, WorthUiApp};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionAspectRelevanceDetail, UiInspectionQuery,
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

const ALPHA_MODULE_PATH: &str = "app/aspect_slice_alpha.wui";
const BETA_MODULE_PATH: &str = "app/aspect_slice_beta.wui";
const SHARED_ASPECT: &str = "content.text";

#[test]
fn provenance_expanded_aspect_slices_keep_exact_public_shape_and_replay_stability() {
    let app = aspect_slice_app();
    let plain = app.inspect(published_aspect_query());
    let first = app.inspect(published_aspect_with_provenance_query());
    let second = app.inspect(published_aspect_with_provenance_query());
    let plain_slice = plain
        .evidence_slice()
        .expect("plain published aspect query should return a slice");
    let first_slice = first
        .evidence_slice()
        .expect("provenance-expanded aspect query should return a slice");
    let second_slice = second
        .evidence_slice()
        .expect("replayed provenance-expanded aspect query should return a slice");

    assert_eq!(plain.evidence_slice_ref(), Some(plain_slice.slice_ref()));
    assert_eq!(first.evidence_slice_ref(), Some(first_slice.slice_ref()));
    assert_eq!(second.evidence_slice_ref(), Some(second_slice.slice_ref()));
    assert_eq!(
        first.authority_generation(),
        Some(first_slice.authority_generation())
    );
    assert_eq!(
        second.authority_generation(),
        Some(second_slice.authority_generation())
    );
    assert_eq!(first_slice.slice_ref(), second_slice.slice_ref());
    assert_eq!(first_slice.refs(), second_slice.refs());
    assert_eq!(
        first_slice.family_summaries(),
        second_slice.family_summaries()
    );
    assert_eq!(first_slice.omission(), None);
    assert!(first_slice.materialized_detail().is_none());
    assert_eq!(
        ordered_ref_keys(first_slice.refs()),
        sorted_ref_keys(first_slice.refs())
    );
    assert_eq!(plain_slice.refs().len(), 4);
    assert_eq!(first_slice.refs().len(), 6);
    assert_eq!(
        family_counts(plain_slice.refs()),
        vec![(UiEvidenceFamily::Aspect, 4)]
    );
    assert_eq!(
        family_counts(first_slice.refs()),
        vec![
            (UiEvidenceFamily::Declaration, 2),
            (UiEvidenceFamily::Aspect, 4)
        ]
    );
    assert_eq!(
        declaration_ref_digests(first_slice.refs()),
        declaration_artifact_digests(&app, &[ALPHA_MODULE_PATH, BETA_MODULE_PATH])
    );
}

fn aspect_slice_app() -> WorthUiApp {
    WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.certification.aspect-evidence-slice",
            )
            .with_semantic_artifact_spec(
                control_spec("ui.aspect.alpha", ALPHA_MODULE_PATH, "control:alpha")
                    .with_published_aspect(UiDslAspectName::new(SHARED_ASPECT)),
            )
            .with_semantic_artifact_spec(
                control_spec("ui.aspect.beta", BETA_MODULE_PATH, "control:beta")
                    .with_published_aspect(UiDslAspectName::new(SHARED_ASPECT)),
            ),
        )
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("application preparation should succeed")
}

fn published_aspect_query() -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::published_aspect(SHARED_ASPECT),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(UiRelevanceFilter::family(
        UiRelevanceFamily::Aspect,
    )))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn published_aspect_with_provenance_query() -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::published_aspect(SHARED_ASPECT),
        UiInspectionScope::graph(),
    )
    .with_relevance(
        UiInspectionRelevance::local(UiRelevanceFilter::family(UiRelevanceFamily::Aspect))
            .with_aspect_detail(
                UiInspectionAspectRelevanceDetail::new().include_direct_provenance_refs(),
            ),
    )
    .with_richness(UiEvidenceRichness::refs_only())
}

fn control_spec(
    semantic_key: &str,
    module_path: &str,
    structural_token: &str,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(semantic_key),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored(module_path, 0),
    )
    .with_structural_token(UiDslStructuralToken::new(structural_token))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
}

fn declaration_artifact_digests(app: &WorthUiApp, module_paths: &[&str]) -> Vec<u64> {
    let mut digests = module_paths
        .iter()
        .map(|path| {
            app.declaration_artifacts()
                .iter()
                .find(|artifact| artifact.provenance().source_provenance().module_path() == *path)
                .expect("declaration artifact should resolve by module path")
                .identity()
                .digest()
                .raw()
        })
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

fn declaration_ref_digests(refs: &[worth_ui::facade::inspection::UiEvidenceRef]) -> Vec<u64> {
    let mut digests = refs
        .iter()
        .filter(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Declaration)
        .map(|evidence_ref| evidence_ref.identity().digest())
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
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

fn ordered_ref_keys(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
) -> Vec<(UiEvidenceFamily, u64, u64, u64)> {
    refs.iter().map(ref_key).collect()
}

fn sorted_ref_keys(
    refs: &[worth_ui::facade::inspection::UiEvidenceRef],
) -> Vec<(UiEvidenceFamily, u64, u64, u64)> {
    let mut keys = ordered_ref_keys(refs);
    keys.sort_unstable();
    keys
}

fn ref_key(
    evidence_ref: &worth_ui::facade::inspection::UiEvidenceRef,
) -> (UiEvidenceFamily, u64, u64, u64) {
    (
        evidence_ref.family(),
        evidence_ref.authority_generation().as_u64(),
        evidence_ref.identity().digest(),
        evidence_ref.handle().handle_digest(),
    )
}
