use crate::capability::QueryDenialPresentation;
use crate::runtime::{WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus};

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

/// Typed UI runtime posture derived from an admitted binding definition and
/// runtime dependency graph. Reporting projections never participate in it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingPosture {
    query_support_status: WorthUiQuerySupportStatus,
    support_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
    installed_basis_authority: bool,
    lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle,
    async_result_state_available: bool,
    recovery_available: bool,
    inspection_available: bool,
    projection_consumption_available: bool,
    denial_presentation: QueryDenialPresentation,
}

pub(crate) struct WorthUiQueryBindingPostureInput {
    pub support_receipt: WorthUiQuerySupportReceipt,
    pub installed_basis_authority: bool,
    pub lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle,
    pub async_result_state_available: bool,
    pub recovery_available: bool,
    pub inspection_available: bool,
    pub projection_consumption_available: bool,
    pub denial_presentation: QueryDenialPresentation,
}

impl WorthUiQueryBindingPosture {
    pub(crate) fn new(input: WorthUiQueryBindingPostureInput) -> Self {
        let WorthUiQueryBindingPostureInput {
            support_receipt,
            installed_basis_authority,
            lifecycle,
            async_result_state_available,
            recovery_available,
            inspection_available,
            projection_consumption_available,
            denial_presentation,
        } = input;
        Self {
            query_support_status: support_receipt.status(),
            support_contract_identity: support_receipt.contract_identity(),
            installed_basis_authority,
            lifecycle,
            async_result_state_available,
            recovery_available,
            inspection_available,
            projection_consumption_available,
            denial_presentation,
        }
    }

    pub fn query_support_status(&self) -> WorthUiQuerySupportStatus {
        self.query_support_status
    }

    pub fn support_contract_identity(
        &self,
    ) -> worth_ui_query_binding::WorthUiQueryBindingContractIdentity {
        self.support_contract_identity
    }

    pub fn has_installed_basis_authority(&self) -> bool {
        self.installed_basis_authority
    }

    pub fn lifecycle(&self) -> worth_ui_query_binding::WorthUiQueryViewLifecycle {
        self.lifecycle
    }

    pub fn has_async_result_state(&self) -> bool {
        self.async_result_state_available
    }

    pub fn has_recovery(&self) -> bool {
        self.recovery_available
    }

    pub fn has_inspection(&self) -> bool {
        self.inspection_available
    }

    pub fn has_projection_consumption(&self) -> bool {
        self.projection_consumption_available
    }

    pub fn denial_presentation(&self) -> QueryDenialPresentation {
        self.denial_presentation
    }

    pub fn canonical_identity(&self) -> u64 {
        let mut identity = 0x7175_6572_7970_6f73_u64;
        for value in [
            query_support_status_tag(self.query_support_status),
            self.support_contract_identity.as_u64(),
            u64::from(self.installed_basis_authority),
            lifecycle_tag(self.lifecycle),
            u64::from(self.async_result_state_available),
            u64::from(self.recovery_available),
            u64::from(self.inspection_available),
            u64::from(self.projection_consumption_available),
            denial_presentation_tag(self.denial_presentation),
        ] {
            identity = identity.rotate_left(11).wrapping_mul(0x100_0000_01b3) ^ value;
        }
        identity
    }

    pub(crate) fn drift_families_against(
        &self,
        other: &Self,
    ) -> Vec<WorthUiQueryBindingPostureDriftFamily> {
        let mut families = Vec::new();
        push_if_changed(
            &mut families,
            self.query_support_status != other.query_support_status
                || self.support_contract_identity != other.support_contract_identity,
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission,
        );
        push_if_changed(
            &mut families,
            self.installed_basis_authority != other.installed_basis_authority,
            WorthUiQueryBindingPostureDriftFamily::BasisCapability,
        );
        push_if_changed(
            &mut families,
            self.lifecycle != other.lifecycle,
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
        );
        push_if_changed(
            &mut families,
            self.async_result_state_available != other.async_result_state_available,
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState,
        );
        push_if_changed(
            &mut families,
            self.recovery_available != other.recovery_available,
            WorthUiQueryBindingPostureDriftFamily::Recovery,
        );
        push_if_changed(
            &mut families,
            self.inspection_available != other.inspection_available,
            WorthUiQueryBindingPostureDriftFamily::Inspection,
        );
        push_if_changed(
            &mut families,
            self.projection_consumption_available != other.projection_consumption_available,
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption,
        );
        push_if_changed(
            &mut families,
            self.denial_presentation != other.denial_presentation,
            WorthUiQueryBindingPostureDriftFamily::DenialPresentation,
        );
        families
    }

    #[cfg(test)]
    pub(crate) fn with_query_support_status_for_test(
        &self,
        query_support_status: WorthUiQuerySupportStatus,
    ) -> Self {
        Self {
            query_support_status,
            support_contract_identity: self.support_contract_identity,
            installed_basis_authority: self.installed_basis_authority,
            lifecycle: self.lifecycle,
            async_result_state_available: self.async_result_state_available,
            recovery_available: self.recovery_available,
            inspection_available: self.inspection_available,
            projection_consumption_available: self.projection_consumption_available,
            denial_presentation: self.denial_presentation,
        }
    }
}

fn query_support_status_tag(status: WorthUiQuerySupportStatus) -> u64 {
    match status {
        WorthUiQuerySupportStatus::Supported => 1,
        WorthUiQuerySupportStatus::Deferred => 2,
        WorthUiQuerySupportStatus::Unsupported => 3,
    }
}

fn lifecycle_tag(lifecycle: worth_ui_query_binding::WorthUiQueryViewLifecycle) -> u64 {
    match lifecycle {
        worth_ui_query_binding::WorthUiQueryViewLifecycle::Snapshot => 1,
        worth_ui_query_binding::WorthUiQueryViewLifecycle::Live => 2,
    }
}

fn denial_presentation_tag(presentation: QueryDenialPresentation) -> u64 {
    match presentation {
        QueryDenialPresentation::Hidden => 1,
        QueryDenialPresentation::AdvisoryText => 2,
        QueryDenialPresentation::StructuredStatus => 3,
    }
}

fn push_if_changed(
    families: &mut Vec<WorthUiQueryBindingPostureDriftFamily>,
    changed: bool,
    family: WorthUiQueryBindingPostureDriftFamily,
) {
    if changed {
        families.push(family);
    }
}
