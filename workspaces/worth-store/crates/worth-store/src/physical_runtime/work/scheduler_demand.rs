use sha2::{Digest, Sha256};
use worth_store_io_scheduler::foreground_reservation::{
    ForegroundIoLaneKind, ForegroundReservationReceipt, PhysicalInstanceForegroundCapacityLease,
    PhysicalInstanceForegroundReservation,
};
use worth_store_io_scheduler::{
    admit_queue_execution_plan, admit_queue_policy_receipt, lower_physical_foreground_work,
    IoSchedulerBackendCapabilityAdmission, PhysicalForegroundWorkDeclaration,
    QueueExecutionAdmissionDenial, QueueExecutionAdmissionRequest, QueueLocalityIdentity,
    QueueLocalityRange, QueueWorkDeclaration, SecureIoPreservationReceipt,
};
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::RecordArtifactFile;

use super::{
    PhysicalWorkDurabilityRequirement, PhysicalWorkOperationFamily, ReadyPhysicalWork,
    ResourceAdmittedPhysicalWork,
};

mod residency;

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
        ready
            .require_consumer_active()
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let intent = ready.intent();
        require_lane(intent.operation(), reservation.receipt().lane())?;
        ready
            .admit_scheduler_pressure(pressure_class(reservation.receipt().lane()))
            .map_err(PhysicalSchedulerDenial::PreEffect)?;
        let (receipt, capacity) = reservation.into_parts();
        let declaration = physical_foreground_declaration(
            intent,
            receipt,
            physical_locality(intent.identity().store(), intent.scope()),
        );
        let declaration = match secure_io {
            Some(secure_io) => declaration.with_secure_io_scope(secure_io),
            None => declaration,
        };
        let work =
            lower_physical_foreground_work(declaration).map_err(PhysicalSchedulerDenial::Queue)?;
        Ok(Self {
            ready,
            work,
            capacity,
        })
    }

    pub const fn intent(&self) -> &super::PhysicalWorkIntent {
        self.ready.intent()
    }

    pub const fn queue_work(&self) -> &QueueWorkDeclaration {
        &self.work
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
        let policy =
            admit_queue_policy_receipt(work, policy).map_err(PhysicalSchedulerDenial::Queue)?;
        let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(policy, backend))
            .map_err(PhysicalSchedulerDenial::Queue)?;
        Ok(ResourceAdmittedPhysicalWork::new(ready, plan, capacity))
    }
}

fn physical_foreground_declaration(
    intent: &super::PhysicalWorkIntent,
    reservation: ForegroundReservationReceipt,
    locality: QueueLocalityIdentity,
) -> PhysicalForegroundWorkDeclaration {
    let resources = intent.resources();
    match (intent.operation(), intent.durability()) {
        (
            PhysicalWorkOperationFamily::ArtifactMetadataRead
            | PhysicalWorkOperationFamily::ArtifactRangeRead,
            PhysicalWorkDurabilityRequirement::ReadOnly,
        ) => PhysicalForegroundWorkDeclaration::read(
            reservation,
            locality,
            resources.queue_shape(),
            resources.flush_epoch(),
        ),
        (
            PhysicalWorkOperationFamily::ArtifactRangeWrite,
            PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
            ),
        ) => PhysicalForegroundWorkDeclaration::buffered_write(
            reservation,
            locality,
            resources.queue_shape(),
            resources.flush_epoch(),
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
        ) => PhysicalForegroundWorkDeclaration::durable_write(
            reservation,
            locality,
            resources.queue_shape(),
            resources.flush_epoch(),
        ),
        _ => unreachable!("physical declaration admission already proved operation durability"),
    }
}
