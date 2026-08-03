use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::model::change::{
    expected_change, AuthoredMeaningDelta, ExpectedChangePosture, PixelDelta,
};
use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn production_semantic_posture_ignores_equal_and_different_pixel_twins() {
    let initial = FilesystemApplicationLifecycleScenario::current_source_text();
    let changed = classify(
        "phase-312-tt03-changed",
        &initial,
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    assert_eq!(
        posture(changed),
        expected_change(
            AuthoredMeaningDelta::Appearance,
            pixel_delta([0, 255, 0, 255], [0, 255, 0, 255]),
        ),
        "equal pixels cannot erase authored semantic change"
    );

    let evidence_only = classify(
        "phase-312-tt03-evidence-only",
        &initial,
        &format!("{initial}\n"),
    );
    assert_eq!(
        posture(evidence_only),
        expected_change(
            AuthoredMeaningDelta::ProvenanceOnly,
            pixel_delta([255, 255, 0, 255], [0, 255, 0, 255]),
        ),
        "different pixels cannot invent an authored semantic change"
    );

    let no_change = classify("phase-312-tt03-no-change", &initial, &initial);
    assert_eq!(
        posture(no_change),
        expected_change(
            AuthoredMeaningDelta::None,
            pixel_delta([255, 255, 0, 255], [0, 255, 0, 255]),
        )
    );
}

fn classify(
    label: &str,
    initial_source: &str,
    successor_source: &str,
) -> UiChangeClassificationOutcome {
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let workspace = FilesystemContractWorkspace::new(label);
    workspace.write("app/main.wui", initial_source);
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let capabilities = scenario.capability_application();
    let initial = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("initial filesystem source reads"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_application(initial)
        .launch()
        .expect("initial filesystem application launches");

    workspace.write("app/main.wui", successor_source);
    let candidate = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("successor filesystem source reads"),
        session.capabilities(),
    );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let outcome = session.classify_observations(admitted).unwrap();
    let _ = session.shutdown();
    workspace.close();
    outcome
}

fn posture(outcome: UiChangeClassificationOutcome) -> ExpectedChangePosture {
    match outcome {
        UiChangeClassificationOutcome::ObservedNoChange(_) => {
            ExpectedChangePosture::ObservedNoChange
        }
        UiChangeClassificationOutcome::EvidenceOnly(_) => ExpectedChangePosture::EvidenceOnly,
        UiChangeClassificationOutcome::Changed(_) => ExpectedChangePosture::Changed,
    }
}

fn pixel_delta(predecessor: [u8; 4], successor: [u8; 4]) -> PixelDelta {
    if predecessor == successor {
        PixelDelta::Equal
    } else {
        PixelDelta::Different
    }
}
