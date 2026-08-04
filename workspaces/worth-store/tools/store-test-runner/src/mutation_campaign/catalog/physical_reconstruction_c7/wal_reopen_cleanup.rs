use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 118,
        predicate: "c7-trailing-empty-wal-cleanup-omitted",
        source: "crates/worth-store/src/physical_runtime/durability/wal/inventory/reopen/trailing_empty_segment.rs",
        needle: "    if trailing_bytes != 0 {",
        replacement: "    if true {",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::wal_reopen::trailing_empty_segment_from_interrupted_rotation_is_cleaned_before_reopen",
    },
    ControlledMutation {
        id: 119,
        predicate: "c7-interrupted-active-tail-cleanup-omitted",
        source: "crates/worth-store/src/physical_runtime/durability/wal/inventory/reopen/interrupted_active_tail.rs",
        needle: "        tree.truncate_file_durably(&self.artifact, self.proof.valid_prefix_bytes())\n            .map_err(PhysicalWalOpenFailure::Media)",
        replacement: "        let _repair_omitted = (tree, self.artifact, self.proof);\n        Ok(())",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::wal_reopen::interrupted_tail::interrupted_final_frame_is_truncated_to_its_verified_prefix_before_reopen",
    },
];
