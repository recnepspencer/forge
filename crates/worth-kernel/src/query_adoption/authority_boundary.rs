use super::classification::{
    WorthQueryAdoptionInventoryRow, WorthQueryAuthorityCategory, WorthQueryAuthorityDomain,
    WorthQueryAuthorityPromotionTarget,
};
use super::report::WorthQueryAdoptionInventoryReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthorityProjectionRow {
    source_set: &'static str,
    machine_inventory_category: WorthQueryAuthorityCategory,
    support_report_category: WorthQueryAuthorityCategory,
    docs_category: WorthQueryAuthorityCategory,
}

impl WorthQueryAuthorityProjectionRow {
    fn new(row: &WorthQueryAdoptionInventoryRow) -> Self {
        Self {
            source_set: row.source_set(),
            machine_inventory_category: row.authority_category(),
            support_report_category: row.authority_category(),
            docs_category: row.authority_category(),
        }
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn machine_inventory_category(&self) -> WorthQueryAuthorityCategory {
        self.machine_inventory_category
    }

    pub const fn support_report_category(&self) -> WorthQueryAuthorityCategory {
        self.support_report_category
    }

    pub const fn docs_category(&self) -> WorthQueryAuthorityCategory {
        self.docs_category
    }

    pub fn categories_are_in_parity(&self) -> bool {
        self.machine_inventory_category == self.support_report_category
            && self.machine_inventory_category == self.docs_category
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthorityBoundaryReport {
    rows: Vec<WorthQueryAuthorityProjectionRow>,
}

impl WorthQueryAuthorityBoundaryReport {
    pub fn from_inventory(report: &WorthQueryAdoptionInventoryReport) -> Self {
        Self {
            rows: report
                .rows()
                .iter()
                .map(WorthQueryAuthorityProjectionRow::new)
                .collect(),
        }
    }

    pub fn rows(&self) -> &[WorthQueryAuthorityProjectionRow] {
        &self.rows
    }

    pub fn require_source_set(
        &self,
        source_set: &str,
    ) -> Option<&WorthQueryAuthorityProjectionRow> {
        self.rows.iter().find(|row| row.source_set() == source_set)
    }

    pub fn all_rows_are_in_parity(&self) -> bool {
        self.rows
            .iter()
            .all(WorthQueryAuthorityProjectionRow::categories_are_in_parity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthorityPromotionDenial {
    source_set: &'static str,
    authority_category: WorthQueryAuthorityCategory,
    authority_domain: WorthQueryAuthorityDomain,
    target: WorthQueryAuthorityPromotionTarget,
}

impl WorthQueryAuthorityPromotionDenial {
    fn new(
        row: &WorthQueryAdoptionInventoryRow,
        target: WorthQueryAuthorityPromotionTarget,
    ) -> Self {
        Self {
            source_set: row.source_set(),
            authority_category: row.authority_category(),
            authority_domain: row.authority_domain(),
            target,
        }
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn authority_category(&self) -> WorthQueryAuthorityCategory {
        self.authority_category
    }

    pub const fn authority_domain(&self) -> WorthQueryAuthorityDomain {
        self.authority_domain
    }

    pub const fn target(&self) -> WorthQueryAuthorityPromotionTarget {
        self.target
    }
}

pub fn assert_authority_promotion_allowed(
    row: &WorthQueryAdoptionInventoryRow,
    target: WorthQueryAuthorityPromotionTarget,
) -> Result<(), WorthQueryAuthorityPromotionDenial> {
    if row.authority_category() != WorthQueryAuthorityCategory::Authoritative {
        return Err(WorthQueryAuthorityPromotionDenial::new(row, target));
    }

    let domain_matches = match target {
        WorthQueryAuthorityPromotionTarget::TopologyTruth => {
            row.authority_domain() == WorthQueryAuthorityDomain::TopologyTruth
        }
        WorthQueryAuthorityPromotionTarget::SpatialWitnessTruth => {
            row.authority_domain() == WorthQueryAuthorityDomain::SpatialWitnessTruth
        }
        WorthQueryAuthorityPromotionTarget::SupportPin => {
            row.authority_domain() == WorthQueryAuthorityDomain::QuerySupport
        }
        WorthQueryAuthorityPromotionTarget::EvidenceReport => {
            row.authority_domain() == WorthQueryAuthorityDomain::QueryEvidence
        }
    };

    domain_matches
        .then_some(())
        .ok_or_else(|| WorthQueryAuthorityPromotionDenial::new(row, target))
}
