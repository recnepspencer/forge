use crate::skeleton::{S8CratePrimaryRole, S8ProjectionOutputPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8CrateResponsibilityRow {
    crate_name: &'static str,
    primary_role: S8CratePrimaryRole,
    minted_authority: &'static str,
    consumed_authority: &'static str,
    projection_outputs: S8ProjectionOutputPosture,
    public_facade_home: &'static str,
    phase_obligations: &'static [u8],
}

const PHASE_ZERO_ONLY: &[u8] = &[0];

const RESPONSIBILITY_ROWS: &[S8CrateResponsibilityRow] = &[
    S8CrateResponsibilityRow::new(
        "forge-store-contracts",
        S8CratePrimaryRole::SharedContractVocabulary,
        "shared Store ids and contract names",
        "none",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_contracts",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-layout-indexes",
        S8CratePrimaryRole::LayoutAccessGrammar,
        "S.8 layout/access grammar and sealed progression witnesses",
        "Store family authority declarations",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_layout_indexes::{layout_families,layout_strategy_admission,access_planning,access_lowering,access_execution,layout_rebuild,layout_migration,layout_counters,layout_readmission,layout_customization,layout_closeout,layout_certification}",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-physical-format",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "physical byte/layout execution authority",
        "S.8 layout grammar",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_physical_format::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-wal",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "WAL/checkpoint execution authority",
        "S.8 strategy and access grammar",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_wal::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-recovery-physics",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "recovery physics execution authority",
        "S.8 layout access state machines",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_recovery_physics::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-buffer-pool",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "residency and allocation execution authority",
        "S.8 access budgets and shapes",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_buffer_pool::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-physical-integrity",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "pre-decode integrity authority",
        "S.8 corruption and readmission vocabulary",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_physical_integrity::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-physical-isolation",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "stable-read and interference authority",
        "S.8 access footprints and counters",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_physical_isolation::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-io-scheduler",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "queue admission and pacing authority",
        "S.8 budgets and counter envelopes",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_io_scheduler::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-blob-chunks",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "blob lifecycle authority",
        "S.8 layout/index law",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_blob_chunks::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-security",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "tenant/key/custody/authenticity scope authority",
        "S.8 key-domain partition law",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_security::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-operations",
        S8CratePrimaryRole::FamilyExecutionAuthority,
        "backup/repair/import/export workflow posture",
        "S.8 readmission and layout grammar",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_operations::layout_access",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-maintenance",
        S8CratePrimaryRole::SpecializedConsumer,
        "maintenance workflow execution posture",
        "S.8 maintenance and degraded-access contracts",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_maintenance",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-retention",
        S8CratePrimaryRole::SpecializedConsumer,
        "retention workflow execution posture",
        "S.8 layout/access contracts for retained families",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_retention",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-tiering",
        S8CratePrimaryRole::SpecializedConsumer,
        "tiering workflow execution posture",
        "S.8 layout/access contracts for cold and recalled families",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_tiering",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-snapshots",
        S8CratePrimaryRole::SpecializedConsumer,
        "snapshot execution posture",
        "S.8 layout/access contracts for snapshot-owned families",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_snapshots",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-branch-deltas",
        S8CratePrimaryRole::SpecializedConsumer,
        "branch-delta execution posture",
        "S.8 layout/access contracts for branch-owned families",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_branch_deltas",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-compatibility",
        S8CratePrimaryRole::SpecializedConsumer,
        "compatibility execution posture",
        "S.8 migration and readmission contracts",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_compatibility",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-replication",
        S8CratePrimaryRole::SpecializedConsumer,
        "replication execution posture",
        "S.8 readmission and bounded-access contracts",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_replication",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-bulk",
        S8CratePrimaryRole::SpecializedConsumer,
        "bulk workflow execution posture",
        "S.8 access-shape and budget contracts",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_bulk",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-live-query",
        S8CratePrimaryRole::SpecializedConsumer,
        "live-query execution posture",
        "S.8 bounded-access and invalidation contracts",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_live_query",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-offline-verifier",
        S8CratePrimaryRole::TerminalOfflineObservation,
        "offline observation only",
        "persisted layout evidence",
        S8ProjectionOutputPosture::TerminalObservation,
        "forge_store_offline_verifier",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-readiness",
        S8CratePrimaryRole::ReadinessHandoff,
        "S.8 to S.9 handoff vocabulary",
        "lower-crate closeout evidence",
        S8ProjectionOutputPosture::ProductionBoundaryEvidence,
        "forge_store_readiness",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-physical-certification",
        S8CratePrimaryRole::PhysicalCertificationCourtroom,
        "courtroom simulation evidence",
        "production witnesses and executed evidence",
        S8ProjectionOutputPosture::CourtroomOnlyEvidence,
        "forge_store_physical_certification::layout_harness",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-certification",
        S8CratePrimaryRole::CertificationCloseoutCourtroom,
        "certification closeout evidence",
        "production law evidence",
        S8ProjectionOutputPosture::CourtroomOnlyEvidence,
        "forge_store_certification::s8_layout_closeout",
        PHASE_ZERO_ONLY,
    ),
    S8CrateResponsibilityRow::new(
        "forge-store-test-support",
        S8CratePrimaryRole::HonestTestFixtureSupport,
        "non-authority fixture support",
        "production facades",
        S8ProjectionOutputPosture::NonAuthorityFixture,
        "forge_store_test_support::harness::production_facade::s8_layout_access",
        PHASE_ZERO_ONLY,
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8CrateResponsibilityMap;

impl S8CrateResponsibilityRow {
    pub const fn new(
        crate_name: &'static str,
        primary_role: S8CratePrimaryRole,
        minted_authority: &'static str,
        consumed_authority: &'static str,
        projection_outputs: S8ProjectionOutputPosture,
        public_facade_home: &'static str,
        phase_obligations: &'static [u8],
    ) -> Self {
        Self {
            crate_name,
            primary_role,
            minted_authority,
            consumed_authority,
            projection_outputs,
            public_facade_home,
            phase_obligations,
        }
    }

    pub const fn crate_name(&self) -> &'static str {
        self.crate_name
    }

    pub const fn primary_role(&self) -> S8CratePrimaryRole {
        self.primary_role
    }

    pub const fn minted_authority(&self) -> &'static str {
        self.minted_authority
    }

    pub const fn consumed_authority(&self) -> &'static str {
        self.consumed_authority
    }

    pub const fn projection_outputs(&self) -> S8ProjectionOutputPosture {
        self.projection_outputs
    }

    pub const fn public_facade_home(&self) -> &'static str {
        self.public_facade_home
    }

    pub const fn phase_obligations(&self) -> &'static [u8] {
        self.phase_obligations
    }
}

impl S8CrateResponsibilityMap {
    pub const fn current() -> Self {
        Self
    }

    pub const fn rows(&self) -> &'static [S8CrateResponsibilityRow] {
        RESPONSIBILITY_ROWS
    }
}
