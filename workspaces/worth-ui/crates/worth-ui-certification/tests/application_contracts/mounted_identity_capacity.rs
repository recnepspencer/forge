use worth_ui_runtime::facade::mounted::UiMountedIdentityDenial;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;

use super::mounted_application_lifecycle::known_empty_surface_world::{
    active_session, first_node, registered_surface,
};

#[test]
fn semantic_surface_capacity_denies_without_minting_an_extra_surface() {
    let mut session = active_session();
    let mut admitted = 0;
    loop {
        match session.create_semantic_surface() {
            Ok(_) => admitted += 1,
            Err(denial) => {
                assert_eq!(
                    denial,
                    UiMountedIdentityDenial::SemanticSurfaceCapacityExceeded
                );
                break;
            }
        }
    }
    assert_eq!(admitted, 256);
}

#[test]
fn one_graph_node_has_an_explicit_bounded_mounted_set() {
    let mut session = active_session();
    let surface = registered_surface(&mut session);
    let node = first_node(&session);
    for _ in 0..1_024 {
        session.mount_instance(node, surface).unwrap();
    }
    assert_eq!(
        session.mount_instance(node, surface),
        Err(UiMountedIdentityDenial::GraphNodeMountCapacityExceeded)
    );
    assert_eq!(session.mounted_instances_for(node).unwrap().len(), 1_024);
}

#[test]
fn the_complete_current_mounted_closure_has_an_independent_bound() {
    let mut session = active_session();
    let surface = registered_surface(&mut session);
    let nodes = session
        .graph()
        .node_identities()
        .take(3)
        .map(|identity| session.mounted_graph_node(identity).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        nodes.len(),
        3,
        "the real fixture must expose enough distinct graph nodes"
    );
    for node in &nodes[..2] {
        for _ in 0..1_024 {
            session.mount_instance(*node, surface).unwrap();
        }
    }
    assert_eq!(
        session.mount_instance(nodes[2], surface),
        Err(UiMountedIdentityDenial::MountedClosureCapacityExceeded)
    );
    assert_eq!(
        session.inspect_mounted_identity().mounted_instances().len(),
        2_048
    );
}
