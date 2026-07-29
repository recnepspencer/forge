use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::UiProducedFactFamily;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn authored_change_classification_is_exact_and_evidence_aware() {
    let current = FilesystemApplicationLifecycleScenario::current_source_text();
    let exact = classify_filesystem_successor("phase-312-classification-exact", &current, &current);
    assert!(matches!(
        exact,
        UiChangeClassificationOutcome::ObservedNoChange(_)
    ));

    let evidence = classify_filesystem_successor(
        "phase-312-classification-evidence",
        &current,
        &format!("{current}\n\n"),
    );
    assert!(matches!(
        evidence,
        UiChangeClassificationOutcome::EvidenceOnly(_)
    ));

    let changed = classify_filesystem_successor(
        "phase-312-classification-changed",
        &current,
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    let changed = match changed {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("the semantic filesystem edit must produce changed facts"),
    };
    assert!(
        changed.facts().len() >= 2,
        "the production comparator must enumerate every semantic difference"
    );
    assert!(changed
        .facts()
        .iter()
        .all(|fact| fact.family() == UiProducedFactFamily::AuthoredSource));
}

fn classify_filesystem_successor(
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
        provider.read().expect("initial filesystem snapshot reads"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_application(initial)
        .launch()
        .expect("initial filesystem application launches");

    workspace.write("app/main.wui", successor_source);
    let successor = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider
            .read()
            .expect("successor filesystem snapshot reads"),
        session.capabilities(),
    );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(successor).unwrap();
    let admitted = turn.seal().unwrap();
    let outcome = session.classify_observations(admitted).unwrap();
    let _ = session.shutdown();
    workspace.close();
    outcome
}
