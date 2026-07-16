use worth_store_physical_backend::{
    ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    StorageBoundaryTrace,
};

use super::{FaultDeliveryDenial, PhysicalFaultEvent, PhysicalFaultEventKind};
use crate::{PhysicalActorId, PhysicalActorStep, PhysicalBoundarySeam, PhysicalScheduleExecution};

#[derive(Debug, Clone)]
pub struct PhysicalStorageFaultInjection {
    event_kind: PhysicalFaultEventKind,
    target_actor: PhysicalActorId,
    seam: ProductionStorageBoundarySeam,
    fault: StorageBoundaryFault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStorageFaultExecution {
    event_kind: PhysicalFaultEventKind,
    target_actor: PhysicalActorId,
    seam: ProductionStorageBoundarySeam,
    fault: StorageBoundaryFault,
    trace: StorageBoundaryTrace,
}

impl PhysicalStorageFaultInjection {
    pub fn for_actor_step(
        event: &PhysicalFaultEvent,
        target: &PhysicalActorStep,
    ) -> Result<Self, FaultDeliveryDenial> {
        let PhysicalBoundarySeam::ProductionStorage(seam) = event.required_seam() else {
            return Err(FaultDeliveryDenial::FaultHasNoProductionStorageInjection(
                event.kind(),
            ));
        };
        let offset = event
            .locus()
            .and_then(|locus| locus.offset())
            .map_or(0, |offset| offset.byte_offset());
        let fault = match event.kind() {
            PhysicalFaultEventKind::Crash => StorageBoundaryFault::Interrupt,
            PhysicalFaultEventKind::TornWrite => StorageBoundaryFault::TearWrite {
                retained_bytes: offset,
            },
            PhysicalFaultEventKind::DroppedFlush => {
                StorageBoundaryFault::AbortBeforeDurabilityBarrier
            }
            PhysicalFaultEventKind::ByteCorruption => StorageBoundaryFault::CorruptByte {
                relative_offset: offset,
                xor_mask: 0xff,
            },
            kind => {
                return Err(FaultDeliveryDenial::FaultHasNoProductionStorageInjection(
                    kind,
                ))
            }
        };
        Ok(Self {
            event_kind: event.kind(),
            target_actor: target.actor_id_proof().clone(),
            seam,
            fault,
        })
    }

    pub(crate) fn control_for_step(
        &self,
        step: &PhysicalActorStep,
        scheduled_seam: ProductionStorageBoundarySeam,
    ) -> ScriptedStorageBoundaryControl {
        if step.actor_id_proof() == &self.target_actor && self.seam == scheduled_seam {
            ScriptedStorageBoundaryControl::inject(self.seam, self.fault)
        } else {
            ScriptedStorageBoundaryControl::observe(scheduled_seam)
        }
    }

    pub fn confirm_execution(
        &self,
        execution: &PhysicalScheduleExecution,
    ) -> Result<PhysicalStorageFaultExecution, FaultDeliveryDenial> {
        let actor_execution = execution
            .execution_for_actor(self.target_actor.as_str())
            .ok_or(FaultDeliveryDenial::ProductionStorageFaultWasNotExecuted)?;
        let trace = actor_execution.storage_trace();
        if execution.storage_seam() != self.seam
            || !trace.injected().contains(&(self.seam, self.fault))
        {
            return Err(FaultDeliveryDenial::ProductionStorageFaultWasNotExecuted);
        }
        Ok(PhysicalStorageFaultExecution {
            event_kind: self.event_kind,
            target_actor: self.target_actor.clone(),
            seam: self.seam,
            fault: self.fault,
            trace: trace.clone(),
        })
    }
}

impl PhysicalStorageFaultExecution {
    pub const fn event_kind(&self) -> PhysicalFaultEventKind {
        self.event_kind
    }

    pub fn target_actor(&self) -> &str {
        self.target_actor.as_str()
    }

    pub const fn seam(&self) -> ProductionStorageBoundarySeam {
        self.seam
    }

    pub const fn fault(&self) -> StorageBoundaryFault {
        self.fault
    }

    pub const fn trace(&self) -> &StorageBoundaryTrace {
        &self.trace
    }
}
