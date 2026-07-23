use worth_signal::facade::{
    AsyncNodeAdmissionClass, AsyncNodeConditionBlockClass, ResourceAttemptId, ResourceBranchEpoch,
    ResourceGeneration, ResourcePayloadContractDigest, ResourcePolicyDigest, ResourceRequestHandle,
};
use worth_store_io_scheduler::QueueExecutionReadyPlan;

use super::{
    AdmittedPhysicalWorkAuthority, PhysicalWorkIntent, PhysicalWorkOperationFamily,
    PhysicalWorkSignalFamily,
};

pub struct AdmittedPhysicalWork {
    intent: PhysicalWorkIntent,
    authority: AdmittedPhysicalWorkAuthority,
    capacity: super::submission::PhysicalWorkCapacityLease,
}

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
    admitted: AdmittedPhysicalWork,
    signal: PhysicalSignalReadinessEvidence,
}

pub(in crate::physical_runtime) struct PhysicalSignalReadinessEvidence {
    pub(in crate::physical_runtime) signal_request: ResourceRequestHandle,
    pub(in crate::physical_runtime) revalidated_from: Option<ResourceRequestHandle>,
    pub(in crate::physical_runtime) attempt: ResourceAttemptId,
    pub(in crate::physical_runtime) capability_registry: ResourcePolicyDigest,
    pub(in crate::physical_runtime) capability_bundle: ResourcePolicyDigest,
    pub(in crate::physical_runtime) payload_contract: ResourcePayloadContractDigest,
}

pub struct ResourceAdmittedPhysicalWork {
    ready: ReadyPhysicalWork,
    queue_plan: QueueExecutionReadyPlan,
}

pub struct DispatchedPhysicalWork {
    resource_admitted: ResourceAdmittedPhysicalWork,
}

pub struct SettledPhysicalWork {
    dispatched: DispatchedPhysicalWork,
}

impl AdmittedPhysicalWork {
    pub(in crate::physical_runtime::work) fn new(
        intent: PhysicalWorkIntent,
        authority: AdmittedPhysicalWorkAuthority,
        capacity: super::submission::PhysicalWorkCapacityLease,
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

    fn mark_stage(&self, stage: super::PhysicalWorkTerminalStage) {
        self.capacity.mark_stage(stage);
    }

    pub(in crate::physical_runtime) const fn signal_family(&self) -> PhysicalWorkSignalFamily {
        match self.intent.operation() {
            PhysicalWorkOperationFamily::ArtifactRangeRead => PhysicalWorkSignalFamily::ReadFault,
            PhysicalWorkOperationFamily::ArtifactRangeWrite => {
                PhysicalWorkSignalFamily::ExactWriteback
            }
            PhysicalWorkOperationFamily::ArtifactPublication => {
                PhysicalWorkSignalFamily::Publication
            }
        }
    }
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
        admitted.mark_stage(super::PhysicalWorkTerminalStage::Ready);
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
        admitted.mark_stage(super::PhysicalWorkTerminalStage::Ready);
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

    pub const fn revalidated_from_signal_request(&self) -> Option<ResourceRequestHandle> {
        self.signal.revalidated_from
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

    pub(in crate::physical_runtime) fn into_signal_parts(
        self,
    ) -> (AdmittedPhysicalWork, ResourceRequestHandle) {
        (self.admitted, self.signal.signal_request)
    }
}

impl ResourceAdmittedPhysicalWork {
    pub(in crate::physical_runtime) fn new(
        ready: ReadyPhysicalWork,
        queue_plan: QueueExecutionReadyPlan,
    ) -> Self {
        ready
            .admitted
            .mark_stage(super::PhysicalWorkTerminalStage::Queued);
        Self { ready, queue_plan }
    }

    pub const fn intent(&self) -> &PhysicalWorkIntent {
        self.ready.intent()
    }

    pub const fn queue_plan(&self) -> &QueueExecutionReadyPlan {
        &self.queue_plan
    }
}

impl DispatchedPhysicalWork {
    pub const fn intent(&self) -> &PhysicalWorkIntent {
        self.resource_admitted.intent()
    }
}

impl SettledPhysicalWork {
    pub const fn intent(&self) -> &PhysicalWorkIntent {
        self.dispatched.intent()
    }
}
