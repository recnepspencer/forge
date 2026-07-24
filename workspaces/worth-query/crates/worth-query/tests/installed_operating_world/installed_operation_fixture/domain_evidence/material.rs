use worth_foundational::facade::FoundationalPerformanceCounterName;
use worth_query::facade::domain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceScenario {
    Honest,
    OmitSidecars,
    MissingRequiredCounter,
    BackwardCounter,
    ImpossibleAggregate,
    SearchOverclaim,
    LossMismatch,
}

pub(super) fn evidence_material(
    output_occurrence_identity: &str,
    scenario: EvidenceScenario,
) -> domain::WorthQueryDomainEvidenceMaterial {
    let candidate_initial = if scenario == EvidenceScenario::BackwardCounter {
        7
    } else {
        0
    };
    let candidate_observed = 6;
    let work_observed = if scenario == EvidenceScenario::ImpossibleAggregate {
        9
    } else {
        10
    };
    let mut material = domain::WorthQueryDomainEvidenceMaterial::new()
        .counter(observation("bytes", 0, 128))
        .counter(observation("elements", 0, 4))
        .counter(observation(
            "candidate-comparisons",
            candidate_initial,
            candidate_observed,
        ))
        .decision(domain::WorthQueryDecisionSummary::new(
            decision_kind(),
            domain::WorthQueryDecisionSummaryCounts::new(1, 1, 1, 0),
        ))
        .candidate_search(candidate_summary(scenario))
        .transformation(transformation_summary(output_occurrence_identity, scenario));
    if scenario != EvidenceScenario::MissingRequiredCounter {
        material = material.counter(observation("work", 0, work_observed));
    }
    if scenario != EvidenceScenario::OmitSidecars {
        material = material.with_sidecar(sidecars(output_occurrence_identity));
    }
    material
}

fn candidate_summary(scenario: EvidenceScenario) -> domain::WorthQueryCandidateSearchSummary {
    let completeness = if scenario == EvidenceScenario::SearchOverclaim {
        domain::WorthQueryCandidateSearchPosture::Exhaustive
    } else {
        domain::WorthQueryCandidateSearchPosture::Sampled {
            sample_identity: "sample-v1".into(),
        }
    };
    domain::WorthQueryCandidateSearchSummary::from_parts(
        domain::WorthQueryCandidateSearchSummaryParts {
            universe: domain::WorthQueryDomainEvidenceValue::new("candidate-universe", "sample-v1"),
            considered_count: 2,
            termination_family: "candidate-termination".into(),
            termination: domain::WorthQueryCandidateTerminationClass::SampleCompleted,
            completeness,
            feasibility_family: "candidate-feasibility".into(),
            feasibility: domain::WorthQueryCandidateFeasibilityClass::FeasibleCandidateFound,
            comparison_authority: domain::WorthQueryDomainEvidenceValue::new(
                "candidate-comparison",
                "score-v1",
            ),
            optimality: domain::WorthQueryCandidateOptimalityPosture::BestInDeclaredSample {
                sample_identity: "sample-v1".into(),
            },
            rejected_count: 1,
            incumbent_family: "candidate-incumbent".into(),
            incumbent: domain::WorthQueryCandidateIncumbentDisposition::Selected,
        },
    )
}

fn transformation_summary(
    output_occurrence_identity: &str,
    scenario: EvidenceScenario,
) -> domain::WorthQueryTransformationSummary {
    let loss = if scenario == EvidenceScenario::LossMismatch {
        domain::WorthQueryTransformationLossPosture::Lossless
    } else {
        domain::WorthQueryTransformationLossPosture::DeclaredLossy
    };
    domain::WorthQueryTransformationSummary::from_parts(
        domain::WorthQueryTransformationSummaryParts {
            source_occurrence: domain::WorthQueryDomainEvidenceValue::new(
                "source-occurrence",
                "source-1",
            ),
            output_occurrence_identity: output_occurrence_identity.into(),
            transformation_family: "normalize-candidates".into(),
            transformation_version: 1,
            correspondence: domain::WorthQuerySourceOutputCorrespondence::OneToMany,
            disposition: domain::WorthQueryTransformationDisposition::Normalized,
            error: domain::WorthQueryTransformationErrorPosture::Bounded,
            loss,
        },
    )
}

fn sidecars(output_occurrence_identity: &str) -> domain::WorthQueryDomainEvidenceSidecar {
    domain::WorthQueryDomainEvidenceSidecar::new()
        .decision_records([domain::WorthQueryDecisionRecord::from_parts(
            domain::WorthQueryDecisionRecordParts {
                kind: decision_kind(),
                reason_family: "search-reason".into(),
                artifact_key_family: "candidate".into(),
                artifact_key: "candidate-1".into(),
                causal_parent: domain::WorthQueryDecisionCausalParent::Single(
                    "candidate-root".into(),
                ),
                payload_version: 1,
                payload: "rejected-by-score".into(),
                recovery_relevant: false,
            },
        )])
        .candidate_records([
            domain::WorthQueryCandidateRecord::new(
                "candidate-1",
                domain::WorthQueryCandidateRecordDisposition::Rejected,
            ),
            domain::WorthQueryCandidateRecord::new(
                "candidate-2",
                domain::WorthQueryCandidateRecordDisposition::Incumbent,
            ),
        ])
        .transformation_records([domain::WorthQueryTransformationRecord::new(
            "source-1",
            [output_occurrence_identity],
            domain::WorthQueryTransformationDisposition::Normalized,
            domain::WorthQueryTransformationErrorPosture::Bounded,
        )])
}

fn observation(
    name: &str,
    initial: u64,
    observed: u64,
) -> domain::WorthQueryStructuralCounterObservation {
    domain::WorthQueryStructuralCounterObservation::new(counter(name), initial, observed)
}

fn decision_kind() -> domain::WorthQueryDecisionKind {
    domain::WorthQueryDecisionKind::new("candidate-rejected").unwrap()
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
