use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationFamilyCatalogCounters {
    family_count: usize,
    required_family_count: usize,
    query_required_family_count: usize,
    legality_required_family_count: usize,
    spatial_receipt_required_family_count: usize,
    no_spatial_evidence_family_count: usize,
    bounded_rebuild_family_count: usize,
    incremental_eligible_family_count: usize,
}

impl DerivedInvalidationFamilyCatalogCounters {
    pub(crate) const fn new(
        family_count: usize,
        required_family_count: usize,
        query_required_family_count: usize,
        legality_required_family_count: usize,
        spatial_receipt_required_family_count: usize,
        no_spatial_evidence_family_count: usize,
        bounded_rebuild_family_count: usize,
        incremental_eligible_family_count: usize,
    ) -> Self {
        Self {
            family_count,
            required_family_count,
            query_required_family_count,
            legality_required_family_count,
            spatial_receipt_required_family_count,
            no_spatial_evidence_family_count,
            bounded_rebuild_family_count,
            incremental_eligible_family_count,
        }
    }

    pub const fn family_count(self) -> usize {
        self.family_count
    }

    pub const fn required_family_count(self) -> usize {
        self.required_family_count
    }

    pub const fn query_required_family_count(self) -> usize {
        self.query_required_family_count
    }

    pub const fn legality_required_family_count(self) -> usize {
        self.legality_required_family_count
    }

    pub const fn spatial_receipt_required_family_count(self) -> usize {
        self.spatial_receipt_required_family_count
    }

    pub const fn no_spatial_evidence_family_count(self) -> usize {
        self.no_spatial_evidence_family_count
    }

    pub const fn bounded_rebuild_family_count(self) -> usize {
        self.bounded_rebuild_family_count
    }

    pub const fn incremental_eligible_family_count(self) -> usize {
        self.incremental_eligible_family_count
    }
}
