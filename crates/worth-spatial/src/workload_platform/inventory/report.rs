use super::classification::{
    LegacyFixtureClassification, ReceiptPosture, SurfaceAuthority, SurfaceScope, WorkloadSurfaceId,
};
use super::decision::InventoryDecision;
use super::validation::{validate_inventory_rows, InventoryValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeedInventoryRow {
    classification: LegacyFixtureClassification,
    decision: InventoryDecision,
    source_path: &'static str,
}

impl SeedInventoryRow {
    pub const fn new(
        classification: LegacyFixtureClassification,
        decision: InventoryDecision,
        source_path: &'static str,
    ) -> Self {
        Self {
            classification,
            decision,
            source_path,
        }
    }

    pub const fn classification(&self) -> LegacyFixtureClassification {
        self.classification
    }

    pub const fn decision(&self) -> InventoryDecision {
        self.decision
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn surface_id(&self) -> WorkloadSurfaceId {
        self.classification.surface_id()
    }

    pub const fn is_workload_candidate(&self) -> bool {
        matches!(self.classification.scope(), SurfaceScope::WorkloadCandidate)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeedInventoryReport {
    rows: Vec<SeedInventoryRow>,
    counters: SeedInventoryCounters,
}

impl SeedInventoryReport {
    pub fn certify_existing_surfaces() -> Result<Self, InventoryValidationError> {
        Self::from_rows(super::registry::existing_seed_inventory_rows())
    }

    pub fn from_rows(rows: Vec<SeedInventoryRow>) -> Result<Self, InventoryValidationError> {
        validate_inventory_rows(&rows)?;
        let counters = SeedInventoryCounters::from_rows(&rows);
        Ok(Self { rows, counters })
    }

    pub fn rows(&self) -> &[SeedInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &SeedInventoryCounters {
        &self.counters
    }

    pub fn require_surface(&self, surface: &str) -> Option<&SeedInventoryRow> {
        self.rows
            .iter()
            .find(|row| row.surface_id().as_str() == surface)
    }

    pub fn assert_every_surface_has_human_readable_decision(&self) -> bool {
        self.rows
            .iter()
            .all(|row| !matches!(row.classification().human_reason(), ""))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SeedInventoryCounters {
    registered_surfaces: usize,
    query_backed_surfaces: usize,
    production_receipt_surfaces: usize,
    unit_only_fixtures: usize,
    legacy_migration_surfaces: usize,
    test_local_surfaces: usize,
    workload_candidates: usize,
}

impl SeedInventoryCounters {
    fn from_rows(rows: &[SeedInventoryRow]) -> Self {
        Self {
            registered_surfaces: rows.len(),
            query_backed_surfaces: rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.classification().authority(),
                        SurfaceAuthority::QueryBackedTopology
                            | SurfaceAuthority::QueryBackedSpatialContract
                    )
                })
                .count(),
            production_receipt_surfaces: rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.classification().receipt_posture(),
                        ReceiptPosture::ProductionOwned
                    )
                })
                .count(),
            unit_only_fixtures: rows
                .iter()
                .filter(|row| matches!(row.classification().scope(), SurfaceScope::UnitSupportOnly))
                .count(),
            legacy_migration_surfaces: rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.classification().scope(),
                        SurfaceScope::LegacyMigrationOnly
                    )
                })
                .count(),
            test_local_surfaces: rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.classification().authority(),
                        SurfaceAuthority::TestLocalConvenience
                    )
                })
                .count(),
            workload_candidates: rows
                .iter()
                .filter(|row| row.is_workload_candidate())
                .count(),
        }
    }

    pub const fn registered_surfaces(&self) -> usize {
        self.registered_surfaces
    }

    pub const fn query_backed_surfaces(&self) -> usize {
        self.query_backed_surfaces
    }

    pub const fn production_receipt_surfaces(&self) -> usize {
        self.production_receipt_surfaces
    }

    pub const fn unit_only_fixtures(&self) -> usize {
        self.unit_only_fixtures
    }

    pub const fn legacy_migration_surfaces(&self) -> usize {
        self.legacy_migration_surfaces
    }

    pub const fn test_local_surfaces(&self) -> usize {
        self.test_local_surfaces
    }

    pub const fn workload_candidates(&self) -> usize {
        self.workload_candidates
    }
}
