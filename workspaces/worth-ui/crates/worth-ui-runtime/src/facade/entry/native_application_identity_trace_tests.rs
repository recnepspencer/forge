use crate::runtime::tests::active_application_session_test_support::{
    component_candidate_submission, source_backed_component_app_with_host,
};

use super::native_application_identity_trace_test_support::{
    assert_indexed_trace_cost, assert_same_authored_affinity, assert_trace_authored_affinity,
    assert_trace_runtime_identity, authored_presented_identity, completed, frame_receipt,
    mounted_instance, mounted_source_oracle, only_presented_identity, presented_graph_node,
    remount_presented_instance, replace_application_with_provenance, retained_trace,
};
use super::native_identity_trace_host::NativeIdentityTraceHost;

#[test]
fn retained_predecessor_keeps_its_authored_trace_across_application_replacement() {
    let app = source_backed_component_app_with_host(NativeIdentityTraceHost::default());
    let declaration_artifacts = app.declaration_artifacts().to_vec();
    let mut shell = app
        .launch_native_surface()
        .expect("source-backed application should launch through the native lifecycle");
    let predecessor_oracle = mounted_source_oracle(&declaration_artifacts, shell.session.graph());
    let predecessor = frame_receipt(completed(shell.present_frame(100, 1)));
    let predecessor_identity = authored_presented_identity(
        &shell,
        &predecessor,
        predecessor_oracle.authored_provenance_digest(),
        "predecessor",
    );
    let predecessor_generation = predecessor.generation().clone();

    let candidate = component_candidate_submission(
        &shell.session,
        "identity-trace-replacement",
        "workspace.component.active_session_candidate",
    );
    let (successor, successor_provenance) =
        replace_application_with_provenance(&mut shell, candidate);
    let successor_identity = only_presented_identity(&shell, &successor);
    let successor_graph_node = presented_graph_node(&shell, successor_identity);
    let successor_oracle = successor_provenance
        .iter()
        .find(|oracle| oracle.graph_node() == successor_graph_node)
        .expect("mounted successor declaration should retain candidate provenance");

    let predecessor_trace = retained_trace(&shell, &predecessor, predecessor_identity);
    let successor_trace = retained_trace(&shell, &successor, successor_identity);

    assert_eq!(predecessor_trace.generation(), &predecessor_generation);
    assert_eq!(successor_trace.generation(), successor.generation());
    assert_ne!(predecessor_trace.generation(), successor_trace.generation());
    assert_trace_runtime_identity(&predecessor_trace, predecessor_identity);
    assert_trace_runtime_identity(&successor_trace, successor_identity);
    assert_eq!(
        predecessor_trace
            .authored_provenance()
            .source_artifact()
            .path(),
        "app/main.wui"
    );
    assert_trace_authored_affinity(&predecessor_trace, &predecessor_oracle);
    assert_trace_authored_affinity(&successor_trace, successor_oracle);
    assert_ne!(
        predecessor_trace.authored_provenance().source_generation(),
        successor_trace.authored_provenance().source_generation()
    );
    assert_indexed_trace_cost(predecessor_trace.cost());
    assert_indexed_trace_cost(successor_trace.cost());
    let shutdown = shell.shutdown();
    assert!(shutdown.host_session_released());
    assert_eq!(shutdown.released_surface_count(), 1);
}

#[test]
fn remount_mints_new_runtime_identity_without_losing_authored_affinity() {
    let app = source_backed_component_app_with_host(NativeIdentityTraceHost::default());
    let declaration_artifacts = app.declaration_artifacts().to_vec();
    let mut shell = app
        .launch_native_surface()
        .expect("source-backed application should launch through the native lifecycle");
    let authored_oracle = mounted_source_oracle(&declaration_artifacts, shell.session.graph());
    let predecessor = frame_receipt(completed(shell.present_frame(100, 1)));
    let predecessor_identity = authored_presented_identity(
        &shell,
        &predecessor,
        authored_oracle.authored_provenance_digest(),
        "predecessor",
    );
    let (successor, successor_instance, predecessor_incarnation) =
        remount_presented_instance(&mut shell, predecessor_identity);
    assert_ne!(successor.frame(), predecessor.frame());
    assert_eq!(successor.predecessor(), Some(predecessor.frame()));
    let successor_identity = authored_presented_identity(
        &shell,
        &successor,
        authored_oracle.authored_provenance_digest(),
        "successor",
    );
    let successor_view = mounted_instance(&shell.session.mounted.view(), successor_instance);

    let predecessor_trace = retained_trace(&shell, &predecessor, predecessor_identity);
    let successor_trace = retained_trace(&shell, &successor, successor_identity);

    assert_ne!(
        predecessor_identity.mounted_instance_identity(),
        successor_identity.mounted_instance_identity()
    );
    assert_ne!(
        predecessor_identity.node_receipt_identity(),
        successor_identity.node_receipt_identity()
    );
    assert_ne!(predecessor_incarnation, successor_view.mount_incarnation());
    assert_eq!(predecessor_trace.incarnation(), predecessor_incarnation);
    assert_eq!(
        successor_trace.incarnation(),
        successor_view.mount_incarnation()
    );
    assert_same_authored_affinity(&predecessor_trace, &successor_trace);
    assert_trace_authored_affinity(&predecessor_trace, &authored_oracle);
    assert_indexed_trace_cost(predecessor_trace.cost());
    assert_indexed_trace_cost(successor_trace.cost());
    let shutdown = shell.shutdown();
    assert!(shutdown.host_session_released());
    assert_eq!(shutdown.released_surface_count(), 1);
}
