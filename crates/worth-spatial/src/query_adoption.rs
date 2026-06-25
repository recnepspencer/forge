mod consumer_kit;
mod performance_counters;
mod residue;
mod support_projection;

use crate::workload_platform::evidence_ledger::{
    SpatialEvidenceSurfaceDeletionAction, SpatialEvidenceSurfaceOwner,
};
pub use consumer_kit::{
    current_spatial_query_consumer_kit_adoption_status,
    spatial_query_graph_obligation_adoption_proof,
    spatial_query_graph_obligation_adoption_proof_for_descriptor,
    spatial_query_graph_obligation_residue_manifest, WorthSpatialQueryConsumerKitAdoptionError,
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
    exported_facade_path: &'static str,
    responsibility: &'static str,
    classification: WorthSpatialQueryAdoptionClassification,
    authority_category: WorthSpatialQueryAuthorityCategory,
    authority_domain: WorthSpatialQueryAuthorityDomain,
    forbidden_pattern: Option<WorthSpatialQueryAdoptionForbiddenPattern>,
    replacement_surface: &'static str,
    current_caller: &'static str,
    deletion_action: SpatialEvidenceSurfaceDeletionAction,
    owner: SpatialEvidenceSurfaceOwner,
    cap: &'static str,
    removal_trigger: &'static str,
}

impl WorthSpatialQueryAdoptionInventoryRow {
    pub const fn new(
        source_set: &'static str,
        exported_facade_path: &'static str,
        responsibility: &'static str,
        classification: WorthSpatialQueryAdoptionClassification,
        authority_category: WorthSpatialQueryAuthorityCategory,
        authority_domain: WorthSpatialQueryAuthorityDomain,
        forbidden_pattern: Option<WorthSpatialQueryAdoptionForbiddenPattern>,
        replacement_surface: &'static str,
        current_caller: &'static str,
        deletion_action: SpatialEvidenceSurfaceDeletionAction,
        owner: SpatialEvidenceSurfaceOwner,
        cap: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            source_set,
            exported_facade_path,
            responsibility,
            classification,
            authority_category,
            authority_domain,
            forbidden_pattern,
            replacement_surface,
            current_caller,
            deletion_action,
            owner,
            cap,
            removal_trigger,
        }
    }

    pub const fn source_set(&self) -> &'static str {
        self.source_set
    }

    pub const fn exported_facade_path(&self) -> &'static str {
        self.exported_facade_path
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

    pub const fn current_caller(&self) -> &'static str {
        self.current_caller
    }

    pub const fn deletion_action(&self) -> SpatialEvidenceSurfaceDeletionAction {
        self.deletion_action
    }

    pub const fn owner(&self) -> SpatialEvidenceSurfaceOwner {
        self.owner
    }

    pub const fn cap(&self) -> &'static str {
        self.cap
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

pub fn spatial_query_adoption_inventory() -> Vec<WorthSpatialQueryAdoptionInventoryRow> {
    use SpatialEvidenceSurfaceDeletionAction::{
        CappedResidue, CertificationOnly as CertificationOnlyAction,
        CollapseToQueryConsumerKitProof,
    };
    use SpatialEvidenceSurfaceOwner::WorthSpatial;
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

    const FACADE_PATH: &str = "worth_spatial::facade::query_adoption";
    const DOWNSTREAM_CALLER: &str = "downstream workload query adoption inventory report";
    const QUERY_TRIGGER: &str =
        "Phase 8 Query consumer-kit adoption replaces local query adoption inventory rows.";
    const INVENTORY_CAP: &str =
        "Query adoption inventory rows cannot construct or satisfy spatial evidence authority.";

    vec![
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/workload_platform",
            FACADE_PATH,
            "Spatial workload evidence vocabulary and workload receipts.",
            Production,
            Authoritative,
            SpatialEvidence,
            Some(ForgedEvidenceRow),
            "crates/worth-spatial/src/query_adoption/consumer_kit.rs::spatial_query_graph_obligation_adoption_proof",
            DOWNSTREAM_CALLER,
            CollapseToQueryConsumerKitProof,
            WorthSpatial,
            INVENTORY_CAP,
            QUERY_TRIGGER,
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/witness_resolution",
            FACADE_PATH,
            "Witness admission and resolution before spatial proof consumption.",
            Production,
            Authoritative,
            SpatialWitnessTruth,
            Some(TestFixtureTruthPromotion),
            "crates/worth-spatial/src/query_adoption/consumer_kit.rs::spatial_query_graph_obligation_adoption_proof",
            DOWNSTREAM_CALLER,
            CollapseToQueryConsumerKitProof,
            WorthSpatial,
            INVENTORY_CAP,
            QUERY_TRIGGER,
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/certification/public_facade_contracts",
            FACADE_PATH,
            "Public facade certification and compile-fail proof fixtures.",
            CertificationOnly,
            CertificationAuthority,
            CertificationProof,
            Some(TestFixtureTruthPromotion),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
            DOWNSTREAM_CALLER,
            CertificationOnlyAction,
            WorthSpatial,
            "Certification contracts cannot construct ordinary spatial evidence authority.",
            "Certification-only rows are removed when public facade contracts migrate to Query proof fixtures.",
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/test_support",
            FACADE_PATH,
            "Spatial-only fixture scaffolding for crate-local tests.",
            TestSupport,
            TestSupportOnly,
            TestSupportDomain,
            Some(DirectSupportPostureAssumption),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
            DOWNSTREAM_CALLER,
            CappedResidue,
            WorthSpatial,
            "Test support is crate-local residue and cannot satisfy public authority APIs.",
            "Delete when spatial touch authority has first-party deterministic fixtures.",
        ),
        WorthSpatialQueryAdoptionInventoryRow::new(
            "crates/worth-spatial/src/workload_platform/vocabulary",
            FACADE_PATH,
            "Stage evidence ledger vocabulary still awaiting Query report pinning.",
            ExplicitResidue,
            Diagnostic,
            DiagnosticResidue,
            Some(ForgedEvidenceRow),
            SPATIAL_QUERY_ADOPTION_RESIDUE_SURFACE,
            DOWNSTREAM_CALLER,
            CappedResidue,
            WorthSpatial,
            "Diagnostic vocabulary residue is capped to reports and cannot mint authority.",
            "Collapse when workload vocabulary diagnostics are Query-pinned.",
        ),
    ]
}
