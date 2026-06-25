use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologyQueryReceiptPosture,
    DerivedTopologyUpdatePosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationPlannedDisposition {
    IncrementalUpdate,
    BoundedRebuild,
}

impl DerivedInvalidationPlannedDisposition {
    pub const fn from_update_posture(posture: DerivedTopologyUpdatePosture) -> Self {
        match posture {
            DerivedTopologyUpdatePosture::IncrementalEligible => Self::IncrementalUpdate,
            DerivedTopologyUpdatePosture::BoundedRebuildRequired => Self::BoundedRebuild,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncrementalUpdate => "incremental_update",
            Self::BoundedRebuild => "bounded_rebuild",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationSelectedRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    query_posture: DerivedTopologyQueryReceiptPosture,
    query_receipt_digest: Option<String>,
    legality_posture: DerivedTopologyLegalityReceiptPosture,
    legality_receipt_digest: Option<String>,
    planned_disposition: DerivedInvalidationPlannedDisposition,
    row_digest: String,
}

impl DerivedInvalidationSelectedRow {
    pub(crate) fn from_family(
        family: &DerivedTopologyProductFamilyRecord,
        query_receipt_digest: Option<&str>,
        legality_receipt_digest: Option<&str>,
    ) -> Self {
        let planned_disposition =
            DerivedInvalidationPlannedDisposition::from_update_posture(family.update_posture());
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-selected-row:v1".to_string(),
            format!("family:{}", family.identity().as_str()),
            format!("family-digest:{}", family.family_digest()),
            format!("query:{}", family.query_receipt_posture().as_str()),
            format!(
                "query-receipt:{}",
                query_receipt_digest.unwrap_or("not-required")
            ),
            format!("legality:{}", family.legality_receipt_posture().as_str()),
            format!(
                "legality-receipt:{}",
                legality_receipt_digest.unwrap_or("not-required")
            ),
            format!("disposition:{}", planned_disposition.as_str()),
        ]);
        Self {
            family_identity: family.identity(),
            family_digest: family.family_digest().to_string(),
            query_posture: family.query_receipt_posture(),
            query_receipt_digest: query_receipt_digest.map(str::to_string),
            legality_posture: family.legality_receipt_posture(),
            legality_receipt_digest: legality_receipt_digest.map(str::to_string),
            planned_disposition,
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub const fn query_posture(&self) -> DerivedTopologyQueryReceiptPosture {
        self.query_posture
    }

    pub fn query_receipt_digest(&self) -> Option<&str> {
        self.query_receipt_digest.as_deref()
    }

    pub const fn legality_posture(&self) -> DerivedTopologyLegalityReceiptPosture {
        self.legality_posture
    }

    pub fn legality_receipt_digest(&self) -> Option<&str> {
        self.legality_receipt_digest.as_deref()
    }

    pub const fn planned_disposition(&self) -> DerivedInvalidationPlannedDisposition {
        self.planned_disposition
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
