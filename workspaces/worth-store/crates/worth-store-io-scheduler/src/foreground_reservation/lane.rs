use crate::IoSchedulerBackendCapabilityRequirement;

use super::{
    ForegroundLatencyEnvelope, ForegroundReservationAdmissionDenial, ForegroundResourceBudget,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForegroundIoLaneKind {
    PointRead,
    RangeRead,
    CommitCriticalWalAppend,
    CommitCriticalWalWrite,
    RootPublication,
    OrdinaryPageWrite,
    InteractiveRead,
    InternalForegroundRead,
    ArtifactMetadataRead,
}

impl ForegroundIoLaneKind {
    pub const fn default_backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        match self {
            Self::PointRead
            | Self::RangeRead
            | Self::InteractiveRead
            | Self::InternalForegroundRead => IoSchedulerBackendCapabilityRequirement::DirectIo,
            Self::ArtifactMetadataRead => IoSchedulerBackendCapabilityRequirement::BufferedFile,
            Self::CommitCriticalWalAppend => IoSchedulerBackendCapabilityRequirement::BufferedFile,
            Self::CommitCriticalWalWrite => IoSchedulerBackendCapabilityRequirement::Fsync,
            Self::RootPublication => {
                IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync
            }
            Self::OrdinaryPageWrite => IoSchedulerBackendCapabilityRequirement::BufferedFile,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundLaneDeclaration {
    lane: ForegroundIoLaneKind,
    backend_requirement: IoSchedulerBackendCapabilityRequirement,
    envelope: Option<ForegroundLatencyEnvelope>,
    requested_budget: ForegroundResourceBudget,
}

impl ForegroundLaneDeclaration {
    pub const fn point_read() -> Self {
        Self::new(ForegroundIoLaneKind::PointRead)
    }

    pub const fn range_read() -> Self {
        Self::new(ForegroundIoLaneKind::RangeRead)
    }

    pub const fn commit_critical_wal_append() -> Self {
        Self::new(ForegroundIoLaneKind::CommitCriticalWalAppend)
    }

    pub const fn commit_critical_wal_write() -> Self {
        Self::new(ForegroundIoLaneKind::CommitCriticalWalWrite)
    }

    pub const fn filesystem_admitted_wal_barrier(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        let lane = Self::commit_critical_wal_write().with_store_owned_backend_requirement(
            IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync,
        );
        if lane.backend_requirement_is_store_owned() {
            Ok(lane)
        } else {
            Err(
                ForegroundReservationAdmissionDenial::LaneBackendRequirementNotStoreOwned {
                    lane: lane.lane,
                    backend_requirement: lane.backend_requirement,
                },
            )
        }
    }

    pub const fn root_candidate_synchronization(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        Self::root_publication_with(
            IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync,
        )
    }

    pub const fn root_candidate_materialization(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        Self::root_publication_with(IoSchedulerBackendCapabilityRequirement::BufferedFile)
    }

    pub const fn root_catalog_replacement() -> Result<Self, ForegroundReservationAdmissionDenial> {
        Self::root_publication_with(
            IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedDurableRename,
        )
    }

    pub const fn root_namespace_synchronization(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        Self::root_publication_with(
            IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedDirectorySync,
        )
    }

    pub const fn ordinary_page_write() -> Self {
        Self::new(ForegroundIoLaneKind::OrdinaryPageWrite)
    }

    pub const fn interactive_read() -> Self {
        Self::new(ForegroundIoLaneKind::InteractiveRead)
    }

    pub const fn internal_foreground_read() -> Self {
        Self::new(ForegroundIoLaneKind::InternalForegroundRead)
    }

    pub const fn artifact_metadata_read() -> Self {
        Self::new(ForegroundIoLaneKind::ArtifactMetadataRead)
    }

    pub const fn secure_frame_internal_foreground_read(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        let lane = Self::internal_foreground_read().with_store_owned_backend_requirement(
            IoSchedulerBackendCapabilityRequirement::SecureFrameIo,
        );
        if lane.backend_requirement_is_store_owned() {
            Ok(lane)
        } else {
            Err(
                ForegroundReservationAdmissionDenial::LaneBackendRequirementNotStoreOwned {
                    lane: lane.lane,
                    backend_requirement: lane.backend_requirement,
                },
            )
        }
    }

    pub const fn buffered_file_internal_foreground_read(
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        let lane = Self::internal_foreground_read().with_store_owned_backend_requirement(
            IoSchedulerBackendCapabilityRequirement::BufferedFile,
        );
        if lane.backend_requirement_is_store_owned() {
            Ok(lane)
        } else {
            Err(
                ForegroundReservationAdmissionDenial::LaneBackendRequirementNotStoreOwned {
                    lane: lane.lane,
                    backend_requirement: lane.backend_requirement,
                },
            )
        }
    }

    pub const fn with_latency_envelope(mut self, envelope: ForegroundLatencyEnvelope) -> Self {
        self.envelope = Some(envelope);
        self
    }

    pub const fn with_budget(mut self, requested_budget: ForegroundResourceBudget) -> Self {
        self.requested_budget = requested_budget;
        self
    }

    pub const fn lane(self) -> ForegroundIoLaneKind {
        self.lane
    }

    pub const fn backend_requirement(self) -> IoSchedulerBackendCapabilityRequirement {
        self.backend_requirement
    }

    pub const fn backend_requirement_is_store_owned(self) -> bool {
        matches!(
            (self.lane, self.backend_requirement),
            (
                ForegroundIoLaneKind::PointRead,
                IoSchedulerBackendCapabilityRequirement::DirectIo,
            ) | (
                ForegroundIoLaneKind::RangeRead,
                IoSchedulerBackendCapabilityRequirement::DirectIo,
            ) | (
                ForegroundIoLaneKind::CommitCriticalWalAppend,
                IoSchedulerBackendCapabilityRequirement::BufferedFile,
            ) | (
                ForegroundIoLaneKind::CommitCriticalWalWrite,
                IoSchedulerBackendCapabilityRequirement::Fsync
                    | IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync,
            ) | (
                ForegroundIoLaneKind::OrdinaryPageWrite,
                IoSchedulerBackendCapabilityRequirement::BufferedFile,
            ) | (
                ForegroundIoLaneKind::RootPublication,
                IoSchedulerBackendCapabilityRequirement::BufferedFile
                    | IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedFsync
                    | IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedDurableRename
                    | IoSchedulerBackendCapabilityRequirement::FilesystemAdmittedDirectorySync,
            ) | (
                ForegroundIoLaneKind::InteractiveRead,
                IoSchedulerBackendCapabilityRequirement::DirectIo,
            ) | (
                ForegroundIoLaneKind::InternalForegroundRead,
                IoSchedulerBackendCapabilityRequirement::DirectIo,
            ) | (
                ForegroundIoLaneKind::InternalForegroundRead,
                IoSchedulerBackendCapabilityRequirement::SecureFrameIo,
            ) | (
                ForegroundIoLaneKind::InternalForegroundRead,
                IoSchedulerBackendCapabilityRequirement::BufferedFile,
            ) | (
                ForegroundIoLaneKind::ArtifactMetadataRead,
                IoSchedulerBackendCapabilityRequirement::BufferedFile,
            )
        )
    }

    pub const fn envelope(self) -> Option<ForegroundLatencyEnvelope> {
        self.envelope
    }

    pub const fn requested_budget(self) -> ForegroundResourceBudget {
        self.requested_budget
    }

    const fn new(lane: ForegroundIoLaneKind) -> Self {
        Self {
            lane,
            backend_requirement: lane.default_backend_requirement(),
            envelope: None,
            requested_budget: ForegroundResourceBudget::new(),
        }
    }

    const fn with_store_owned_backend_requirement(
        mut self,
        backend_requirement: IoSchedulerBackendCapabilityRequirement,
    ) -> Self {
        self.backend_requirement = backend_requirement;
        self
    }

    const fn root_publication_with(
        backend_requirement: IoSchedulerBackendCapabilityRequirement,
    ) -> Result<Self, ForegroundReservationAdmissionDenial> {
        let lane = Self::new(ForegroundIoLaneKind::RootPublication)
            .with_store_owned_backend_requirement(backend_requirement);
        if lane.backend_requirement_is_store_owned() {
            Ok(lane)
        } else {
            Err(
                ForegroundReservationAdmissionDenial::LaneBackendRequirementNotStoreOwned {
                    lane: lane.lane,
                    backend_requirement: lane.backend_requirement,
                },
            )
        }
    }
}
