use super::test_support::{background_budget_with_queue_slots, read_pressure_budget, World};

use crate::{
    BackgroundDebtKind, BackgroundInterferenceEvidence, BackgroundIoPressureClass,
    BackgroundIoPressureShape, BackgroundPacingProgressionEvidence, BackgroundResourceBudget,
    InterferenceAttribution, InterferenceCounterName, LatencyEnvelopeAssessmentStatus, QueueSlot,
};

#[test]
fn background_yield_outcome_materializes_interference_counter() {
    let world = World::new();
    let requested = read_pressure_budget();
    let outcome = super::super::admit_background_pacing(
        world
            .request(BackgroundIoPressureShape::scrub_scan().requesting(requested))
            .with_foreground_pressure_events(1),
    );

    let evidence = BackgroundInterferenceEvidence::from_background_pacing_outcome(
        "s6-profile/posix-file",
        crate::QueueWorkClass::Background(BackgroundIoPressureClass::ScrubScan),
        outcome,
    );

    assert_eq!(
        evidence.status(),
        LatencyEnvelopeAssessmentStatus::EnvelopeExceeded
    );
    let yield_events = evidence
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::BackgroundYieldEvents)
        .expect("background yield row should be materialized");
    assert_eq!(yield_events.value(), 1);
    assert_eq!(
        yield_events.attribution(),
        Some(InterferenceAttribution::BackgroundYield)
    );
}

#[test]
fn admitted_with_debt_outcome_materializes_policy_debt_counter() {
    let world = World::new();
    let requested = read_pressure_budget();
    let admitted = background_budget_with_queue_slots(QueueSlot::new(1).unwrap());
    let debt_limit = requested.debt_after(admitted);
    let outcome = super::super::admit_background_pacing(world.request_with(
        BackgroundIoPressureShape::compaction_rewrite().requesting(requested),
        admitted,
        admitted,
        debt_limit,
        BackgroundPacingProgressionEvidence::current(world.readiness()),
    ));

    let evidence = BackgroundInterferenceEvidence::from_background_pacing_outcome(
        "s6-profile/posix-file",
        crate::QueueWorkClass::Background(BackgroundIoPressureClass::CompactionRewrite),
        outcome,
    );

    assert_eq!(
        evidence.status(),
        LatencyEnvelopeAssessmentStatus::PolicyDebtIncurred
    );
    let policy_debt = evidence
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::PolicyDebtEvents)
        .expect("policy debt row should be materialized");
    assert_eq!(policy_debt.value(), 1);
    assert_eq!(
        policy_debt.attribution(),
        Some(InterferenceAttribution::PolicyDebt)
    );
}

#[test]
fn late_yield_violation_materializes_execution_violated_evidence() {
    let world = World::new();
    let requested = read_pressure_budget();
    let admitted = background_budget_with_queue_slots(QueueSlot::new(1).unwrap());
    let debt_limit = requested.debt_after(admitted);
    let outcome = super::super::admit_background_pacing(
        world
            .request_with(
                BackgroundIoPressureShape::replication_prep_read().requesting(requested),
                admitted,
                admitted,
                debt_limit,
                BackgroundPacingProgressionEvidence::current(world.readiness()),
            )
            .with_foreground_pressure_events(1)
            .with_late_yield(),
    );

    let evidence = BackgroundInterferenceEvidence::from_background_pacing_outcome(
        "s6-profile/posix-file",
        crate::QueueWorkClass::Background(BackgroundIoPressureClass::ReplicationPrepRead),
        outcome,
    );

    assert_eq!(
        evidence.status(),
        LatencyEnvelopeAssessmentStatus::ExecutionViolated
    );
    let violation = evidence
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::BackgroundViolationEvents)
        .expect("background violation row should be materialized");
    assert_eq!(violation.value(), 1);
    assert_eq!(
        violation.attribution(),
        Some(InterferenceAttribution::ExecutionViolation)
    );
    let debt = evidence
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::BackgroundDebtUnits)
        .expect("background debt row should be materialized");
    assert_eq!(debt.value(), total_budget_units(debt_limit));
    assert_eq!(
        debt.attribution(),
        Some(InterferenceAttribution::BackgroundDebt(
            BackgroundDebtKind::ReplicationPrepPressure
        ))
    );
    let policy_debt = evidence
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::PolicyDebtEvents)
        .expect("policy debt row should remain materialized");
    assert_eq!(policy_debt.value(), 0);
    assert_eq!(policy_debt.attribution(), None);
}

fn total_budget_units(budget: BackgroundResourceBudget) -> u64 {
    budget
        .queue_slots()
        .saturating_add(budget.bandwidth_tokens())
        .saturating_add(budget.flush_permits())
        .saturating_add(budget.sync_debt())
        .saturating_add(budget.read_ahead_window())
        .saturating_add(budget.write_back_window())
        .saturating_add(budget.dirty_page_budget())
        .saturating_add(budget.worker_permits())
        .saturating_add(budget.cache_residency_hints())
        .saturating_add(budget.reclaim_permits())
}
