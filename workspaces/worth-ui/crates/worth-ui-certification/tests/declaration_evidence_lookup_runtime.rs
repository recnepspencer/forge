use worth_ui::facade::admission::{UiAdmissionTarget, UiAdmissionWorld};
use worth_ui::facade::inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget, UiRelevanceFilter,
};
use worth_ui::facade::{
    app::WorthUi, UiAuthoredSourceProvenanceRef, UiInspectionDeclarationIdentity,
};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn declaration_identity_lookup_returns_authored_evidence_refs_on_public_lookup_lane() {
    let app = declaration_lookup_app("ui.workflow.editor", "control:workflow");
    let artifact = authored_artifact(&app);
    let identity = artifact.identity().inspection_identity();
    let observation_before = app.inspection_observation();

    let receipt = app.inspect(declaration_identity_query(identity));
    let slice = receipt
        .evidence_slice()
        .expect("declaration identity lookup should retain one authored evidence slice");
    let observation_after = app.inspection_observation();

    assert_eq!(
        receipt.authority_generation(),
        Some(
            worth_ui::facade::inspection::UiEvidenceAuthorityGeneration::new(
                app.graph().generation().as_u64(),
            )
        )
    );
    assert_eq!(
        observation_after.authored_lookup_count() - observation_before.authored_lookup_count(),
        1
    );
    assert_eq!(slice.refs().len(), 2);
    assert!(slice
        .refs()
        .iter()
        .any(|evidence_ref| { evidence_ref.family() == UiEvidenceFamily::Declaration }));
    assert!(slice.refs().contains(&real_admission_ref(&app, artifact)));
}

#[test]
fn authored_source_provenance_lookup_converges_with_declaration_identity() {
    let app = declaration_lookup_app("ui.workflow.editor", "control:workflow");
    let artifact = authored_artifact(&app);
    let declaration_identity = artifact.identity().inspection_identity();
    let authored_provenance = artifact
        .provenance()
        .inspection_authored_source_provenance_ref();
    let observation_before = app.inspection_observation();

    let identity_receipt = app.inspect(declaration_identity_query(declaration_identity));
    let provenance_receipt = app.inspect(authored_provenance_query(authored_provenance));
    let observation_after = app.inspection_observation();

    assert_eq!(
        observation_after.authored_lookup_count() - observation_before.authored_lookup_count(),
        2
    );

    assert_eq!(
        identity_receipt
            .evidence_slice()
            .map(|slice| slice.refs().to_vec()),
        provenance_receipt
            .evidence_slice()
            .map(|slice| slice.refs().to_vec())
    );
    assert_eq!(
        identity_receipt.evidence_slice_ref(),
        provenance_receipt.evidence_slice_ref()
    );
}

#[test]
fn authored_provenance_lookup_rejects_stale_source_generation_without_fuzzy_match() {
    let baseline = declaration_lookup_app("ui.workflow.editor", "control:workflow");
    let changed = declaration_lookup_app("ui.workflow.editor.changed", "control:workflow:changed");
    let stale_provenance = authored_artifact(&baseline)
        .provenance()
        .inspection_authored_source_provenance_ref();

    let receipt = changed.inspect(authored_provenance_query(stale_provenance));

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert!(receipt.evidence_slice().is_none());
    assert!(receipt.evidence_slice_ref().is_none());
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

#[test]
fn authored_source_generation_tracks_source_artifact_generation_not_declaration_digest() {
    let baseline = declaration_lookup_app("ui.workflow.editor", "control:workflow");
    let changed = WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declaration.lookup.changed")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.editor"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/declaration_evidence_lookup_runtime.wui",
                            0,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:workflow")),
                )
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new("ui.workflow.editor.sidebar"),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/declaration_evidence_lookup_runtime.wui",
                            1,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new("control:sidebar")),
                ),
        )
        .freeze()
        .expect("application preparation should succeed");
    let baseline_artifact = authored_artifact(&baseline);
    let changed_artifact = authored_artifact(&changed);
    let stale_provenance = baseline_artifact
        .provenance()
        .inspection_authored_source_provenance_ref();

    assert_eq!(
        baseline_artifact.provenance().semantic_input_digest(),
        changed_artifact.provenance().semantic_input_digest()
    );
    assert_ne!(
        baseline_artifact
            .provenance()
            .inspection_source_generation(),
        changed_artifact.provenance().inspection_source_generation()
    );

    let receipt = changed.inspect(authored_provenance_query(stale_provenance));

    assert_eq!(
        receipt.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert!(receipt.evidence_slice().is_none());
    assert!(receipt.evidence_slice_ref().is_none());
}

#[test]
fn admit_semantic_artifact_returns_package_authoritative_source_generation() {
    let package = WorthUiDslPackage::named("worth-ui.certification.declaration.lookup.authority")
        .with_semantic_artifact_spec(
            UiDslSemanticArtifactSpec::new(
                UiDslSemanticKey::new("ui.workflow.editor"),
                UiDslSemanticFamily::Control,
                UiDslSourceProvenance::file_authored(
                    "app/declaration_evidence_lookup_runtime.wui",
                    0,
                ),
            )
            .with_structural_token(UiDslStructuralToken::new("control:workflow")),
        );
    let admitted = package.admit_semantic_artifact(
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new("ui.workflow.sidebar"),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/declaration_evidence_lookup_runtime.wui", 1),
        )
        .with_structural_token(UiDslStructuralToken::new("control:sidebar")),
    );
    let authoritative = package
        .clone()
        .with_semantic_artifact_spec(
            UiDslSemanticArtifactSpec::new(
                UiDslSemanticKey::new("ui.workflow.sidebar"),
                UiDslSemanticFamily::Control,
                UiDslSourceProvenance::file_authored(
                    "app/declaration_evidence_lookup_runtime.wui",
                    1,
                ),
            )
            .with_structural_token(UiDslStructuralToken::new("control:sidebar")),
        )
        .admitted_declarations()[1]
        .clone();

    assert_eq!(
        admitted.source_artifact_generation(),
        authoritative.source_artifact_generation()
    );
}

fn declaration_lookup_app(semantic_key: &str, structural_token: &str) -> WorthUiApp {
    WorthUi::app()
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.declaration.lookup")
                .with_semantic_artifact_spec(
                    UiDslSemanticArtifactSpec::new(
                        UiDslSemanticKey::new(semantic_key),
                        UiDslSemanticFamily::Control,
                        UiDslSourceProvenance::file_authored(
                            "app/declaration_evidence_lookup_runtime.wui",
                            0,
                        ),
                    )
                    .with_structural_token(UiDslStructuralToken::new(structural_token)),
                ),
        )
        .freeze()
        .expect("application preparation should succeed")
}

type WorthUiApp = worth_ui::facade::app::WorthUiApp;

fn authored_artifact(app: &WorthUiApp) -> &worth_ui::facade::declaration::UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/declaration_evidence_lookup_runtime.wui"
        })
        .expect("lookup app should contain one file-authored declaration artifact")
}

fn real_admission_ref(
    app: &WorthUiApp,
    artifact: &worth_ui::facade::declaration::UiDeclarationArtifact,
) -> worth_ui::facade::inspection::UiEvidenceRef {
    let graph_node_identity = app
        .graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("lookup app should project one graph node for the authored declaration");

    app.admission()
        .report(UiAdmissionTarget::graph_node(
            graph_node_identity,
            UiAdmissionWorld::authoritative(),
        ))
        .evidence_ref()
}
