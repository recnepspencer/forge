use serde::Serialize;

use super::family::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologySpatialEvidencePosture,
    DerivedTopologyUpdatePosture,
};
use super::{catalog_digest, DerivedInvalidationFamilyCatalogCounters};
use crate::derived_topology::invalidation_plan::inventory::DerivedInvalidationPhaseTwoSeed;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationFamilyCatalog {
    phase_two_seed: DerivedInvalidationPhaseTwoSeed,
    families: Vec<DerivedTopologyProductFamilyRecord>,
    counters: DerivedInvalidationFamilyCatalogCounters,
    catalog_digest: String,
}

impl DerivedInvalidationFamilyCatalog {
    pub(crate) fn new(
        phase_two_seed: DerivedInvalidationPhaseTwoSeed,
        mut families: Vec<DerivedTopologyProductFamilyRecord>,
    ) -> Self {
        families.sort_by_key(DerivedTopologyProductFamilyRecord::identity);
        let counters = catalog_counters(&families);
        let mut parts = vec![
            "worth-topo:derived-invalidation-family-catalog:v1".to_string(),
            format!("phase-two-seed:{}", phase_two_seed.seed_digest()),
            format!("family-count:{}", counters.family_count()),
        ];
        parts.extend(
            families
                .iter()
                .map(|family| format!("family-digest:{}", family.family_digest())),
        );
        let catalog_digest = catalog_digest(parts);
        Self {
            phase_two_seed,
            families,
            counters,
            catalog_digest,
        }
    }

    pub fn phase_two_seed(&self) -> &DerivedInvalidationPhaseTwoSeed {
        &self.phase_two_seed
    }

    pub fn families(&self) -> &[DerivedTopologyProductFamilyRecord] {
        &self.families
    }

    pub fn family(
        &self,
        identity: DerivedTopologyProductFamilyIdentity,
    ) -> Option<&DerivedTopologyProductFamilyRecord> {
        self.families
            .iter()
            .find(|family| family.identity() == identity)
    }

    pub const fn counters(&self) -> DerivedInvalidationFamilyCatalogCounters {
        self.counters
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }
}

fn catalog_counters(
    families: &[DerivedTopologyProductFamilyRecord],
) -> DerivedInvalidationFamilyCatalogCounters {
    let query_required_family_count = families
        .iter()
        .filter(|family| family.query_receipt_posture().requires_query_support())
        .count();
    let legality_required_family_count = families
        .iter()
        .filter(|family| {
            family.legality_receipt_posture()
                != DerivedTopologyLegalityReceiptPosture::NotRequiredForFamilyDeclaration
        })
        .count();
    let bounded_rebuild_family_count = families
        .iter()
        .filter(|family| {
            family.update_posture() == DerivedTopologyUpdatePosture::BoundedRebuildRequired
        })
        .count();
    let incremental_eligible_family_count = families
        .iter()
        .filter(|family| {
            family.update_posture() == DerivedTopologyUpdatePosture::IncrementalEligible
        })
        .count();
    let spatial_receipt_required_family_count = families
        .iter()
        .filter(|family| family.spatial_evidence_posture().requires_spatial_receipt())
        .count();
    let no_spatial_evidence_family_count = families
        .iter()
        .filter(|family| {
            family.spatial_evidence_posture()
                == DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed
        })
        .count();
    DerivedInvalidationFamilyCatalogCounters::new(
        families.len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len(),
        query_required_family_count,
        legality_required_family_count,
        spatial_receipt_required_family_count,
        no_spatial_evidence_family_count,
        bounded_rebuild_family_count,
        incremental_eligible_family_count,
    )
}
