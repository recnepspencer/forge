mod boundary_audit;
mod consumer_kit;
mod evidence_reports;
mod performance_counters;
mod residue;
mod runtime_support;

pub use consumer_kit::{
    current_topology_query_consumer_kit_adoption_status, WorthTopoQueryConsumerKitAdoptionError,
    WorthTopoQueryConsumerKitAdoptionStatus,
};
pub use performance_counters::{
    current_topology_phase_eight_performance_counters, WorthTopoPhaseEightPerformanceCounters,
};
use residue::TOPOLOGY_QUERY_ADOPTION_RESIDUE_SURFACE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopoQueryAdoptionClassification {
    Production,
    TestSupport,
    CertificationOnly,
    ExplicitResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopoQueryAdoptionForbiddenPattern {
    SyntheticReceipt,
    ForgedEvidenceRow,
    DirectSupportPostureAssumption,
    LowerAuthorityIdentityReconstruction,
    TestFixtureTruthPromotion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopoQueryAuthorityCategory {
    Authoritative,
    Diagnostic,
    CertificationOnly,
    TestSupportOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTopoQueryAuthorityDomain {
    TopologyTruth,
    CertificationProof,
    TestSupport,
    DiagnosticResidue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthTopoQueryAdoptionInventoryRow {
    source_set: &'static str,
    responsibility: &'static str,
    classification: WorthTopoQueryAdoptionClassification,
    authority_category: WorthTopoQueryAuthorityCategory,
    authority_domain: WorthTopoQueryAuthorityDomain,
    forbidden_pattern: Option<WorthTopoQueryAdoptionForbiddenPattern>,
    replacement_surface: &'static str,
}

impl WorthTopoQueryAdoptionInventoryRow {
    pub const fn new(
        source_set: &'static str,
        responsibility: &'static str,
        classification: WorthTopoQueryAdoptionClassification,
        authority_category: WorthTopoQueryAuthorityCategory,
        authority_domain: WorthTopoQueryAuthorityDomain,
        forbidden_pattern: Option<WorthTopoQueryAdoptionForbiddenPattern>,
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

    pub const fn classification(&self) -> WorthTopoQueryAdoptionClassification {
        self.classification
    }

    pub const fn authority_category(&self) -> WorthTopoQueryAuthorityCategory {
        self.authority_category
    }

    pub const fn authority_domain(&self) -> WorthTopoQueryAuthorityDomain {
        self.authority_domain
    }

    pub const fn forbidden_pattern(&self) -> Option<WorthTopoQueryAdoptionForbiddenPattern> {
        self.forbidden_pattern
    }

    pub const fn replacement_surface(&self) -> &'static str {
        self.replacement_surface
    }
}

pub fn topology_query_adoption_inventory() -> Vec<WorthTopoQueryAdoptionInventoryRow> {
    use WorthTopoQueryAdoptionClassification::{
        CertificationOnly, ExplicitResidue, Production, TestSupport,
    };
    use WorthTopoQueryAdoptionForbiddenPattern::{
        DirectSupportPostureAssumption, SyntheticReceipt, TestFixtureTruthPromotion,
    };
    use WorthTopoQueryAuthorityCategory::{
        Authoritative, CertificationOnly as CertificationAuthority, Diagnostic, TestSupportOnly,
    };
    use WorthTopoQueryAuthorityDomain::{
        CertificationProof, DiagnosticResidue, TestSupport as TestSupportDomain, TopologyTruth,
    };

    vec![
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/src/projection/runtime_boundary",
            "Query runtime bridge, read lowering, execution, and mutation receipts.",
            Production,
            Authoritative,
            TopologyTruth,
            None,
            "crates/worth-topo/src/query_adoption/runtime_support.rs",
        ),
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/src/projection/runtime_boundary/read_lowering/relationship_proof.rs",
            "Runtime read lowering defers relationship-proof authority until Query read graph admission.",
            Production,
            Authoritative,
            TopologyTruth,
            None,
            "crates/worth-topo/src/query_adoption/runtime_support.rs",
        ),
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/src/workload_platform",
            "Topology workload declarations, seed receipts, and NMT topology construction.",
            Production,
            Authoritative,
            TopologyTruth,
            Some(DirectSupportPostureAssumption),
            "crates/worth-topo/src/query_adoption/evidence_reports.rs",
        ),
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/src/test_support",
            "Topology-only fixtures for primitive and schema topology authoring.",
            TestSupport,
            TestSupportOnly,
            TestSupportDomain,
            Some(TestFixtureTruthPromotion),
            TOPOLOGY_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/tests/ui",
            "Compile-fail protection around runtime boundary authority.",
            CertificationOnly,
            CertificationAuthority,
            CertificationProof,
            Some(SyntheticReceipt),
            "crates/worth-topo/src/query_adoption/boundary_audit.rs",
        ),
        WorthTopoQueryAdoptionInventoryRow::new(
            "crates/worth-topo/src/projection/runtime_boundary/query_support",
            "Runtime support reporting that still predates crate-level Query adoption pins.",
            ExplicitResidue,
            Diagnostic,
            DiagnosticResidue,
            Some(DirectSupportPostureAssumption),
            TOPOLOGY_QUERY_ADOPTION_RESIDUE_SURFACE,
        ),
    ]
}
