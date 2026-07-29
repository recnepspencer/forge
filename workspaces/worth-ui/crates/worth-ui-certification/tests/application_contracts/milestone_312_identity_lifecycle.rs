use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::{
    UiGraphFactConsumerKind, UiIdentityLifecycleDecision, UiResolvedIdentityLifecycle,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_runtime::certification_support::{
    identity_lifecycle_decision_for_certification, UiIdentityLifecyclePresence,
    UiResolvedIdentityLifecycleCertificationExt, WorthUiNodeLifecycleTransition,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn rebind_identity_model_matches_production_scope() {
    assert_closed_transition_model();
    let lifecycle = real_dual_generation_lifecycle();
    assert_real_scope_decisions(&lifecycle);
    assert_unaffected_control(&lifecycle);
}

fn assert_closed_transition_model() {
    let transitions = [
        WorthUiNodeLifecycleTransition::Preserve,
        WorthUiNodeLifecycleTransition::Replace,
        WorthUiNodeLifecycleTransition::Drop,
        WorthUiNodeLifecycleTransition::Create,
        WorthUiNodeLifecycleTransition::Move,
        WorthUiNodeLifecycleTransition::Rebind,
        WorthUiNodeLifecycleTransition::LaneChange,
    ];
    let kinds = [
        UiGraphFactConsumerKind::GraphNode,
        UiGraphFactConsumerKind::MountEligibilitySlot,
    ];
    let presences = [
        UiIdentityLifecyclePresence::Both,
        UiIdentityLifecyclePresence::CandidateOnly,
        UiIdentityLifecyclePresence::PredecessorOnly,
        UiIdentityLifecyclePresence::Neither,
    ];
    for transition in transitions {
        for kind in kinds {
            for presence in presences {
                assert_eq!(
                    identity_lifecycle_decision_for_certification(transition, kind, presence),
                    independent_decision(transition, kind, presence),
                    "production lifecycle diverged for {transition:?}/{kind:?}/{presence:?}"
                );
            }
        }
    }
}

fn independent_decision(
    transition: WorthUiNodeLifecycleTransition,
    kind: UiGraphFactConsumerKind,
    presence: UiIdentityLifecyclePresence,
) -> Option<UiIdentityLifecycleDecision> {
    match presence {
        UiIdentityLifecyclePresence::CandidateOnly => Some(UiIdentityLifecycleDecision::Create),
        UiIdentityLifecyclePresence::PredecessorOnly => Some(UiIdentityLifecycleDecision::Retire),
        UiIdentityLifecyclePresence::Neither => None,
        UiIdentityLifecyclePresence::Both => match transition {
            WorthUiNodeLifecycleTransition::Preserve => Some(UiIdentityLifecycleDecision::Preserve),
            WorthUiNodeLifecycleTransition::Move => Some(UiIdentityLifecycleDecision::Move),
            WorthUiNodeLifecycleTransition::Rebind => Some(UiIdentityLifecycleDecision::Rebind),
            WorthUiNodeLifecycleTransition::Replace
            | WorthUiNodeLifecycleTransition::LaneChange => match kind {
                UiGraphFactConsumerKind::GraphNode => Some(UiIdentityLifecycleDecision::Rebind),
                UiGraphFactConsumerKind::MountEligibilitySlot => {
                    Some(UiIdentityLifecycleDecision::Remount)
                }
            },
            WorthUiNodeLifecycleTransition::Drop | WorthUiNodeLifecycleTransition::Create => None,
        },
    }
}

fn real_dual_generation_lifecycle() -> UiResolvedIdentityLifecycle {
    let label = "phase-312-identity-lifecycle";
    let scenario = FilesystemApplicationLifecycleScenario::new(label);
    let workspace = FilesystemContractWorkspace::new(label);
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::dual_generation_scope_initial_source_text(),
    );
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let capabilities = scenario.capability_application();
    let initial = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("initial filesystem world reads"),
        capabilities.capabilities(),
    );
    let mut session = scenario
        .prepare_application(initial)
        .launch()
        .expect("initial filesystem world launches");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::dual_generation_scope_candidate_source_text(),
    );
    let candidate = FilesystemApplicationLifecycleScenario::lower_snapshot(
        provider.read().expect("candidate filesystem world reads"),
        session.capabilities(),
    );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("create/retire world must classify as changed"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .expect("production replacement proof resolves exact identity lifecycle");
    let _ = session.shutdown();
    workspace.close();
    lifecycle
}

fn assert_real_scope_decisions(lifecycle: &UiResolvedIdentityLifecycle) {
    let retired = FilesystemApplicationLifecycleScenario::current_component_declaration_identity();
    let created =
        FilesystemApplicationLifecycleScenario::candidate_component_declaration_identity();
    assert_eq!(lifecycle.selected().len(), 4);
    for entry in lifecycle.selected() {
        let expected = match entry.key().authored_identity() {
            identity if identity == retired => UiIdentityLifecycleDecision::Retire,
            identity if identity == created => UiIdentityLifecycleDecision::Create,
            identity => panic!("unmodeled selected identity: {identity}"),
        };
        assert_eq!(entry.decision(), expected);
        assert!(!entry.decision().preserves_instance());
        assert!(!entry.decision().preserves_incarnation());
    }
}

fn assert_unaffected_control(lifecycle: &UiResolvedIdentityLifecycle) {
    let preserved =
        FilesystemApplicationLifecycleScenario::imported_current_component_declaration_identity();
    let keys = lifecycle.known_consumer_keys_for_certification();
    let control_keys = keys
        .iter()
        .filter(|key| key.authored_identity() == preserved)
        .collect::<Vec<_>>();
    assert_eq!(
        control_keys.len(),
        2,
        "one graph node and its nested mount slot remain independently addressable"
    );
    assert_ne!(control_keys[0].kind(), control_keys[1].kind());
    assert_eq!(
        control_keys[0].repeated_instance_basis_digest(),
        control_keys[1].repeated_instance_basis_digest()
    );
    for key in control_keys {
        assert_eq!(
            lifecycle.decision_for(key),
            Some(UiIdentityLifecycleDecision::Unaffected)
        );
    }
}
