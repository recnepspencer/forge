use worth_store_budgets::CounterEvidenceStrength;

use crate::{
    BackgroundDebtKind, BackgroundPacingCounterSnapshot, BackgroundPacingOutcome,
    BackgroundPacingViolation, BackgroundResourceBudget, QueueWorkClass,
};

use super::{
    InterferenceAttribution, InterferenceCounterName, InterferenceCounterRow,
    InterferenceReplayScope, LatencyEnvelopeAssessmentStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundInterferenceEvidence {
    status: LatencyEnvelopeAssessmentStatus,
    replay_scope: InterferenceReplayScope,
    counter_rows: Vec<InterferenceCounterRow>,
}

impl BackgroundInterferenceEvidence {
    pub fn from_background_pacing_outcome(
        profile_scope: &'static str,
        lane: QueueWorkClass,
        outcome: BackgroundPacingOutcome,
    ) -> Self {
        let (status, counters, debt_kind) = background_status_counters_and_debt(outcome);
        Self {
            status,
            replay_scope: InterferenceReplayScope::deterministic_policy_counter_and_proof_scope(),
            counter_rows: background_counter_rows(profile_scope, lane, counters, debt_kind, status),
        }
    }

    pub fn from_background_violation(
        profile_scope: &'static str,
        lane: QueueWorkClass,
        violation: BackgroundPacingViolation,
    ) -> Self {
        Self::from_background_pacing_outcome(
            profile_scope,
            lane,
            BackgroundPacingOutcome::Violation(violation),
        )
    }

    pub const fn status(&self) -> LatencyEnvelopeAssessmentStatus {
        self.status
    }

    pub const fn replay_scope(&self) -> InterferenceReplayScope {
        self.replay_scope
    }

    pub fn counter_rows(&self) -> &[InterferenceCounterRow] {
        &self.counter_rows
    }
}

fn background_status_counters_and_debt(
    outcome: BackgroundPacingOutcome,
) -> (
    LatencyEnvelopeAssessmentStatus,
    BackgroundPacingCounterSnapshot,
    Option<BackgroundDebtKind>,
) {
    match outcome {
        BackgroundPacingOutcome::Yield(evidence) => (
            LatencyEnvelopeAssessmentStatus::EnvelopeExceeded,
            evidence.counters(),
            None,
        ),
        BackgroundPacingOutcome::Deferred(evidence) => (
            LatencyEnvelopeAssessmentStatus::Held,
            evidence.counters(),
            None,
        ),
        BackgroundPacingOutcome::Denied(evidence) => (
            LatencyEnvelopeAssessmentStatus::Held,
            evidence.counters(),
            None,
        ),
        BackgroundPacingOutcome::Throttled(evidence) => (
            LatencyEnvelopeAssessmentStatus::Held,
            evidence.counters(),
            None,
        ),
        BackgroundPacingOutcome::AdmittedWithDebt(evidence) => (
            LatencyEnvelopeAssessmentStatus::PolicyDebtIncurred,
            evidence.counters(),
            Some(evidence.debt().kind()),
        ),
        BackgroundPacingOutcome::Violation(evidence) => (
            LatencyEnvelopeAssessmentStatus::ExecutionViolated,
            evidence.counters(),
            Some(evidence.causal_debt().kind()),
        ),
    }
}

fn background_counter_rows(
    profile_scope: &'static str,
    lane: QueueWorkClass,
    counters: BackgroundPacingCounterSnapshot,
    debt_kind: Option<BackgroundDebtKind>,
    status: LatencyEnvelopeAssessmentStatus,
) -> Vec<InterferenceCounterRow> {
    let debt_units = total_budget_units(counters.debt_budget());
    let policy_debt_events = counters.admitted_with_debt_events();
    let envelope_exceeded = u64::from(status == LatencyEnvelopeAssessmentStatus::EnvelopeExceeded);
    vec![
        row(
            InterferenceCounterName::BackgroundYieldEvents,
            counters.yield_events(),
            CounterEvidenceStrength::Exact,
            profile_scope,
            lane,
            nonzero(
                counters.yield_events(),
                InterferenceAttribution::BackgroundYield,
            ),
        ),
        row(
            InterferenceCounterName::BackgroundDebtUnits,
            debt_units,
            CounterEvidenceStrength::Exact,
            profile_scope,
            lane,
            background_debt_attribution(debt_units, debt_kind),
        ),
        row(
            InterferenceCounterName::BackgroundViolationEvents,
            counters.violation_events(),
            CounterEvidenceStrength::Exact,
            profile_scope,
            lane,
            nonzero(
                counters.violation_events(),
                InterferenceAttribution::ExecutionViolation,
            ),
        ),
        row(
            InterferenceCounterName::PolicyDebtEvents,
            policy_debt_events,
            CounterEvidenceStrength::Exact,
            profile_scope,
            lane,
            nonzero(policy_debt_events, InterferenceAttribution::PolicyDebt),
        ),
        row(
            InterferenceCounterName::EnvelopeExceededEvents,
            envelope_exceeded,
            CounterEvidenceStrength::Exact,
            profile_scope,
            lane,
            nonzero(envelope_exceeded, InterferenceAttribution::EnvelopeExceeded),
        ),
    ]
}

fn row(
    name: InterferenceCounterName,
    value: u64,
    strength: CounterEvidenceStrength,
    profile: &'static str,
    lane: QueueWorkClass,
    attribution: Option<InterferenceAttribution>,
) -> InterferenceCounterRow {
    InterferenceCounterRow::new(name, value, strength, profile, lane, attribution)
}

fn nonzero(value: u64, attribution: InterferenceAttribution) -> Option<InterferenceAttribution> {
    if value > 0 {
        Some(attribution)
    } else {
        None
    }
}

fn background_debt_attribution(
    value: u64,
    kind: Option<BackgroundDebtKind>,
) -> Option<InterferenceAttribution> {
    if value > 0 {
        kind.map(InterferenceAttribution::BackgroundDebt)
    } else {
        None
    }
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
