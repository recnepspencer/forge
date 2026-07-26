//! Canonical immutable Query world used by graph-world certification scenarios.

use worth_ui::facade::{declaration::ViewBindingId, graph::UiGraphWorldProfile};

/// Derive a settled Query graph world from a real installed Query view.
///
/// The installed domain is an immutable suite baseline. Each scenario authors
/// only its view-identity delta, so tests reuse installation work without
/// sharing mutable runtime state.
pub fn settled_query_world_profile(
    view_binding_id: ViewBindingId,
    query_view_identity: impl Into<String>,
) -> UiGraphWorldProfile {
    let fixture_label = query_view_identity.into();
    let fixture = worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture::new(
        &fixture_label,
    );
    UiGraphWorldProfile::settled_query_binding(view_binding_id, fixture.binding_reference())
}
