use worth_foundational::facade::{FoundationalPerformanceCounterName, RetentionDeliveryProfile};

use crate::domain_computation_artifact_fixture::{active_compatibility, base_builder};
use crate::facade::*;

#[test]
fn equivalent_evidence_schema_order_converges() {
    let forward = contract(
        counters(false, WorthQueryStructuralCounterUnit::Comparisons),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let reversed = contract(
        counters(true, WorthQueryStructuralCounterUnit::Comparisons),
        decisions(true, RetentionDeliveryProfile::Retained),
    );

    assert_eq!(forward.identity(), reversed.identity());
    assert_eq!(forward.counters(), reversed.counters());
    assert_eq!(forward.decisions(), reversed.decisions());
}

#[test]
fn counter_unit_aggregation_requiredness_and_replay_change_identity() {
    let baseline = identity(
        counters(false, WorthQueryStructuralCounterUnit::Comparisons),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let unit = identity(
        counters(false, WorthQueryStructuralCounterUnit::Iterations),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let aggregation = identity(
        counters_with_domain_aggregation(WorthQueryStructuralCounterAggregation::MaximumOf(vec![
            counter("source-comparisons"),
        ])),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let requiredness = identity(
        counters_with_domain_requiredness(WorthQueryStructuralCounterRequiredness::RequiredCore),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let replay = identity(
        counters_with_domain_replay(WorthQueryStructuralCounterReplayPosture::NonDecreasing),
        decisions(false, RetentionDeliveryProfile::Retained),
    );

    for changed in [unit, aggregation, requiredness, replay] {
        assert_ne!(baseline, changed);
    }
}

#[test]
fn decision_retention_changes_identity() {
    let retained = identity(
        counters(false, WorthQueryStructuralCounterUnit::Comparisons),
        decisions(false, RetentionDeliveryProfile::Retained),
    );
    let durable = identity(
        counters(false, WorthQueryStructuralCounterUnit::Comparisons),
        decisions(false, RetentionDeliveryProfile::Durable),
    );

    assert_ne!(retained, durable);
}

#[test]
fn unknown_and_cyclic_aggregate_relations_are_denied() {
    let unknown =
        counters_with_work_aggregation(WorthQueryStructuralCounterAggregation::SumOf(vec![
            counter("not-installed"),
        ]));
    let cycle = WorthQueryStructuralCounterContract::declare([
        foundation(
            "bytes",
            WorthQueryStructuralCounterRole::Bytes,
            WorthQueryStructuralCounterUnit::Bytes,
        ),
        foundation(
            "elements",
            WorthQueryStructuralCounterRole::Elements,
            WorthQueryStructuralCounterUnit::Elements,
        ),
        schema(
            "work",
            WorthQueryStructuralCounterRole::StructuralWork,
            WorthQueryStructuralCounterUnit::Operations,
            WorthQueryStructuralCounterAggregation::SumOf(vec![counter("cycle")]),
            WorthQueryStructuralCounterRequiredness::RequiredCore,
            WorthQueryStructuralCounterReplayPosture::Exact,
        ),
        schema(
            "cycle",
            WorthQueryStructuralCounterRole::DomainWork,
            WorthQueryStructuralCounterUnit::Operations,
            WorthQueryStructuralCounterAggregation::SumOf(vec![counter("work")]),
            WorthQueryStructuralCounterRequiredness::RequiredCore,
            WorthQueryStructuralCounterReplayPosture::Exact,
        ),
    ]);

    for invalid in [unknown, cycle] {
        let denial = base_builder()
            .counters(invalid)
            .compatibility(active_compatibility())
            .finish()
            .unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryArtifactContractValidationDenialKind::InvalidStructuralCounterContract
        );
    }
}

#[test]
fn optional_rows_cannot_shadow_required_foundation_roles() {
    for (role, unit) in [
        (
            WorthQueryStructuralCounterRole::Bytes,
            WorthQueryStructuralCounterUnit::Bytes,
        ),
        (
            WorthQueryStructuralCounterRole::Elements,
            WorthQueryStructuralCounterUnit::Elements,
        ),
        (
            WorthQueryStructuralCounterRole::StructuralWork,
            WorthQueryStructuralCounterUnit::Operations,
        ),
    ] {
        let mut rows = counters(false, WorthQueryStructuralCounterUnit::Comparisons)
            .rows()
            .to_vec();
        rows.push(schema(
            "foundation-shadow",
            role,
            unit,
            WorthQueryStructuralCounterAggregation::Independent,
            WorthQueryStructuralCounterRequiredness::OptionalSidecar,
            WorthQueryStructuralCounterReplayPosture::NotCompared,
        ));
        let denial = base_builder()
            .counters(WorthQueryStructuralCounterContract::declare(rows))
            .compatibility(active_compatibility())
            .finish()
            .unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryArtifactContractValidationDenialKind::InvalidStructuralCounterContract
        );
    }
}

#[test]
fn zero_version_decision_schema_is_denied() {
    let denial = base_builder()
        .decisions(WorthQueryDecisionRecordContract::declared([decision(
            "decision",
            WorthQueryDecisionPayloadVersion::new(0),
            RetentionDeliveryProfile::Retained,
        )]))
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidDecisionRecordContract
    );
}

fn contract(
    counters: WorthQueryStructuralCounterContract,
    decisions: WorthQueryDecisionRecordContract,
) -> WorthQueryPortableArtifactContract {
    base_builder()
        .counters(counters)
        .decisions(decisions)
        .governance(WorthQueryArtifactGovernanceContract::new(
            ["internal"],
            WorthQueryArtifactClassification::Restricted,
            WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly,
            RetentionDeliveryProfile::Durable,
            WorthQueryArtifactDeletionPosture::DeleteAfterRetention,
            WorthQueryArtifactLegalHoldPosture::DomainControlled,
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap()
}

fn identity(
    counters: WorthQueryStructuralCounterContract,
    decisions: WorthQueryDecisionRecordContract,
) -> String {
    contract(counters, decisions).identity().as_str().to_owned()
}

fn counters(
    reverse: bool,
    domain_unit: WorthQueryStructuralCounterUnit,
) -> WorthQueryStructuralCounterContract {
    let mut rows = vec![
        foundation(
            "bytes",
            WorthQueryStructuralCounterRole::Bytes,
            WorthQueryStructuralCounterUnit::Bytes,
        ),
        foundation(
            "elements",
            WorthQueryStructuralCounterRole::Elements,
            WorthQueryStructuralCounterUnit::Elements,
        ),
        foundation(
            "work",
            WorthQueryStructuralCounterRole::StructuralWork,
            WorthQueryStructuralCounterUnit::Operations,
        ),
        schema(
            "candidate-comparisons",
            WorthQueryStructuralCounterRole::DomainWork,
            domain_unit,
            WorthQueryStructuralCounterAggregation::Independent,
            WorthQueryStructuralCounterRequiredness::OptionalSidecar,
            WorthQueryStructuralCounterReplayPosture::NotCompared,
        ),
    ];
    if reverse {
        rows.reverse();
    }
    WorthQueryStructuralCounterContract::declare(rows)
}

fn counters_with_work_aggregation(
    aggregation: WorthQueryStructuralCounterAggregation,
) -> WorthQueryStructuralCounterContract {
    let mut contract = counters(false, WorthQueryStructuralCounterUnit::Comparisons);
    let mut rows = contract.rows().to_vec();
    let work = rows
        .iter_mut()
        .find(|row| row.role() == WorthQueryStructuralCounterRole::StructuralWork)
        .unwrap();
    *work = schema(
        "work",
        WorthQueryStructuralCounterRole::StructuralWork,
        WorthQueryStructuralCounterUnit::Operations,
        aggregation,
        WorthQueryStructuralCounterRequiredness::RequiredCore,
        WorthQueryStructuralCounterReplayPosture::Exact,
    );
    contract = WorthQueryStructuralCounterContract::declare(rows);
    contract
}

fn counters_with_domain_aggregation(
    aggregation: WorthQueryStructuralCounterAggregation,
) -> WorthQueryStructuralCounterContract {
    let mut rows = counters(false, WorthQueryStructuralCounterUnit::Comparisons)
        .rows()
        .to_vec();
    let domain = rows
        .iter_mut()
        .find(|row| row.role() == WorthQueryStructuralCounterRole::DomainWork)
        .unwrap();
    *domain = schema(
        "candidate-comparisons",
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Comparisons,
        aggregation,
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    );
    rows.push(schema(
        "source-comparisons",
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Comparisons,
        WorthQueryStructuralCounterAggregation::Independent,
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    ));
    WorthQueryStructuralCounterContract::declare(rows)
}

fn counters_with_domain_requiredness(
    requiredness: WorthQueryStructuralCounterRequiredness,
) -> WorthQueryStructuralCounterContract {
    replace_domain_row(
        requiredness,
        WorthQueryStructuralCounterReplayPosture::NotCompared,
    )
}

fn counters_with_domain_replay(
    replay: WorthQueryStructuralCounterReplayPosture,
) -> WorthQueryStructuralCounterContract {
    replace_domain_row(
        WorthQueryStructuralCounterRequiredness::OptionalSidecar,
        replay,
    )
}

fn replace_domain_row(
    requiredness: WorthQueryStructuralCounterRequiredness,
    replay: WorthQueryStructuralCounterReplayPosture,
) -> WorthQueryStructuralCounterContract {
    let mut rows = counters(false, WorthQueryStructuralCounterUnit::Comparisons)
        .rows()
        .to_vec();
    let domain = rows
        .iter_mut()
        .find(|row| row.role() == WorthQueryStructuralCounterRole::DomainWork)
        .unwrap();
    *domain = schema(
        "candidate-comparisons",
        WorthQueryStructuralCounterRole::DomainWork,
        WorthQueryStructuralCounterUnit::Comparisons,
        WorthQueryStructuralCounterAggregation::Independent,
        requiredness,
        replay,
    );
    WorthQueryStructuralCounterContract::declare(rows)
}

fn decisions(
    reverse: bool,
    retention: RetentionDeliveryProfile,
) -> WorthQueryDecisionRecordContract {
    let mut schemas = vec![
        decision(
            "candidate-rejected",
            WorthQueryDecisionPayloadVersion::new(1),
            retention,
        ),
        decision(
            "incumbent-selected",
            WorthQueryDecisionPayloadVersion::new(1),
            retention,
        ),
    ];
    if reverse {
        schemas.reverse();
    }
    WorthQueryDecisionRecordContract::declared(schemas)
}

fn decision(
    kind: &str,
    version: WorthQueryDecisionPayloadVersion,
    retention: RetentionDeliveryProfile,
) -> WorthQueryDecisionSchema {
    WorthQueryDecisionSchema::new(
        WorthQueryDecisionIdentity::new(
            WorthQueryDecisionKind::new(kind).unwrap(),
            WorthQueryDecisionReasonFamily::new("routing-reason").unwrap(),
            WorthQueryArtifactKeyFamily::new("candidate-key").unwrap(),
        ),
        WorthQueryDecisionCausalParentShape::OptionalSingle,
        version,
        WorthQueryDecisionGovernance::new(WorthQueryArtifactClassification::Internal, retention),
    )
}

fn foundation(
    name: &str,
    role: WorthQueryStructuralCounterRole,
    unit: WorthQueryStructuralCounterUnit,
) -> WorthQueryStructuralCounterSchema {
    schema(
        name,
        role,
        unit,
        WorthQueryStructuralCounterAggregation::Independent,
        WorthQueryStructuralCounterRequiredness::RequiredCore,
        WorthQueryStructuralCounterReplayPosture::Exact,
    )
}

fn schema(
    name: &str,
    role: WorthQueryStructuralCounterRole,
    unit: WorthQueryStructuralCounterUnit,
    aggregation: WorthQueryStructuralCounterAggregation,
    requiredness: WorthQueryStructuralCounterRequiredness,
    replay: WorthQueryStructuralCounterReplayPosture,
) -> WorthQueryStructuralCounterSchema {
    WorthQueryStructuralCounterSchema::new(
        counter(name),
        role,
        unit,
        aggregation,
        WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        WorthQueryStructuralCounterScope::ArtifactOccurrence,
        WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
        requiredness,
        replay,
    )
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
