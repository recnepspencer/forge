use std::sync::Arc;

use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken,
};
use worth_ui_inspection::{
    UiEvidenceFamily, UiEvidenceRichness, UiInspectionQuery, UiInspectionRelevance,
    UiInspectionRelevanceOutcome, UiInspectionScope, UiInspectionTarget, UiRelevanceFilter,
};

use super::UiDeclarationAuthoredEvidenceIndex;
use crate::declaration::UiDeclarationArtifact;
use crate::facade::{WorthUi, WorthUiApp, WorthUiRustAuthoredDeclarationFixture};
use crate::graph::{UiRuntimeDataInstanceKeyToken, UiRuntimeInstanceBasisAdmission};

#[test]
fn authored_lookup_omits_admission_ref_when_declaration_correspondence_is_ambiguous() {
    let app = repeated_instance_app();
    let control = control_artifact(&app);
    let index = UiDeclarationAuthoredEvidenceIndex::rebuild(
        app.declaration_artifacts(),
        app.graph().snapshot(),
    );
    let by_identity = index
        .lookup_declaration_identity(control.identity().inspection_identity())
        .expect("ambiguous authored lookup should still retain declaration evidence");
    let by_provenance = index
        .lookup_authored_provenance(
            &control
                .provenance()
                .inspection_authored_source_provenance_ref(),
        )
        .expect("ambiguous authored provenance should still retain declaration evidence");

    assert_eq!(by_identity.cost().declaration_identity_index_lookups(), 1);
    assert_eq!(by_identity.cost().authored_provenance_index_lookups(), 0);
    assert_eq!(by_identity.cost().declaration_artifact_scans(), 0);
    assert_eq!(by_provenance.cost().declaration_identity_index_lookups(), 0);
    assert_eq!(by_provenance.cost().authored_provenance_index_lookups(), 1);
    assert_eq!(by_provenance.cost().declaration_artifact_scans(), 0);
    assert_eq!(
        by_identity.neighborhood().refs(),
        by_provenance.neighborhood().refs()
    );
    assert_eq!(by_identity.neighborhood().refs().len(), 1);
    assert_eq!(
        by_identity.neighborhood().refs()[0].family(),
        UiEvidenceFamily::Declaration
    );
}

#[test]
fn public_authored_lookup_omits_admission_ref_when_declaration_correspondence_is_ambiguous() {
    let app = repeated_instance_app();
    let control = control_artifact(&app);
    let observation_before = app.inspection_observation();
    let by_identity = app.inspect(declaration_identity_query(
        control.identity().inspection_identity(),
    ));
    let by_provenance = app.inspect(authored_provenance_query(
        control
            .provenance()
            .inspection_authored_source_provenance_ref(),
    ));
    let observation_after = app.inspection_observation();
    let identity_slice = by_identity
        .evidence_slice()
        .expect("ambiguous declaration identity should retain declaration evidence");
    let provenance_slice = by_provenance
        .evidence_slice()
        .expect("ambiguous authored provenance should retain declaration evidence");

    assert_eq!(
        by_identity.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        by_provenance.relevance_outcome(),
        UiInspectionRelevanceOutcome::Matched
    );
    assert_eq!(
        observation_after.authored_lookup_count() - observation_before.authored_lookup_count(),
        2
    );
    assert_eq!(
        by_identity.evidence_slice_ref(),
        by_provenance.evidence_slice_ref()
    );
    assert_eq!(identity_slice.refs(), provenance_slice.refs());
    assert_eq!(identity_slice.refs().len(), 1);
    assert_eq!(
        identity_slice.refs()[0].family(),
        UiEvidenceFamily::Declaration
    );
    assert!(!identity_slice
        .refs()
        .iter()
        .any(|evidence_ref| evidence_ref.family() == UiEvidenceFamily::Admission));
}

#[test]
fn rebuilding_authored_index_from_authority_preserves_public_lookup_answers() {
    let mut app = WorthUi::app()
        .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(
            WorthUiRustAuthoredDeclarationFixture::named(
                "worth-ui.runtime.authored-evidence-index.rebuild",
            )
            .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let control = control_artifact(&app);
    let declaration_identity = control.identity().inspection_identity();
    let authored_provenance = control
        .provenance()
        .inspection_authored_source_provenance_ref();
    let before_identity = app.inspect(declaration_identity_query(declaration_identity));
    let before_provenance = app.inspect(authored_provenance_query(authored_provenance.clone()));

    app.rebuild_prepared_derived_indexes();

    let rebuilt_index = UiDeclarationAuthoredEvidenceIndex::rebuild(
        app.declaration_artifacts(),
        app.graph().snapshot(),
    );
    let rebuilt_identity = rebuilt_index
        .lookup_declaration_identity(declaration_identity)
        .expect("rebuilt index should retain declaration-identity neighborhood");
    let rebuilt_provenance = rebuilt_index
        .lookup_authored_provenance(&authored_provenance)
        .expect("rebuilt index should retain authored-provenance neighborhood");

    let after_identity = app.inspect(declaration_identity_query(declaration_identity));
    let after_provenance = app.inspect(authored_provenance_query(authored_provenance));

    assert_eq!(
        rebuilt_identity.cost().declaration_identity_index_lookups(),
        1
    );
    assert_eq!(rebuilt_identity.cost().declaration_artifact_scans(), 0);
    assert_eq!(
        rebuilt_provenance
            .cost()
            .authored_provenance_index_lookups(),
        1
    );
    assert_eq!(rebuilt_provenance.cost().declaration_artifact_scans(), 0);

    assert_eq!(
        before_identity.authority_generation(),
        after_identity.authority_generation()
    );
    assert_eq!(
        before_identity.evidence_slice_ref(),
        after_identity.evidence_slice_ref()
    );
    assert_eq!(
        before_identity
            .evidence_slice()
            .map(|slice| slice.refs().to_vec()),
        after_identity
            .evidence_slice()
            .map(|slice| slice.refs().to_vec())
    );
    assert_eq!(
        before_provenance.authority_generation(),
        after_provenance.authority_generation()
    );
    assert_eq!(
        before_provenance.evidence_slice_ref(),
        after_provenance.evidence_slice_ref()
    );
    assert_eq!(
        before_provenance
            .evidence_slice()
            .map(|slice| slice.refs().to_vec()),
        after_provenance
            .evidence_slice()
            .map(|slice| slice.refs().to_vec())
    );
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("ui.workflow.editor"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/authored_evidence_index.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:workflow"))
}

fn runtime_basis(
    runtime_key: &str,
    declaration_identity: &crate::declaration::UiDeclarationIdentity,
) -> UiRuntimeInstanceBasisAdmission {
    UiRuntimeInstanceBasisAdmission::admit_runtime_data_keyed(
        declaration_identity,
        UiRuntimeDataInstanceKeyToken::new(Arc::<str>::from(runtime_key)),
    )
    .expect("typed runtime basis key should admit")
}

fn declaration_identity_query(
    identity: worth_ui_inspection::UiInspectionDeclarationIdentity,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::declaration_identity(identity),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn authored_provenance_query(
    provenance: worth_ui_inspection::UiAuthoredSourceProvenanceRef,
) -> UiInspectionQuery {
    UiInspectionQuery::new(
        UiInspectionTarget::authored_source_provenance(provenance),
        UiInspectionScope::graph(),
    )
    .with_relevance(UiInspectionRelevance::local(
        UiRelevanceFilter::target_local(),
    ))
    .with_richness(UiEvidenceRichness::refs_only())
}

fn repeated_instance_app() -> WorthUiApp {
    let dsl_package = WorthUiRustAuthoredDeclarationFixture::named(
        "worth-ui.runtime.authored-evidence-index.ambiguous",
    )
    .with_semantic_artifact_spec(control_spec());
    let baseline = WorthUi::app()
        .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(dsl_package.clone())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let control_handoff = control_artifact(&baseline)
        .graph_handoff()
        .expect("control should lower to graph handoff")
        .clone();
    let runtime_bases = [
        runtime_basis("row:user-7", control_handoff.identity()),
        runtime_basis("row:user-8", control_handoff.identity()),
    ];

    WorthUi::app()
        .with_change_profile(crate::facade::rebind::UiChangeProfile::platform_pulse())
        .with_rust_authored_declaration_fixture(dsl_package)
        .with_runtime_instance_basis_admissions(runtime_bases)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("typed repeated-instance input should prepare one complete app authority")
}

fn control_artifact(app: &WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/authored_evidence_index.wui"
        })
        .expect("control artifact should exist")
}
