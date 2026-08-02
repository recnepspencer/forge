use worth_store_contracts::QueueProducerResourceShape;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::{PhysicalWorkDurabilityRequirement, PhysicalWorkOperationFamily, PhysicalWorkScope};

/// Store-owned demand retained before lowering into scheduler units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkResourceDemand {
    queue: QueueProducerResourceShape,
    flush_epoch: u64,
}

impl PhysicalWorkResourceDemand {
    pub(super) fn derive(
        scope: &PhysicalWorkScope,
        operation: PhysicalWorkOperationFamily,
        durability: PhysicalWorkDurabilityRequirement,
    ) -> Self {
        let members = scope.member_count() as u64;
        let bytes = scope.wal_reclamation_target().map_or_else(
            || {
                scope.checkpoint_target().map_or_else(
                    || {
                        scope.wal_append_target().map_or_else(
                            || {
                                scope.wal_barrier_target().map_or_else(
                                    || {
                                        scope.root_publication_target().map_or_else(
                                            || {
                                                scope.coordinates().iter().fold(
                                                    0_u64,
                                                    |total, coordinate| {
                                                        total.saturating_add(u64::from(
                                                            coordinate.length(),
                                                        ))
                                                    },
                                                )
                                            },
                                            |root| root.accounted_bytes(),
                                        )
                                    },
                                    |_| 1,
                                )
                            },
                            |wal| wal.byte_count(),
                        )
                    },
                    |checkpoint| checkpoint.accounted_bytes(),
                )
            },
            |reclamation| reclamation.byte_count(),
        );
        let queue = QueueProducerResourceShape::new()
            .with_queue_slots(members)
            .with_worker_permits(members)
            .with_bandwidth_tokens(bytes)
            .with_write_back_windows(
                if matches!(operation, PhysicalWorkOperationFamily::ArtifactRangeWrite) {
                    members
                } else {
                    0
                },
            )
            .with_flush_permits(u64::from(matches!(
                durability,
                PhysicalWorkDurabilityRequirement::ArtifactRangeWrite(
                    ArtifactRangeWriteDurabilityRequirement::FileDataSynchronization
                ) | PhysicalWorkDurabilityRequirement::WalDurabilityBarrier
                    | PhysicalWorkDurabilityRequirement::CheckpointCapture
                    | PhysicalWorkDurabilityRequirement::WalReclamation
                    | PhysicalWorkDurabilityRequirement::RootPublication
            )))
            .with_sync_debt(u64::from(matches!(
                operation,
                PhysicalWorkOperationFamily::ArtifactPublication
                    | PhysicalWorkOperationFamily::CheckpointCapture
                    | PhysicalWorkOperationFamily::DurabilityBarrier
                    | PhysicalWorkOperationFamily::WalReclamation
                    | PhysicalWorkOperationFamily::RootPublication
            )));
        Self {
            queue,
            flush_epoch: 0,
        }
    }

    pub const fn queue_shape(self) -> QueueProducerResourceShape {
        self.queue
    }

    pub const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }
}
