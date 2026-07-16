use worth_ui::facade::WorthUi;
use worth_ui::facade::query_binding::{
    WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryViewDefinition,
};

use super::view_binding_fixtures::{
    detail_view_binding_from, table_view_binding_from, test_installed_domain,
};

#[test]
fn invalid_query_definition_identity_stops_before_registry_mutation() {
    let denial = WorthUiQueryViewDefinition::measurement_snapshot("")
        .expect_err("empty identity cannot create a semantic definition");
    assert_eq!(
        denial,
        worth_ui::facade::query_binding::WorthUiQueryViewIdentityError::Empty
    );
}

#[test]
fn duplicate_view_binding_id_rejected_before_snapshot_freeze() {
    let installed = test_installed_domain("duplicate-view-binding");
    let builder = WorthUi::app()
        .register_query_view(table_view_binding_from(
            &installed,
            "workspace.view_binding.duplicate",
        ))
        .expect("first installed view should register");
    let denial = match builder.register_query_view(detail_view_binding_from(
            &installed,
            "workspace.view_binding.duplicate",
        )) {
        Ok(_) => panic!("duplicate semantic identity should stop at Query registration"),
        Err(denial) => denial,
    };
    let worth_ui::facade::app::WorthUiQueryViewRegistrationError::Binding(denial) = denial else {
        panic!("duplicate valid identity should be a binding denial");
    };
    assert_eq!(
        denial.kind(),
        WorthUiQueryBindingRegistrationDenialKind::DuplicateViewIdentity,
    );
    assert_eq!(denial.identity().as_str(), "workspace.view_binding.duplicate");
}

#[test]
fn foreign_installed_domain_is_rejected_before_registry_mutation() {
    let left = test_installed_domain("left-installed-domain");
    let foreign = test_installed_domain("foreign-installed-domain");
    let builder = WorthUi::app()
        .register_query_view(table_view_binding_from(
            &left,
            "workspace.view_binding.tasks",
        ))
        .expect("first installed view should register");
    let denial = match builder.register_query_view(table_view_binding_from(
        &foreign,
        "workspace.view_binding.foreign",
    )) {
        Ok(_) => panic!("foreign installed authority must stop before registry mutation"),
        Err(denial) => denial,
    };
    let worth_ui::facade::app::WorthUiQueryViewRegistrationError::Binding(denial) = denial else {
        panic!("valid foreign view identity should be a binding denial");
    };
    assert_eq!(
        denial.kind(),
        WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain,
    );
    assert_eq!(denial.identity().as_str(), "workspace.view_binding.foreign");
}
