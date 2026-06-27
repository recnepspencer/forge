use crate::runtime::WorthUiQuerySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiQueryBindingPostureDriftFamily {
    SupportAdmission,
    BasisCapability,
    LiveCompatibility,
    AsyncResultState,
    Recovery,
    Inspection,
    ProjectionConsumption,
    DenialPresentation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPosture {
    query_support_status: WorthUiQuerySupportStatus,
    support_admission_digest: String,
    basis_capability_digest: String,
    live_compatibility_digest: String,
    async_result_state_digest: String,
    recovery_digest: String,
    inspection_digest: String,
    projection_consumption_digest: String,
    denial_presentation_digest: String,
}

impl WorthUiQueryBindingPosture {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        query_support_status: WorthUiQuerySupportStatus,
        support_admission_digest: String,
        basis_capability_digest: String,
        live_compatibility_digest: String,
        async_result_state_digest: String,
        recovery_digest: String,
        inspection_digest: String,
        projection_consumption_digest: String,
        denial_presentation_digest: String,
    ) -> Self {
        Self {
            query_support_status,
            support_admission_digest,
            basis_capability_digest,
            live_compatibility_digest,
            async_result_state_digest,
            recovery_digest,
            inspection_digest,
            projection_consumption_digest,
            denial_presentation_digest,
        }
    }

    pub fn query_support_status(&self) -> WorthUiQuerySupportStatus {
        self.query_support_status
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

    pub fn async_result_state_digest(&self) -> &str {
        &self.async_result_state_digest
    }

    pub fn recovery_digest(&self) -> &str {
        &self.recovery_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn projection_consumption_digest(&self) -> &str {
        &self.projection_consumption_digest
    }

    pub fn denial_presentation_digest(&self) -> &str {
        &self.denial_presentation_digest
    }

    pub(crate) fn drift_families_against(
        &self,
        other: &Self,
    ) -> Vec<WorthUiQueryBindingPostureDriftFamily> {
        let mut families = Vec::new();
        if self.query_support_status() != other.query_support_status()
            || self.support_admission_digest() != other.support_admission_digest()
        {
            families.push(WorthUiQueryBindingPostureDriftFamily::SupportAdmission);
        }
        push_if_changed(
            &mut families,
            self.basis_capability_digest(),
            other.basis_capability_digest(),
            WorthUiQueryBindingPostureDriftFamily::BasisCapability,
        );
        push_if_changed(
            &mut families,
            self.live_compatibility_digest(),
            other.live_compatibility_digest(),
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
        );
        push_if_changed(
            &mut families,
            self.async_result_state_digest(),
            other.async_result_state_digest(),
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState,
        );
        push_if_changed(
            &mut families,
            self.recovery_digest(),
            other.recovery_digest(),
            WorthUiQueryBindingPostureDriftFamily::Recovery,
        );
        push_if_changed(
            &mut families,
            self.inspection_digest(),
            other.inspection_digest(),
            WorthUiQueryBindingPostureDriftFamily::Inspection,
        );
        push_if_changed(
            &mut families,
            self.projection_consumption_digest(),
            other.projection_consumption_digest(),
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption,
        );
        push_if_changed(
            &mut families,
            self.denial_presentation_digest(),
            other.denial_presentation_digest(),
            WorthUiQueryBindingPostureDriftFamily::DenialPresentation,
        );
        families
    }
}

fn push_if_changed(
    families: &mut Vec<WorthUiQueryBindingPostureDriftFamily>,
    active: &str,
    candidate: &str,
    family: WorthUiQueryBindingPostureDriftFamily,
) {
    if active != candidate {
        families.push(family);
    }
}
