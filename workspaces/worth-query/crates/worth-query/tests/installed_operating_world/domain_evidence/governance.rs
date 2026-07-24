use worth_query::facade::{domain, runtime};

use super::super::installed_operation_fixture::EvidenceScenario;
use super::assertions::assert_mandatory_core;
use super::execution::{admitted_receipt, evidence, inspection};

#[test]
fn never_disclose_discards_malformed_optional_payload_before_sidecar_validation() {
    for (name, scenario) in [
        (
            "domain-evidence-never-disclose-malformed-records",
            EvidenceScenario::MalformedSidecars,
        ),
        (
            "domain-evidence-never-disclose-malformed-counter",
            EvidenceScenario::MalformedOptionalCounter,
        ),
        (
            "domain-evidence-never-disclose-malformed-transformation",
            EvidenceScenario::MalformedTransformationSidecar,
        ),
    ] {
        let receipt = admitted_receipt(
            name,
            scenario,
            domain::WorthQueryArtifactRedactionPosture::NeverDisclose,
        );
        let evidence = evidence(&receipt);

        assert_mandatory_core(evidence.core());
        assert!(matches!(
            evidence.counter_sidecar(),
            domain::WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        ));
        assert!(matches!(
            evidence.decision_sidecar(),
            domain::WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        ));
        assert!(matches!(
            evidence.candidate_sidecar(),
            domain::WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        ));
        assert!(matches!(
            evidence.transformation_sidecar(),
            domain::WorthQueryAdmittedDomainEvidenceSidecar::Omitted
        ));
    }
}

#[test]
fn governance_redaction_never_recovers_payload_and_preserves_policy() {
    for (name, redaction, expect_digest) in [
        (
            "domain-evidence-canonical-projection",
            domain::WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly,
            true,
        ),
        (
            "domain-evidence-never-disclose",
            domain::WorthQueryArtifactRedactionPosture::NeverDisclose,
            false,
        ),
        (
            "domain-evidence-domain-redactor-required",
            domain::WorthQueryArtifactRedactionPosture::DomainRedactorRequired,
            false,
        ),
    ] {
        let receipt = admitted_receipt(name, EvidenceScenario::Honest, redaction);
        let evidence = evidence(&receipt);
        assert_mandatory_core(evidence.core());
        assert_eq!(evidence.governance().redaction(), redaction);
        assert_eq!(evidence.counter_sidecar().digest().is_some(), expect_digest);
        assert_eq!(
            evidence.decision_sidecar().digest().is_some(),
            expect_digest
        );
        assert_eq!(
            evidence.candidate_sidecar().digest().is_some(),
            expect_digest
        );
        assert_eq!(
            evidence.transformation_sidecar().digest().is_some(),
            expect_digest
        );
        assert!(evidence.counter_sidecar().records().is_none());
        assert!(evidence.decision_sidecar().records().is_none());
        assert!(evidence.candidate_sidecar().records().is_none());
        assert!(evidence.transformation_sidecar().records().is_none());

        let inspection = inspection(
            evidence,
            runtime::CausalInspectionRedactionPolicy::PreserveDetail,
        );
        assert_eq!(inspection.governance(), evidence.governance());
        assert!(inspection.counter_sidecar().records().is_none());
        assert!(inspection.decision_sidecar().records().is_none());
        assert!(inspection.candidate_sidecar().records().is_none());
        assert!(inspection.transformation_sidecar().records().is_none());
        let bundle =
            worth_query::facade::certification::WorthQueryDomainEvidenceCertificationBundle::derive(
                &inspection,
            );
        assert_eq!(bundle.governance(), evidence.governance());
        assert_eq!(bundle.counter_sidecar().digest().is_some(), expect_digest);
        assert_eq!(bundle.decision_sidecar().digest().is_some(), expect_digest);
        assert_eq!(bundle.candidate_sidecar().digest().is_some(), expect_digest);
        assert_eq!(
            bundle.transformation_sidecar().digest().is_some(),
            expect_digest
        );
    }
}
