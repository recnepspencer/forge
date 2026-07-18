use crate::facade::lifecycle::{
    WorthUiApplicationPreparationDenial, WorthUiApplicationPreparationPhase,
};
use crate::facade::prepared_application_authority::WorthUiPreparedApplicationArtifactPosture;
use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_boundary_test_support::{
    lower_file_submission, lower_rust_submission, source_backed_package_component,
    source_backed_package_region, source_backed_package_sizing,
};
use crate::runtime::tests::source_ingress_test_support::{
    file_import_provider, file_import_provider_for, rust_import_provider,
};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

#[test]
fn file_watcher_uses_one_composition_pipeline_for_file_and_rust_inputs() {
    let snapshot = WorthUi::app()
        .freeze()
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
fn equivalent_file_and_rust_compositions_prepare_equivalent_application_generations() {
    let registered_builder = || {
        WorthUi::app().register_component(source_backed_package_component(
            "workspace.component.phase6_convergence",
        ))
    };
    let snapshot = registered_builder()
        .freeze()
        .expect("convergence snapshot should prepare");
    let file_submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("phase6-file").with_file(
            "app/main.wui",
            "component workspace.component.phase6_convergence {}",
        ),
        [WorthUiWatcherEvent::provider_revision("phase6-file")],
        snapshot.capabilities(),
    );
    let rust_submission = lower_rust_submission(
        WorthUiSourceProvider::rust_authored("phase6-rust").with_rust_authored_input(
            crate::source::WorthUiRustAuthoredArtifactInput::from_modules([
                crate::source::WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                    .with_component("workspace.component.phase6_convergence"),
            ]),
        ),
        [WorthUiWatcherEvent::provider_revision("phase6-rust")],
        snapshot.capabilities(),
    );

    assert_eq!(
        file_submission.composition_basis(),
        rust_submission.composition_basis()
    );
    let file_declaration_identity = file_submission
        .composition_basis()
        .declaration_source_identity()
        .clone();
    let rust_declaration_identity = rust_submission
        .composition_basis()
        .declaration_source_identity()
        .clone();
    let file_app = registered_builder()
        .with_candidate_submission(file_submission)
        .freeze()
        .expect("file composition should prepare");
    let rust_app = registered_builder()
        .with_candidate_submission(rust_submission)
        .freeze()
        .expect("Rust composition should prepare");

    assert_eq!(
        file_app.generation_identity(),
        rust_app.generation_identity()
    );
    assert_eq!(
        file_app.prepared_authority().declaration_source_identity(),
        &file_declaration_identity
    );
    assert_eq!(
        rust_app.prepared_authority().declaration_source_identity(),
        &rust_declaration_identity
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
fn canonical_artifact_drift_changes_prepared_identity_without_declaration_source_drift() {
    let snapshot = WorthUi::app()
        .freeze()
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

    assert_eq!(
        left_submission
            .composition_basis()
            .declaration_source_identity(),
        right_submission
            .composition_basis()
            .declaration_source_identity()
    );
    assert_ne!(
        left_submission.composition_basis().candidate_basis(),
        right_submission.composition_basis().candidate_basis()
    );

    let left = WorthUi::app()
        .with_candidate_submission(left_submission)
        .freeze()
        .expect("left composition should prepare");
    let right = WorthUi::app()
        .with_candidate_submission(right_submission)
        .freeze()
        .expect("right composition should prepare");

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left.prepared_authority().declaration_source_identity(),
        right.prepared_authority().declaration_source_identity()
    );
    assert_ne!(left.generation_identity(), right.generation_identity());
}

#[test]
fn candidate_snapshot_drift_returns_deterministic_preparation_denial_without_mutating_prior_truth()
{
    let baseline = WorthUi::app()
        .freeze()
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
        WorthUi::app().register_component(source_backed_package_component(
            "workspace.component.phase7_snapshot_drift",
        ))
    };
    let drifted_snapshot = make_drifted_builder()
        .freeze()
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
fn file_authored_source_ingress_prepares_declarations_without_package_extraction() {
    let registered_builder = || {
        WorthUi::app()
            .register_component(source_backed_package_component(
                "workspace.component.workflow_editor",
            ))
            .register_component(source_backed_package_component(
                "workspace.component.workflow_editor.peer_a",
            ))
            .register_component(source_backed_package_component(
                "workspace.component.workflow_editor.peer_b",
            ))
            .register_mosaic_region_kind(source_backed_package_region())
            .register_mosaic_sizing_contract(source_backed_package_sizing())
    };
    let snapshot = registered_builder()
        .freeze()
        .expect("application preparation should succeed");
    let submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("source-backed-package").with_file(
            "app/source_backed_package.wui",
            r#"
component workspace.component.workflow_editor {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}

component workspace.component.workflow_editor.peer_a {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
component workspace.component.workflow_editor.peer_b {
    region workspace.region.primary {
        sizing workspace.sizing.mosaic_support;
    }
}
"#,
        ),
        [WorthUiWatcherEvent::provider_revision(
            "source-backed-package",
        )],
        snapshot.capabilities(),
    );
    let prepared = registered_builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("the complete watched composition should prepare");

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
    assert_eq!(
        prepared
            .declaration_artifacts()
            .iter()
            .find(|artifact| {
                artifact.provenance().source_provenance().module_path()
                    == "app/source_backed_package.wui"
            })
            .expect("source-backed declaration should be present")
            .graph_handoff()
            .expect("source-backed declaration handoff remains admitted")
            .mosaic_sizing_contract_id()
            .map(|identity| identity.as_str()),
        Some("workspace.sizing.mosaic_support")
    );
}
