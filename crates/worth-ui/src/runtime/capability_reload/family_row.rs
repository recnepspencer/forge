use super::{
    WorthUiCapabilityReloadDenialCode, WorthUiCapabilityReloadFamilyCounters,
    WorthUiCapabilityReloadFamilyKind, WorthUiComponentCompatibility,
    WorthUiComponentReloadReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadFamilyStatus {
    AdmittedChanged,
    EquivalentNoOp,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCapabilityReloadFamilyRow {
    family: WorthUiCapabilityReloadFamilyKind,
    status: WorthUiCapabilityReloadFamilyStatus,
    request_digest: u64,
    counters: WorthUiCapabilityReloadFamilyCounters,
    denial_detail: Option<String>,
    denial_code: Option<WorthUiCapabilityReloadDenialCode>,
    component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
}

impl WorthUiCapabilityReloadFamilyRow {
    #[cfg(test)]
    pub(crate) fn admitted(
        family: WorthUiCapabilityReloadFamilyKind,
        request_digest: u64,
        counters: WorthUiCapabilityReloadFamilyCounters,
        candidate_changed: bool,
    ) -> Self {
        Self::admitted_with_component_reload_receipt(
            family,
            request_digest,
            counters,
            candidate_changed,
            None,
        )
    }

    pub(crate) fn admitted_with_component_reload_receipt(
        family: WorthUiCapabilityReloadFamilyKind,
        request_digest: u64,
        counters: WorthUiCapabilityReloadFamilyCounters,
        candidate_changed: bool,
        component_reload_receipt: Option<WorthUiComponentReloadReceipt>,
    ) -> Self {
        Self {
            family,
            status: if candidate_changed {
                WorthUiCapabilityReloadFamilyStatus::AdmittedChanged
            } else {
                WorthUiCapabilityReloadFamilyStatus::EquivalentNoOp
            },
            request_digest,
            counters,
            denial_detail: None,
            denial_code: None,
            component_reload_receipt,
        }
    }

    pub(crate) fn denied_with_counters(
        family: WorthUiCapabilityReloadFamilyKind,
        request_digest: u64,
        counters: WorthUiCapabilityReloadFamilyCounters,
        denial_code: Option<WorthUiCapabilityReloadDenialCode>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            family,
            status: WorthUiCapabilityReloadFamilyStatus::Denied,
            request_digest,
            counters,
            denial_detail: Some(detail.into()),
            denial_code,
            component_reload_receipt: None,
        }
    }

    pub fn family(&self) -> WorthUiCapabilityReloadFamilyKind {
        self.family
    }

    pub fn status(&self) -> WorthUiCapabilityReloadFamilyStatus {
        self.status
    }

    pub fn request_digest(&self) -> u64 {
        self.request_digest
    }

    pub fn counters(&self) -> WorthUiCapabilityReloadFamilyCounters {
        self.counters
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    pub fn denial_code(&self) -> Option<WorthUiCapabilityReloadDenialCode> {
        self.denial_code
    }

    pub fn component_compatibility(&self) -> Option<&WorthUiComponentCompatibility> {
        self.component_reload_receipt
            .as_ref()
            .map(WorthUiComponentReloadReceipt::compatibility)
    }

    pub fn component_reload_receipt(&self) -> Option<&WorthUiComponentReloadReceipt> {
        self.component_reload_receipt.as_ref()
    }
}
