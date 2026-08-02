use crate::PhysicalStorageFaultInjection;
use worth_store_physical_backend::{
    ProductionStorageBoundaryControl, ProductionStorageBoundarySeam,
    ScriptedStorageBoundaryControl, StorageBoundaryExecutionIdentity, StorageBoundaryTrace,
};

use super::{PhysicalActorStep, PhysicalInterleavingSchedule, ScheduleReplayIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalActorStorageExecution {
    step: PhysicalActorStep,
    storage_trace: StorageBoundaryTrace,
    owner_execution: PhysicalScheduleOwnerExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScheduleOwnerExecution {
    storage_boundary_execution: StorageBoundaryExecutionIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScheduleExecution {
    schedule_identity: ScheduleReplayIdentity,
    storage_seam: ProductionStorageBoundarySeam,
    completed_steps: Vec<PhysicalActorStorageExecution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalScheduleExecutionError<E> {
    ActorExecution(E),
    ActorDidNotReachStorageSeam { actor_id: String },
    OwnerDidNotExecuteThroughActorControl { actor_id: String },
    OwnerExecutionReused { actor_id: String },
}

pub fn execute_physical_schedule<E>(
    schedule: &PhysicalInterleavingSchedule,
    storage_seam: ProductionStorageBoundarySeam,
    fault: Option<&PhysicalStorageFaultInjection>,
    mut execute_actor: impl FnMut(
        &PhysicalActorStep,
        &ScriptedStorageBoundaryControl,
    ) -> Result<StorageBoundaryExecutionIdentity, E>,
) -> Result<PhysicalScheduleExecution, PhysicalScheduleExecutionError<E>> {
    let mut completed_steps = Vec::with_capacity(schedule.actor_steps().len());
    let mut owner_execution_identities = std::collections::BTreeSet::new();
    for step in schedule.actor_steps() {
        let control = fault.map_or_else(
            || ScriptedStorageBoundaryControl::observe(storage_seam),
            |fault| fault.control_for_step(step, storage_seam),
        );
        let owner_execution_identity = execute_actor(step, &control)
            .map_err(PhysicalScheduleExecutionError::ActorExecution)?;
        let storage_trace = control.trace();
        if !storage_trace.reached().contains(&storage_seam) {
            return Err(
                PhysicalScheduleExecutionError::ActorDidNotReachStorageSeam {
                    actor_id: step.actor_id().to_owned(),
                },
            );
        }
        if control.execution_identity() != Some(owner_execution_identity) {
            return Err(
                PhysicalScheduleExecutionError::OwnerDidNotExecuteThroughActorControl {
                    actor_id: step.actor_id().to_owned(),
                },
            );
        }
        if !owner_execution_identities.insert(owner_execution_identity) {
            return Err(PhysicalScheduleExecutionError::OwnerExecutionReused {
                actor_id: step.actor_id().to_owned(),
            });
        }
        completed_steps.push(PhysicalActorStorageExecution {
            step: step.clone(),
            storage_trace,
            owner_execution: PhysicalScheduleOwnerExecution {
                storage_boundary_execution: owner_execution_identity,
            },
        });
    }
    Ok(PhysicalScheduleExecution {
        schedule_identity: schedule.identity().clone(),
        storage_seam,
        completed_steps,
    })
}

impl PhysicalActorStorageExecution {
    pub const fn step(&self) -> &PhysicalActorStep {
        &self.step
    }

    pub const fn storage_trace(&self) -> &StorageBoundaryTrace {
        &self.storage_trace
    }

    pub const fn owner_execution(&self) -> &PhysicalScheduleOwnerExecution {
        &self.owner_execution
    }
}

impl PhysicalScheduleOwnerExecution {
    pub const fn storage_boundary_execution_identity(&self) -> StorageBoundaryExecutionIdentity {
        self.storage_boundary_execution
    }
}

impl PhysicalScheduleExecution {
    pub const fn schedule_identity(&self) -> &ScheduleReplayIdentity {
        &self.schedule_identity
    }

    pub const fn storage_seam(&self) -> ProductionStorageBoundarySeam {
        self.storage_seam
    }

    pub fn completed_steps(&self) -> &[PhysicalActorStorageExecution] {
        &self.completed_steps
    }

    pub(crate) fn execution_for_actor(
        &self,
        actor_id: &str,
    ) -> Option<&PhysicalActorStorageExecution> {
        self.completed_steps
            .iter()
            .find(|execution| execution.step().actor_id() == actor_id)
    }
}
