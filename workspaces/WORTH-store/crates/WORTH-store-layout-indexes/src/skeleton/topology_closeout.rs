use crate::skeleton::S8CrateResponsibilityMap;

const LAYOUT_INDEXES_HOMES: &[&str] = &[
    "artifact_family",
    "key_domain",
    "strategy",
    "strategy/btree",
    "strategy/lsm",
    "strategy_registry",
    "materialization",
    "access_shape",
    "planning",
    "budget",
    "execution",
    "maintenance",
    "migration",
    "corruption",
    "bootstrap",
    "degraded_access",
    "legacy_disposition",
    "skeleton",
    "handoff",
    "facade",
    "compile_fail",
];

const FAMILY_HOMES: &[&str] = &[
    "worth-store-physical-format::layout_access",
    "worth-store-wal::layout_access",
    "worth-store-recovery-physics::layout_access",
    "worth-store-buffer-pool::layout_access",
    "worth-store-physical-integrity::layout_access",
    "worth-store-physical-isolation::layout_access",
    "worth-store-io-scheduler::layout_access",
    "worth-store-blob-chunks::layout_access",
    "worth-store-security::layout_access",
    "worth-store-operations::layout_access",
];

const COURTROOM_HOMES: &[&str] = &[
    "worth-store-physical-certification::harness::by_milestone::s8_layout_access",
    "worth-store-certification::s8_layout_closeout",
    "worth-store-test-support::harness::milestone::s8_layout_access",
];

const FAMILY_REQUIRED_FILES: &[&str] = &[
    "crates/worth-store-physical-format/src/layout_access/mod.rs",
    "crates/worth-store-physical-format/src/layout_access/page_family.rs",
    "crates/worth-store-physical-format/src/layout_access/frame_family.rs",
    "crates/worth-store-physical-format/src/layout_access/segment_family.rs",
    "crates/worth-store-physical-format/src/layout_access/extent_family.rs",
    "crates/worth-store-physical-format/src/layout_access/record_family.rs",
    "crates/worth-store-physical-format/src/layout_access/manifest_family.rs",
    "crates/worth-store-physical-format/src/layout_access/root_discovery_family.rs",
    "crates/worth-store-physical-format/src/layout_access/format_family_closeout.rs",
    "crates/worth-store-wal/src/layout_access/mod.rs",
    "crates/worth-store-wal/src/layout_access/wal_record_family.rs",
    "crates/worth-store-wal/src/layout_access/wal_segment_family.rs",
    "crates/worth-store-wal/src/layout_access/checkpoint_family.rs",
    "crates/worth-store-wal/src/layout_access/durable_mutation_family.rs",
    "crates/worth-store-wal/src/layout_access/replay_tail_family.rs",
    "crates/worth-store-wal/src/layout_access/wal_layout_closeout.rs",
    "crates/worth-store-recovery-physics/src/layout_access/mod.rs",
    "crates/worth-store-recovery-physics/src/layout_access/recovery_source_family.rs",
    "crates/worth-store-recovery-physics/src/layout_access/replay_index_family.rs",
    "crates/worth-store-recovery-physics/src/layout_access/crash_boundary_family.rs",
    "crates/worth-store-recovery-physics/src/layout_access/checkpoint_cutover_family.rs",
    "crates/worth-store-recovery-physics/src/layout_access/readmission_family.rs",
    "crates/worth-store-recovery-physics/src/layout_access/recovery_layout_closeout.rs",
    "crates/worth-store-buffer-pool/src/layout_access/mod.rs",
    "crates/worth-store-buffer-pool/src/layout_access/resident_frame_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/page_lease_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/dirty_state_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/zero_copy_view_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/read_ahead_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/write_behind_family.rs",
    "crates/worth-store-buffer-pool/src/layout_access/buffer_pool_layout_closeout.rs",
    "crates/worth-store-physical-integrity/src/layout_access/mod.rs",
    "crates/worth-store-physical-integrity/src/layout_access/checksum_family.rs",
    "crates/worth-store-physical-integrity/src/layout_access/pre_decode_family.rs",
    "crates/worth-store-physical-integrity/src/layout_access/scrub_family.rs",
    "crates/worth-store-physical-integrity/src/layout_access/damage_map_family.rs",
    "crates/worth-store-physical-integrity/src/layout_access/quarantine_family.rs",
    "crates/worth-store-physical-integrity/src/layout_access/integrity_layout_closeout.rs",
    "crates/worth-store-physical-isolation/src/layout_access/mod.rs",
    "crates/worth-store-physical-isolation/src/layout_access/stable_read_family.rs",
    "crates/worth-store-physical-isolation/src/layout_access/reclaim_barrier_family.rs",
    "crates/worth-store-physical-isolation/src/layout_access/compaction_interlock_family.rs",
    "crates/worth-store-physical-isolation/src/layout_access/movable_stability_family.rs",
    "crates/worth-store-physical-isolation/src/layout_access/orphan_reclaim_family.rs",
    "crates/worth-store-physical-isolation/src/layout_access/isolation_layout_closeout.rs",
    "crates/worth-store-io-scheduler/src/layout_access/mod.rs",
    "crates/worth-store-io-scheduler/src/layout_access/foreground_admission_family.rs",
    "crates/worth-store-io-scheduler/src/layout_access/background_reservation_family.rs",
    "crates/worth-store-io-scheduler/src/layout_access/queue_execution_family.rs",
    "crates/worth-store-io-scheduler/src/layout_access/pacing_family.rs",
    "crates/worth-store-io-scheduler/src/layout_access/io_layout_closeout.rs",
    "crates/worth-store-blob-chunks/src/layout_access/mod.rs",
    "crates/worth-store-blob-chunks/src/layout_access/blob_object_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/chunk_tree_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/streaming_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/dedupe_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/reachability_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/retention_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/reclaim_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/compaction_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/export_import_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/capsule_family.rs",
    "crates/worth-store-blob-chunks/src/layout_access/blob_layout_closeout.rs",
    "crates/worth-store-security/src/layout_access/mod.rs",
    "crates/worth-store-security/src/layout_access/tenant_scope_family.rs",
    "crates/worth-store-security/src/layout_access/key_scope_family.rs",
    "crates/worth-store-security/src/layout_access/custody_family.rs",
    "crates/worth-store-security/src/layout_access/authenticity_family.rs",
    "crates/worth-store-security/src/layout_access/repair_blast_radius_family.rs",
    "crates/worth-store-security/src/layout_access/security_layout_closeout.rs",
    "crates/worth-store-operations/src/layout_access/mod.rs",
    "crates/worth-store-operations/src/layout_access/backup_family.rs",
    "crates/worth-store-operations/src/layout_access/restore_family.rs",
    "crates/worth-store-operations/src/layout_access/import_family.rs",
    "crates/worth-store-operations/src/layout_access/export_family.rs",
    "crates/worth-store-operations/src/layout_access/repair_family.rs",
    "crates/worth-store-operations/src/layout_access/capsule_operation_family.rs",
    "crates/worth-store-operations/src/layout_access/operations_layout_closeout.rs",
];

const COURTROOM_REQUIRED_FILES: &[&str] = &[
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/mod.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/scenario.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/actors.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/drivers.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/faults.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/observers.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/oracles.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/coverage.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/transcript.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/simulation.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/shortcut_denials.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/heavy_blob_profile.rs",
    "crates/worth-store-physical-certification/src/harness/by_milestone/s8_layout_access/tests.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/mod.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/sources.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/classifier.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/verifier.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/certificate.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/handoffs.rs",
    "crates/worth-store-certification/src/s8_layout_closeout/denial.rs",
    "crates/worth-store-test-support/src/harness/milestone/s8_layout_access/mod.rs",
    "crates/worth-store-test-support/src/harness/milestone/s8_layout_access/fixtures.rs",
    "crates/worth-store-test-support/src/harness/milestone/s8_layout_access/family_builders.rs",
    "crates/worth-store-test-support/src/harness/milestone/s8_layout_access/scenario_builders.rs",
    "crates/worth-store-test-support/src/harness/milestone/s8_layout_access/adversarial_inputs.rs",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8DomainSkeletonInventory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8SubsystemTopologyCloseout;

impl S8DomainSkeletonInventory {
    pub const fn current() -> Self {
        Self
    }

    pub const fn responsibility_rows(
        &self,
    ) -> &'static [crate::skeleton::S8CrateResponsibilityRow] {
        S8CrateResponsibilityMap::current().rows()
    }
}

impl S8SubsystemTopologyCloseout {
    pub const fn current() -> Self {
        Self
    }

    pub const fn layout_indexes_homes(&self) -> &'static [&'static str] {
        LAYOUT_INDEXES_HOMES
    }

    pub const fn family_homes(&self) -> &'static [&'static str] {
        FAMILY_HOMES
    }

    pub const fn courtroom_homes(&self) -> &'static [&'static str] {
        COURTROOM_HOMES
    }

    pub const fn family_required_files(&self) -> &'static [&'static str] {
        FAMILY_REQUIRED_FILES
    }

    pub const fn courtroom_required_files(&self) -> &'static [&'static str] {
        COURTROOM_REQUIRED_FILES
    }
}
