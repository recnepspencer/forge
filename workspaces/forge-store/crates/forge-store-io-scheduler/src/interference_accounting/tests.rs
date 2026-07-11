use forge_store_budgets::CounterEvidenceStrength;
use forge_store_physical_backend::{
    BackendQueueExecutionBackpressure, BackendQueueExecutionPlanBinding, BackendTargetProfile,
    CapabilityEvidenceClass,
};

use crate::queue_execution::test_support::{
    admitted_plan, completion_for_binding, completion_for_plan, speculative_scope,
};
use crate::{
    assess_queue_latency_envelope, execute_ready_queue_plan, InterferenceAttribution,
    InterferenceCounterDenial, InterferenceCounterName, InterferenceCounterRequirement,
    InterferenceCounterRow, InterferenceReplayScope, LatencyEnvelopeAssessment,
    LatencyEnvelopeAssessmentStatus, LatencyEnvelopeClaim, QueueBackpressureCause,
};

#[test]
fn queue_execution_assessment_reports_exact_strength_for_every_counter_row() {
    let plan = admitted_plan();
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        plan.replay_identity(),
        "s6-profile/posix-file",
        plan.work().class(),
    )
    .require_counter(InterferenceCounterRequirement::queue_depth());
    let scope = speculative_scope(&plan);
    let completion = completion_for_plan(&plan, 1, Some(scope), 0, None)
        .observe_queue_depth(3)
        .complete();
    let outcome = execute_ready_queue_plan(plan, completion);

    let assessment = assess_queue_latency_envelope(&claim, &outcome)
        .expect("sampled queue-depth claim should assess through real queue execution");

    assert_eq!(assessment.status(), LatencyEnvelopeAssessmentStatus::Held);
    let queue_depth = assessment
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::QueuePeakDepth)
        .expect("queue depth row should be materialized");
    assert_eq!(queue_depth.strength(), CounterEvidenceStrength::Sampled);
    assert_eq!(
        queue_depth.attribution(),
        Some(InterferenceAttribution::Queueing)
    );
    assert_held_queue_counter_topology(&assessment);
}

#[test]
fn sampled_counter_cannot_satisfy_exact_latency_claim() {
    let plan = admitted_plan();
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        plan.replay_identity(),
        "s6-profile/posix-file",
        plan.work().class(),
    )
    .require_counter(
        InterferenceCounterRequirement::queue_depth().with_strength(CounterEvidenceStrength::Exact),
    );
    let completion = completion_for_plan(&plan, 0, None, 0, None)
        .observe_queue_depth(2)
        .complete();
    let outcome = execute_ready_queue_plan(plan, completion);

    let denial = assess_queue_latency_envelope(&claim, &outcome)
        .expect_err("sampled queue depth must not satisfy an exact claim");

    assert_eq!(
        denial,
        InterferenceCounterDenial::InsufficientCounterStrength {
            counter: InterferenceCounterName::QueuePeakDepth,
            required: CounterEvidenceStrength::Exact,
            actual: CounterEvidenceStrength::Sampled,
        }
    );
}

#[test]
fn post_admission_latency_violation_has_causal_attribution() {
    let plan = admitted_plan();
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        plan.replay_identity(),
        "s6-profile/posix-file",
        plan.work().class(),
    )
    .with_max_interference_events(0)
    .require_counter(InterferenceCounterRequirement::foreground_wait().requiring_attribution());
    let completion = completion_for_plan(&plan, 0, None, 0, None)
        .observe_queue_depth(2)
        .observe_foreground_wait_events(1)
        .observe_backpressure(BackendQueueExecutionBackpressure::QueueDepthSaturated)
        .complete();
    let outcome = execute_ready_queue_plan(plan, completion);

    let assessment = assess_queue_latency_envelope(&claim, &outcome)
        .expect("post-admission backpressure should assess as causal violation");

    assert_eq!(
        assessment.status(),
        LatencyEnvelopeAssessmentStatus::EnvelopeExceeded
    );
    let foreground_wait = assessment
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::QueueForegroundWaitEvents)
        .expect("foreground wait row should be materialized");
    assert_eq!(
        foreground_wait.attribution(),
        Some(InterferenceAttribution::ForegroundWait)
    );
    let envelope = assessment
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::EnvelopeExceededEvents)
        .expect("envelope-exceeded row should be materialized");
    assert_eq!(envelope.value(), 1);
    assert_eq!(
        envelope.attribution(),
        Some(InterferenceAttribution::EnvelopeExceeded)
    );
    assert_counter(
        &assessment,
        InterferenceCounterName::QueueBackpressureEvents,
        1,
        CounterEvidenceStrength::Exact,
        Some(InterferenceAttribution::Backpressure(
            QueueBackpressureCause::QueueDepthSaturated,
        )),
    );
}

#[test]
fn backend_contradiction_is_typed_post_admission_violation() {
    let plan = admitted_plan();
    let binding = plan
        .backend_completion_binding()
        .backend_execution_binding();
    let wrong_binding = BackendQueueExecutionPlanBinding::from_store_replay_binding(
        binding.primary(),
        binding.secondary(),
        BackendTargetProfile::SimulatedStrictDurable,
        CapabilityEvidenceClass::CertifiedBackendProfile,
        binding.grouped_writes(),
    );
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        plan.replay_identity(),
        "s6-profile/posix-file",
        plan.work().class(),
    )
    .require_counter(
        InterferenceCounterRequirement::new(
            InterferenceCounterName::BackendContradictionEvents,
            CounterEvidenceStrength::Exact,
        )
        .requiring_attribution(),
    );
    let completion = completion_for_binding(wrong_binding, 0, None, 0, None).complete();
    let outcome = execute_ready_queue_plan(plan, completion);

    let assessment = assess_queue_latency_envelope(&claim, &outcome)
        .expect("backend contradiction should carry typed causal attribution");

    assert_eq!(
        assessment.status(),
        LatencyEnvelopeAssessmentStatus::BackendContradictedWitness
    );
    let contradiction = assessment
        .counter_rows()
        .iter()
        .find(|row| row.name() == InterferenceCounterName::BackendContradictionEvents)
        .expect("backend contradiction row should be materialized");
    assert_eq!(contradiction.value(), 1);
    assert_eq!(
        contradiction.attribution(),
        Some(InterferenceAttribution::BackendContradictedWitness)
    );
}

#[test]
fn replay_scope_is_policy_counter_and_proof_deterministic_only() {
    let replay_scope = InterferenceReplayScope::deterministic_policy_counter_and_proof_scope();

    assert!(replay_scope.includes_policy_decisions());
    assert!(replay_scope.includes_counter_topology());
    assert!(replay_scope.includes_proof_progression());
    assert!(replay_scope.excludes_wall_clock_timing());
    assert!(replay_scope.excludes_os_completion_order());
}

#[test]
fn repeated_queue_assessment_preserves_counter_topology_deterministically() {
    let first = deterministic_assessment();
    let second = deterministic_assessment();

    assert_eq!(first.status(), second.status());
    assert_eq!(first.replay_identity(), second.replay_identity());
    assert_eq!(first.replay_scope(), second.replay_scope());
    assert_eq!(first.counter_rows(), second.counter_rows());
}

fn deterministic_assessment() -> LatencyEnvelopeAssessment {
    let plan = admitted_plan();
    let claim = LatencyEnvelopeClaim::for_queue_execution(
        plan.replay_identity(),
        "s6-profile/posix-file",
        plan.work().class(),
    )
    .require_counter(InterferenceCounterRequirement::queue_depth());
    let scope = speculative_scope(&plan);
    let completion = completion_for_plan(&plan, 1, Some(scope), 0, None)
        .observe_queue_depth(3)
        .complete();
    let outcome = execute_ready_queue_plan(plan, completion);

    assess_queue_latency_envelope(&claim, &outcome)
        .expect("deterministic queue assessment should succeed")
}

fn assert_held_queue_counter_topology(assessment: &LatencyEnvelopeAssessment) {
    assert_eq!(assessment.counter_rows().len(), 20);
    assert_counter(
        assessment,
        InterferenceCounterName::QueueSubmittedUnits,
        4100,
        CounterEvidenceStrength::Exact,
        None,
    );
    assert_counter(
        assessment,
        InterferenceCounterName::QueueAdmittedUnits,
        4100,
        CounterEvidenceStrength::Exact,
        None,
    );
    assert_counter(
        assessment,
        InterferenceCounterName::QueueDeniedUnits,
        0,
        CounterEvidenceStrength::Exact,
        Some(InterferenceAttribution::Backpressure(
            QueueBackpressureCause::QueueDepthSaturated,
        )),
    );
    assert_counter(
        assessment,
        InterferenceCounterName::QueuePeakDepth,
        3,
        CounterEvidenceStrength::Sampled,
        Some(InterferenceAttribution::Queueing),
    );
    assert_counter(
        assessment,
        InterferenceCounterName::QueueReadAheadUnits,
        1,
        CounterEvidenceStrength::Exact,
        None,
    );
    for name in [
        InterferenceCounterName::QueueGroupedWrites,
        InterferenceCounterName::QueueWriteBackUnits,
        InterferenceCounterName::QueueMechanicalRetries,
        InterferenceCounterName::QueuePartialReadEvents,
        InterferenceCounterName::QueueShortWriteEvents,
        InterferenceCounterName::QueueViolationEvents,
        InterferenceCounterName::FlushDelayEvents,
        InterferenceCounterName::SyncDebtUnits,
        InterferenceCounterName::BackendContradictionEvents,
        InterferenceCounterName::EnvelopeExceededEvents,
    ] {
        assert_counter(assessment, name, 0, CounterEvidenceStrength::Exact, None);
    }
    assert_counter(
        assessment,
        InterferenceCounterName::QueueBackpressureEvents,
        1,
        CounterEvidenceStrength::Exact,
        Some(InterferenceAttribution::Backpressure(
            QueueBackpressureCause::QueueDepthSaturated,
        )),
    );
    assert_counter(
        assessment,
        InterferenceCounterName::QueueForegroundWaitEvents,
        0,
        CounterEvidenceStrength::Exact,
        Some(InterferenceAttribution::ForegroundWait),
    );
    for name in [
        InterferenceCounterName::PageCacheWaitEvents,
        InterferenceCounterName::WorkerHandoffWaitEvents,
        InterferenceCounterName::PolicyDebtEvents,
    ] {
        assert_counter(
            assessment,
            name,
            0,
            CounterEvidenceStrength::Unavailable,
            None,
        );
    }
}

fn assert_counter(
    assessment: &LatencyEnvelopeAssessment,
    name: InterferenceCounterName,
    value: u64,
    strength: CounterEvidenceStrength,
    attribution: Option<InterferenceAttribution>,
) {
    let row = counter_row(assessment, name);
    assert_eq!(row.value(), value, "{name:?} value drifted");
    assert_eq!(row.strength(), strength, "{name:?} strength drifted");
    assert_eq!(
        row.attribution(),
        attribution,
        "{name:?} attribution drifted"
    );
}

fn counter_row(
    assessment: &LatencyEnvelopeAssessment,
    name: InterferenceCounterName,
) -> InterferenceCounterRow {
    *assessment
        .counter_rows()
        .iter()
        .find(|row| row.name() == name)
        .unwrap_or_else(|| panic!("{name:?} row should be materialized"))
}
