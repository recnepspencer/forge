use crate::skeleton::S8CrateResponsibilityMap;

const LAYOUT_INDEXES_INTERNAL_HOMES: &[&str] = &[
    "artifact_family",
    "key_domain",
    "strategy",
    "strategy_registry",
    "materialization",
    "access_shape",
    "planning",
    "budget",
    "execution",
    "maintenance",
    "migration",
    "corruption",
    "customization",
    "bootstrap",
    "degraded_access",
    "legacy_disposition",
    "skeleton",
    "production_transition",
    "handoff",
    "facade",
    "compile_fail",
];

const LAYOUT_INDEXES_PUBLIC_FACADES: &[&str] = &[
    "layout_families.rs",
    "layout_strategy_admission.rs",
    "access_planning.rs",
    "access_lowering.rs",
    "access_execution.rs",
    "layout_rebuild.rs",
    "layout_migration.rs",
    "layout_counters.rs",
    "layout_readmission.rs",
    "layout_customization.rs",
    "layout_closeout.rs",
    "layout_certification.rs",
];

const FAMILY_HOMES: &[&str] = &[
    "forge-store-physical-format::layout_access",
    "forge-store-wal::layout_access",
    "forge-store-recovery-physics::layout_access",
    "forge-store-buffer-pool::layout_access",
    "forge-store-physical-integrity::layout_access",
    "forge-store-physical-isolation::layout_access",
    "forge-store-io-scheduler::layout_access",
    "forge-store-blob-chunks::layout_access",
    "forge-store-security::layout_access",
    "forge-store-operations::layout_access",
];

const COURTROOM_HOMES: &[&str] = &[
    "forge-store-physical-certification::layout_harness",
    "forge-store-certification::s8_layout_closeout",
    "forge-store-test-support::harness::production_facade::s8_layout_access",
];

const FAMILY_REQUIRED_FILES: &[&str] = &[
    "crates/forge-store-physical-format/src/layout_access/mod.rs",
    "crates/forge-store-wal/src/layout_access/mod.rs",
    "crates/forge-store-recovery-physics/src/layout_access/mod.rs",
    "crates/forge-store-buffer-pool/src/layout_access/mod.rs",
    "crates/forge-store-physical-integrity/src/layout_access/mod.rs",
    "crates/forge-store-physical-isolation/src/layout_access/mod.rs",
    "crates/forge-store-io-scheduler/src/layout_access/mod.rs",
    "crates/forge-store-blob-chunks/src/layout_access/mod.rs",
    "crates/forge-store-security/src/layout_access/mod.rs",
    "crates/forge-store-operations/src/layout_access/mod.rs",
];

const COURTROOM_REQUIRED_FILES: &[&str] = &[
    "crates/forge-store-physical-certification/src/layout_harness/mod.rs",
    "crates/forge-store-certification/src/s8_layout_closeout/mod.rs",
    "crates/forge-store-test-support/src/harness/production_facade.rs",
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
        LAYOUT_INDEXES_INTERNAL_HOMES
    }

    pub const fn layout_indexes_public_facades(&self) -> &'static [&'static str] {
        LAYOUT_INDEXES_PUBLIC_FACADES
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
