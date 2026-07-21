use crate::runtime::{
    WorthUiPlanNodeInput, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryRebindRequiredSurface,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryLaneSupportLinks {
    plan_index: u32,
    binding_identity: WorthUiQueryBindingIdentity,
    posture: WorthUiQueryBindingPosture,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryLaneSupportLinks {
    pub(crate) fn from_plan_node_input(
        plan_index: u32,
        node_input: &WorthUiPlanNodeInput,
    ) -> Option<Self> {
        Some(Self {
            plan_index,
            binding_identity: node_input.query_binding_identity()?.clone(),
            posture: node_input.query_binding_posture()?.clone(),
            required_surfaces: node_input.query_required_surfaces().to_vec(),
        })
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn view_binding_id(&self) -> &str {
        self.binding_identity.view_binding_id()
    }

    pub fn posture(&self) -> &WorthUiQueryBindingPosture {
        &self.posture
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }
}
