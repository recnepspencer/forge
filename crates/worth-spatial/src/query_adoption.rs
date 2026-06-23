mod boundary_audit;
mod consumer_kit;
mod evidence_reports;
mod performance_counters;
mod residue;
mod support_projection;

pub use consumer_kit::{
    current_spatial_query_consumer_kit_adoption_status, WorthSpatialQueryConsumerKitAdoptionError,
    WorthSpatialQueryConsumerKitAdoptionStatus,
};
pub use performance_counters::{
    current_spatial_phase_eight_performance_counters, WorthSpatialPhaseEightPerformanceCounters,
};
use residue::SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE;
pub use support_projection::{
    current_spatial_workload_support_pin_rows, WorthSpatialWorkloadSupportFamily,
    WorthSpatialWorkloadSupportPinRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthSpatialQueryAdoptionClassification {
    Production,
    TestSupport,
    CertificationOnly,
    ExplicitResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthSpatialQueryAdoptionForbiddenPattern {
    SyntheticReceipt,
    ForgedEvidenceRow,
    DirectSupportPostureAssumption,
    LowerAuthorityIdentityReconstruction,
    TestFixtureTruthPromotion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthSpatialQueryAuthorityCategory {
    Authoritative,
    Derived,
    Diagnostic,
    CertificationOnly,
    TestSupportOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthSpatialQueryAuthorityDomain {
    SpatialWitnessTruth,
    SpatialEvidence,
    CertificationProof,
    TestSupport,
    DiagnosticResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthSpatialQueryAdoptionInventoryRow {
    source_set: &'static str,
    responsibility: &'static str,
    classification: WorthSpatialQueryAdoptionClassification,
    authority_category: WorthSpatialQueryAuthorityCategory,
    authority_domain: WorthSpatialQueryAuthorityDomain,
    forbidden_pattern: Option<WorthSpatialQueryAdoptionForbiddenPattern>,
    replacement_surface: &'static str,
}

impl WorthSpatialQueryAdoptionInventoryRow {
    pub const fn new(
        source_set: &'static str,
        responsibility: &'static str,
        classification: WorthSpatialQueryAdoptionClassification,
        authority_category: WorthSpatialQueryAuthorityCategory,
        authority_domain: WorthSpatialQueryAuthorityDomain,
        forbidden_pattern: Option<WorthSpatialQueryAdoptionForbiddenPattern>,
        replacement_surface: &'static str,
    ) -> Self {
        Self {
            source_set,
            responsibility,
            classification,
            authority_category,
            authority_domain,
            forbidden_pattern,
            replacement_surface,
        }
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn responsibility(&self) -> &'static str {
        self.responsibility
    }

    pub const fn classification(&self) -> WorthSpatialQueryAdoptionClassification {
        self.classification
    }

    pub const fn authority_category(&self) -> WorthSpatialQueryAuthorityCategory {
        self.authority_category
    }

    pub const fn authority_domain(&self) -> WorthSpatialQueryAuthorityDomain {
        self.authority_domain
    }

    pub const fn forbidden_pattern(&self) -> Option<WorthSpatialQueryAdoptionForbiddenPattern> {
        self.forbidden_pattern
    }

    pub const fn replacement_surface(&self) -> &'static str {
        self.replacement_surface
    }
}

pub fn spatial_query_adoption_inventory() -> Vec<WorthSpatialQueryAdoptionInventoryRow> {
    use WorthSpatialQueryAdoptionClassification::{
        CertificationOnly, ExplicitResidue, Production, TestSupport,
    };
    use WorthSpatialQueryAdoptionForbiddenPattern::{
        DirectSupportPostureAssumption, ForgedEvidenceRow, TestFixtureTruthPromotion,
    };
    use WorthSpatialQueryAuthorityCategory::{
        Authoritative, CertificationOnly as CertificationAuthority, Diagnostic, TestSupportOnly,
    };
    use WorthSpatialQueryAuthorityDomain::{
        CertificationProof, DiagnosticResidue, SpatialEvidence, SpatialWitnessTruth,
        TestSupport as TestSupportDomain,
    };

    vec![
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/workload_platform",
            "Spatial workload evidence vocabulary and workload receipts.",
            Production,
            Authoritative,
            SpatialEvidence,
            Some(ForgedEvidenceRow),
            "crates/worth-spatial/src/query_adoption/evidence_reports.rs",
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/witness_resolution",
            "Witness admission and resolution before spatial proof consumption.",
            Production,
            Authoritative,
            SpatialWitnessTruth,
            Some(TestFixtureTruthPromotion),
            "crates/worth-spatial/src/query_adoption/boundary_audit.rs",
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/certification/public_facade_contracts",
            "Public facade certification and compile-fail proof fixtures.",
            CertificationOnly,
            CertificationAuthority,
            CertificationProof,
            Some(TestFixtureTruthPromotion),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/test_support",
            "Spatial-only fixture scaffolding for crate-local tests.",
            TestSupport,
            TestSupportOnly,
            TestSupportDomain,
            Some(DirectSupportPostureAssumption),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/workload_platform/vocabulary",
            "Stage evidence ledger vocabulary still awaiting Query report pinning.",
            ExplicitResidue,
            Diagnostic,
            DiagnosticResidue,
            Some(ForgedEvidenceRow),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
    ]
}
