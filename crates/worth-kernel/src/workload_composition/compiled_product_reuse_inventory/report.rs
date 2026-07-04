use super::classification::{
    CompiledProductReuseDisposition, CompiledProductReuseSemanticCategory,
};
use super::row::{CompiledProductReuseInventoryRow, CompiledProductReuseSurfaceIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReuseInventoryCounters {
    row_count: usize,
    migrate_count: usize,
    delete_count: usize,
    cap_count: usize,
    certification_only_count: usize,
    query_gap_count: usize,
    ordinary_row_count: usize,
}

impl CompiledProductReuseInventoryCounters {
    pub(crate) fn from_rows(rows: &[CompiledProductReuseInventoryRow]) -> Self {
        Self {
            row_count: rows.len(),
            migrate_count: count(rows, CompiledProductReuseDisposition::Migrate),
            delete_count: count(rows, CompiledProductReuseDisposition::Delete),
            cap_count: count(rows, CompiledProductReuseDisposition::Cap),
            certification_only_count: count(
                rows,
                CompiledProductReuseDisposition::CertificationOnly,
            ),
            query_gap_count: count(rows, CompiledProductReuseDisposition::QueryGap),
            ordinary_row_count: rows.iter().filter(|row| row.ordinary_path()).count(),
        }
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }
    pub const fn ordinary_row_count(&self) -> usize {
        self.ordinary_row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReuseInventoryReport {
    rows: Vec<CompiledProductReuseInventoryRow>,
    counters: CompiledProductReuseInventoryCounters,
    required_covered_categories: Vec<CompiledProductReuseSemanticCategory>,
    required_surfaces: Vec<CompiledProductReuseSurfaceIdentity>,
}

impl CompiledProductReuseInventoryReport {
    pub(crate) fn new(rows: Vec<CompiledProductReuseInventoryRow>) -> Self {
        Self {
            counters: CompiledProductReuseInventoryCounters::from_rows(&rows),
            rows,
            required_covered_categories: CompiledProductReuseSemanticCategory::REQUIRED_COVERED
                .to_vec(),
            required_surfaces: required_surfaces(),
        }
    }

    pub fn rows(&self) -> &[CompiledProductReuseInventoryRow] {
        &self.rows
    }

    pub fn ordinary_rows(&self) -> impl Iterator<Item = &CompiledProductReuseInventoryRow> {
        self.rows.iter().filter(|row| row.ordinary_path())
    }

    pub const fn counters(&self) -> &CompiledProductReuseInventoryCounters {
        &self.counters
    }

    pub fn required_covered_categories(&self) -> &[CompiledProductReuseSemanticCategory] {
        &self.required_covered_categories
    }

    pub fn required_surfaces(&self) -> &[CompiledProductReuseSurfaceIdentity] {
        &self.required_surfaces
    }
}

fn count(
    rows: &[CompiledProductReuseInventoryRow],
    disposition: CompiledProductReuseDisposition,
) -> usize {
    rows.iter()
        .filter(|row| row.disposition() == disposition)
        .count()
}

fn required_surfaces() -> Vec<CompiledProductReuseSurfaceIdentity> {
    use CompiledProductReuseSurfaceIdentity as Surface;
    vec![
        Surface::BuildDerivedEquivalenceContract,
        Surface::CompareDerivedEquivalenceContracts,
        Surface::DerivedInvalidationPlannedDispositionFromUpdatePosture,
        Surface::HistoricalPathReuseDescriptorRetainedReuse,
        Surface::ReuseEvidenceLookupIndexProduct,
        Surface::ReplayParityReportFromRetainedProjectionMatch,
        Surface::LookupConsumedWorkloadCompositionAdmit,
        Surface::CurrentEvidenceLookupPublicCloseout,
        Surface::CurrentWorthTouchedGraphConflictPublicCloseout,
    ]
}
