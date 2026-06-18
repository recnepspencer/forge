use crate::ForgeQueryEvidenceIdentity;

use super::evidence::{
    consumer_kit_embedded_source_identity, consumer_kit_reference_residue_identity,
    consumer_kit_residue_breakdown_identity,
};

const WORTH_AUTHORING_RS: &str =
    include_str!("../../../../../worth-kernel/src/construction/authoring.rs");
const WORTH_QUERY_SUPPORT_PINS_RS: &str =
    include_str!("../../../../../worth-kernel/src/construction/query_support_pins.rs");
const WORTH_QUERY_SUPPORT_PINS_JSON: &str =
    include_str!("../../../../../worth-kernel/src/construction/query_support_pins.json");
const WORTH_PHASE_FIVE_CLOSEOUT_RS: &str = include_str!(
    "../../../../../worth-kernel/src/construction/certification/phase_five_boundary_closeout_tests.rs"
);
const WORTH_ADOPTION_INVENTORY_RS: &str = include_str!(
    "../../../../../worth-kernel/src/construction/query_enforcement_adoption/adoption_inventory.rs"
);
const WORTH_RESIDUE_ASSERTIONS_RS: &str = include_str!(
    "../../../../../worth-kernel/src/construction/query_enforcement_adoption/residue_assertions.rs"
);
const WORTH_QUERY_ENFORCEMENT_ADOPTION_TEST_RS: &str = include_str!(
    "../../../../../worth-kernel/src/construction/tests/query_enforcement_adoption.rs"
);
const WORTH_QUERY_EVIDENCE_REPORT_ADOPTION_TEST_RS: &str = include_str!(
    "../../../../../worth-kernel/src/construction/tests/query_evidence_report_adoption.rs"
);
const WORTH_SUPPORT_PINNING_ADOPTION_TEST_RS: &str =
    include_str!("../../../../../worth-kernel/src/construction/tests/support_pinning_adoption.rs");

const RESIDUE_SOURCE_PATHS: &[&str] = &[
    "crates/worth-kernel/src/construction/authoring.rs",
    "crates/worth-kernel/src/construction/query_support_pins.rs",
    "crates/worth-kernel/src/construction/query_support_pins.json",
    "crates/worth-kernel/src/construction/certification/phase_five_boundary_closeout_tests.rs",
    "crates/worth-kernel/src/construction/query_enforcement_adoption/adoption_inventory.rs",
    "crates/worth-kernel/src/construction/query_enforcement_adoption/residue_assertions.rs",
    "crates/worth-kernel/src/construction/tests/query_enforcement_adoption.rs",
    "crates/worth-kernel/src/construction/tests/query_evidence_report_adoption.rs",
    "crates/worth-kernel/src/construction/tests/support_pinning_adoption.rs",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitReferenceResidue {
    query_owned_residue_count: usize,
    defended_residue_count: usize,
    breakdown: ForgeQueryConsumerKitResidueBreakdown,
    backend_applicability: &'static str,
    backend_applicability_certified: bool,
    residue_source_digest: String,
    residue_identity: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitResidueBreakdown {
    report_digest_residue_count: usize,
    prohibition_audit_residue_count: usize,
    support_pinning_residue_count: usize,
    test_backend_residue_count: usize,
    defended_worth_domain_residue_count: usize,
    breakdown_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerKitResidueBreakdown {
    fn current() -> Self {
        Self::new(
            report_digest_residue_count(),
            prohibition_audit_residue_count(),
            support_pinning_residue_count(),
            test_backend_residue_count(),
            defended_residue_count(),
        )
    }

    #[cfg(test)]
    pub(crate) fn new(
        report_digest_residue_count: usize,
        prohibition_audit_residue_count: usize,
        support_pinning_residue_count: usize,
        test_backend_residue_count: usize,
        defended_worth_domain_residue_count: usize,
    ) -> Self {
        let breakdown_identity = consumer_kit_residue_breakdown_identity(
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
        );
        Self {
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
            breakdown_identity,
        }
    }

    #[cfg(not(test))]
    fn new(
        report_digest_residue_count: usize,
        prohibition_audit_residue_count: usize,
        support_pinning_residue_count: usize,
        test_backend_residue_count: usize,
        defended_worth_domain_residue_count: usize,
    ) -> Self {
        let breakdown_identity = consumer_kit_residue_breakdown_identity(
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
        );
        Self {
            report_digest_residue_count,
            prohibition_audit_residue_count,
            support_pinning_residue_count,
            test_backend_residue_count,
            defended_worth_domain_residue_count,
            breakdown_identity,
        }
    }

    pub fn report_digest_residue_count(&self) -> usize {
        self.report_digest_residue_count
    }

    pub fn prohibition_audit_residue_count(&self) -> usize {
        self.prohibition_audit_residue_count
    }

    pub fn support_pinning_residue_count(&self) -> usize {
        self.support_pinning_residue_count
    }

    pub fn test_backend_residue_count(&self) -> usize {
        self.test_backend_residue_count
    }

    pub fn defended_worth_domain_residue_count(&self) -> usize {
        self.defended_worth_domain_residue_count
    }

    pub fn query_owned_residue_count(&self) -> usize {
        self.report_digest_residue_count
            + self.prohibition_audit_residue_count
            + self.support_pinning_residue_count
            + self.test_backend_residue_count
    }

    pub fn breakdown_digest(&self) -> &str {
        self.breakdown_identity.as_str()
    }
}

impl ForgeQueryConsumerKitReferenceResidue {
    pub(crate) fn current() -> Self {
        Self::from_embedded_reference_evidence()
    }

    #[cfg(test)]
    pub(crate) fn new(
        query_owned_residue_count: usize,
        defended_residue_count: usize,
        backend_applicability: &'static str,
    ) -> Self {
        Self::new_with_certification(
            query_owned_residue_count,
            defended_residue_count,
            ForgeQueryConsumerKitResidueBreakdown::new(
                query_owned_residue_count,
                0,
                0,
                0,
                defended_residue_count,
            ),
            backend_applicability,
            backend_applicability.contains("zero hand-implemented Query runtime adapters"),
            "manual-reference-residue-sabotage".to_owned(),
        )
    }

    fn from_embedded_reference_evidence() -> Self {
        Self::new_with_certification(
            current_residue_breakdown().query_owned_residue_count(),
            defended_residue_count(),
            current_residue_breakdown(),
            backend_applicability(),
            backend_applicability_is_certified(),
            residue_source_digest(),
        )
    }

    fn new_with_certification(
        query_owned_residue_count: usize,
        defended_residue_count: usize,
        breakdown: ForgeQueryConsumerKitResidueBreakdown,
        backend_applicability: &'static str,
        backend_applicability_certified: bool,
        residue_source_digest: String,
    ) -> Self {
        let residue_identity = consumer_kit_reference_residue_identity(
            query_owned_residue_count,
            defended_residue_count,
            &breakdown,
            backend_applicability,
            backend_applicability_certified,
            &residue_source_digest,
        );
        Self {
            query_owned_residue_count,
            defended_residue_count,
            breakdown,
            backend_applicability,
            backend_applicability_certified,
            residue_source_digest,
            residue_identity,
        }
    }

    pub fn query_owned_residue_count(&self) -> usize {
        self.query_owned_residue_count
    }

    pub fn defended_residue_count(&self) -> usize {
        self.defended_residue_count
    }

    pub fn breakdown(&self) -> &ForgeQueryConsumerKitResidueBreakdown {
        &self.breakdown
    }

    pub fn backend_applicability(&self) -> &'static str {
        self.backend_applicability
    }

    pub fn backend_applicability_certified(&self) -> bool {
        self.backend_applicability_certified
    }

    pub fn residue_source_digest(&self) -> &str {
        &self.residue_source_digest
    }

    pub fn is_query_owned_clean(&self) -> bool {
        self.query_owned_residue_count == 0 && self.backend_applicability_certified
    }

    pub fn residue_digest(&self) -> &str {
        self.residue_identity.as_str()
    }

    pub fn residue_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.residue_identity
    }
}

fn current_residue_breakdown() -> ForgeQueryConsumerKitResidueBreakdown {
    ForgeQueryConsumerKitResidueBreakdown::current()
}

fn report_digest_residue_count() -> usize {
    0
}

fn prohibition_audit_residue_count() -> usize {
    phase_five_closeout_residue_violations()
}

fn support_pinning_residue_count() -> usize {
    authoring_residue_violations()
}

fn test_backend_residue_count() -> usize {
    usize::from(!backend_applicability_is_certified())
}

fn authoring_residue_violations() -> usize {
    [
        "REQUIRED_QUERY_FAMILIES",
        "REPORTED_QUERY_FAMILIES",
        "PrimitiveConstructionQueryGapRow",
        "support_pinning_contract(\"worth-kernel\")",
    ]
    .iter()
    .filter(|pattern| WORTH_AUTHORING_RS.contains(**pattern))
    .count()
}

fn phase_five_closeout_residue_violations() -> usize {
    [
        "FORBIDDEN_RUNTIME_PATTERNS",
        "query_runtime_violation_count",
    ]
    .iter()
    .filter(|pattern| WORTH_PHASE_FIVE_CLOSEOUT_RS.contains(**pattern))
    .count()
}

fn defended_residue_count() -> usize {
    [
        "phase-eight kernel-minimization topology hygiene",
        "phase-five construction-boundary legacy-deletion hygiene",
        "phase-five boundary pattern inventory hygiene",
    ]
    .iter()
    .filter(|label| WORTH_RESIDUE_ASSERTIONS_RS.contains(**label))
    .count()
}

fn backend_applicability() -> &'static str {
    if backend_applicability_is_certified() {
        return "worth-kernel construction publishes zero hand-implemented Query runtime adapters and zero hand-fabricated mutation receipts";
    }
    "worth-kernel construction backend applicability evidence is missing"
}

fn backend_applicability_is_certified() -> bool {
    WORTH_ADOPTION_INVENTORY_RS.contains("NotApplicableNoHandAssemblyResidue")
        && WORTH_RESIDUE_ASSERTIONS_RS.contains("assert_no_hand_assembled_test_backend_residue")
        && WORTH_QUERY_ENFORCEMENT_ADOPTION_TEST_RS.contains("NotApplicableNoHandAssemblyResidue")
}

fn residue_source_digest() -> String {
    consumer_kit_embedded_source_identity(
        "reference-consumer-residue",
        RESIDUE_SOURCE_PATHS.iter().copied(),
        [
            WORTH_AUTHORING_RS,
            WORTH_QUERY_SUPPORT_PINS_RS,
            WORTH_QUERY_SUPPORT_PINS_JSON,
            WORTH_PHASE_FIVE_CLOSEOUT_RS,
            WORTH_ADOPTION_INVENTORY_RS,
            WORTH_RESIDUE_ASSERTIONS_RS,
            WORTH_QUERY_ENFORCEMENT_ADOPTION_TEST_RS,
            WORTH_QUERY_EVIDENCE_REPORT_ADOPTION_TEST_RS,
            WORTH_SUPPORT_PINNING_ADOPTION_TEST_RS,
        ],
    )
    .as_str()
    .to_owned()
}
