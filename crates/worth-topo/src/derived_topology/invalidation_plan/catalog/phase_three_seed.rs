use serde::Serialize;

use super::{catalog_digest as make_catalog_digest, DerivedInvalidationFamilyCatalog};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPhaseThreeSeed {
    inventory_seed_digest: String,
    catalog_digest: String,
    required_family_count: usize,
    declared_family_count: usize,
    query_required_family_count: usize,
    legality_required_family_count: usize,
    spatial_receipt_required_family_count: usize,
    no_spatial_evidence_family_count: usize,
    bounded_rebuild_family_count: usize,
    incremental_eligible_family_count: usize,
    seed_digest: String,
}

impl DerivedInvalidationPhaseThreeSeed {
    pub(crate) fn from_catalog(catalog: &DerivedInvalidationFamilyCatalog) -> Self {
        let counters = catalog.counters();
        let inventory_seed_digest = catalog.phase_two_seed().seed_digest().to_string();
        let catalog_digest = catalog.catalog_digest().to_string();
        let required_family_count = counters.required_family_count();
        let declared_family_count = counters.family_count();
        let query_required_family_count = counters.query_required_family_count();
        let legality_required_family_count = counters.legality_required_family_count();
        let spatial_receipt_required_family_count =
            counters.spatial_receipt_required_family_count();
        let no_spatial_evidence_family_count = counters.no_spatial_evidence_family_count();
        let bounded_rebuild_family_count = counters.bounded_rebuild_family_count();
        let incremental_eligible_family_count = counters.incremental_eligible_family_count();
        let seed_digest = make_catalog_digest(vec![
            inventory_seed_digest.clone(),
            catalog_digest.clone(),
            format!("required:{required_family_count}"),
            format!("declared:{declared_family_count}"),
            format!("query:{query_required_family_count}"),
            format!("legality:{legality_required_family_count}"),
            format!("spatial-required:{spatial_receipt_required_family_count}"),
            format!("no-spatial:{no_spatial_evidence_family_count}"),
            format!("bounded:{bounded_rebuild_family_count}"),
            format!("incremental:{incremental_eligible_family_count}"),
        ]);
        Self {
            inventory_seed_digest,
            catalog_digest,
            required_family_count,
            declared_family_count,
            query_required_family_count,
            legality_required_family_count,
            spatial_receipt_required_family_count,
            no_spatial_evidence_family_count,
            bounded_rebuild_family_count,
            incremental_eligible_family_count,
            seed_digest,
        }
    }

    pub fn inventory_seed_digest(&self) -> &str {
        &self.inventory_seed_digest
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
