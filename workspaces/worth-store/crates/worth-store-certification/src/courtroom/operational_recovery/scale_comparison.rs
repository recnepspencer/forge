use std::collections::BTreeMap;

use sha2::{Digest, Sha256};
use worth_store_operations::{
    OperationalCounterReceipt, OperationalSessionDisposition, OperationalSessionKind,
};

use super::{S10OperationalScenarioEvidence, S10OperationalScenarioKind, S10ScenarioSuiteEvidence};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10ScaleComparisonDenial {
    DimensionRegressed(S10OperationalScenarioKind),
    WorkloadDidNotExpand(S10OperationalScenarioKind),
    MeasuredOperationBreadthDidNotExpand(S10OperationalScenarioKind),
    ScheduleBreadthRegressed(S10OperationalScenarioKind),
    CounterKindMissing {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    UnexpectedReleaseCounterKind {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    CounterAggregateOverflow {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    CounterSessionCountChanged {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    CounterBreadthRegressed {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    DeclaredSlopeExceeded {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
    ResidentBudgetExceeded {
        scenario: S10OperationalScenarioKind,
        kind: OperationalSessionKind,
        disposition: OperationalSessionDisposition,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10ScaleComparisonRow {
    scenario: S10OperationalScenarioKind,
    ci_identity: [u8; 32],
    release_identity: [u8; 32],
    comparison_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10ScaleComparisonMatrix {
    rows: [S10ScaleComparisonRow; 3],
    matrix_identity: [u8; 32],
}

impl S10ScaleComparisonMatrix {
    pub(super) fn from_suites(
        ci: &S10ScenarioSuiteEvidence,
        release: &S10ScenarioSuiteEvidence,
    ) -> Result<Self, S10ScaleComparisonDenial> {
        let rows = [
            compare_scenario(
                ci.scenario(S10OperationalScenarioKind::BurningPrimary),
                release.scenario(S10OperationalScenarioKind::BurningPrimary),
            )?,
            compare_scenario(
                ci.scenario(S10OperationalScenarioKind::SplitBrainPromotion),
                release.scenario(S10OperationalScenarioKind::SplitBrainPromotion),
            )?,
            compare_scenario(
                ci.scenario(S10OperationalScenarioKind::AuthorityRepairRollback),
                release.scenario(S10OperationalScenarioKind::AuthorityRepairRollback),
            )?,
        ];
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-scale-comparison-matrix-v1");
        for row in &rows {
            digest.update(row.comparison_identity);
        }
        Ok(Self {
            rows,
            matrix_identity: digest.finalize().into(),
        })
    }

    pub const fn rows(&self) -> &[S10ScaleComparisonRow; 3] {
        &self.rows
    }
    pub const fn matrix_identity(&self) -> [u8; 32] {
        self.matrix_identity
    }
}

impl S10ScaleComparisonRow {
    pub const fn scenario(self) -> S10OperationalScenarioKind {
        self.scenario
    }
    pub const fn ci_identity(self) -> [u8; 32] {
        self.ci_identity
    }
    pub const fn release_identity(self) -> [u8; 32] {
        self.release_identity
    }
    pub const fn comparison_identity(self) -> [u8; 32] {
        self.comparison_identity
    }
}

fn compare_scenario(
    ci: &S10OperationalScenarioEvidence,
    release: &S10OperationalScenarioEvidence,
) -> Result<S10ScaleComparisonRow, S10ScaleComparisonDenial> {
    let scenario = ci.program().kind();
    let ci_dimensions = ci.scale().dimensions();
    let release_dimensions = release.scale().dimensions();
    if !release_dimensions.dominates(ci_dimensions) {
        return Err(S10ScaleComparisonDenial::DimensionRegressed(scenario));
    }
    if !release_dimensions.strictly_expands(ci_dimensions) {
        return Err(S10ScaleComparisonDenial::WorkloadDidNotExpand(scenario));
    }
    if release.scale().schedules_executed() < ci.scale().schedules_executed() {
        return Err(S10ScaleComparisonDenial::ScheduleBreadthRegressed(scenario));
    }
    let ci_counters = aggregate_counters(ci.counters()).map_err(|key| {
        S10ScaleComparisonDenial::CounterAggregateOverflow {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        }
    })?;
    let release_counters = aggregate_counters(release.counters()).map_err(|key| {
        S10ScaleComparisonDenial::CounterAggregateOverflow {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        }
    })?;
    if let Some(key) = release_counters
        .keys()
        .find(|key| !ci_counters.contains_key(key))
    {
        return Err(S10ScaleComparisonDenial::UnexpectedReleaseCounterKind {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        });
    }
    let mut measured_breadth_expanded = false;
    for (key, ci_counter) in ci_counters {
        let release_counter = release_counters.get(&key).copied().ok_or(
            S10ScaleComparisonDenial::CounterKindMissing {
                scenario,
                kind: key.kind,
                disposition: key.disposition,
            },
        )?;
        if release_counter.session_count != ci_counter.session_count {
            return Err(S10ScaleComparisonDenial::CounterSessionCountChanged {
                scenario,
                kind: key.kind,
                disposition: key.disposition,
            });
        }
        measured_breadth_expanded |= release_counter.work_units > ci_counter.work_units
            || release_counter.source_bytes_read > ci_counter.source_bytes_read
            || release_counter.output_bytes_written > ci_counter.output_bytes_written;
        compare_counter(ci, release, key, ci_counter, release_counter)?;
    }
    if !measured_breadth_expanded {
        return Err(S10ScaleComparisonDenial::MeasuredOperationBreadthDidNotExpand(scenario));
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-scale-comparison-row-v1");
    digest.update(scenario.token().as_bytes());
    digest.update(ci.evidence_identity());
    digest.update(release.evidence_identity());
    Ok(S10ScaleComparisonRow {
        scenario,
        ci_identity: ci.evidence_identity(),
        release_identity: release.evidence_identity(),
        comparison_identity: digest.finalize().into(),
    })
}

fn compare_counter(
    ci: &S10OperationalScenarioEvidence,
    release: &S10OperationalScenarioEvidence,
    key: OperationalCounterClass,
    ci_counter: CounterAggregate,
    release_counter: CounterAggregate,
) -> Result<(), S10ScaleComparisonDenial> {
    let scenario = ci.program().kind();
    if ci_counter.maximum_resident_bytes > ci.scale().resident_budget_bytes()
        || release_counter.maximum_resident_bytes > release.scale().resident_budget_bytes()
    {
        return Err(S10ScaleComparisonDenial::ResidentBudgetExceeded {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        });
    }
    if release_counter.work_units < ci_counter.work_units
        || release_counter.source_bytes_read < ci_counter.source_bytes_read
        || release_counter.output_bytes_written < ci_counter.output_bytes_written
    {
        return Err(S10ScaleComparisonDenial::CounterBreadthRegressed {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        });
    }
    let ci_breadth = ci.scale().dimensions().total_breadth();
    let release_breadth = release.scale().dimensions().total_breadth();
    if exceeds_two_x_slope(
        ci_counter.work_units,
        release_counter.work_units,
        ci_breadth,
        release_breadth,
    ) || exceeds_two_x_slope(
        ci_counter
            .source_bytes_read
            .saturating_add(ci_counter.output_bytes_written),
        release_counter
            .source_bytes_read
            .saturating_add(release_counter.output_bytes_written),
        ci_breadth,
        release_breadth,
    ) {
        return Err(S10ScaleComparisonDenial::DeclaredSlopeExceeded {
            scenario,
            kind: key.kind,
            disposition: key.disposition,
        });
    }
    Ok(())
}

fn exceeds_two_x_slope(ci: u64, release: u64, ci_breadth: u128, release_breadth: u128) -> bool {
    ci > 0
        && (release as u128).saturating_mul(ci_breadth)
            > (ci as u128)
                .saturating_mul(release_breadth)
                .saturating_mul(2)
}

#[derive(Debug, Clone, Copy, Default)]
struct CounterAggregate {
    session_count: u64,
    work_units: u64,
    source_bytes_read: u64,
    output_bytes_written: u64,
    maximum_resident_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct OperationalCounterClass {
    kind: OperationalSessionKind,
    disposition: OperationalSessionDisposition,
}

fn aggregate_counters(
    counters: &[OperationalCounterReceipt],
) -> Result<BTreeMap<OperationalCounterClass, CounterAggregate>, OperationalCounterClass> {
    let mut by_class: BTreeMap<OperationalCounterClass, CounterAggregate> = BTreeMap::new();
    for counter in counters {
        let class = OperationalCounterClass {
            kind: counter.kind(),
            disposition: counter.disposition(),
        };
        let aggregate = by_class.entry(class).or_default();
        aggregate.session_count = aggregate.session_count.checked_add(1).ok_or(class)?;
        aggregate.work_units = aggregate
            .work_units
            .checked_add(counter.work_units())
            .ok_or(class)?;
        aggregate.source_bytes_read = aggregate
            .source_bytes_read
            .checked_add(counter.source_bytes_read())
            .ok_or(class)?;
        aggregate.output_bytes_written = aggregate
            .output_bytes_written
            .checked_add(counter.output_bytes_written())
            .ok_or(class)?;
        aggregate.maximum_resident_bytes = aggregate
            .maximum_resident_bytes
            .max(counter.maximum_resident_bytes());
    }
    Ok(by_class)
}
