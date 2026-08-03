use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[ControlledMutation {
    id: 127,
    predicate: "c7-interrupted-successor-prefix-cleanup-omitted",
    source: "crates/worth-store/src/physical_runtime/durability/wal/inventory/reopen/interrupted_active_tail.rs",
    needle: "        Ok(InterruptedActiveSegment {\n            artifact: self.artifact,\n            proof: self.proof,\n        })",
    replacement: "        Err(PhysicalWalOpenFailure::SegmentInspection(\n            worth_store_wal::WalArtifactStoreDenial::InvalidFrame,\n        ))",
    package: "worth-store",
    target: MutationTarget::Integration("physical_record_journeys"),
    selector: "durability_admission::wal_reopen::interrupted_tail::partial_first_frame_in_the_exact_active_successor_is_removed_before_reopen",
}];
