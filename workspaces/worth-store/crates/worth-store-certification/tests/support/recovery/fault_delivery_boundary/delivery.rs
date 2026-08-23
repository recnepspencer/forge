use super::scenarios::{context_for_yieldpoint, scenario_for_yieldpoint};
use worth_store_physical_backend::ProductionStorageBoundarySeam;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, ExpectedFaultLocalization, FaultDeliveryDenial,
    ObservedFaultBoundary, PhysicalArtifactKind, PhysicalBoundarySeam, PhysicalBoundaryYieldpoint,
    PhysicalFaultEvent, PhysicalFaultEventKind, PhysicalFaultFieldKind,
};
use worth_store_test_support::harness::test_authority::{
    io_pressure_fault_locus, observed_checksum_mismatch_boundary, observed_io_pressure_boundary,
    observed_torn_frame_boundary, page_generation_fault_locus, wal_frame_payload_fault_locus,
};

#[derive(Clone)]
pub struct StorageFaultDeliveryCase {
    pub event_kind: PhysicalFaultEventKind,
    event: PhysicalFaultEvent,
    pub yieldpoint: PhysicalBoundaryYieldpoint,
    pub expected_seam: PhysicalBoundarySeam,
    pub expected_localization: ExpectedFaultLocalization,
    pub actual_boundary: ObservedFaultBoundary,
    pub artifact_kind: PhysicalArtifactKind,
    pub field_kind: PhysicalFaultFieldKind,
    pub requires_offset: bool,
}

pub fn deliver_storage_event(
    case: &StorageFaultDeliveryCase,
) -> Result<worth_store_physical_certification::FaultDeliveryReceipt, FaultDeliveryDenial> {
    let plan = lower_physical_simulation_plan(
        scenario_for_yieldpoint(case.yieldpoint.name()),
        context_for_yieldpoint(case.yieldpoint.clone()),
    )
    .unwrap();
    case.event.clone().deliver_through(
        plan.yieldpoint_binding(),
        case.yieldpoint.clone(),
        case.actual_boundary.clone(),
    )
}

pub fn storage_fault_delivery_cases() -> Vec<StorageFaultDeliveryCase> {
    vec![
        storage_case(
            PhysicalFaultEvent::torn_write(
                ProductionStorageBoundarySeam::WalAppendBeforeFlush,
                page_generation_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::TornWrite,
            ProductionStorageBoundarySeam::WalAppendBeforeFlush,
            ExpectedFaultLocalization::PhysicalIntegrityBoundary,
            observed_torn_frame_boundary(),
            PhysicalArtifactKind::PageImage,
            PhysicalFaultFieldKind::GenerationField,
        ),
        storage_case(
            PhysicalFaultEvent::dropped_flush(
                ProductionStorageBoundarySeam::WalFlush,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::DroppedFlush,
            ProductionStorageBoundarySeam::WalFlush,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::reordered_persistence(
                ProductionStorageBoundarySeam::RootSwap,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::ReorderedPersistence,
            ProductionStorageBoundarySeam::RootSwap,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::byte_corruption(
                ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::ByteCorruption,
            ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::stale_generation(
                ProductionStorageBoundarySeam::PagePin,
                page_generation_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::StaleGeneration,
            ProductionStorageBoundarySeam::PagePin,
            ExpectedFaultLocalization::PhysicalIntegrityBoundary,
            observed_torn_frame_boundary(),
            PhysicalArtifactKind::PageImage,
            PhysicalFaultFieldKind::GenerationField,
        ),
        storage_case(
            PhysicalFaultEvent::delayed_release(
                ProductionStorageBoundarySeam::LeasePublish,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::DelayedRelease,
            ProductionStorageBoundarySeam::LeasePublish,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        storage_case(
            PhysicalFaultEvent::blocked_reclaim(
                ProductionStorageBoundarySeam::ReclaimEligibility,
                wal_frame_payload_fault_locus(),
            )
            .unwrap(),
            PhysicalFaultEventKind::BlockedReclaim,
            ProductionStorageBoundarySeam::ReclaimEligibility,
            ExpectedFaultLocalization::PreDecodeBoundary,
            observed_checksum_mismatch_boundary(),
            PhysicalArtifactKind::WalFrame,
            PhysicalFaultFieldKind::ChecksumProtectedPayload,
        ),
        io_stall_case(),
    ]
}

fn storage_case(
    event: PhysicalFaultEvent,
    event_kind: PhysicalFaultEventKind,
    seam: ProductionStorageBoundarySeam,
    expected_localization: ExpectedFaultLocalization,
    actual_boundary: ObservedFaultBoundary,
    artifact_kind: PhysicalArtifactKind,
    field_kind: PhysicalFaultFieldKind,
) -> StorageFaultDeliveryCase {
    StorageFaultDeliveryCase {
        event_kind,
        event,
        yieldpoint: PhysicalBoundaryYieldpoint::production_storage(seam),
        expected_seam: PhysicalBoundarySeam::ProductionStorage(seam),
        expected_localization,
        actual_boundary,
        artifact_kind,
        field_kind,
        requires_offset: true,
    }
}

fn io_stall_case() -> StorageFaultDeliveryCase {
    StorageFaultDeliveryCase {
        event_kind: PhysicalFaultEventKind::IoStall,
        event: PhysicalFaultEvent::io_stall(io_pressure_fault_locus()).unwrap(),
        yieldpoint: PhysicalBoundaryYieldpoint::io_pressure_boundary(),
        expected_seam: PhysicalBoundarySeam::IoPressure,
        expected_localization: ExpectedFaultLocalization::ProductionDriverBoundary,
        actual_boundary: observed_io_pressure_boundary(),
        artifact_kind: PhysicalArtifactKind::PageImage,
        field_kind: PhysicalFaultFieldKind::SlotState,
        requires_offset: true,
    }
}
