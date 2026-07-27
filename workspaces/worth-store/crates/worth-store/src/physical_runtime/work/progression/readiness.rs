use worth_signal::facade::{
    AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass, ResourceAttemptId, ResourceBranchEpoch,
    ResourceGeneration, ResourceRequestHandle,
};

use super::{AdmittedPhysicalWork, PhysicalSignalReadinessEvidence};
use crate::physical_runtime::work::{
    AdmittedPhysicalWorkAuthority, PhysicalWorkIntent, PhysicalWorkPressureClass,
};

// These move-owned packets deliberately stay inline: boxing every readiness
// transition would add allocator traffic to the ordinary physical-work path.
#[allow(clippy::large_enum_variant)]
pub enum PhysicalWorkReadiness {
    Blocked(BlockedPhysicalWork),
    Ready(ReadyPhysicalWork),
}

pub struct BlockedPhysicalWork {
    admitted: AdmittedPhysicalWork,
    class: AsyncNodeAdmissionClass,
    condition: Option<AsyncNodeConditionBlockClass>,
    active_request: Option<ResourceRequestHandle>,
}

pub struct ReadyPhysicalWork {
    pub(super) admitted: AdmittedPhysicalWork,
    pub(super) signal: PhysicalSignalReadinessEvidence,
}

impl PhysicalWorkReadiness {
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }
}

impl BlockedPhysicalWork {
    pub(in crate::physical_runtime) fn new(
        admitted: AdmittedPhysicalWork,
        class: AsyncNodeAdmissionClass,
        condition: Option<AsyncNodeConditionBlockClass>,
    ) -> Self {
        admitted.mark_stage(super::super::PhysicalWorkTerminalStage::Blocked);
        Self {
            admitted,
            class,
            condition,
            active_request: None,
        }
    }

    pub(in crate::physical_runtime) fn from_revalidation(
        admitted: AdmittedPhysicalWork,
        class: AsyncNodeAdmissionClass,
        condition: Option<AsyncNodeConditionBlockClass>,
        active_request: ResourceRequestHandle,
    ) -> Self {
        admitted.mark_stage(super::super::PhysicalWorkTerminalStage::Blocked);
        Self {
            admitted,
            class,
            condition,
            active_request: Some(active_request),
        }
    }

    pub const fn intent(&self) -> &PhysicalWorkIntent {
        self.admitted.intent()
    }

    pub const fn authority(&self) -> &AdmittedPhysicalWorkAuthority {
        self.admitted.authority()
    }

    pub const fn class(&self) -> AsyncNodeAdmissionClass {
        self.class
    }

    pub const fn condition(&self) -> Option<AsyncNodeConditionBlockClass> {
        self.condition
    }

    pub const fn active_request(&self) -> Option<ResourceRequestHandle> {
        self.active_request
    }

    pub fn into_admitted(self) -> AdmittedPhysicalWork {
        self.admitted
    }

    pub(in crate::physical_runtime) fn into_revalidation_parts(
        self,
    ) -> Option<(AdmittedPhysicalWork, ResourceRequestHandle)> {
        self.active_request
            .map(|active_request| (self.admitted, active_request))
    }
}

impl ReadyPhysicalWork {
    pub(in crate::physical_runtime) fn new(
        admitted: AdmittedPhysicalWork,
        signal: PhysicalSignalReadinessEvidence,
    ) -> Self {
        admitted.mark_stage(super::super::PhysicalWorkTerminalStage::Ready);
        let superseded = signal.replaces;
        let signal_bound = admitted.bind_signal(
            signal.signal_request,
            admitted.authority().binding(),
            superseded,
        );
        debug_assert!(signal_bound);
        Self { admitted, signal }
    }

    pub const fn intent(&self) -> &PhysicalWorkIntent {
        self.admitted.intent()
    }

    pub const fn authority(&self) -> &AdmittedPhysicalWorkAuthority {
        self.admitted.authority()
    }

    pub const fn signal_request(&self) -> ResourceRequestHandle {
        self.signal.signal_request
    }

    pub fn consumer_handle(&self) -> super::super::PhysicalWorkConsumerHandle {
        super::super::PhysicalWorkConsumerHandle::new(
            self.intent().identity(),
            self.signal.signal_request,
            self.authority().binding(),
        )
    }

    pub fn revalidated_from_signal_request(&self) -> Option<ResourceRequestHandle> {
        self.signal
            .supersession
            .as_ref()
            .map(|record| record.previous())
    }

    pub fn supersession(&self) -> Option<super::super::PhysicalWorkSupersessionJoin> {
        self.signal
            .supersession
            .clone()
            .map(super::super::PhysicalWorkSupersessionJoin::before_dispatch)
    }

    pub fn request_generation(&self) -> ResourceGeneration {
        self.signal.signal_request.generation()
    }

    pub fn request_epoch(&self) -> ResourceBranchEpoch {
        self.signal.signal_request.branch_epoch()
    }

    pub const fn request_attempt(&self) -> ResourceAttemptId {
        self.signal.attempt
    }

    pub fn capability_registry_digest(&self) -> &str {
        self.signal.capability_registry.as_str()
    }

    pub fn capability_bundle_digest(&self) -> &str {
        self.signal.capability_bundle.as_str()
    }

    pub fn payload_contract_digest(&self) -> &str {
        self.signal.payload_contract.as_str()
    }

    pub(in crate::physical_runtime::work) fn admit_scheduler_pressure(
        &self,
        pressure: PhysicalWorkPressureClass,
    ) -> Result<(), super::super::PhysicalWorkPreEffectDenial> {
        self.admitted.admit_scheduler_pressure(pressure)
    }

    pub(in crate::physical_runtime) fn require_consumer_active(
        &self,
    ) -> Result<(), super::super::PhysicalWorkPreEffectDenial> {
        self.admitted.require_consumer_active()
    }

    pub(in crate::physical_runtime) fn into_signal_parts(
        self,
    ) -> (AdmittedPhysicalWork, ResourceRequestHandle) {
        (self.admitted, self.signal.signal_request)
    }
}
