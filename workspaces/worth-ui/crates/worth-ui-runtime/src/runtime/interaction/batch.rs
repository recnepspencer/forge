use worth_ui_host_contract::UiHostObservationCanonicalCore;

use super::{
    UiInteractionLifecycleSettlementReceipt, UiInteractionStateSnapshot, UiInteractionTransition,
};

#[derive(Debug)]
pub struct UiInteractionBatchReceipt {
    pub(super) core: UiHostObservationCanonicalCore,
    pub(super) frame_relation: crate::facade::observation_report::UiHostObservationFrameRelation,
    pub(super) disposition: crate::facade::observation_report::UiHostObservationBatchDisposition,
    pub(super) transitions: Box<[UiInteractionTransition]>,
    pub(super) ignored_reports: usize,
    pub(super) state: UiInteractionStateSnapshot,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiInteractionObservationDenial {
    pub(super) denial: crate::facade::observation_report::UiHostObservationReportDenial,
    pub(super) settlement: UiInteractionLifecycleSettlementReceipt,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiQuarantinedHostInteractionBatch {
    pub(super) quarantine: crate::facade::observation_report::UiQuarantinedHostObservationBatch,
    pub(super) settlement: UiInteractionLifecycleSettlementReceipt,
}

#[derive(Debug)]
pub enum UiHostInteractionIngressOutcome {
    Applied(UiInteractionBatchReceipt),
    Duplicate(crate::facade::observation_report::UiDuplicateHostObservationBatch),
    Quarantined(UiQuarantinedHostInteractionBatch),
    Denied(UiInteractionObservationDenial),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct UiInteractionShutdownReport {
    pub(super) settlement: Option<UiInteractionLifecycleSettlementReceipt>,
}

impl UiInteractionBatchReceipt {
    pub const fn canonical_core(&self) -> UiHostObservationCanonicalCore {
        self.core
    }

    pub const fn frame_relation(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationFrameRelation {
        self.frame_relation
    }

    pub const fn disposition(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationBatchDisposition {
        self.disposition
    }

    pub fn transitions(&self) -> &[UiInteractionTransition] {
        &self.transitions
    }

    pub fn into_transitions(self) -> Box<[UiInteractionTransition]> {
        self.transitions
    }

    pub const fn ignored_reports(&self) -> usize {
        self.ignored_reports
    }

    pub const fn state(&self) -> UiInteractionStateSnapshot {
        self.state
    }
}

impl UiInteractionObservationDenial {
    pub(crate) const fn new(
        denial: crate::facade::observation_report::UiHostObservationReportDenial,
        settlement: UiInteractionLifecycleSettlementReceipt,
    ) -> Self {
        Self { denial, settlement }
    }

    pub const fn denial(&self) -> crate::facade::observation_report::UiHostObservationReportDenial {
        self.denial
    }

    pub const fn settlement(&self) -> &UiInteractionLifecycleSettlementReceipt {
        &self.settlement
    }
}

impl UiQuarantinedHostInteractionBatch {
    pub(crate) const fn new(
        quarantine: crate::facade::observation_report::UiQuarantinedHostObservationBatch,
        settlement: UiInteractionLifecycleSettlementReceipt,
    ) -> Self {
        Self {
            quarantine,
            settlement,
        }
    }

    pub const fn quarantine(
        &self,
    ) -> crate::facade::observation_report::UiQuarantinedHostObservationBatch {
        self.quarantine
    }

    pub const fn settlement(&self) -> &UiInteractionLifecycleSettlementReceipt {
        &self.settlement
    }
}

impl UiInteractionShutdownReport {
    pub fn cancelled_gestures(&self) -> usize {
        self.settlement
            .as_ref()
            .map_or(0, UiInteractionLifecycleSettlementReceipt::settled_gestures)
    }

    pub fn cancelled_draft_sessions(&self) -> usize {
        self.settlement.as_ref().map_or(
            0,
            UiInteractionLifecycleSettlementReceipt::settled_draft_sessions,
        )
    }

    pub fn final_state(&self) -> Option<UiInteractionStateSnapshot> {
        self.settlement
            .as_ref()
            .map(UiInteractionLifecycleSettlementReceipt::final_state)
    }

    pub const fn settlement(&self) -> Option<&UiInteractionLifecycleSettlementReceipt> {
        self.settlement.as_ref()
    }
}
