use std::collections::BTreeMap;

use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query::facade::{certification, domain, runtime};

use super::super::installed_operation_fixture::EvidenceScenario;
use super::execution::{admitted_receipt, evidence, inspection};

#[test]
fn honest_evidence_preserves_governed_core_and_narrows_sidecars_across_copies() {
    let receipt = admitted_receipt(
        "domain-evidence-honest",
        EvidenceScenario::Honest,
        domain::WorthQueryArtifactRedactionPosture::NotRequired,
    );
    let evidence = evidence(&receipt);

    assert_eq!(
        evidence.binding().binding_identity(),
        receipt.binding_identity()
    );
    assert_eq!(
        evidence.binding().output_occurrence_identity(),
        receipt.output_identity()
    );
    assert!(evidence.binding().run_identity().is_none());
    assert!(evidence.binding().stage_identity().is_none());
    assert!(!evidence.binding().operation_identity().is_empty());
    assert!(!evidence.binding().basis_identity().is_empty());
    assert!(!evidence.binding().execution_snapshot_identity().is_empty());
    assert_governance(evidence.governance());
    assert_mandatory_core(evidence.core());
    assert_eq!(
        evidence.authority_posture(),
        domain::WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    );
    assert_eq!(evidence.decision_sidecar().records().unwrap().len(), 1);
    assert_eq!(evidence.candidate_sidecar().records().unwrap().len(), 2);
    assert_eq!(
        evidence.transformation_sidecar().records().unwrap().len(),
        1
    );

    let preserved = inspection(
        evidence,
        runtime::CausalInspectionRedactionPolicy::PreserveDetail,
    );
    assert_eq!(preserved.core(), evidence.core());
    assert_eq!(preserved.governance(), evidence.governance());
    assert_eq!(preserved.decision_sidecar().records().unwrap().len(), 1);
    assert_eq!(preserved.candidate_sidecar().records().unwrap().len(), 2);
    assert_eq!(
        preserved.transformation_sidecar().records().unwrap().len(),
        1
    );
    assert_eq!(
        preserved.authority_posture(),
        domain::WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    );

    let digested = inspection(
        evidence,
        runtime::CausalInspectionRedactionPolicy::DigestOnly,
    );
    assert_eq!(digested.core(), evidence.core());
    assert_eq!(digested.governance(), evidence.governance());
    assert!(digested.decision_sidecar().records().is_none());
    assert!(digested.candidate_sidecar().records().is_none());
    assert!(digested.transformation_sidecar().records().is_none());
    assert_eq!(
        digested.decision_sidecar().digest(),
        evidence.decision_sidecar().digest()
    );
    assert_eq!(
        digested.candidate_sidecar().digest(),
        evidence.candidate_sidecar().digest()
    );
    assert_eq!(
        digested.transformation_sidecar().digest(),
        evidence.transformation_sidecar().digest()
    );

    let bundle = certification::WorthQueryDomainEvidenceCertificationBundle::derive(&preserved);
    assert_eq!(bundle.core(), evidence.core());
    assert_eq!(bundle.governance(), evidence.governance());
    assert_eq!(
        bundle.decision_sidecar().digest(),
        evidence.decision_sidecar().digest()
    );
    assert_eq!(
        bundle.candidate_sidecar().digest(),
        evidence.candidate_sidecar().digest()
    );
    assert_eq!(
        bundle.transformation_sidecar().digest(),
        evidence.transformation_sidecar().digest()
    );
    assert_eq!(
        bundle.authority_posture(),
        domain::WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    );
}

#[test]
fn provider_omission_preserves_mandatory_search_and_loss_meaning() {
    let receipt = admitted_receipt(
        "domain-evidence-provider-omission",
        EvidenceScenario::OmitSidecars,
        domain::WorthQueryArtifactRedactionPosture::NotRequired,
    );
    let evidence = evidence(&receipt);

    assert_mandatory_core(evidence.core());
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

    let inspection = inspection(
        evidence,
        runtime::CausalInspectionRedactionPolicy::PreserveDetail,
    );
    assert_mandatory_core(inspection.core());
    assert!(inspection.decision_sidecar().is_omitted());
    assert!(inspection.candidate_sidecar().is_omitted());
    assert!(inspection.transformation_sidecar().is_omitted());
    let bundle = certification::WorthQueryDomainEvidenceCertificationBundle::derive(&inspection);
    assert_mandatory_core(bundle.core());
    assert!(bundle.decision_sidecar().is_omitted());
    assert!(bundle.candidate_sidecar().is_omitted());
    assert!(bundle.transformation_sidecar().is_omitted());
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
    ] {
        let receipt = admitted_receipt(name, EvidenceScenario::Honest, redaction);
        let evidence = evidence(&receipt);
        assert_mandatory_core(evidence.core());
        assert_eq!(evidence.governance().redaction(), redaction);
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
        assert!(evidence.decision_sidecar().records().is_none());
        assert!(evidence.candidate_sidecar().records().is_none());
        assert!(evidence.transformation_sidecar().records().is_none());

        let inspection = inspection(
            evidence,
            runtime::CausalInspectionRedactionPolicy::PreserveDetail,
        );
        assert_eq!(inspection.governance(), evidence.governance());
        assert!(inspection.decision_sidecar().records().is_none());
        assert!(inspection.candidate_sidecar().records().is_none());
        assert!(inspection.transformation_sidecar().records().is_none());
        let bundle =
            certification::WorthQueryDomainEvidenceCertificationBundle::derive(&inspection);
        assert_eq!(bundle.governance(), evidence.governance());
        assert_eq!(bundle.decision_sidecar().digest().is_some(), expect_digest);
        assert_eq!(bundle.candidate_sidecar().digest().is_some(), expect_digest);
        assert_eq!(
            bundle.transformation_sidecar().digest().is_some(),
            expect_digest
        );
    }
}

fn assert_governance(governance: &domain::WorthQueryDomainEvidenceGovernance) {
    assert_eq!(governance.audiences(), &["audit", "support"]);
    assert_eq!(
        governance.classification(),
        domain::WorthQueryArtifactClassification::Restricted
    );
    assert_eq!(
        governance.redaction(),
        domain::WorthQueryArtifactRedactionPosture::NotRequired
    );
    assert_eq!(governance.retention(), RetentionDeliveryProfile::Durable);
    assert_eq!(
        governance.deletion(),
        domain::WorthQueryArtifactDeletionPosture::DomainControlled
    );
    assert_eq!(
        governance.legal_hold(),
        domain::WorthQueryArtifactLegalHoldPosture::RequiredWhenDirected
    );
}

fn assert_mandatory_core(core: &domain::WorthQueryDomainEvidenceCore) {
    let counters = core
        .counters()
        .iter()
        .map(|counter| {
            (
                counter.schema().name().as_str(),
                (counter.initial(), counter.observed()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        counters,
        BTreeMap::from([
            ("bytes", (0, 128)),
            ("candidate-comparisons", (0, 6)),
            ("elements", (0, 4)),
            ("work", (0, 10)),
        ])
    );

    let decisions = core.decisions();
    assert_eq!(decisions.len(), 1);
    assert_eq!(decisions[0].schema().kind().as_str(), "candidate-rejected");
    assert_eq!(
        decisions[0].schema().reason_family().as_str(),
        "search-reason"
    );
    assert_eq!(
        decisions[0]
            .schema()
            .affected_artifact_key_family()
            .as_str(),
        "candidate"
    );
    assert_eq!(
        decisions[0].counts(),
        domain::WorthQueryDecisionSummaryCounts::new(1, 1, 1, 0)
    );

    let search = core.candidate_search().unwrap().parts();
    assert_eq!(search.universe.family(), "candidate-universe");
    assert_eq!(search.universe.value(), "sample-v1");
    assert_eq!(search.considered_count, 2);
    assert_eq!(
        search.termination,
        domain::WorthQueryCandidateTerminationClass::SampleCompleted
    );
    assert_eq!(
        search.completeness,
        domain::WorthQueryCandidateSearchPosture::Sampled {
            sample_identity: "sample-v1".into()
        }
    );
    assert_eq!(
        search.feasibility,
        domain::WorthQueryCandidateFeasibilityClass::FeasibleCandidateFound
    );
    assert_eq!(
        search.optimality,
        domain::WorthQueryCandidateOptimalityPosture::BestInDeclaredSample {
            sample_identity: "sample-v1".into()
        }
    );
    assert_eq!(search.rejected_count, 1);
    assert_eq!(
        search.incumbent,
        domain::WorthQueryCandidateIncumbentDisposition::Selected
    );

    let transformation = core.transformation().unwrap().parts();
    assert_eq!(
        transformation.source_occurrence.family(),
        "source-occurrence"
    );
    assert_eq!(transformation.source_occurrence.value(), "source-1");
    assert_eq!(transformation.transformation_family, "normalize-candidates");
    assert_eq!(transformation.transformation_version, 1);
    assert_eq!(
        transformation.correspondence,
        domain::WorthQuerySourceOutputCorrespondence::OneToMany
    );
    assert_eq!(
        transformation.disposition,
        domain::WorthQueryTransformationDisposition::Normalized
    );
    assert_eq!(
        transformation.error,
        domain::WorthQueryTransformationErrorPosture::Bounded
    );
    assert_eq!(
        transformation.loss,
        domain::WorthQueryTransformationLossPosture::DeclaredLossy
    );
}
