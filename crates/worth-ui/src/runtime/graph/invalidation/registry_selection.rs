use super::WorthUiGraphInvalidationRequest;
use crate::runtime::graph::{graph_registry_for_fact, WorthUiGraphFactRegistry};

pub(super) fn registries_for_request(
    request: &WorthUiGraphInvalidationRequest,
) -> Vec<WorthUiGraphFactRegistry> {
    let mut registries = Vec::new();
    for fact in request.authoritative_changed_facts().facts() {
        let Some(registry) = graph_registry_for_fact(fact) else {
            continue;
        };
        if !registries.contains(&registry) {
            registries.push(registry);
        }
    }
    registries
}
