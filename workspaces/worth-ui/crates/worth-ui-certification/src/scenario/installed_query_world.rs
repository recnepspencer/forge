//! Canonical immutable Query world used by graph-world certification scenarios.

use std::sync::OnceLock;

use worth_ui::facade::{
    graph::UiGraphWorldProfile,
    query_binding::{WorthUiInstalledQueryDomain, WorthUiInstalledQueryView},
    registry::ViewBindingId,
};

fn installed_query_domain() -> &'static WorthUiInstalledQueryDomain {
    static DOMAIN: OnceLock<WorthUiInstalledQueryDomain> = OnceLock::new();

    DOMAIN.get_or_init(|| {
        worth_ui_query_binding::certification::worth_ui_installed_test_domain(
            "worth-ui-certification-graph-world",
        )
    })
}

/// Derive a settled Query graph world from a real installed Query view.
///
/// The installed domain is an immutable suite baseline. Each scenario authors
/// only its view-identity delta, so tests reuse installation work without
/// sharing mutable runtime state.
pub fn settled_query_world_profile(
    view_binding_id: ViewBindingId,
    query_view_identity: impl Into<String>,
) -> UiGraphWorldProfile {
    let view: WorthUiInstalledQueryView = installed_query_domain()
        .measurement_view(query_view_identity)
        .expect("certification Query view identity must be valid")
        .into();
    UiGraphWorldProfile::settled_query_view(view_binding_id, &view)
}
