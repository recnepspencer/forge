use worth_foundational::facade::FoundationalPerformanceCounterName;

use super::contract_is_valid;
use crate::domain_computation::{
    WorthQueryStructuralCounterAggregation, WorthQueryStructuralCounterContract,
    WorthQueryStructuralCounterMonotonicity, WorthQueryStructuralCounterReplayPosture,
    WorthQueryStructuralCounterRequiredness, WorthQueryStructuralCounterResetBoundary,
    WorthQueryStructuralCounterRole, WorthQueryStructuralCounterSchema,
    WorthQueryStructuralCounterScope, WorthQueryStructuralCounterUnit,
};

#[test]
fn aggregation_rejects_a_source_measured_in_another_unit() {
    let elements = counter("elements");
    let mut rows = WorthQueryStructuralCounterContract::required_foundation(
        counter("bytes"),
        elements.clone(),
        counter("work"),
    )
    .rows()
    .to_vec();
    rows.push(WorthQueryStructuralCounterSchema::new(
        counter("comparisons"),
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Comparisons,
        WorthQueryStructuralCounterAggregation::SumOf(vec![elements]),
        WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        WorthQueryStructuralCounterScope::ArtifactOccurrence,
        WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    ));

    assert!(!contract_is_valid(
        &WorthQueryStructuralCounterContract::declare(rows)
    ));
}

#[test]
fn aggregation_rejects_a_source_from_another_scope_and_reset_lifecycle() {
    let stage_elements = counter("stage-elements");
    let mut rows = WorthQueryStructuralCounterContract::required_foundation(
        counter("bytes"),
        counter("elements"),
        counter("work"),
    )
    .rows()
    .to_vec();
    rows.push(WorthQueryStructuralCounterSchema::new(
        stage_elements.clone(),
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Elements,
        WorthQueryStructuralCounterAggregation::Independent,
        WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        WorthQueryStructuralCounterScope::Stage,
        WorthQueryStructuralCounterResetBoundary::Stage,
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    ));
    rows.push(WorthQueryStructuralCounterSchema::new(
        counter("occurrence-elements"),
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Elements,
        WorthQueryStructuralCounterAggregation::SumOf(vec![stage_elements]),
        WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        WorthQueryStructuralCounterScope::ArtifactOccurrence,
        WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    ));

    assert!(!contract_is_valid(
        &WorthQueryStructuralCounterContract::declare(rows)
    ));
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
