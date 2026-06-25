use worth_ui::facade::{
    composition_context_adoption_proof, composition_participation_adoption_proof,
    composition_topology_adoption_proof, live_view_state_binding_adoption_proof,
    mounted_interaction_adoption_proof, primitive_construction_adoption_proof,
    primitive_content_anatomy_adoption_proof, primitive_event_dispatch_adoption_proof,
};

#[test]
fn composition_topology_has_execution_backed_query_adoption() {
    let proof = composition_topology_adoption_proof()
        .expect("composition topology should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-composition-topology"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 5);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 5);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn composition_context_has_execution_backed_query_adoption() {
    let proof = composition_context_adoption_proof()
        .expect("composition context should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-composition-context"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 5);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 5);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn composition_participation_has_execution_backed_query_adoption() {
    let proof = composition_participation_adoption_proof()
        .expect("composition participation should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-composition-participation"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 7);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 7);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn primitive_construction_has_execution_backed_query_adoption() {
    let proof = primitive_construction_adoption_proof()
        .expect("primitive construction should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-primitive-construction"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 4);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 4);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn mounted_interaction_activation_has_execution_backed_query_adoption() {
    let proof = mounted_interaction_adoption_proof()
        .expect("mounted interaction activation should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-mounted-interaction"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 8);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 8);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
    assert!(proof.local_ceremony_audit().is_evaluated());
}

#[test]
fn primitive_event_dispatch_has_execution_backed_query_adoption() {
    let proof = primitive_event_dispatch_adoption_proof()
        .expect("primitive event dispatch should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-primitive-event-dispatch"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 6);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 6);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn primitive_content_anatomy_has_execution_backed_query_adoption() {
    let proof = primitive_content_anatomy_adoption_proof()
        .expect("primitive content should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-primitive-content"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 6);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 6);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}

#[test]
fn live_view_state_binding_has_execution_backed_query_adoption() {
    let proof = live_view_state_binding_adoption_proof()
        .expect("live view state binding should adopt Query graph obligations");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-ui-live-view-state-binding"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 7);
    assert_eq!(proof.execution_proof().selected_obligation_count(), 7);
    assert!(proof.execution_proof().has_real_executor_rows());
    assert!(proof.local_ceremony_audit().is_clean());
}
