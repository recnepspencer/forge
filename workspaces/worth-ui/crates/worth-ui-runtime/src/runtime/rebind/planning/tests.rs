use crate::runtime::observation::UiChangeClassificationOutcome;

use super::{
    UiRebindArtifactPolicy, UiRebindCancellationPolicy, UiRebindDeadlinePolicy,
    UiRebindDisclosurePolicy, UiRebindExecutionPolicy, UiRebindIdempotency, UiRebindPlanningDenial,
    UiRebindRetryTolerance, UiRebindSafePoint, UiRebindSafePointPolicy, UiRebindSemanticProof,
    UiRebindSubsystemKind,
};

#[test]
fn changed_scope_compiles_one_complete_immutable_rebind_plan() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let predecessor = session.generation_identity().clone();
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            &session,
            "phase-312-complete-plan",
            "workspace.component.active_session_candidate",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("component replacement must classify as changed"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .expect("resolved scope compiles one plan");

    assert!(plan.scope().is_some());
    assert_complete_subsystem_partition(&plan);
    assert_complete_effect_contract(&plan);
    let UiRebindSemanticProof::Changed(changed) = plan.semantic_proof() else {
        panic!("changed source planning must carry the existing replacement proof")
    };
    assert!(changed.lowering.node_plan().is_unambiguous());
    assert_eq!(
        changed
            .lowering
            .node_plan()
            .candidate_structural_node_count(),
        1
    );
    assert_eq!(
        changed.successor_authority.generation_identity(),
        plan.basis().candidate_generation()
    );
    assert_eq!(plan.cost().graph_and_mounted_entries(), 4);
    assert_eq!(session.generation_identity(), &predecessor);
    drop(plan);
    let _ = session.shutdown();
}

#[test]
fn evidence_only_source_compiles_complete_structurally_empty_plan() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let predecessor = session.generation_identity().clone();
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            &session,
            "phase-312-evidence-only-plan",
            "workspace.component.active_session_current",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let evidence = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
        _ => panic!("equal semantics with new source evidence must remain evidence-only"),
    };
    let plan = session
        .compile_preservation_rebind(evidence, UiRebindExecutionPolicy::ordinary())
        .expect("evidence-only source succession compiles a preservation plan");

    assert_eq!(
        plan.basis().classification().predecessor_generation(),
        &predecessor
    );
    assert_ne!(plan.basis().candidate_generation(), &predecessor);
    assert!(plan.scope().is_none());
    assert_empty_complete_plan(&plan);
    let UiRebindSemanticProof::EvidenceOnly(succession) = plan.semantic_proof() else {
        panic!("evidence-only planning must retain its successor authority")
    };
    assert_eq!(
        succession.successor_authority().generation_identity(),
        plan.basis().candidate_generation()
    );
    assert_eq!(session.generation_identity(), &predecessor);
    drop(plan);
    let _ = session.shutdown();
}

#[test]
fn rebind_execution_policy_is_complete_and_mechanism_free() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let deadline = session.rebind_deadline_at(144);
    let cancellation = session.rebind_cancellation_request();
    let policy = UiRebindExecutionPolicy::ordinary()
        .with_deadline(deadline)
        .with_cancellation(cancellation)
        .with_idempotency(UiRebindIdempotency::SourceEvidence)
        .with_retry_tolerance(UiRebindRetryTolerance::PreEffectOnly)
        .with_artifact_policy(UiRebindArtifactPolicy::Ordinary)
        .with_disclosure_policy(UiRebindDisclosurePolicy::Ordinary)
        .with_safe_point_policy(UiRebindSafePointPolicy::CanonicalPreEffect);
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            &session,
            "phase-312-complete-policy",
            "workspace.component.active_session_candidate",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("component replacement must classify as changed"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = session
        .compile_rebind_plan(lifecycle, policy)
        .expect("session-bound caller policy compiles");

    assert_eq!(plan.execution_policy(), policy);
    assert_eq!(policy.deadline(), UiRebindDeadlinePolicy::At(deadline));
    assert_eq!(deadline.tick(), 144);
    assert_eq!(
        policy.cancellation(),
        UiRebindCancellationPolicy::AtSafePoints(cancellation)
    );
    assert_eq!(policy.idempotency(), UiRebindIdempotency::SourceEvidence);
    assert_eq!(
        policy.retry_tolerance(),
        UiRebindRetryTolerance::PreEffectOnly
    );
    assert_eq!(policy.artifact_policy(), UiRebindArtifactPolicy::Ordinary);
    assert_eq!(
        policy.disclosure_policy(),
        UiRebindDisclosurePolicy::Ordinary
    );
    assert_eq!(
        policy.safe_point_policy().safe_points(),
        &[
            UiRebindSafePoint::PreClassification,
            UiRebindSafePoint::PostClassification,
            UiRebindSafePoint::PostScope,
            UiRebindSafePoint::PostPlan,
            UiRebindSafePoint::PostReservation,
            UiRebindSafePoint::FinalCurrentBasisAdmission,
            UiRebindSafePoint::PreFirstHostEffect,
        ]
    );
    drop(plan);
    let _ = session.shutdown();
}

#[test]
fn rebind_execution_policy_rejects_foreign_session_authority() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let foreign = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let policy = UiRebindExecutionPolicy::ordinary()
        .with_deadline(foreign.rebind_deadline_at(144))
        .with_cancellation(foreign.rebind_cancellation_request());
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            &session,
            "phase-312-foreign-policy",
            "workspace.component.active_session_candidate",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("component replacement must classify as changed"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    assert!(matches!(
        session.compile_rebind_plan(lifecycle, policy),
        Err(UiRebindPlanningDenial::ForeignExecutionPolicySession)
    ));
    let _ = session.shutdown();
    let _ = foreign.shutdown();
}

fn assert_complete_subsystem_partition(plan: &super::UiRebindPlan) {
    assert_eq!(plan.subsystems().len(), 9);
    for kind in [
        UiRebindSubsystemKind::Preservation,
        UiRebindSubsystemKind::Graph,
        UiRebindSubsystemKind::Mount,
        UiRebindSubsystemKind::Measurement,
        UiRebindSubsystemKind::Allocation,
        UiRebindSubsystemKind::Binding,
        UiRebindSubsystemKind::Obligation,
        UiRebindSubsystemKind::Surface,
        UiRebindSubsystemKind::Retirement,
    ] {
        assert_eq!(
            plan.subsystem(kind).map(|subplan| subplan.kind()),
            Some(kind),
            "every required subsystem has an explicit, possibly empty subplan"
        );
    }
    assert_eq!(
        plan.subsystem(UiRebindSubsystemKind::Graph)
            .unwrap()
            .targets()
            .len(),
        2
    );
    assert_eq!(
        plan.subsystem(UiRebindSubsystemKind::Mount)
            .unwrap()
            .targets()
            .len(),
        2
    );
    assert_eq!(
        plan.subsystem(UiRebindSubsystemKind::Retirement)
            .unwrap()
            .targets()
            .len(),
        2
    );
}

fn assert_empty_complete_plan(plan: &super::UiRebindPlan) {
    assert_eq!(plan.subsystems().len(), 9);
    assert!(plan
        .subsystems()
        .iter()
        .all(|subplan| subplan.targets().is_empty()));
    assert!(plan.identity_decisions().is_empty());
    assert!(plan.effects().effects().is_empty());
    assert!(plan.conflicts().reads().is_empty());
    assert!(plan.conflicts().writes().is_empty());
    assert!(plan.conflicts().invalidations().is_empty());
    assert!(plan.parallel_admission().admitted_subsystems().is_empty());
    let cost = plan.cost();
    assert_eq!(cost.selected_decisions(), 0);
    assert_eq!(cost.graph_and_mounted_entries(), 0);
    assert_eq!(cost.measurement_and_allocation_entries(), 0);
    assert_eq!(cost.binding_transitions(), 0);
    assert_eq!(cost.effects(), 0);
}

fn assert_complete_effect_contract(plan: &super::UiRebindPlan) {
    let effect_count = plan.effects().effects().len();
    assert!(effect_count > 0);
    assert_eq!(plan.conflicts().writes().len(), effect_count);
    assert!(plan.conflicts().reads().len() >= effect_count);
    assert!(!plan.conflicts().invalidations().is_empty());
    assert_eq!(plan.cost().selected_decisions(), 4);
    assert_eq!(plan.cost().graph_and_mounted_entries(), 4);
    assert_eq!(plan.cost().measurement_and_allocation_entries(), 4);
    assert_eq!(plan.cost().effects(), effect_count);
    assert!(plan
        .parallel_admission()
        .admitted_subsystems()
        .iter()
        .all(|kind| *kind != UiRebindSubsystemKind::Preservation));
}
