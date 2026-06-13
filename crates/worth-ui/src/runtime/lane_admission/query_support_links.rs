use crate::runtime::{WorthUiPlanNodeInput, WorthUiQueryRebindRequiredSurface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryLaneSupportLinks {
    plan_index: u32,
    view_binding_id: String,
    support_admission_digest: String,
    live_compatibility_digest: String,
    async_result_state_digest: String,
    inspection_digest: String,
    projection_consumption_digest: String,
    recovery_digest: String,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryLaneSupportLinks {
    pub(crate) fn from_plan_node_input(
        plan_index: u32,
        node_input: &WorthUiPlanNodeInput,
    ) -> Option<Self> {
        let identity = node_input.query_binding_identity()?;
        let posture = node_input.query_binding_posture()?;
        Some(Self {
            plan_index,
            view_binding_id: identity.view_binding_id().to_owned(),
            support_admission_digest: posture.support_admission_digest().to_owned(),
            live_compatibility_digest: posture.live_compatibility_digest().to_owned(),
            async_result_state_digest: posture.async_result_state_digest().to_owned(),
            inspection_digest: posture.inspection_digest().to_owned(),
            projection_consumption_digest: posture.projection_consumption_digest().to_owned(),
            recovery_digest: posture.recovery_digest().to_owned(),
            required_surfaces: node_input.query_required_surfaces().to_vec(),
        })
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn view_binding_id(&self) -> &str {
        &self.view_binding_id
    }

    pub fn support_admission_digest(&self) -> &str {
        &self.support_admission_digest
    }

    pub fn live_compatibility_digest(&self) -> &str {
        &self.live_compatibility_digest
    }

    pub fn async_result_state_digest(&self) -> &str {
        &self.async_result_state_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub fn recovery_digest(&self) -> &str {
        &self.recovery_digest
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }
}
