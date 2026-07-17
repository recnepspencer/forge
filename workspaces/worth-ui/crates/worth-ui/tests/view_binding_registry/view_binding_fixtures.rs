use worth_ui::facade::query_binding::{WorthUiInstalledQueryDomain, WorthUiQueryViewRegistration};
use worth_ui::facade::{QueryDenialPresentation, ViewBindingId, VisibleStateBindingDeclaration};

pub(crate) fn table_view_binding(id: &str) -> WorthUiQueryViewRegistration {
    let installed = test_installed_domain(id);
    table_view_binding_from(&installed, id)
}

pub(crate) fn detail_view_binding(id: &str) -> WorthUiQueryViewRegistration {
    let installed = test_installed_domain(id);
    detail_view_binding_from(&installed, id)
}

pub(crate) fn table_view_binding_from(
    installed: &WorthUiInstalledQueryDomain,
    id: &str,
) -> WorthUiQueryViewRegistration {
    complete_view_binding(installed, id, false)
}

pub(crate) fn detail_view_binding_from(
    installed: &WorthUiInstalledQueryDomain,
    id: &str,
) -> WorthUiQueryViewRegistration {
    complete_view_binding(installed, id, true)
}

pub(crate) fn test_installed_domain(name: &str) -> WorthUiInstalledQueryDomain {
    worth_ui_query_binding::certification::worth_ui_installed_test_domain(&format!(
        "view-binding-{name}"
    ))
}

fn complete_view_binding(
    installed: &WorthUiInstalledQueryDomain,
    id: &str,
    live: bool,
) -> WorthUiQueryViewRegistration {
    let view = if live {
        installed.live_measurement_view(id)
    } else {
        installed.measurement_view(id)
    }
    .expect("installed measurement view should admit");
    WorthUiQueryViewRegistration::new(view)
        .with_visible_state_binding(VisibleStateBindingDeclaration::new("loading_posture"))
        .with_denial_presentation(QueryDenialPresentation::structured_status())
}

pub(crate) fn view_binding_id(raw_text: &str) -> ViewBindingId {
    ViewBindingId::new(raw_text).expect("valid view binding id")
}
