use sha2::{Digest, Sha256};
use worth_store_io_scheduler::foreground_reservation::{
    ForegroundIoLaneKind, PhysicalInstanceForegroundCapacityLease,
    PhysicalInstanceForegroundReservation,
};
use worth_store_io_scheduler::{
    admit_queue_execution_plan, admit_queue_policy_receipt, lower_physical_foreground_work,
    IoSchedulerBackendCapabilityAdmission, QueueDurabilityClass, QueueExecutionAdmissionDenial,
    QueueExecutionAdmissionRequest, QueueLocalityIdentity, QueueLocalityRange,
    QueueRecoveryOrdering, QueueWorkDeclaration, QueueWritebackPolicy, SecureIoPreservationReceipt,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::RecordArtifactFile;

use super::{
    PhysicalWorkDurabilityRequirement, PhysicalWorkOperationFamily, ReadyPhysicalWork,
    ResourceAdmittedPhysicalWork,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSchedulerDenial {
    PreEffect(super::PhysicalWorkPreEffectDenial),
    ForegroundLaneMismatch {
        operation: PhysicalWorkOperationFamily,
        lane: ForegroundIoLaneKind,
    },
    Queue(QueueExecutionAdmissionDenial),
    ResidencyWorkMismatch,
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
}

pub struct PhysicalSchedulerDemand {
    ready: ReadyPhysicalWork,
    work: QueueWorkDeclaration,
    capacity: PhysicalInstanceForegroundCapacityLease,
}

pub struct PhysicalWorkScheduler;

impl PhysicalSchedulerDemand {
    pub fn foreground(
        ready: ReadyPhysicalWork,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: Option<SecureIoPreservationReceipt>,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        let intent = ready.intent();
        require_lane(intent.operation(), reservation.receipt().lane())?;
        let pressure_marked = ready.mark_pressure(pressure_class(reservation.receipt().lane()));
        debug_assert!(pressure_marked);
        let (receipt, capacity) = reservation.into_parts();
        let resources = intent.resources();
        let (durability, recovery, writeback) = scheduler_posture(intent);
        let work = lower_physical_foreground_work(
            receipt,
            intent.security(),
            physical_locality(intent.identity().store(), intent.scope()),
            resources.queue_shape(),
            durability,
            resources.flush_epoch(),
            recovery,
            writeback,
            secure_io,
        )
        .map_err(PhysicalSchedulerDenial::Queue)?;
        Ok(Self {
            ready,
            work,
            capacity,
        })
    }

    pub fn residency_writeback(
        ready: ReadyPhysicalWork,
        declaration: worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration,
        reservation: PhysicalInstanceForegroundReservation,
        secure_io: Option<SecureIoPreservationReceipt>,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        let intent = ready.intent();
        let [coordinate] = intent.scope().coordinates() else {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        };
        if intent.operation() != PhysicalWorkOperationFamily::ArtifactRangeWrite
            || declaration.store() != intent.identity().store()
            || declaration.frame() != *coordinate
            || declaration.grouping_scope().security_scope_identity() != intent.security()
        {
            return Err(PhysicalSchedulerDenial::ResidencyWorkMismatch);
        }
        require_lane(intent.operation(), reservation.receipt().lane())?;
        let pressure_marked = ready.mark_pressure(pressure_class(reservation.receipt().lane()));
        debug_assert!(pressure_marked);
        let (receipt, capacity) = reservation.into_parts();
        let mut work =
            worth_store_io_scheduler::lower_buffer_pool_queue_declaration(declaration, receipt)
                .map_err(PhysicalSchedulerDenial::Queue)?;
        if let Some(secure_io) = secure_io {
            work = work.with_secure_io_scope(secure_io);
        }
        Ok(Self {
            ready,
            work,
            capacity,
        })
    }

    pub const fn intent(&self) -> &super::PhysicalWorkIntent {
        self.ready.intent()
    }

    pub fn queue_work(&self) -> QueueWorkDeclaration {
        self.work.clone()
    }

    pub fn with_secure_io(mut self, secure_io: SecureIoPreservationReceipt) -> Self {
        self.work = self.work.with_secure_io_scope(secure_io);
        self
    }
}

const fn pressure_class(lane: ForegroundIoLaneKind) -> super::PhysicalWorkPressureClass {
    match lane {
        ForegroundIoLaneKind::PointRead => super::PhysicalWorkPressureClass::ForegroundPointRead,
        ForegroundIoLaneKind::RangeRead => super::PhysicalWorkPressureClass::ForegroundRangeRead,
        ForegroundIoLaneKind::InteractiveRead => {
            super::PhysicalWorkPressureClass::ForegroundInteractiveRead
        }
        ForegroundIoLaneKind::InternalForegroundRead => {
            super::PhysicalWorkPressureClass::ForegroundInternalRead
        }
        ForegroundIoLaneKind::ArtifactMetadataRead => {
            super::PhysicalWorkPressureClass::ForegroundInternalRead
        }
        ForegroundIoLaneKind::OrdinaryPageWrite => {
            super::PhysicalWorkPressureClass::ForegroundMutation
        }
        ForegroundIoLaneKind::CommitCriticalWalWrite => {
            super::PhysicalWorkPressureClass::ForegroundMutation
        }
    }
}

fn physical_locality(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    scope: &super::PhysicalWorkScope,
) -> QueueLocalityIdentity {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.physical-queue-locality.v1");
    digest.update(store.bytes());
    digest.update((scope.member_count() as u64).to_le_bytes());
    if let Some(artifact) = scope.artifact_target() {
        update_artifact(&mut digest, artifact);
        let identity = digest.finalize().into();
        let mut artifact_identity = Sha256::new();
        artifact_identity.update(b"worth-store.physical-queue-artifact.v1");
        artifact_identity.update(store.bytes());
        update_artifact(&mut artifact_identity, artifact);
        let range = QueueLocalityRange::new(artifact_identity.finalize().into(), 0, u64::MAX)
            .expect("artifact-wide locality is nonempty");
        return QueueLocalityIdentity::from_ranges(identity, [range])
            .expect("one artifact-wide locality range is valid");
    }
    for coordinate in scope.coordinates() {
        update_artifact(&mut digest, coordinate.artifact());
        digest.update(coordinate.offset().to_le_bytes());
        digest.update(coordinate.length().to_le_bytes());
    }
    let identity = digest.finalize().into();
    let ranges = scope.coordinates().iter().map(|coordinate| {
        let mut artifact = Sha256::new();
        artifact.update(b"worth-store.physical-queue-artifact.v1");
        artifact.update(store.bytes());
        update_artifact(&mut artifact, coordinate.artifact());
        QueueLocalityRange::new(
            artifact.finalize().into(),
            coordinate.offset(),
            coordinate
                .offset()
                .saturating_add(u64::from(coordinate.length())),
        )
        .expect("admitted physical scope ranges are nonempty")
    });
    QueueLocalityIdentity::from_ranges(identity, ranges)
        .expect("admitted physical scope ranges are sorted and disjoint")
}

fn update_artifact(digest: &mut Sha256, artifact: RecordArtifactFile) {
    let (tag, first, second): (u8, u64, u64) = match artifact {
        RecordArtifactFile::BootstrapCatalog => (1, 0, 0),
        RecordArtifactFile::CatalogCandidate { publication } => (2, publication, 0),
        RecordArtifactFile::RootManifest { generation } => (3, generation, 0),
        RecordArtifactFile::RootRoutingBlock { generation, block } => (4, generation, block),
        RecordArtifactFile::Segment {
            segment,
            generation,
        } => (5, segment, generation),
        RecordArtifactFile::SegmentManifest {
            segment,
            generation,
        } => (6, segment, generation),
        RecordArtifactFile::SegmentMembershipBlock { generation, block } => (7, generation, block),
        RecordArtifactFile::Extent { extent, generation } => (8, extent, generation),
        RecordArtifactFile::ExtentManifest { extent, generation } => (9, extent, generation),
        RecordArtifactFile::FreeSpaceManifest { generation } => (10, generation, 0),
        RecordArtifactFile::FreeSpaceMembershipBlock { generation, block } => {
            (11, generation, block)
        }
    };
    digest.update([tag]);
    digest.update(first.to_le_bytes());
    digest.update(second.to_le_bytes());
}

fn require_lane(
    operation: PhysicalWorkOperationFamily,
    lane: ForegroundIoLaneKind,
) -> Result<(), PhysicalSchedulerDenial> {
    let compatible = match operation {
        PhysicalWorkOperationFamily::ArtifactMetadataRead => {
            lane == ForegroundIoLaneKind::ArtifactMetadataRead
        }
        PhysicalWorkOperationFamily::ArtifactRangeRead => matches!(
            lane,
            ForegroundIoLaneKind::PointRead
                | ForegroundIoLaneKind::RangeRead
                | ForegroundIoLaneKind::InteractiveRead
                | ForegroundIoLaneKind::InternalForegroundRead
        ),
        PhysicalWorkOperationFamily::ArtifactRangeWrite
        | PhysicalWorkOperationFamily::ArtifactPublication => {
            lane == ForegroundIoLaneKind::OrdinaryPageWrite
        }
    };
    compatible
        .then_some(())
        .ok_or(PhysicalSchedulerDenial::ForegroundLaneMismatch { operation, lane })
}

impl PhysicalWorkScheduler {
    pub fn admit(
        demand: PhysicalSchedulerDemand,
        backend: &IoSchedulerBackendCapabilityAdmission,
        policy: worth_foundational::FoundationalPolicyAdmissionReceipt,
    ) -> Result<ResourceAdmittedPhysicalWork, PhysicalSchedulerDenial> {
        let PhysicalSchedulerDemand {
            ready,
            work,
            capacity,
        } = demand;
        let policy = admit_queue_policy_receipt(work.clone(), policy)
            .map_err(PhysicalSchedulerDenial::Queue)?;
        let plan =
            admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(work, backend, policy))
                .map_err(PhysicalSchedulerDenial::Queue)?;
        Ok(ResourceAdmittedPhysicalWork::new(ready, plan, capacity))
    }
}

fn scheduler_posture(
    intent: &super::PhysicalWorkIntent,
) -> (
    QueueDurabilityClass,
    QueueRecoveryOrdering,
    QueueWritebackPolicy,
) {
    match (intent.operation(), intent.durability()) {
        (
            PhysicalWorkOperationFamily::ArtifactMetadataRead
            | PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkDurabilityRequirement::ReadOnly,
        ) => (
            QueueDurabilityClass::ReadOnly,
            QueueRecoveryOrdering::NotRecoveryCritical,
            QueueWritebackPolicy::None,
        ),
        (
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
            ),
        ) => (
            QueueDurabilityClass::BufferedWrite,
            QueueRecoveryOrdering::NotRecoveryCritical,
            QueueWritebackPolicy::DeferredWithinFlushEpoch,
        ),
        (
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization,
            ),
        )
        | (
            PhysicalWorkOperationFamily::ArtifactPublication,
            PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(_),
        ) => (
            QueueDurabilityClass::PlatformDurable,
            QueueRecoveryOrdering::NotRecoveryCritical,
            QueueWritebackPolicy::Immediate,
        ),
        _ => unreachable!("physical declaration admission already proved operation durability"),
    }
}
