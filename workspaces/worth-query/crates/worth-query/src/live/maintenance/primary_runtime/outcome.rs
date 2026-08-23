use super::super::{
    WorthQueryLivePublicationDenial, WorthQueryMaintenanceDenial, WorthQueryPublishedLiveDelivery,
};

pub enum WorthQueryPrimaryGranularMaintenanceOutcome {
    NoRelevantChange(WorthQueryGranularNoChange),
    Performed(WorthQueryPrimaryGranularMaintenancePerformed),
}

pub struct WorthQueryGranularNoChange {
    pub(super) lower_truth_delivery_count: usize,
    pub(super) lower_signal_performed_delivery_count: usize,
    pub(super) duplicate_delivery_count: usize,
    pub(super) already_settled_delivery_count: usize,
    pub(super) irrelevant_delivery_count: usize,
    pub(super) suppressed_impact_count: usize,
    pub(super) admission_counters: crate::domain_installation::WorthQueryGranularAdmissionCounters,
    pub(super) impact_observations:
        Vec<crate::domain_installation::WorthQueryAdmittedInvalidationObservation>,
}

impl WorthQueryGranularNoChange {
    pub const fn lower_truth_delivery_count(&self) -> usize {
        self.lower_truth_delivery_count
    }
    pub const fn lower_signal_performed_delivery_count(&self) -> usize {
        self.lower_signal_performed_delivery_count
    }
    pub const fn duplicate_delivery_count(&self) -> usize {
        self.duplicate_delivery_count
    }
    pub const fn already_settled_delivery_count(&self) -> usize {
        self.already_settled_delivery_count
    }
    pub const fn irrelevant_delivery_count(&self) -> usize {
        self.irrelevant_delivery_count
    }
    pub const fn suppressed_impact_count(&self) -> usize {
        self.suppressed_impact_count
    }
    pub const fn admission_counters(
        &self,
    ) -> crate::domain_installation::WorthQueryGranularAdmissionCounters {
        self.admission_counters
    }
    #[doc(hidden)]
    pub fn impact_observations(
        &self,
    ) -> &[crate::domain_installation::WorthQueryAdmittedInvalidationObservation] {
        &self.impact_observations
    }
}

pub struct WorthQueryPrimaryGranularMaintenancePerformed {
    pub(super) refresh: crate::domain_installation::WorthQueryLiveProjectionRefresh,
    pub(super) deliveries: Vec<WorthQueryPublishedLiveDelivery>,
    pub(super) admitted_impact_count: usize,
    pub(super) shared_execution_count: usize,
    pub(super) duplicate_delivery_count: usize,
    pub(super) performed_promotion_count: usize,
    pub(super) lower_truth_delivery_count: usize,
    pub(super) lower_signal_performed_delivery_count: usize,
    pub(super) admission_counters: crate::domain_installation::WorthQueryGranularAdmissionCounters,
    pub(super) maintenance_counters: super::super::WorthQueryGranularMaintenanceCounters,
    pub(super) impact_observations:
        Vec<crate::domain_installation::WorthQueryAdmittedInvalidationObservation>,
}

impl WorthQueryPrimaryGranularMaintenancePerformed {
    pub fn deliveries(&self) -> &[WorthQueryPublishedLiveDelivery] {
        &self.deliveries
    }
    pub const fn shared_execution_count(&self) -> usize {
        self.shared_execution_count
    }
    pub const fn duplicate_delivery_count(&self) -> usize {
        self.duplicate_delivery_count
    }
    pub const fn performed_promotion_count(&self) -> usize {
        self.performed_promotion_count
    }
    pub const fn lower_truth_delivery_count(&self) -> usize {
        self.lower_truth_delivery_count
    }
    pub const fn lower_signal_performed_delivery_count(&self) -> usize {
        self.lower_signal_performed_delivery_count
    }
    pub const fn admitted_impact_count(&self) -> usize {
        self.admitted_impact_count
    }
    pub const fn maintenance_operation_count(&self) -> usize {
        self.shared_execution_count
    }
    pub const fn consumer_publication_count(&self) -> usize {
        self.deliveries.len()
    }
    pub const fn admission_counters(
        &self,
    ) -> crate::domain_installation::WorthQueryGranularAdmissionCounters {
        self.admission_counters
    }
    pub const fn maintenance_counters(
        &self,
    ) -> super::super::WorthQueryGranularMaintenanceCounters {
        self.maintenance_counters
    }
    #[doc(hidden)]
    pub fn impact_observations(
        &self,
    ) -> &[crate::domain_installation::WorthQueryAdmittedInvalidationObservation] {
        &self.impact_observations
    }
    pub fn into_parts(
        self,
    ) -> (
        crate::domain_installation::WorthQueryLiveProjectionRefresh,
        Vec<WorthQueryPublishedLiveDelivery>,
    ) {
        (self.refresh, self.deliveries)
    }
}

#[derive(Debug)]
pub enum WorthQueryPrimaryGranularMaintenanceDenial {
    ForeignPrimaryRuntime,
    Admission(crate::domain_installation::WorthQueryImpactAdmissionDenial),
    MixedMaintenancePosture,
    Execution(crate::domain_installation::WorthQueryLiveProjectionRefreshError),
    Maintenance(WorthQueryMaintenanceDenial),
    Publication(WorthQueryLivePublicationDenial),
}
