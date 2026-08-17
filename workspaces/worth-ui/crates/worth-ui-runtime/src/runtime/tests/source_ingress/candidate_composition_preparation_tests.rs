use crate::facade::lifecycle::{
    WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifactPosture;
use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_boundary_test_support::{
    lower_file_submission, lower_rust_submission, source_backed_package_component,
};
use crate::runtime::tests::source_ingress_test_support::{
    file_import_provider, file_import_provider_for, rust_import_provider,
};
use crate::runtime::WorthUiWatcherEvent;

#[path = "candidate_composition_preparation_test_support.rs"]
mod candidate_composition_preparation_test_support;

use candidate_composition_preparation_test_support::{
    convergence_submissions, prepare_convergence_apps, prepare_file_authored_package_app,
};

#[test]
fn file_watcher_uses_one_composition_pipeline_for_file_and_rust_inputs() {
    let snapshot = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let file_submission = lower_file_submission(
        file_import_provider(),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );
    let rust_submission = lower_rust_submission(
        rust_import_provider(),
        [WorthUiWatcherEvent::provider_revision("rust-authored")],
        snapshot.capabilities(),
    );

    assert_eq!(
        file_submission.composition_basis().candidate_basis(),
        rust_submission.composition_basis().candidate_basis()
    );
    assert_eq!(file_submission.authoring_lane().as_str(), "file-authored");
    assert_eq!(rust_submission.authoring_lane().as_str(), "rust-authored");
}

#[test]
fn equivalent_file_and_rust_compositions_share_semantic_basis_and_protocol() {
    let (file_submission, rust_submission) = convergence_submissions();

    assert_eq!(
        file_submission.composition_basis(),
        rust_submission.composition_basis()
    );
    assert_eq!(
        file_submission
            .composition_basis()
            .semantic_handoff()
            .identity(),
        rust_submission
            .composition_basis()
            .semantic_handoff()
            .identity()
    );
    assert_eq!(
        file_submission
            .composition_basis()
            .semantic_handoff()
            .protocol(),
        rust_submission
            .composition_basis()
            .semantic_handoff()
            .protocol()
    );
    assert_ne!(
        file_submission
            .composition_basis()
            .semantic_handoff()
            .authored_mode(),
        rust_submission
            .composition_basis()
            .semantic_handoff()
            .authored_mode()
    );
}

#[test]
fn equivalent_file_and_rust_compositions_prepare_equivalent_application_generations() {
    let (file_submission, rust_submission) = convergence_submissions();
    let (file_app, rust_app) = prepare_convergence_apps(file_submission, rust_submission);

    assert_eq!(
        file_app.generation_identity(),
        rust_app.generation_identity()
    );
    assert_eq!(
        file_app.generation_identity().semantic_package_identity(),
        file_app.prepared_authority().semantic_handoff().identity()
    );
    assert_eq!(
        rust_app.generation_identity().semantic_package_identity(),
        rust_app.prepared_authority().semantic_handoff().identity()
    );
    assert_eq!(
        file_app.prepared_authority().application_artifact_posture(),
        WorthUiPreparedApplicationArtifactPosture::SourceBacked
    );
    assert_eq!(
        rust_app.prepared_authority().application_artifact_posture(),
        WorthUiPreparedApplicationArtifactPosture::SourceBacked
    );
}

#[test]
fn prepared_file_and_rust_compositions_retain_exact_handoff_evidence() {
    let (file_submission, rust_submission) = convergence_submissions();
    let file_declaration_identity = file_submission
        .composition_basis()
        .declaration_source_identity()
        .clone();
    let rust_declaration_identity = rust_submission
        .composition_basis()
        .declaration_source_identity()
        .clone();
    let file_handoff = file_submission
        .composition_basis()
        .semantic_handoff()
        .clone();
    let rust_handoff = rust_submission
        .composition_basis()
        .semantic_handoff()
        .clone();
    let (file_app, rust_app) = prepare_convergence_apps(file_submission, rust_submission);

    assert_eq!(
        file_app.prepared_authority().declaration_source_identity(),
        &file_declaration_identity
    );
    assert_eq!(
        rust_app.prepared_authority().declaration_source_identity(),
        &rust_declaration_identity
    );
    assert_eq!(
        file_app.prepared_authority().semantic_handoff(),
        &file_handoff
    );
    assert_eq!(
        rust_app.prepared_authority().semantic_handoff(),
        &rust_handoff
    );
}

#[test]
fn import_drift_changes_exact_semantic_handoff_and_prepared_identity() {
    let snapshot = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("source snapshot should prepare");
    let left_submission = lower_file_submission(
        file_import_provider_for("app/panels/inspector.wui"),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );
    let right_submission = lower_file_submission(
        file_import_provider_for("app/panels/settings.wui"),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );

    assert_ne!(
        left_submission
            .composition_basis()
            .declaration_source_identity(),
        right_submission
            .composition_basis()
            .declaration_source_identity()
    );
    assert_ne!(
        left_submission
            .composition_basis()
            .semantic_handoff()
            .identity(),
        right_submission
            .composition_basis()
            .semantic_handoff()
            .identity()
    );
    assert_ne!(
        left_submission.composition_basis().candidate_basis(),
        right_submission.composition_basis().candidate_basis()
    );

    let left = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_candidate_submission(left_submission)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("left composition should prepare");
    let right = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_candidate_submission(right_submission)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("right composition should prepare");

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_ne!(
        left.prepared_authority().declaration_source_identity(),
        right.prepared_authority().declaration_source_identity()
    );
    assert_ne!(left.generation_identity(), right.generation_identity());
}

#[test]
fn candidate_snapshot_drift_returns_deterministic_preparation_denial_without_mutating_prior_truth()
{
    let baseline = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("baseline application should prepare");
    let baseline_generation = baseline.generation_identity().clone();
    let baseline_graph_generation = baseline.graph().generation();
    let make_submission = || {
        lower_file_submission(
            file_import_provider(),
            [WorthUiWatcherEvent::modified("app/main.wui")],
            baseline.capabilities(),
        )
    };
    let make_drifted_builder = || {
        WorthUi::app()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
            .register_component(source_backed_package_component(
                "workspace.component.phase7_snapshot_drift",
            ))
    };
    let drifted_snapshot = make_drifted_builder()
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("drifted capability posture should prepare");

    let first = match make_drifted_builder()
        .with_candidate_submission(make_submission())
        .freeze()
    {
        Ok(_) => panic!("foreign candidate snapshot must deny application preparation"),
        Err(denial) => denial,
    };
    let second = match make_drifted_builder()
        .with_candidate_submission(make_submission())
        .freeze()
    {
        Ok(_) => panic!("replayed foreign candidate snapshot must remain denied"),
        Err(denial) => denial,
    };

    assert_eq!(first, second);
    assert_eq!(
        first.phase(),
        WorthUiApplicationPreparationPhase::CandidateBasis
    );
    assert_eq!(
        first,
        WorthUiApplicationPreparationDenial::CandidateSnapshotMismatch {
            candidate_snapshot_digest: baseline.capabilities().digest().as_u64(),
            prepared_snapshot_digest: drifted_snapshot.capabilities().digest().as_u64(),
        }
    );
    assert_eq!(baseline.generation_identity(), &baseline_generation);
    assert_eq!(baseline.graph().generation(), baseline_graph_generation);
}

#[test]
fn file_authored_source_ingress_preserves_exact_declaration_positions() {
    let prepared = prepare_file_authored_package_app();
    let mut observed = prepared
        .declaration_artifacts()
        .iter()
        .filter(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/source_backed_package.wui"
        })
        .map(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            (
                provenance.module_path().to_owned(),
                provenance.declaration_index(),
            )
        })
        .collect::<Vec<_>>();
    observed.sort();

    assert_eq!(
        observed,
        vec![
            ("app/source_backed_package.wui".to_owned(), 0),
            ("app/source_backed_package.wui".to_owned(), 1),
            ("app/source_backed_package.wui".to_owned(), 2),
        ]
    );
}

#[test]
fn file_authored_source_ingress_preserves_structural_sizing_handoff() {
    let prepared = prepare_file_authored_package_app();
    let first_source_declaration = prepared
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/source_backed_package.wui"
        })
        .expect("source-backed declaration should be present");

    assert_eq!(
        first_source_declaration
            .graph_handoff()
            .expect("source-backed declaration handoff remains admitted")
            .mosaic_sizing_contract_id()
            .map(|identity| identity.as_str()),
        Some("workspace.sizing.mosaic_support")
    );
}
