use worth_ui::facade::{VisibleStateBindingDeclaration, WorthUi};

use super::view_binding_assertions::assert_registered_view_binding_ids;
use super::view_binding_fixtures::{
    detail_view_binding, detail_view_binding_from, table_view_binding, table_view_binding_from,
    test_installed_domain, view_binding_id,
};

#[test]
fn equivalent_query_view_references_produce_equivalent_bindings() {
    let left = WorthUi::app()
        .register_query_view(table_view_binding("workspace.view_binding.tasks"))
        .expect("installed view should register")
        .freeze();
    let right = WorthUi::app()
        .register_query_view(table_view_binding("workspace.view_binding.tasks"))
        .expect("installed view should register")
        .freeze();

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left.capabilities().view_bindings().entries()[0].identity(),
        right.capabilities().view_bindings().entries()[0].identity()
    );
}

#[test]
fn different_query_view_reference_meaning_changes_snapshot_digest() {
    let table = WorthUi::app()
        .register_query_view(table_view_binding("workspace.view_binding.main"))
        .expect("installed snapshot view should register")
        .freeze();
    let detail = WorthUi::app()
        .register_query_view(detail_view_binding("workspace.view_binding.main"))
        .expect("installed live view should register")
        .freeze();

    assert_ne!(
        table.capabilities().digest(),
        detail.capabilities().digest()
    );
}

#[test]
fn accepted_view_bindings_remain_inspectable_after_freeze() {
    let installed = test_installed_domain("accepted-view-bindings");
    let app = WorthUi::app()
        .register_query_view(table_view_binding_from(
            &installed,
            "workspace.view_binding.tasks",
        ))
        .expect("installed snapshot view should register")
        .register_query_view(detail_view_binding_from(
            &installed,
            "workspace.view_binding.task_detail",
        ))
        .expect("installed live view from the same workspace should register")
        .freeze();

    assert_registered_view_binding_ids(
        app.capabilities().view_bindings(),
        &[
            "workspace.view_binding.task_detail",
            "workspace.view_binding.tasks",
        ],
    );
    assert!(app
        .capabilities()
        .view_bindings()
        .get(&view_binding_id("workspace.view_binding.tasks"))
        .is_some());
}

#[test]
fn admitted_definition_lifecycle_participates_in_view_binding_key_equivalence() {
    let left = table_view_binding("workspace.view_binding.tasks");
    let right = detail_view_binding("workspace.view_binding.tasks");

    let left_app = WorthUi::app()
        .register_query_view(left)
        .expect("installed snapshot view should register")
        .freeze();
    let right_app = WorthUi::app()
        .register_query_view(right)
        .expect("installed live view should register")
        .freeze();

    assert_ne!(
        left_app.capabilities().view_bindings().entries()[0].identity(),
        right_app.capabilities().view_bindings().entries()[0].identity()
    );
}

#[test]
fn visible_state_bindings_participate_in_view_binding_key_equivalence() {
    let left = table_view_binding("workspace.view_binding.tasks");
    let right = table_view_binding("workspace.view_binding.tasks")
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("empty_posture"));

    let left_app = WorthUi::app()
        .register_query_view(left)
        .expect("installed view should register")
        .freeze();
    let right_app = WorthUi::app()
        .register_query_view(right)
        .expect("installed view should register")
        .freeze();

    assert_ne!(
        left_app.capabilities().view_bindings().entries()[0].identity(),
        right_app.capabilities().view_bindings().entries()[0].identity()
    );
}

#[test]
fn visible_state_binding_key_basis_is_not_delimiter_collision_prone() {
    let split_bindings = table_view_binding("workspace.view_binding.tasks")
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("pending"))
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("retry"));
    let joined_binding = table_view_binding("workspace.view_binding.tasks")
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("pending,retry"));

    let split_app = WorthUi::app()
        .register_query_view(split_bindings)
        .expect("installed view should register")
        .freeze();
    let joined_app = WorthUi::app()
        .register_query_view(joined_binding)
        .expect("installed view should register")
        .freeze();

    assert_ne!(
        split_app.capabilities().view_bindings().entries()[0].identity(),
        joined_app.capabilities().view_bindings().entries()[0].identity()
    );
}
