use sha2::{Digest, Sha256};
use worth_store_io_scheduler::foreground_reservation::{
    ForegroundIoLaneKind, ForegroundReservationReceipt,
};
use worth_store_io_scheduler::{
    admit_queue_execution_plan, admit_queue_policy_receipt, lower_physical_foreground_work,
    IoSchedulerBackendCapabilityAdmission, QueueDurabilityClass, QueueExecutionAdmissionDenial,
    QueueExecutionAdmissionRequest, QueueLocalityIdentity,
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
}

pub struct PhysicalSchedulerDemand {
    ready: ReadyPhysicalWork,
    work: QueueWorkDeclaration,
}

pub struct PhysicalWorkScheduler;

impl PhysicalSchedulerDemand {
    pub fn foreground(
        ready: ReadyPhysicalWork,
        reservation: ForegroundReservationReceipt,
        secure_io: Option<SecureIoPreservationReceipt>,
    ) -> Result<Self, PhysicalSchedulerDenial> {
        let intent = ready.intent();
        require_lane(intent.operation(), reservation.lane())?;
        let resources = intent.resources();
        let (durability, recovery, writeback) = scheduler_posture(intent);
        let work = lower_physical_foreground_work(
            reservation,
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
        Ok(Self { ready, work })
    }

    pub const fn intent(&self) -> &super::PhysicalWorkIntent {
        self.ready.intent()
    }

    pub const fn queue_work(&self) -> QueueWorkDeclaration {
        self.work
    }
}

fn physical_locality(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    scope: &super::PhysicalWorkScope,
) -> QueueLocalityIdentity {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.physical-queue-locality.v1");
    digest.update(store.bytes());
    digest.update((scope.coordinates().len() as u64).to_le_bytes());
    for coordinate in scope.coordinates() {
        update_artifact(&mut digest, coordinate.artifact());
        digest.update(coordinate.offset().to_le_bytes());
        digest.update(coordinate.length().to_le_bytes());
    }
    let identity = digest.finalize().into();
    let coordinates = scope.coordinates();
    let first_artifact = coordinates[0].artifact();
    if coordinates
        .iter()
        .all(|coordinate| coordinate.artifact() == first_artifact)
    {
        let mut artifact = Sha256::new();
        artifact.update(b"worth-store.physical-queue-artifact.v1");
        artifact.update(store.bytes());
        update_artifact(&mut artifact, first_artifact);
        let start = coordinates
            .iter()
            .map(|coordinate| coordinate.offset())
            .min()
            .expect("physical scope is nonempty");
        let end = coordinates
            .iter()
            .map(|coordinate| {
                coordinate
                    .offset()
                    .saturating_add(u64::from(coordinate.length()))
            })
            .max()
            .expect("physical scope is nonempty");
        let covered_bytes = coordinates.iter().fold(0_u64, |total, coordinate| {
            total.saturating_add(u64::from(coordinate.length()))
        });
        return QueueLocalityIdentity::from_single_artifact_scope(
            identity,
            artifact.finalize().into(),
            start,
            end,
            covered_bytes,
            u16::try_from(coordinates.len()).expect("physical scope capacity fits u16"),
        );
    }
    QueueLocalityIdentity::from_digest(identity)
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
        let policy = admit_queue_policy_receipt(demand.work, policy)
            .map_err(PhysicalSchedulerDenial::Queue)?;
        let plan = admit_queue_execution_plan(QueueExecutionAdmissionRequest::new(
            demand.work,
            backend,
            policy,
        ))
        .map_err(PhysicalSchedulerDenial::Queue)?;
        Ok(ResourceAdmittedPhysicalWork::new(demand.ready, plan))
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
            PhysicalWorkOperationFamily::ArtifactRangeRead,
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
