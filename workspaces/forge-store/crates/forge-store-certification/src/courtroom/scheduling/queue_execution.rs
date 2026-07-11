use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};
use forge_store_io_scheduler::{
    QueueExecutedPlan, QueueExecutionCounterSnapshot, QueueExecutionOutcome,
    QueueExecutionProgression, QueueExecutionReplayIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6QueueExecutionCertificationDenial {
    MissingReplayVisibleOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6CertifiedQueueExecutionEvidence {
    replay_identity: QueueExecutionReplayIdentity,
    secondary_replay_identity: Option<QueueExecutionReplayIdentity>,
    counters: QueueExecutionCounterSnapshot,
    progression: QueueExecutionProgression,
    counter_backed_receipt:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl S6CertifiedQueueExecutionEvidence {
    pub fn from_outcome(
        outcome: &QueueExecutionOutcome,
    ) -> Result<Self, S6QueueExecutionCertificationDenial> {
        match outcome {
            QueueExecutionOutcome::Executed(evidence) => Ok(Self {
                replay_identity: evidence.plan().replay_identity(),
                secondary_replay_identity: secondary_replay_identity(evidence.secondary_plan()),
                counters: evidence.counters(),
                progression: QueueExecutionProgression::Executed,
                counter_backed_receipt: counter_backed_receipt(evidence.counters()),
            }),
            QueueExecutionOutcome::Backpressured(evidence) => Ok(Self {
                replay_identity: evidence.plan().replay_identity(),
                secondary_replay_identity: secondary_replay_identity(evidence.secondary_plan()),
                counters: evidence.counters(),
                progression: QueueExecutionProgression::Executed,
                counter_backed_receipt: counter_backed_receipt(evidence.counters()),
            }),
            QueueExecutionOutcome::Denied(evidence) => Ok(Self {
                replay_identity: evidence.plan().replay_identity(),
                secondary_replay_identity: secondary_replay_identity(evidence.secondary_plan()),
                counters: evidence.counters(),
                progression: QueueExecutionProgression::Executed,
                counter_backed_receipt: counter_backed_receipt(evidence.counters()),
            }),
            QueueExecutionOutcome::Violation(evidence) => Ok(Self {
                replay_identity: evidence.plan().replay_identity(),
                secondary_replay_identity: secondary_replay_identity(evidence.secondary_plan()),
                counters: evidence.counters(),
                progression: QueueExecutionProgression::Executed,
                counter_backed_receipt: counter_backed_receipt(evidence.counters()),
            }),
        }
    }

    pub const fn replay_identity(&self) -> QueueExecutionReplayIdentity {
        self.replay_identity
    }

    pub const fn secondary_replay_identity(&self) -> Option<QueueExecutionReplayIdentity> {
        self.secondary_replay_identity
    }

    pub const fn counters(&self) -> QueueExecutionCounterSnapshot {
        self.counters
    }

    pub const fn progression(&self) -> QueueExecutionProgression {
        self.progression
    }

    pub const fn counter_backed_receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.counter_backed_receipt
    }
}

fn secondary_replay_identity(
    plan: Option<&QueueExecutedPlan>,
) -> Option<QueueExecutionReplayIdentity> {
    plan.map(QueueExecutedPlan::replay_identity)
}

fn counter_backed_receipt(
    counters: QueueExecutionCounterSnapshot,
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let specs = counter_specs(counters);
    let mut bundle = performance_api::lower_lane::basis::performance_bundle(counter_claim());
    for spec in &specs {
        bundle = bundle.attach_counter_spec(spec.clone());
    }
    let bundle = bundle
        .finish()
        .expect("queue execution counter bundle should build");
    let mut receipt =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for row in counter_rows(counters) {
        receipt = receipt.attach_counter_row(row);
    }
    receipt
        .finish()
        .expect("queue execution counters should satisfy exact receipt specs")
}

fn counter_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("queue execution counter claim should build")
}

fn counter_specs(
    counters: QueueExecutionCounterSnapshot,
) -> [FoundationalPerformanceCounterSpec; 13] {
    [
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_submitted_units"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.submitted_units(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_admitted_units"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.admitted_units(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_denied_units"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.denied_units(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_peak_depth"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            u64::from(counters.peak_queue_depth()),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_grouped_writes"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            u64::from(counters.grouped_writes()),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_read_ahead_units"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.read_ahead_units(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_write_back_units"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.write_back_units(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_backpressure_events"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.backpressure_events(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_foreground_wait_events"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.foreground_wait_events(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_mechanical_retries"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.mechanical_retries(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_partial_read_events"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.partial_read_events(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_short_write_events"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.short_write_events(),
        ),
        FoundationalPerformanceCounterSpec::new(
            counter_name("queue_violation_events"),
            FoundationalPerformanceWorkClass::ValidationPlanning,
            counters.violation_events(),
        ),
    ]
}

fn counter_rows(
    counters: QueueExecutionCounterSnapshot,
) -> [FoundationalPerformanceCounterRow; 13] {
    [
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_submitted_units"),
            counters.submitted_units(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_admitted_units"),
            counters.admitted_units(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_denied_units"),
            counters.denied_units(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_peak_depth"),
            u64::from(counters.peak_queue_depth()),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_grouped_writes"),
            u64::from(counters.grouped_writes()),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_read_ahead_units"),
            counters.read_ahead_units(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_write_back_units"),
            counters.write_back_units(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_backpressure_events"),
            counters.backpressure_events(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_foreground_wait_events"),
            counters.foreground_wait_events(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_mechanical_retries"),
            counters.mechanical_retries(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_partial_read_events"),
            counters.partial_read_events(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_short_write_events"),
            counters.short_write_events(),
        ),
        FoundationalPerformanceCounterRow::new(
            counter_name("queue_violation_events"),
            counters.violation_events(),
        ),
    ]
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name)
        .expect("static queue counter name should be valid")
}
