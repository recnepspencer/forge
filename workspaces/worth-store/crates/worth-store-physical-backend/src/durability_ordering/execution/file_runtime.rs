use std::io;
use std::path::Path;

use crate::{
    ProductionStorageBoundaryControl, ProductionStorageBoundarySeam,
    UninterruptedStorageBoundaryControl, WalDurabilityBarrier, WalDurabilityBarrierSet,
};

use super::super::{physical_target::StoreDurabilityTarget, StoreDurabilityWriteAccepted};
use super::{
    PhysicalStoreDurabilityExecutor, StoreDurabilityExecutionObservation,
    StoreDurabilityExecutionProof, StoreDurabilityExecutionRequest, StoreDurabilityFileSyncKind,
    StoreOwnedDurabilityExecution,
};

/// Ordinary physical-backend durability executor. Callers submit an admitted
/// write; only this owner derives completion facts from the required steps.
#[derive(Debug, Default)]
pub struct StoreDurabilityRuntime {
    executions: u64,
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "certification-test-authority")]
pub struct StoreDurabilityAppendInput<'input> {
    relative_path: &'input Path,
    encoded_frame: &'input [u8],
    observed_file_bytes: u64,
    valid_prefix_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreDurabilityExecutionBoundary {
    FileSynchronized,
    Complete,
}

impl StoreDurabilityRuntime {
    pub const fn new() -> Self {
        Self { executions: 0 }
    }

    pub fn persist_and_execute<S: Clone>(
        &mut self,
        root: &Path,
        payload: &[u8],
        accepted: &StoreDurabilityWriteAccepted<S>,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        self.persist_and_execute_to(
            root,
            payload,
            accepted,
            StoreDurabilityExecutionBoundary::Complete,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn persist_append_and_execute<S: Clone>(
        &mut self,
        root: &Path,
        append: StoreDurabilityAppendInput<'_>,
        accepted: &StoreDurabilityWriteAccepted<S>,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        self.persist_append_and_execute_with_control(
            root,
            append,
            accepted,
            &UninterruptedStorageBoundaryControl,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn persist_append_and_execute_with_control<S: Clone>(
        &mut self,
        root: &Path,
        append: StoreDurabilityAppendInput<'_>,
        accepted: &StoreDurabilityWriteAccepted<S>,
        control: &impl ProductionStorageBoundaryControl,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        let target = StoreDurabilityTarget::append(
            root,
            append.relative_path,
            accepted.requirement(),
            append.encoded_frame,
            append.observed_file_bytes,
            append.valid_prefix_bytes,
        )?;
        self.execute_target(
            target,
            accepted,
            StoreDurabilityExecutionBoundary::Complete,
            control,
        )
    }

    pub fn persist_and_execute_to<S: Clone>(
        &mut self,
        root: &Path,
        payload: &[u8],
        accepted: &StoreDurabilityWriteAccepted<S>,
        boundary: StoreDurabilityExecutionBoundary,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        self.persist_and_execute_to_with_control(
            root,
            payload,
            accepted,
            boundary,
            &UninterruptedStorageBoundaryControl,
        )
    }

    pub fn persist_and_execute_to_with_control<S: Clone>(
        &mut self,
        root: &Path,
        payload: &[u8],
        accepted: &StoreDurabilityWriteAccepted<S>,
        boundary: StoreDurabilityExecutionBoundary,
        control: &impl ProductionStorageBoundaryControl,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        let target = StoreDurabilityTarget::persist(root, accepted.requirement(), payload)?;
        self.execute_target(target, accepted, boundary, control)
    }

    fn execute_target<S: Clone>(
        &mut self,
        mut target: StoreDurabilityTarget,
        accepted: &StoreDurabilityWriteAccepted<S>,
        boundary: StoreDurabilityExecutionBoundary,
        control: &impl ProductionStorageBoundaryControl,
    ) -> io::Result<StoreDurabilityExecutionProof<S>> {
        let mut backend = FileDurabilityBackend {
            target: &mut target,
            boundary,
            control,
        };
        let proof = StoreOwnedDurabilityExecution::execute_with_backend(&mut backend, accepted)?;
        self.executions = self.executions.saturating_add(1);
        Ok(proof)
    }

    pub const fn executions(&self) -> u64 {
        self.executions
    }
}

#[cfg(feature = "certification-test-authority")]
impl<'input> StoreDurabilityAppendInput<'input> {
    pub const fn new(
        relative_path: &'input Path,
        encoded_frame: &'input [u8],
        observed_file_bytes: u64,
        valid_prefix_bytes: u64,
    ) -> Self {
        Self {
            relative_path,
            encoded_frame,
            observed_file_bytes,
            valid_prefix_bytes,
        }
    }
}

struct FileDurabilityBackend<'target, 'control, Control> {
    target: &'target mut StoreDurabilityTarget,
    boundary: StoreDurabilityExecutionBoundary,
    control: &'control Control,
}

impl<S, Control> PhysicalStoreDurabilityExecutor<S> for FileDurabilityBackend<'_, '_, Control>
where
    Control: ProductionStorageBoundaryControl,
{
    type Error = io::Error;

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error> {
        let requirement = request.requirement();
        self.target.reach_boundary(
            self.control,
            ProductionStorageBoundarySeam::WalAppendBeforeFlush,
        )?;
        self.target
            .reach_boundary(self.control, ProductionStorageBoundarySeam::WalFlush)?;
        match requirement.required_file_sync() {
            StoreDurabilityFileSyncKind::Fdatasync => self.target.sync_data()?,
            StoreDurabilityFileSyncKind::Fsync => self.target.sync_all()?,
        }
        let mut observation = StoreDurabilityExecutionObservation::new(
            completed_file_barriers(requirement.required_barriers()),
            requirement.required_file_sync(),
        );
        if self.boundary == StoreDurabilityExecutionBoundary::FileSynchronized {
            return Ok(observation.with_persisted_artifact(
                self.target.persisted_path(false).to_path_buf(),
                self.target.persisted_offset(),
                self.target.bytes_written(),
            ));
        }
        if requirement.requires_rename_durable() {
            self.target.rename_publication()?;
            observation = observation.with_rename_completed();
        }
        if requirement.requires_directory_sync() {
            self.target
                .reach_boundary(self.control, ProductionStorageBoundarySeam::DirectorySync)?;
            self.target
                .sync_parent_namespace(requirement.requires_rename_durable())?;
            observation = observation
                .with_directory_sync_completed()
                .with_completed_barriers(completed_directory_barriers(
                    requirement.required_barriers(),
                ));
        }
        if requirement.requires_ordering_barrier() {
            observation = observation
                .with_ordering_barrier_completed()
                .with_completed_barriers(completed_ordering_barriers(
                    requirement.required_barriers(),
                ));
        }
        let rename_completed = observation.rename_completed;
        Ok(observation.with_persisted_artifact(
            self.target.persisted_path(rename_completed).to_path_buf(),
            self.target.persisted_offset(),
            self.target.bytes_written(),
        ))
    }
}

const fn completed_file_barriers(required: WalDurabilityBarrierSet) -> WalDurabilityBarrierSet {
    retain_barriers(
        required,
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::SimulatedDurableCommit)
            .insert(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WindowsFlushFileBuffers),
    )
}

const fn completed_directory_barriers(
    required: WalDurabilityBarrierSet,
) -> WalDurabilityBarrierSet {
    retain_barriers(
        required,
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalDirectoryFsync)
            .insert(WalDurabilityBarrier::WindowsDirectorySync),
    )
}

const fn completed_ordering_barriers(required: WalDurabilityBarrierSet) -> WalDurabilityBarrierSet {
    retain_barriers(
        required,
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::OrderedPersistenceFence),
    )
}

const fn retain_barriers(
    required: WalDurabilityBarrierSet,
    candidates: WalDurabilityBarrierSet,
) -> WalDurabilityBarrierSet {
    WalDurabilityBarrierSet::from_bits(required.bits() & candidates.bits())
}
