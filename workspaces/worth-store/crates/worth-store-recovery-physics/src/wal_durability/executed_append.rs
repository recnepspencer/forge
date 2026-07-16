use std::io;
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendDurabilityBarrierDenial, BackendDurabilityProfile,
    BackendTargetProfile, StoreDurabilityAdmission, StoreDurabilityAppendInput,
    StoreDurabilityDenial, StoreDurabilityExecutionProof, StoreDurabilityOrderingBarrierDurable,
    StoreDurabilityRequirement, StoreDurabilityRuntime, UninterruptedStorageBoundaryControl,
    WalDurabilityBarrier,
};

use super::{
    AcknowledgmentPrecondition, DurableAckReceipt, IllegalAcknowledgmentDenial,
    WalAppendDurabilityScope, WalAppendPlan, WalAppendReceipt,
};

#[derive(Debug)]
pub enum WalDurabilityExecutionError {
    BackendProfileMismatch {
        expected: BackendTargetProfile,
        actual: BackendTargetProfile,
    },
    Admission(StoreDurabilityDenial),
    PhysicalIo(io::Error),
    Barrier(BackendDurabilityBarrierDenial),
    Acknowledgment(IllegalAcknowledgmentDenial),
    Artifact(worth_store_wal::WalArtifactStoreDenial),
}

#[derive(Debug)]
pub struct ExecutedWalDurabilityOutcome<P: BackendDurabilityProfile> {
    execution: StoreDurabilityExecutionProof<WalAppendDurabilityScope>,
    durability: StoreDurabilityOrderingBarrierDurable<WalAppendDurabilityScope>,
    append: WalAppendReceipt<P>,
    acknowledgment: DurableAckReceipt<P>,
}

impl<P: BackendDurabilityProfile> ExecutedWalDurabilityOutcome<P> {
    pub fn execution(&self) -> &StoreDurabilityExecutionProof<WalAppendDurabilityScope> {
        &self.execution
    }

    pub fn durability(&self) -> &StoreDurabilityOrderingBarrierDurable<WalAppendDurabilityScope> {
        &self.durability
    }

    pub const fn append(&self) -> &WalAppendReceipt<P> {
        &self.append
    }

    pub const fn acknowledgment(&self) -> &DurableAckReceipt<P> {
        &self.acknowledgment
    }
}

/// Executes the ordinary WAL durability chain from bytes to legal acknowledgment.
/// Every barrier receipt is derived from the sealed physical execution proof.
pub fn execute_wal_durability<P: BackendDurabilityProfile>(
    planner: &worth_store_wal::WalAppendPlanner,
    payload: &[u8],
    plan: WalAppendPlan<P>,
    backend: &AdmittedBackendCapabilityWitness,
) -> Result<ExecutedWalDurabilityOutcome<P>, WalDurabilityExecutionError> {
    execute_wal_durability_with_control(
        planner,
        payload,
        plan,
        backend,
        &UninterruptedStorageBoundaryControl,
    )
}

#[cfg(feature = "certification-test-authority")]
pub fn execute_wal_durability_with_boundary_control<P: BackendDurabilityProfile>(
    planner: &worth_store_wal::WalAppendPlanner,
    payload: &[u8],
    plan: WalAppendPlan<P>,
    backend: &AdmittedBackendCapabilityWitness,
    control: &impl worth_store_physical_backend::ProductionStorageBoundaryControl,
) -> Result<ExecutedWalDurabilityOutcome<P>, WalDurabilityExecutionError> {
    execute_wal_durability_with_control(planner, payload, plan, backend, control)
}

fn execute_wal_durability_with_control<P: BackendDurabilityProfile>(
    planner: &worth_store_wal::WalAppendPlanner,
    payload: &[u8],
    plan: WalAppendPlan<P>,
    backend: &AdmittedBackendCapabilityWitness,
    control: &impl worth_store_physical_backend::ProductionStorageBoundaryControl,
) -> Result<ExecutedWalDurabilityOutcome<P>, WalDurabilityExecutionError> {
    if backend.profile() != P::TARGET {
        return Err(WalDurabilityExecutionError::BackendProfileMismatch {
            expected: P::TARGET,
            actual: backend.profile(),
        });
    }
    let progress = plan.record_written_bytes(payload.len() as u64);
    let scope = progress.durability_scope();
    if planner.segment_id() != scope.segment_id().get()
        || planner.generation() != scope.generation().get()
    {
        return Err(WalDurabilityExecutionError::Artifact(
            worth_store_wal::WalArtifactStoreDenial::StoreBindingMismatch,
        ));
    }
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(P::REQUIRED_BARRIERS);
    let admission = StoreDurabilityAdmission::admit(requirement, backend)
        .map_err(WalDurabilityExecutionError::Admission)?;
    let accepted = admission.submit_write(scope.clone()).backend_accepted();
    let execution = execute_framed_append(planner, payload, &scope, &accepted, control)?;

    let mut completed = progress;
    for barrier in ALL_WAL_BARRIERS {
        if P::REQUIRED_BARRIERS.contains(barrier) {
            let receipt = execution
                .certify_completed_barrier::<P>(barrier)
                .map_err(WalDurabilityExecutionError::Barrier)?;
            completed = completed
                .complete_barrier(receipt)
                .map_err(WalDurabilityExecutionError::Acknowledgment)?;
        }
    }

    let boundary = accepted
        .reach_durability_boundary(execution.clone())
        .map_err(WalDurabilityExecutionError::Admission)?;
    let durability = if requirement.requires_parent_namespace_durable() {
        boundary
            .parent_namespace_durable()
            .and_then(|parent| parent.ordering_barrier_durable())
    } else {
        boundary.ordering_barrier_durable()
    }
    .map_err(WalDurabilityExecutionError::Admission)?;

    let append = completed
        .finish()
        .map_err(WalDurabilityExecutionError::Acknowledgment)?;
    let precondition = AcknowledgmentPrecondition::from_append_receipt(append.clone())
        .map_err(WalDurabilityExecutionError::Acknowledgment)?;
    let acknowledgment = DurableAckReceipt::acknowledge(precondition);
    Ok(ExecutedWalDurabilityOutcome {
        execution,
        durability,
        append,
        acknowledgment,
    })
}

fn execute_framed_append<S: Clone>(
    planner: &worth_store_wal::WalAppendPlanner,
    payload: &[u8],
    scope: &WalAppendDurabilityScope,
    accepted: &worth_store_physical_backend::StoreDurabilityWriteAccepted<S>,
    control: &impl worth_store_physical_backend::ProductionStorageBoundaryControl,
) -> Result<StoreDurabilityExecutionProof<S>, WalDurabilityExecutionError> {
    const MAX_CONCURRENT_REPLANS: usize = 8;
    let mut runtime = StoreDurabilityRuntime::new();
    for _ in 0..MAX_CONCURRENT_REPLANS {
        let append = planner
            .prepare_append(
                scope.lsn_range().start().get(),
                scope.lsn_range().end_exclusive().get(),
                scope.frame_digest().as_str(),
                payload,
            )
            .map_err(WalDurabilityExecutionError::Artifact)?;
        match runtime.persist_append_and_execute_with_control(
            planner.root(),
            StoreDurabilityAppendInput::new(
                append.relative_path(),
                append.encoded_frame(),
                append.observed_file_bytes(),
                append.valid_prefix_bytes(),
            ),
            accepted,
            control,
        ) {
            Ok(execution) => return Ok(execution),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => continue,
            Err(error) => return Err(WalDurabilityExecutionError::PhysicalIo(error)),
        }
    }
    Err(WalDurabilityExecutionError::PhysicalIo(io::Error::new(
        io::ErrorKind::WouldBlock,
        "WAL segment changed during every bounded append replan",
    )))
}

const ALL_WAL_BARRIERS: [WalDurabilityBarrier; 6] = [
    WalDurabilityBarrier::SimulatedDurableCommit,
    WalDurabilityBarrier::WalFileFsync,
    WalDurabilityBarrier::WalDirectoryFsync,
    WalDurabilityBarrier::WindowsFlushFileBuffers,
    WalDurabilityBarrier::WindowsDirectorySync,
    WalDurabilityBarrier::OrderedPersistenceFence,
];
