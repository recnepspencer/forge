use worth_signal::facade::ResourceRequestHandle;

use super::super::submission::{PhysicalEffectActivity, PhysicalWorkCapacityLease};
use super::super::{
    AdmittedPhysicalWorkAuthority, PhysicalWorkIntent, PhysicalWorkOperationFamily,
    PhysicalWorkPressureClass, PhysicalWorkRecoveryDisposition, PhysicalWorkSignalFamily,
    PhysicalWorkTerminalStage,
};

pub struct AdmittedPhysicalWork {
    intent: PhysicalWorkIntent,
    authority: AdmittedPhysicalWorkAuthority,
    capacity: PhysicalWorkCapacityLease,
}

impl AdmittedPhysicalWork {
    pub(in crate::physical_runtime::work) fn new(
        intent: PhysicalWorkIntent,
        authority: AdmittedPhysicalWorkAuthority,
        capacity: PhysicalWorkCapacityLease,
    ) -> Self {
        Self {
            intent,
            authority,
            capacity,
        }
    }

    pub const fn intent(&self) -> &PhysicalWorkIntent {
        &self.intent
    }

    pub const fn authority(&self) -> &AdmittedPhysicalWorkAuthority {
        &self.authority
    }

    pub(super) fn mark_stage(&self, stage: PhysicalWorkTerminalStage) {
        self.capacity.mark_stage(stage);
    }

    pub(super) fn bind_signal(
        &self,
        request: ResourceRequestHandle,
        route: super::super::PhysicalSignalAspectBindingDigest,
        superseded: Option<ResourceRequestHandle>,
    ) -> bool {
        self.capacity.bind_signal(request, route, superseded)
    }

    pub(in crate::physical_runtime) fn register_signal_locality(
        &self,
        route: super::super::PhysicalSignalAspectBindingDigest,
    ) -> bool {
        self.capacity.register_signal_locality(route)
    }

    pub(super) fn release_settled(
        &self,
        fate: super::super::PhysicalWorkEffectFate,
        recovery: PhysicalWorkRecoveryDisposition,
    ) {
        self.capacity.release_settled(fate, recovery);
    }

    pub(super) fn mark_retry_pending(&self) {
        self.capacity.mark_retry_pending();
    }

    pub(super) fn is_cancelled(&self) -> bool {
        self.capacity.is_cancelled()
    }

    pub(super) fn begin_dispatch(&self) -> Option<PhysicalEffectActivity> {
        self.capacity.begin_dispatch()
    }

    pub(super) fn mark_pressure(&self, pressure: PhysicalWorkPressureClass) -> bool {
        self.capacity.mark_pressure(pressure)
    }

    pub(in crate::physical_runtime) const fn signal_family(&self) -> PhysicalWorkSignalFamily {
        match self.intent.operation() {
            PhysicalWorkOperationFamily::ArtifactMetadataRead
            | PhysicalWorkOperationFamily::ArtifactRangeRead => PhysicalWorkSignalFamily::ReadFault,
            PhysicalWorkOperationFamily::ArtifactRangeWrite => {
                PhysicalWorkSignalFamily::ExactWriteback
            }
            PhysicalWorkOperationFamily::ArtifactPublication => {
                PhysicalWorkSignalFamily::Publication
            }
        }
    }
}
