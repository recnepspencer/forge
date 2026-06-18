use topology::facade::{
    topology_query_adoption_inventory, WorthTopoQueryAdoptionClassification,
    WorthTopoQueryAdoptionForbiddenPattern, WorthTopoQueryAdoptionInventoryRow,
    WorthTopoQueryAuthorityCategory, WorthTopoQueryAuthorityDomain,
};
use worth_spatial::facade::query_adoption::{
    spatial_query_adoption_inventory, WorthSpatialQueryAdoptionClassification,
    WorthSpatialQueryAdoptionForbiddenPattern, WorthSpatialQueryAdoptionInventoryRow,
    WorthSpatialQueryAuthorityCategory, WorthSpatialQueryAuthorityDomain,
};

use super::classification::{
    WorthQueryAdoptionClassification, WorthQueryAdoptionForbiddenPattern,
    WorthQueryAdoptionInventoryOwner, WorthQueryAdoptionInventoryRow, WorthQueryAuthorityCategory,
    WorthQueryAuthorityDomain,
};
use super::residue::{
    KERNEL_PUBLIC_FACADE_CERTIFICATION_RESIDUE_SURFACE, KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE,
};

pub(super) fn cross_crate_inventory_rows() -> Vec<WorthQueryAdoptionInventoryRow> {
    let mut rows = kernel_inventory_rows();
    rows.extend(
        spatial_query_adoption_inventory()
            .into_iter()
            .map(spatial_row),
    );
    rows.extend(
        topology_query_adoption_inventory()
            .into_iter()
            .map(topology_row),
    );
    rows.push(forge_query_support_row());
    rows.push(forge_query_evidence_row());
    rows
}

fn kernel_inventory_rows() -> Vec<WorthQueryAdoptionInventoryRow> {
    use WorthQueryAdoptionClassification::{CertificationOnly, ExplicitResidue, Production};
    use WorthQueryAdoptionForbiddenPattern::{
        ForgedEvidenceRow, SyntheticReceipt, TestFixtureTruthPromotion,
    };
    use WorthQueryAdoptionInventoryOwner::Kernel;
    use WorthQueryAuthorityCategory::{
        CertificationOnly as CertificationAuthority, Derived, Diagnostic,
    };
    use WorthQueryAuthorityDomain::{CertificationProof, DiagnosticResidue, KernelOrchestration};

    vec![
        WorthQueryAdoptionInventoryRow::new(
            Kernel,
            "crates/worth-kernel/src/workload_composition",
            "Kernel workload orchestration over topology and spatial receipts.",
            Production,
            Derived,
            KernelOrchestration,
            Some(ForgedEvidenceRow),
            "crates/worth-kernel/src/query_adoption/evidence_reports.rs",
        ),
        WorthQueryAdoptionInventoryRow::new(
            Kernel,
            "crates/worth-kernel/src/workload_composition/workload_catalog",
            "Catalog construction and recipe support for reusable workload families.",
            Production,
            Derived,
            KernelOrchestration,
            Some(TestFixtureTruthPromotion),
            "crates/worth-kernel/src/query_adoption/support_pins.rs",
        ),
        WorthQueryAdoptionInventoryRow::new(
            Kernel,
            "crates/worth-kernel/src/certification/public_facade_contracts",
            "Kernel public facade certification and compile-fail proof.",
            CertificationOnly,
            CertificationAuthority,
            CertificationProof,
            Some(SyntheticReceipt),
            KERNEL_PUBLIC_FACADE_CERTIFICATION_RESIDUE_SURFACE,
        ),
        WorthQueryAdoptionInventoryRow::new(
            Kernel,
            "crates/worth-kernel/src/binding/tests",
            "Kernel binding tests that consume cross-crate workload evidence.",
            ExplicitResidue,
            Diagnostic,
            DiagnosticResidue,
            Some(ForgedEvidenceRow),
            KERNEL_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
    ]
}

fn forge_query_support_row() -> WorthQueryAdoptionInventoryRow {
    WorthQueryAdoptionInventoryRow::new(
        WorthQueryAdoptionInventoryOwner::ForgeQuery,
        "crates/forge-query/src/consumer_kit/support_pinning",
        "Query-owned support snapshots and support pin evaluation.",
        WorthQueryAdoptionClassification::Production,
        WorthQueryAuthorityCategory::Authoritative,
        WorthQueryAuthorityDomain::QuerySupport,
        None,
        "crates/forge-query/src/consumer_kit/support_pinning",
    )
}

fn forge_query_evidence_row() -> WorthQueryAdoptionInventoryRow {
    WorthQueryAdoptionInventoryRow::new(
        WorthQueryAdoptionInventoryOwner::ForgeQuery,
        "crates/forge-query/src/consumer_kit/evidence_report",
        "Query-owned evidence reports and hard-prohibition evidence identity.",
        WorthQueryAdoptionClassification::Production,
        WorthQueryAuthorityCategory::Authoritative,
        WorthQueryAuthorityDomain::QueryEvidence,
        None,
        "crates/forge-query/src/consumer_kit/evidence_report",
    )
}

fn spatial_row(row: WorthSpatialQueryAdoptionInventoryRow) -> WorthQueryAdoptionInventoryRow {
    WorthQueryAdoptionInventoryRow::new(
        WorthQueryAdoptionInventoryOwner::Spatial,
        row.source_set(),
        row.responsibility(),
        spatial_classification(row.classification()),
        spatial_authority_category(row.authority_category()),
        spatial_authority_domain(row.authority_domain()),
        row.forbidden_pattern().map(spatial_pattern),
        row.replacement_surface(),
    )
}

fn spatial_classification(
    classification: WorthSpatialQueryAdoptionClassification,
) -> WorthQueryAdoptionClassification {
    match classification {
        WorthSpatialQueryAdoptionClassification::Production => {
            WorthQueryAdoptionClassification::Production
        }
        WorthSpatialQueryAdoptionClassification::TestSupport => {
            WorthQueryAdoptionClassification::TestSupport
        }
        WorthSpatialQueryAdoptionClassification::CertificationOnly => {
            WorthQueryAdoptionClassification::CertificationOnly
        }
        WorthSpatialQueryAdoptionClassification::ExplicitResidue => {
            WorthQueryAdoptionClassification::ExplicitResidue
        }
    }
}

fn topology_row(row: WorthTopoQueryAdoptionInventoryRow) -> WorthQueryAdoptionInventoryRow {
    WorthQueryAdoptionInventoryRow::new(
        WorthQueryAdoptionInventoryOwner::Topology,
        row.source_set(),
        row.responsibility(),
        topology_classification(row.classification()),
        topology_authority_category(row.authority_category()),
        topology_authority_domain(row.authority_domain()),
        row.forbidden_pattern().map(topology_pattern),
        row.replacement_surface(),
    )
}

fn topology_classification(
    classification: WorthTopoQueryAdoptionClassification,
) -> WorthQueryAdoptionClassification {
    match classification {
        WorthTopoQueryAdoptionClassification::Production => {
            WorthQueryAdoptionClassification::Production
        }
        WorthTopoQueryAdoptionClassification::TestSupport => {
            WorthQueryAdoptionClassification::TestSupport
        }
        WorthTopoQueryAdoptionClassification::CertificationOnly => {
            WorthQueryAdoptionClassification::CertificationOnly
        }
        WorthTopoQueryAdoptionClassification::ExplicitResidue => {
            WorthQueryAdoptionClassification::ExplicitResidue
        }
    }
}

fn spatial_authority_category(
    category: WorthSpatialQueryAuthorityCategory,
) -> WorthQueryAuthorityCategory {
    match category {
        WorthSpatialQueryAuthorityCategory::Authoritative => {
            WorthQueryAuthorityCategory::Authoritative
        }
        WorthSpatialQueryAuthorityCategory::Derived => WorthQueryAuthorityCategory::Derived,
        WorthSpatialQueryAuthorityCategory::Diagnostic => WorthQueryAuthorityCategory::Diagnostic,
        WorthSpatialQueryAuthorityCategory::CertificationOnly => {
            WorthQueryAuthorityCategory::CertificationOnly
        }
        WorthSpatialQueryAuthorityCategory::TestSupportOnly => {
            WorthQueryAuthorityCategory::TestSupportOnly
        }
    }
}

fn spatial_authority_domain(domain: WorthSpatialQueryAuthorityDomain) -> WorthQueryAuthorityDomain {
    match domain {
        WorthSpatialQueryAuthorityDomain::SpatialWitnessTruth => {
            WorthQueryAuthorityDomain::SpatialWitnessTruth
        }
        WorthSpatialQueryAuthorityDomain::SpatialEvidence => {
            WorthQueryAuthorityDomain::SpatialEvidence
        }
        WorthSpatialQueryAuthorityDomain::CertificationProof => {
            WorthQueryAuthorityDomain::CertificationProof
        }
        WorthSpatialQueryAuthorityDomain::TestSupport => WorthQueryAuthorityDomain::TestSupport,
        WorthSpatialQueryAuthorityDomain::DiagnosticResidue => {
            WorthQueryAuthorityDomain::DiagnosticResidue
        }
    }
}

fn topology_authority_category(
    category: WorthTopoQueryAuthorityCategory,
) -> WorthQueryAuthorityCategory {
    match category {
        WorthTopoQueryAuthorityCategory::Authoritative => {
            WorthQueryAuthorityCategory::Authoritative
        }
        WorthTopoQueryAuthorityCategory::Diagnostic => WorthQueryAuthorityCategory::Diagnostic,
        WorthTopoQueryAuthorityCategory::CertificationOnly => {
            WorthQueryAuthorityCategory::CertificationOnly
        }
        WorthTopoQueryAuthorityCategory::TestSupportOnly => {
            WorthQueryAuthorityCategory::TestSupportOnly
        }
    }
}

fn topology_authority_domain(domain: WorthTopoQueryAuthorityDomain) -> WorthQueryAuthorityDomain {
    match domain {
        WorthTopoQueryAuthorityDomain::TopologyTruth => WorthQueryAuthorityDomain::TopologyTruth,
        WorthTopoQueryAuthorityDomain::CertificationProof => {
            WorthQueryAuthorityDomain::CertificationProof
        }
        WorthTopoQueryAuthorityDomain::TestSupport => WorthQueryAuthorityDomain::TestSupport,
        WorthTopoQueryAuthorityDomain::DiagnosticResidue => {
            WorthQueryAuthorityDomain::DiagnosticResidue
        }
    }
}

fn spatial_pattern(
    pattern: WorthSpatialQueryAdoptionForbiddenPattern,
) -> WorthQueryAdoptionForbiddenPattern {
    match pattern {
        WorthSpatialQueryAdoptionForbiddenPattern::SyntheticReceipt => {
            WorthQueryAdoptionForbiddenPattern::SyntheticReceipt
        }
        WorthSpatialQueryAdoptionForbiddenPattern::ForgedEvidenceRow => {
            WorthQueryAdoptionForbiddenPattern::ForgedEvidenceRow
        }
        WorthSpatialQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption => {
            WorthQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption
        }
        WorthSpatialQueryAdoptionForbiddenPattern::LowerAuthorityIdentityReconstruction => {
            WorthQueryAdoptionForbiddenPattern::LowerAuthorityIdentityReconstruction
        }
        WorthSpatialQueryAdoptionForbiddenPattern::TestFixtureTruthPromotion => {
            WorthQueryAdoptionForbiddenPattern::TestFixtureTruthPromotion
        }
    }
}

fn topology_pattern(
    pattern: WorthTopoQueryAdoptionForbiddenPattern,
) -> WorthQueryAdoptionForbiddenPattern {
    match pattern {
        WorthTopoQueryAdoptionForbiddenPattern::SyntheticReceipt => {
            WorthQueryAdoptionForbiddenPattern::SyntheticReceipt
        }
        WorthTopoQueryAdoptionForbiddenPattern::ForgedEvidenceRow => {
            WorthQueryAdoptionForbiddenPattern::ForgedEvidenceRow
        }
        WorthTopoQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption => {
            WorthQueryAdoptionForbiddenPattern::DirectSupportPostureAssumption
        }
        WorthTopoQueryAdoptionForbiddenPattern::LowerAuthorityIdentityReconstruction => {
            WorthQueryAdoptionForbiddenPattern::LowerAuthorityIdentityReconstruction
        }
        WorthTopoQueryAdoptionForbiddenPattern::TestFixtureTruthPromotion => {
            WorthQueryAdoptionForbiddenPattern::TestFixtureTruthPromotion
        }
    }
}
