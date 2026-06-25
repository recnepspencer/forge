use crate::capability::SurfaceId;
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority};

impl WorthUiRuntimeGraphAuthority {
    pub fn plan_mounted_interaction_operability(
        &self,
        surface_id: &SurfaceId,
        interaction_id: &str,
        extra_dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Vec<WorthUiRuntimeFactId> {
        let mut dependency_facts = vec![
            WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
            WorthUiRuntimeFactId::primitive_interaction(surface_id.as_str()),
            WorthUiRuntimeFactId::primitive_event_geometry(surface_id.as_str()),
            WorthUiRuntimeFactId::component_interaction_state(interaction_id.to_owned()),
        ];
        for fact in extra_dependency_facts {
            if !dependency_facts.contains(&fact) {
                dependency_facts.push(fact);
            }
        }
        dependency_facts
    }
}
