use worth_store_buffer_pool::{
    PhysicalBoundedFrameAccess, PhysicalBoundedFrameKey, PhysicalFrameAccess,
    PhysicalFrameFaultError, PhysicalFrameKey, PhysicalResidencyPool,
};
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::read_source::{frame_source_failure, frame_source_fault, FrameReadSource};
use super::{
    FrameLoadFailure, FrameLoadFailureKind, FrameLoadFaultCause, FrameLoadPort,
    LoadedPhysicalFrame, ObservedArtifactLength,
};
use crate::physical_runtime::record_serving::residency::frame_work_trace::FrameWorkTrace;

#[derive(Clone)]
pub(in crate::physical_runtime::record_serving) struct BoundedFrameLoader {
    pool: PhysicalResidencyPool,
}

impl BoundedFrameLoader {
    pub(in crate::physical_runtime::record_serving) const fn new(
        pool: PhysicalResidencyPool,
    ) -> Self {
        Self { pool }
    }
}

impl FrameLoadPort for BoundedFrameLoader {
    fn load_exact(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let coordinate = RecordFrameCoordinate::new(artifact, offset, length).ok_or(
            FrameLoadFailure::new(FrameLoadFailureKind::InvalidCoordinate),
        )?;
        let key = PhysicalFrameKey::new(self.pool.store_identity(), coordinate);
        let access = self
            .pool
            .access_frame(allocation, key)
            .map_err(|reason| FrameLoadFailure::new(FrameLoadFailureKind::Residency(reason)))?;
        let (lease, work, projection_failure) = match access {
            PhysicalFrameAccess::Hit(lease) => (lease, FrameWorkTrace::none(), None),
            PhysicalFrameAccess::Coalesced(waiter) => {
                let terminal = waiter.wait().map_err(|terminal| {
                    FrameLoadFailure::new(FrameLoadFailureKind::CoalescedFault(terminal))
                })?;
                (terminal, FrameWorkTrace::none(), None)
            }
            PhysicalFrameAccess::Fault(fault) => {
                let prepared = match source.prepare_exact(artifact, offset, length) {
                    Ok(prepared) => prepared,
                    Err(failure) => {
                        let terminal = fault.reject_before_source();
                        return Err(frame_source_fault(failure, terminal));
                    }
                };
                let work = FrameWorkTrace::one(prepared.identity());
                let mut projection_failure = None;
                let lease = fault
                    .load(|target| {
                        projection_failure = prepared.execute(target)?;
                        Ok(())
                    })
                    .map_err(|failure| match failure {
                        PhysicalFrameFaultError::Residency { terminal, denial } => {
                            FrameLoadFailure::new(FrameLoadFailureKind::FaultTerminated {
                                terminal,
                                cause: FrameLoadFaultCause::Residency(denial),
                            })
                            .with_complete_work_trace(work)
                        }
                        PhysicalFrameFaultError::Source { terminal, failure } => {
                            frame_source_fault(failure, terminal).with_complete_work_trace(work)
                        }
                    })?;
                (lease, work, projection_failure)
            }
        };
        LoadedPhysicalFrame::bind(
            self.pool.store_identity(),
            coordinate,
            lease,
            work,
            projection_failure,
        )
    }

    fn load_bounded(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<LoadedPhysicalFrame, FrameLoadFailure> {
        let limit = std::num::NonZeroU32::new(limit).ok_or(FrameLoadFailure::new(
            FrameLoadFailureKind::AccessLimitExceeded,
        ))?;
        let key = PhysicalBoundedFrameKey::new(self.pool.store_identity(), artifact, limit);
        let access = self
            .pool
            .access_bounded_frame(allocation, key)
            .map_err(|reason| FrameLoadFailure::new(FrameLoadFailureKind::Residency(reason)))?;
        let work = std::cell::Cell::new(FrameWorkTrace::none());
        let mut projection_failure = None;
        let lease = match access {
            PhysicalBoundedFrameAccess::Hit(lease) => lease,
            PhysicalBoundedFrameAccess::Coalesced(waiter) => waiter.wait().map_err(|terminal| {
                FrameLoadFailure::new(FrameLoadFailureKind::CoalescedFault(terminal))
            })?,
            PhysicalBoundedFrameAccess::Fault(owner) => owner
                .load(
                    |admitted_limit| {
                        let length = source.file_length(artifact).map_err(frame_source_failure)?;
                        if length.bytes() == 0
                            || length.bytes() > u64::from(admitted_limit)
                            || length.bytes() > u64::from(u32::MAX)
                        {
                            let observed = length.reject_structural_damage();
                            work.set(observed);
                            return Err(FrameLoadFailure::new(
                                FrameLoadFailureKind::AccessLimitExceeded,
                            )
                            .with_complete_work_trace(observed));
                        }
                        work.set(length.work_trace());
                        Ok(length.bytes() as u32)
                    },
                    |target: &mut [u8]| {
                        let prepared = source
                            .prepare_exact(artifact, 0, target.len() as u32)
                            .map_err(|failure| {
                                frame_source_failure(failure).preceded_by(work.get())
                            })?;
                        work.set(work.get().then(FrameWorkTrace::one(prepared.identity())));
                        projection_failure = prepared.execute(target).map_err(|failure| {
                            frame_source_failure(failure).with_complete_work_trace(work.get())
                        })?;
                        Ok(())
                    },
                )
                .map_err(|failure| bounded_fault_failure(failure, work.get()))?,
        };
        let coordinate = lease.key().coordinate();
        LoadedPhysicalFrame::bind(
            self.pool.store_identity(),
            coordinate,
            lease,
            work.get(),
            projection_failure,
        )
    }

    fn file_length(
        &self,
        source: &dyn FrameReadSource,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameLoadFailure> {
        source.file_length(artifact).map_err(frame_source_failure)
    }
}

fn bounded_fault_failure(
    failure: PhysicalFrameFaultError<FrameLoadFailure>,
    work: FrameWorkTrace,
) -> FrameLoadFailure {
    match failure {
        PhysicalFrameFaultError::Residency { terminal, denial } => {
            FrameLoadFailure::new(FrameLoadFailureKind::FaultTerminated {
                terminal,
                cause: FrameLoadFaultCause::Residency(denial),
            })
            .with_complete_work_trace(work)
        }
        PhysicalFrameFaultError::Source { terminal, failure } => match failure.kind() {
            FrameLoadFailureKind::Backend(cause) => {
                FrameLoadFailure::new(FrameLoadFailureKind::FaultTerminated {
                    terminal,
                    cause: FrameLoadFaultCause::Backend(cause),
                })
                .with_complete_work_trace(failure.work_trace())
            }
            FrameLoadFailureKind::Work(cause) => {
                FrameLoadFailure::new(FrameLoadFailureKind::FaultTerminated {
                    terminal,
                    cause: FrameLoadFaultCause::Work(cause),
                })
                .with_complete_work_trace(failure.work_trace())
            }
            FrameLoadFailureKind::Residency(cause) => {
                FrameLoadFailure::new(FrameLoadFailureKind::FaultTerminated {
                    terminal,
                    cause: FrameLoadFaultCause::Residency(cause),
                })
                .with_complete_work_trace(failure.work_trace())
            }
            _ => failure,
        },
    }
}
