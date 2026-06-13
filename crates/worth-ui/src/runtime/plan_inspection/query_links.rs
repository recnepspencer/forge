use crate::runtime::{WorthUiQueryBindingIdentity, WorthUiQueryRebindRequiredSurface};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryInspectionLinks {
    binding_identity: WorthUiQueryBindingIdentity,
    support_admission_digest: String,
    basis_capability_digest: String,
    live_compatibility_digest: String,
    inspection_digest: String,
    projection_consumption_digest: String,
    async_result_state_digest: String,
    recovery_digest: String,
    preservation_receipt: Option<String>,
    required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
}

impl WorthUiQueryInspectionLinks {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_query_posture(
        binding_identity: WorthUiQueryBindingIdentity,
        support_admission_digest: String,
        basis_capability_digest: String,
        live_compatibility_digest: String,
        inspection_digest: String,
        projection_consumption_digest: String,
        async_result_state_digest: String,
        recovery_digest: String,
        preservation_receipt: Option<String>,
        required_surfaces: Vec<WorthUiQueryRebindRequiredSurface>,
    ) -> Self {
        Self {
            binding_identity,
            support_admission_digest,
            basis_capability_digest,
            live_compatibility_digest,
            inspection_digest,
            projection_consumption_digest,
            async_result_state_digest,
            recovery_digest,
            preservation_receipt,
            required_surfaces,
        }
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn support_admission_digest(&self) -> &str {
        &self.support_admission_digest
    }

    pub fn basis_capability_digest(&self) -> &str {
        &self.basis_capability_digest
    }

    pub fn live_compatibility_digest(&self) -> &str {
        &self.live_compatibility_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub fn async_result_state_digest(&self) -> &str {
        &self.async_result_state_digest
    }

    pub fn recovery_digest(&self) -> &str {
        &self.recovery_digest
    }

    pub fn preservation_receipt(&self) -> Option<&str> {
        self.preservation_receipt.as_deref()
    }

    pub fn required_surfaces(&self) -> &[WorthUiQueryRebindRequiredSurface] {
        &self.required_surfaces
    }
}
