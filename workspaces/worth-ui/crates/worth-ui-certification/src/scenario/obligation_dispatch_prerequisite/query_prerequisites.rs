//! Settled Query world identity used by obligation scenarios.

use worth_ui::facade::graph::UiGraphWorldProfile;
use worth_ui::facade::registry::ViewBindingId;

use crate::scenario::installed_query_world;

pub fn settled_query_world_profile(
    settlement_label: &str,
    binding_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let binding_identity = binding_parts.join(".").replace('-', "_");
    installed_query_world::settled_query_world_profile(
        ViewBindingId::new(binding_identity.clone()).expect("valid settled Query view binding id"),
        format!("{binding_identity}.{settlement_label}").replace('-', "_"),
    )
}
