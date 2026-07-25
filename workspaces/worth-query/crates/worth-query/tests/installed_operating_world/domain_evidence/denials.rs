use worth_query::facade::domain;

use super::super::installed_operation_fixture::EvidenceScenario;
use super::execution::denied_execution;

#[test]
fn dishonest_counter_search_and_loss_material_fail_receipt_admission_exactly() {
    let cases = [
        (
            "domain-evidence-missing-counter",
            EvidenceScenario::MissingRequiredCounter,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::MissingRequiredCounter,
        ),
        (
            "domain-evidence-backward-counter",
            EvidenceScenario::BackwardCounter,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CounterMovedBackward,
        ),
        (
            "domain-evidence-impossible-aggregate",
            EvidenceScenario::ImpossibleAggregate,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CounterAggregateMismatch,
        ),
        (
            "domain-evidence-search-overclaim",
            EvidenceScenario::SearchOverclaim,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSearchOverclaim,
        ),
        (
            "domain-evidence-search-not-applicable",
            EvidenceScenario::SearchNotApplicable,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSearchOverclaim,
        ),
        (
            "domain-evidence-search-no-feasible-selected",
            EvidenceScenario::SearchNoFeasibleSelected,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSearchOverclaim,
        ),
        (
            "domain-evidence-search-all-feasible-rejected",
            EvidenceScenario::SearchAllFeasibleRejected,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSearchOverclaim,
        ),
        (
            "domain-evidence-loss-mismatch",
            EvidenceScenario::LossMismatch,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSummaryMismatch,
        ),
        (
            "domain-evidence-malformed-sidecars",
            EvidenceScenario::MalformedSidecars,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSidecarMismatch,
        ),
        (
            "domain-evidence-malformed-transformation-sidecar",
            EvidenceScenario::MalformedTransformationSidecar,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSidecarMismatch,
        ),
        (
            "domain-evidence-malformed-optional-counter",
            EvidenceScenario::MalformedOptionalCounter,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::CounterMovedBackward,
        ),
        (
            "domain-evidence-undeclared-counter",
            EvidenceScenario::UndeclaredCounter,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredCounter,
        ),
        (
            "domain-evidence-duplicate-counter",
            EvidenceScenario::DuplicateCounter,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::DuplicateCounter,
        ),
        (
            "domain-evidence-missing-provider-certification",
            EvidenceScenario::ProviderCertificationMissing,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::ProviderCertificationMissing,
        ),
        (
            "domain-evidence-missing-decision-summary",
            EvidenceScenario::MissingDecisionSummary,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::MissingDecisionSummary,
        ),
        (
            "domain-evidence-duplicate-decision-summary",
            EvidenceScenario::DuplicateDecisionSummary,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::DuplicateDecisionSummary,
        ),
        (
            "domain-evidence-undeclared-decision-summary",
            EvidenceScenario::UndeclaredDecisionSummary,
            domain::WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredDecisionSummary,
        ),
    ];

    for (name, scenario, expected) in cases {
        let denial = denied_execution(name, scenario);
        assert_eq!(
            denial.kind(),
            &domain::WorthQueryBoundExecutionDenialKind::DomainEvidence(expected),
            "wrong evidence denial for {scenario:?}: {}",
            denial.detail()
        );
        assert_eq!(denial.counters().executor_contacts, 1);
        assert_eq!(denial.counters().publication_checks, 0);
        assert_eq!(denial.counters().consumption_contacts, 0);
    }
}
