use worth_foundational::{BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator};
use worth_store_physical_backend::ProductionStorageBoundarySeam;

use super::{
    ExpectedFaultLocalization, FaultDeliveryDenial, PhysicalArtifactFaultLocus, PhysicalFaultEvent,
    PhysicalFaultFieldKind, PhysicalFaultOffset,
};
use crate::PhysicalScenarioFaultKind;

pub fn s5_stable_read_plan_fault_event(
    fault: PhysicalScenarioFaultKind,
) -> Result<Option<PhysicalFaultEvent>, FaultDeliveryDenial> {
    match fault {
        PhysicalScenarioFaultKind::NoFault => Ok(None),
        PhysicalScenarioFaultKind::StaleGeneration => PhysicalFaultEvent::stale_generation(
            ProductionStorageBoundarySeam::PagePin,
            page_generation_locus(),
        )
        .map(Some),
        PhysicalScenarioFaultKind::MissingReadPlanRelease => PhysicalFaultEvent::delayed_release(
            ProductionStorageBoundarySeam::LeasePublish,
            read_plan_release_locus(),
        )
        .map(Some),
        PhysicalScenarioFaultKind::ExecutionTimeReferenceDiscovery => {
            PhysicalFaultEvent::execution_time_reference_discovery(
                ProductionStorageBoundarySeam::PagePin,
                execution_discovery_locus(),
            )
            .map(Some)
        }
        PhysicalScenarioFaultKind::UnboundedReadPlanFootprint => {
            PhysicalFaultEvent::unbounded_read_plan_footprint(
                ProductionStorageBoundarySeam::ReclaimEligibility,
                read_plan_footprint_locus(),
            )
            .map(Some)
        }
        PhysicalScenarioFaultKind::EarlyReclaim => PhysicalFaultEvent::blocked_reclaim(
            ProductionStorageBoundarySeam::ReclaimEligibility,
            reclaim_barrier_locus(),
        )
        .map(Some),
        PhysicalScenarioFaultKind::StaleEpochReuse => PhysicalFaultEvent::stale_generation(
            ProductionStorageBoundarySeam::PagePin,
            stale_epoch_reuse_locus(),
        )
        .map(Some),
        PhysicalScenarioFaultKind::InPlaceCompactionOverwrite => {
            PhysicalFaultEvent::byte_corruption(
                ProductionStorageBoundarySeam::CompactionCutover,
                compaction_product_locus(),
            )
            .map(Some)
        }
        PhysicalScenarioFaultKind::MixedRootRead => PhysicalFaultEvent::reordered_persistence(
            ProductionStorageBoundarySeam::RootSwap,
            mixed_root_locus(),
        )
        .map(Some),
        PhysicalScenarioFaultKind::S6BackendLatencyInjection
        | PhysicalScenarioFaultKind::S6QueueDepthSaturation
        | PhysicalScenarioFaultKind::S6BandwidthThrottle
        | PhysicalScenarioFaultKind::S6DelayedSync
        | PhysicalScenarioFaultKind::S6PageCachePressure
        | PhysicalScenarioFaultKind::S6BackgroundPacingLateYield
        | PhysicalScenarioFaultKind::BlobCrashAfterChunkWrite
        | PhysicalScenarioFaultKind::BlobCrashAfterSessionCheckpoint
        | PhysicalScenarioFaultKind::BlobCrashAfterRootPublication
        | PhysicalScenarioFaultKind::BlobTierMoveInterruption
        | PhysicalScenarioFaultKind::BlobExportInterruption
        | PhysicalScenarioFaultKind::BlobReclaimInterruption => Ok(None),
        PhysicalScenarioFaultKind::FutureExtensionSlot => Ok(None),
    }
}

fn page_generation_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::page_image(
        artifact(5201, BoundaryArtifactField::Payload),
        PhysicalFaultFieldKind::GenerationField,
        PhysicalFaultOffset::at(8),
        ExpectedFaultLocalization::PhysicalIntegrityBoundary,
    )
}

fn read_plan_release_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::root_pointer(
        artifact(5202, BoundaryArtifactField::Basis),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn execution_discovery_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::page_image(
        artifact(5203, BoundaryArtifactField::Payload),
        PhysicalFaultFieldKind::SlotState,
        PhysicalFaultOffset::at(16),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn read_plan_footprint_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::root_pointer(
        artifact(5204, BoundaryArtifactField::Basis),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn reclaim_barrier_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::root_pointer(
        artifact(5211, BoundaryArtifactField::Basis),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn stale_epoch_reuse_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::page_image(
        artifact(5212, BoundaryArtifactField::Payload),
        PhysicalFaultFieldKind::GenerationField,
        PhysicalFaultOffset::at(24),
        ExpectedFaultLocalization::PhysicalIntegrityBoundary,
    )
}

fn compaction_product_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::page_image(
        artifact(5213, BoundaryArtifactField::Payload),
        PhysicalFaultFieldKind::SlotState,
        PhysicalFaultOffset::at(32),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn mixed_root_locus() -> PhysicalArtifactFaultLocus {
    PhysicalArtifactFaultLocus::root_pointer(
        artifact(5214, BoundaryArtifactField::Basis),
        ExpectedFaultLocalization::ProductionDriverBoundary,
    )
}

fn artifact(id: u64, field: BoundaryArtifactField) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(BoundaryArtifactId::new(id), field)
}
