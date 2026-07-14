use worth_query::facade::runtime::QuerySubscriptionFamily;
use worth_ui::facade::{
    QueryLiveCompatibility, QueryResultShapeReference, VisibleStateBindingDeclaration, WorthUi,
};

use super::view_binding_assertions::assert_registered_view_binding_ids;
use super::view_binding_fixtures::{detail_view_binding, table_view_binding, view_binding_id};

#[test]
fn equivalent_query_view_references_produce_equivalent_bindings() {
    let left = WorthUi::app()
        .register_view_binding(table_view_binding("workspace.view_binding.tasks"))
        .freeze();
    let right = WorthUi::app()
        .register_view_binding(table_view_binding("workspace.view_binding.tasks"))
        .freeze();

    assert_eq!(left.capabilities().digest(), right.capabilities().digest());
    assert_eq!(
        left.capabilities().view_bindings().entries()[0].query_binding_key(),
        right.capabilities().view_bindings().entries()[0].query_binding_key()
    );
}

#[test]
fn different_query_view_reference_meaning_changes_snapshot_digest() {
    let table = WorthUi::app()
        .register_view_binding(table_view_binding("workspace.view_binding.main"))
        .freeze();
    let detail = WorthUi::app()
        .register_view_binding(detail_view_binding("workspace.view_binding.main"))
        .freeze();

    assert_ne!(
        table.capabilities().digest(),
        detail.capabilities().digest()
    );
}

#[test]
fn accepted_view_bindings_remain_inspectable_after_freeze() {
    let app = WorthUi::app()
        .register_view_binding(table_view_binding("workspace.view_binding.tasks"))
        .register_view_binding(detail_view_binding("workspace.view_binding.task_detail"))
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
fn result_shape_participates_in_view_binding_key_equivalence() {
    let left = table_view_binding("workspace.view_binding.tasks");
    let right = table_view_binding("workspace.view_binding.tasks").with_result_shape(
        QueryResultShapeReference::from_result_shape_family(
            worth_query::facade::foundation::ResultShapeFamily::Detail,
        ),
    );

    let left_app = WorthUi::app().register_view_binding(left).freeze();
    let right_app = WorthUi::app().register_view_binding(right).freeze();

    assert_ne!(
        left_app.capabilities().view_bindings().entries()[0].query_binding_key(),
        right_app.capabilities().view_bindings().entries()[0].query_binding_key()
    );
}

#[test]
fn live_compatibility_participates_in_view_binding_key_equivalence() {
    let left = table_view_binding("workspace.view_binding.tasks");
    let right = table_view_binding("workspace.view_binding.tasks").with_live_compatibility(
        QueryLiveCompatibility::declaration_only(QuerySubscriptionFamily::DetailExact),
    );

    let left_app = WorthUi::app().register_view_binding(left).freeze();
    let right_app = WorthUi::app().register_view_binding(right).freeze();

    assert_ne!(
        left_app.capabilities().view_bindings().entries()[0].query_binding_key(),
        right_app.capabilities().view_bindings().entries()[0].query_binding_key()
    );
}

#[test]
fn visible_state_bindings_participate_in_view_binding_key_equivalence() {
    let left = table_view_binding("workspace.view_binding.tasks");
    let right = table_view_binding("workspace.view_binding.tasks")
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("empty_posture"));

    let left_app = WorthUi::app().register_view_binding(left).freeze();
    let right_app = WorthUi::app().register_view_binding(right).freeze();

    assert_ne!(
        left_app.capabilities().view_bindings().entries()[0].query_binding_key(),
        right_app.capabilities().view_bindings().entries()[0].query_binding_key()
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
        .register_view_binding(split_bindings)
        .freeze();
    let joined_app = WorthUi::app()
        .register_view_binding(joined_binding)
        .freeze();

    assert_ne!(
        split_app.capabilities().view_bindings().entries()[0].query_binding_key(),
        joined_app.capabilities().view_bindings().entries()[0].query_binding_key()
    );
}
