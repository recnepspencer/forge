use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use crate::physical_runtime::{
    instance::PhysicalExecutionCall,
    record_serving::{
        residency::{
            frame_load_failure::{FrameLoadFailure, FrameLoadFailureKind},
            frame_loading::{FrameReadWorkAdmission, LoadedPhysicalFrame},
        },
        CanonicalRecordReadFailure,
    },
    PhysicalWorkIdentity, PhysicalWorkPreEffectDenial,
};

use super::super::C6PhysicalWorkHandoffIdentity;
use super::C6PhysicalResidencyWork;

/// One pinned physical frame acquired through the canonical C.5.1 read path.
///
/// Dropping this value releases the framework-owned pin lease.
pub struct C6PhysicalFrameLease {
    handoff: C6PhysicalWorkHandoffIdentity,
    coordinate: RecordFrameCoordinate,
    frame: LoadedPhysicalFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum C6PhysicalFrameReadFailure {
    PhysicalWork(C6PhysicalFrameWorkFailure),
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    AccessLimitExceeded,
    InvalidCoordinate,
    ReturnedFrameIdentityMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum C6PhysicalFrameWorkFailure {
    RuntimeReleased,
    InvalidCoordinate,
    SubmissionRejected,
    PreEffect(PhysicalWorkPreEffectDenial),
    DependencyBlocked,
    SchedulerReservationRejected,
    SchedulerRejected,
    CommandRejected,
    Backend(worth_store_physical_backend::ArtifactTreeFailure),
    SchedulerSettlementRejected,
    SettlementMismatch,
}

impl C6PhysicalResidencyWork {
    pub fn pin_exact(
        &self,
        coordinate: RecordFrameCoordinate,
    ) -> Result<C6PhysicalFrameLease, C6PhysicalFrameReadFailure> {
        let _call = self.admit_frame_call()?;
        let frame = self
            .frame_ports
            .loader()
            .load_exact(
                &self.frame_source,
                coordinate.artifact(),
                coordinate.offset(),
                coordinate.length(),
                FrameReadWorkAdmission::ResidencyFaultOnly,
            )
            .map_err(C6PhysicalFrameReadFailure::from)?;
        Ok(C6PhysicalFrameLease {
            handoff: self.identity,
            coordinate,
            frame,
        })
    }

    pub fn pin_bounded(
        &self,
        artifact: RecordArtifactFile,
        limit: u32,
    ) -> Result<C6PhysicalFrameLease, C6PhysicalFrameReadFailure> {
        let _call = self.admit_frame_call()?;
        let frame = self
            .frame_ports
            .loader()
            .load_bounded(
                &self.frame_source,
                artifact,
                limit,
                FrameReadWorkAdmission::ResidencyFaultOnly,
            )
            .map_err(C6PhysicalFrameReadFailure::from)?;
        let coordinate = RecordFrameCoordinate::new(artifact, 0, frame.len() as u32)
            .ok_or(C6PhysicalFrameReadFailure::InvalidCoordinate)?;
        Ok(C6PhysicalFrameLease {
            handoff: self.identity,
            coordinate,
            frame,
        })
    }

    fn admit_frame_call(&self) -> Result<PhysicalExecutionCall, C6PhysicalFrameReadFailure> {
        self.execution.admit_call().map_err(|failure| {
            C6PhysicalFrameReadFailure::PhysicalWork(C6PhysicalFrameWorkFailure::PreEffect(failure))
        })
    }
}

impl C6PhysicalFrameLease {
    pub const fn coordinate(&self) -> RecordFrameCoordinate {
        self.coordinate
    }

    pub fn bytes(&self) -> &[u8] {
        &self.frame
    }

    pub fn len(&self) -> usize {
        self.frame.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frame.is_empty()
    }

    pub fn copy_range_into(&self, range: std::ops::Range<usize>, target: &mut [u8]) {
        self.frame.copy_range_into(range, target);
    }

    pub const fn physical_work_count(&self) -> u64 {
        self.frame.work_trace().count()
    }

    pub const fn first_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.frame.work_trace().first()
    }

    pub const fn last_physical_work(&self) -> Option<PhysicalWorkIdentity> {
        self.frame.work_trace().last()
    }

    pub(super) const fn handoff_identity(&self) -> C6PhysicalWorkHandoffIdentity {
        self.handoff
    }

    pub(super) fn into_dirty_candidate(
        self,
        bytes: Vec<u8>,
    ) -> Result<
        (
            worth_store_buffer_pool::DirtyPhysicalFrame,
            super::super::super::residency::frame_work_trace::FrameWorkTrace,
        ),
        worth_store_buffer_pool::PhysicalResidencyDenial,
    > {
        self.frame.replace_with_dirty_candidate(bytes)
    }
}

impl From<FrameLoadFailure> for C6PhysicalFrameReadFailure {
    fn from(failure: FrameLoadFailure) -> Self {
        match failure.kind() {
            FrameLoadFailureKind::Backend(failure) => Self::Backend(failure),
            FrameLoadFailureKind::Work(failure) => Self::PhysicalWork(failure.into()),
            FrameLoadFailureKind::Residency(failure) => Self::Residency(failure),
            FrameLoadFailureKind::AccessLimitExceeded => Self::AccessLimitExceeded,
            FrameLoadFailureKind::InvalidCoordinate => Self::InvalidCoordinate,
            FrameLoadFailureKind::ReturnedFrameIdentityMismatch => {
                Self::ReturnedFrameIdentityMismatch
            }
        }
    }
}

impl From<CanonicalRecordReadFailure> for C6PhysicalFrameWorkFailure {
    fn from(failure: CanonicalRecordReadFailure) -> Self {
        match failure {
            CanonicalRecordReadFailure::RuntimeReleased => Self::RuntimeReleased,
            CanonicalRecordReadFailure::InvalidCoordinate => Self::InvalidCoordinate,
            CanonicalRecordReadFailure::SubmissionRejected => Self::SubmissionRejected,
            CanonicalRecordReadFailure::PreEffect(failure) => Self::PreEffect(failure),
            CanonicalRecordReadFailure::DependencyBlocked => Self::DependencyBlocked,
            CanonicalRecordReadFailure::SchedulerReservation(_) => {
                Self::SchedulerReservationRejected
            }
            CanonicalRecordReadFailure::Scheduler(_) => Self::SchedulerRejected,
            CanonicalRecordReadFailure::Command(_) => Self::CommandRejected,
            CanonicalRecordReadFailure::Backend(failure) => Self::Backend(failure),
            CanonicalRecordReadFailure::SchedulerSettlementRejected => {
                Self::SchedulerSettlementRejected
            }
            CanonicalRecordReadFailure::SettlementMismatch => Self::SettlementMismatch,
        }
    }
}
