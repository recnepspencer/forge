use std::collections::{BTreeMap, BTreeSet};

use super::{
    WorthQueryStructuralCounterAggregation, WorthQueryStructuralCounterContract,
    WorthQueryStructuralCounterRequiredness, WorthQueryStructuralCounterResetBoundary,
    WorthQueryStructuralCounterRole, WorthQueryStructuralCounterScope,
    WorthQueryStructuralCounterUnit,
};

pub(super) fn contract_is_valid(contract: &WorthQueryStructuralCounterContract) -> bool {
    let rows = contract.rows();
    if rows.is_empty() || duplicate_names(contract) {
        return false;
    }
    if !required_foundation_roles_are_exact(rows) {
        return false;
    }
    if rows.iter().any(|row| {
        !unit_is_portable(row.unit())
            || !scope_matches_reset(row.scope(), row.reset_boundary())
            || aggregation_is_invalid(contract, row)
    }) {
        return false;
    }
    !aggregation_cycle_exists(contract)
}

fn duplicate_names(contract: &WorthQueryStructuralCounterContract) -> bool {
    contract
        .rows()
        .windows(2)
        .any(|pair| pair[0].name() == pair[1].name())
}

fn required_foundation_roles_are_exact(rows: &[super::WorthQueryStructuralCounterSchema]) -> bool {
    [
        WorthQueryStructuralCounterRole::Bytes,
        WorthQueryStructuralCounterRole::Elements,
        WorthQueryStructuralCounterRole::StructuralWork,
    ]
    .into_iter()
    .all(|role| {
        rows.iter()
            .filter(|row| {
                row.role() == role
                    && row.requiredness() == WorthQueryStructuralCounterRequiredness::RequiredCore
            })
            .count()
            == 1
    })
}

fn unit_is_portable(unit: &WorthQueryStructuralCounterUnit) -> bool {
    match unit {
        WorthQueryStructuralCounterUnit::Domain(identity) => portable_identity(identity),
        _ => true,
    }
}

fn scope_matches_reset(
    scope: WorthQueryStructuralCounterScope,
    reset: WorthQueryStructuralCounterResetBoundary,
) -> bool {
    matches!(
        (scope, reset),
        (
            WorthQueryStructuralCounterScope::Operation,
            WorthQueryStructuralCounterResetBoundary::Operation
        ) | (
            WorthQueryStructuralCounterScope::Run,
            WorthQueryStructuralCounterResetBoundary::Run
        ) | (
            WorthQueryStructuralCounterScope::Stage,
            WorthQueryStructuralCounterResetBoundary::Stage
        ) | (
            WorthQueryStructuralCounterScope::Attempt,
            WorthQueryStructuralCounterResetBoundary::Attempt
        ) | (
            WorthQueryStructuralCounterScope::ArtifactOccurrence,
            WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence
        )
    )
}

fn aggregation_is_invalid(
    contract: &WorthQueryStructuralCounterContract,
    row: &super::WorthQueryStructuralCounterSchema,
) -> bool {
    let sources = row.aggregation().sources();
    if matches!(
        row.aggregation(),
        WorthQueryStructuralCounterAggregation::Independent
    ) {
        return false;
    }
    sources.is_empty()
        || sources.iter().any(|source| {
            source == row.name()
                || contract.row(source).is_none()
                || (row.requiredness() == WorthQueryStructuralCounterRequiredness::RequiredCore
                    && contract.row(source).is_some_and(|source| {
                        source.requiredness()
                            == WorthQueryStructuralCounterRequiredness::OptionalSidecar
                    }))
        })
}

fn aggregation_cycle_exists(contract: &WorthQueryStructuralCounterContract) -> bool {
    let edges = contract
        .rows()
        .iter()
        .map(|row| {
            (
                row.name(),
                row.aggregation().sources().iter().collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    contract
        .rows()
        .iter()
        .any(|row| reaches(row.name(), row.name(), &edges, &mut BTreeSet::new()))
}

fn reaches<'a>(
    start: &'a worth_foundational::facade::FoundationalPerformanceCounterName,
    current: &'a worth_foundational::facade::FoundationalPerformanceCounterName,
    edges: &BTreeMap<
        &'a worth_foundational::facade::FoundationalPerformanceCounterName,
        Vec<&'a worth_foundational::facade::FoundationalPerformanceCounterName>,
    >,
    visited: &mut BTreeSet<&'a worth_foundational::facade::FoundationalPerformanceCounterName>,
) -> bool {
    let Some(next) = edges.get(current) else {
        return false;
    };
    for candidate in next {
        if *candidate == start {
            return true;
        }
        if visited.insert(*candidate) && reaches(start, candidate, edges, visited) {
            return true;
        }
    }
    false
}

fn portable_identity(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
