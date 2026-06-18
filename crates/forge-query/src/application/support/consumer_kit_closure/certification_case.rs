use crate::ForgeQueryEvidenceIdentity;

use super::certification_gate::{
    certification_source_contains, certification_source_paths_for_family,
};
use super::evidence::consumer_kit_certification_case_identity;
use super::family::ForgeQueryConsumerKitFamilyName;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryConsumerKitCertificationTier {
    CompileFail,
    Integration,
    Adoption,
    Documentation,
}

impl ForgeQueryConsumerKitCertificationTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompileFail => "compile-fail",
            Self::Integration => "integration",
            Self::Adoption => "adoption",
            Self::Documentation => "documentation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerKitCertificationCase {
    family: ForgeQueryConsumerKitFamilyName,
    case_id: &'static str,
    requirement: &'static str,
    tier: ForgeQueryConsumerKitCertificationTier,
    required_signal: &'static str,
    evidence_source_paths: Vec<&'static str>,
    satisfied: bool,
    case_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryConsumerKitCertificationCase {
    fn derive(
        family: ForgeQueryConsumerKitFamilyName,
        case_id: &'static str,
        requirement: &'static str,
        tier: ForgeQueryConsumerKitCertificationTier,
        required_signal: &'static str,
    ) -> Self {
        let evidence_source_paths = certification_source_paths_for_family(family);
        let satisfied = !evidence_source_paths.is_empty()
            && certification_source_contains(family, required_signal);
        let case_identity = consumer_kit_certification_case_identity(
            family,
            case_id,
            tier.as_str(),
            requirement,
            required_signal,
            satisfied,
            &evidence_source_paths,
        );
        Self {
            family,
            case_id,
            requirement,
            tier,
            required_signal,
            evidence_source_paths,
            satisfied,
            case_identity,
        }
    }

    pub fn family(&self) -> ForgeQueryConsumerKitFamilyName {
        self.family
    }

    pub fn case_id(&self) -> &'static str {
        self.case_id
    }

    pub fn requirement(&self) -> &'static str {
        self.requirement
    }

    pub fn tier(&self) -> ForgeQueryConsumerKitCertificationTier {
        self.tier
    }

    pub fn required_signal(&self) -> &'static str {
        self.required_signal
    }

    pub fn evidence_source_paths(&self) -> &[&'static str] {
        &self.evidence_source_paths
    }

    pub fn satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn case_digest(&self) -> &str {
        self.case_identity.as_str()
    }
}

pub(super) fn required_consumer_kit_certification_cases(
) -> Vec<ForgeQueryConsumerKitCertificationCase> {
    use ForgeQueryConsumerKitCertificationTier::{Adoption, CompileFail, Integration};
    use ForgeQueryConsumerKitFamilyName::{
        BoundaryAudit, EvidenceReportKit, HardProhibitionRegistry, InMemoryTestBackend,
        ReferenceConsumerAdoption, SupportPinning, SupportSnapshot,
    };

    [
        (
            EvidenceReportKit,
            "evidence-report-compile-fail-boundary",
            "report misuse fails at compile time and adoption uses canonical evidence reports",
            CompileFail,
            "evidence_report_boundaries_are_compile_time_enforced",
        ),
        (
            HardProhibitionRegistry,
            "hard-prohibition-compile-fail-boundary",
            "hard prohibition seams are registered and consumer unreachable",
            CompileFail,
            "hard_prohibition_seams_are_not_consumer_reachable",
        ),
        (
            BoundaryAudit,
            "boundary-audit-seeded-bypass-detection",
            "shipped audit detects seeded bypasses through registry-owned rows",
            Integration,
            "detects_seeded_method_call_bypass_from_registry",
        ),
        (
            SupportSnapshot,
            "support-snapshot-live-matrix-equivalence",
            "snapshot remains a digest-bound projection of the live support matrix",
            Integration,
            "support_snapshot_matches_live_matrix_row_for_row_and_digest_for_digest",
        ),
        (
            SupportPinning,
            "support-pinning-drift-localization",
            "support pinning localizes row drift and blocks only pinned consumers",
            Integration,
            "drift_fails_only_consumers_pinned_to_regressed_row",
        ),
        (
            InMemoryTestBackend,
            "in-memory-test-backend-equivalence",
            "test backend proves covered behavior against a bridge-backed harness",
            Integration,
            "in_memory_test_backend_matches_bridge_harness_for_covered_live_write_path",
        ),
        (
            ReferenceConsumerAdoption,
            "reference-consumer-enforcement-adoption",
            "worth-kernel adopts Query-owned audit, pinning, and residue reports",
            Adoption,
            "support_pinning_and_adoption_inventory_are_query_owned_evidence",
        ),
    ]
    .into_iter()
    .map(|(family, case_id, requirement, tier, required_signal)| {
        ForgeQueryConsumerKitCertificationCase::derive(
            family,
            case_id,
            requirement,
            tier,
            required_signal,
        )
    })
    .collect()
}
