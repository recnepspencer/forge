use worth_store_buffer_pool::AllocationScope;
use worth_store_physical_certification::{
    admit_physical_counter_evidence, reject_hostile_counter_evidence_for_readmission,
    CounterContractKind, CounterExpectationKind, CounterMismatchEvidence,
    CounterStrengthJustification, CounterStrengthPosture, HostileCounterEvidenceRow,
    HostileResourceEnvelopeObservation, PhysicalCounterContract, PhysicalCounterExpectation,
    PhysicalExecutedCounterEvidence, PhysicalInterleavingSchedule, ReplaySeed, StateSpaceBudget,
};

use worth_store_test_support::harness::recovery::counter_evidence as support;

use support::{
    assert_counter, executed_counter_evidence, execution_sources_for_plan,
    execution_sources_with_schedule, hostile_resource_observation_within_envelope,
    hostile_satisfied_rows, lower_physical_isolation_plan, lower_physical_isolation_shortcut_plan,
    lower_shortcut_plan, observed_trace, replace_row, shortcut_trace,
};

#[test]
fn lowered_plan_uses_weakest_sufficient_counter_strengths() {
    let plan = lower_physical_isolation_plan();
    assert_counter(
        &plan,
        CounterContractKind::ActorStepExact,
        CounterExpectationKind::Exact,
        Some(2),
        CounterStrengthPosture::ExactnessIsClaim,
        CounterStrengthJustification::DeterministicEventStructure,
    );
    assert_counter(
        &plan,
        CounterContractKind::ReplayIdentityExact,
        CounterExpectationKind::Exact,
        Some(1),
        CounterStrengthPosture::ExactnessIsClaim,
        CounterStrengthJustification::ReplayIdentity,
    );
    assert_counter(
        &plan,
        CounterContractKind::ProfileResourceEnvelope,
        CounterExpectationKind::ProfileScoped,
        None,
        CounterStrengthPosture::WeakestSufficient,
        CounterStrengthJustification::ProfileResourceEnvelope,
    );
    assert_counter(
        &plan,
        CounterContractKind::AllocationBytes,
        CounterExpectationKind::Bounded,
        Some(64 * 1024),
        CounterStrengthPosture::WeakestSufficient,
        CounterStrengthJustification::ImplementationSensitiveCost,
    );
    assert_counter(
        &plan,
        CounterContractKind::Retries,
        CounterExpectationKind::Monotonic,
        None,
        CounterStrengthPosture::WeakestSufficient,
        CounterStrengthJustification::ImplementationSensitiveCost,
    );
}

#[test]
fn forbidden_behavior_and_structure_counters_are_exact_claims() {
    let shortcut_contract_plan = lower_shortcut_plan();
    assert_counter(
        &shortcut_contract_plan,
        CounterContractKind::ForbiddenShortcutExact,
        CounterExpectationKind::Exact,
        Some(0),
        CounterStrengthPosture::ExactnessIsClaim,
        CounterStrengthJustification::ForbiddenBehaviorMustRemainZero,
    );
    assert_counter(
        &shortcut_contract_plan,
        CounterContractKind::ReplayIdentityExact,
        CounterExpectationKind::Exact,
        Some(1),
        CounterStrengthPosture::ExactnessIsClaim,
        CounterStrengthJustification::ReplayIdentity,
    );
    let plan = lower_physical_isolation_shortcut_plan();
    assert_counter(
        &plan,
        CounterContractKind::BlockedReclaimAttempts,
        CounterExpectationKind::Positive,
        None,
        CounterStrengthPosture::WeakestSufficient,
        CounterStrengthJustification::ImplementationSensitiveCost,
    );
    let receipt = admit_physical_counter_evidence(
        &plan,
        executed_counter_evidence(&plan, shortcut_trace(&plan)),
    )
    .unwrap();
    let blocked = receipt
        .rows()
        .iter()
        .find(|row| row.kind() == CounterContractKind::BlockedReclaimAttempts)
        .expect("shortcut plan must emit blocked-attempt evidence");
    assert_eq!(blocked.observed_count(), 1);
}

#[test]
fn admitted_counter_rows_package_foundational_counter_backed_receipt() {
    let plan = lower_physical_isolation_plan();
    let evidence = executed_counter_evidence(&plan, observed_trace(&plan));
    let receipt = admit_physical_counter_evidence(&plan, evidence).unwrap();
    assert_eq!(
        receipt.rows().len(),
        plan.counter_contracts().iter().count()
    );
    assert_eq!(
        receipt.foundational_receipt().counter_rows().len(),
        receipt.rows().len()
    );
}

#[test]
fn executed_counter_sources_deny_mixed_plan_schedule_identity() {
    let plan = lower_physical_isolation_plan();
    let other_plan = lower_physical_isolation_shortcut_plan();
    let other_schedule = PhysicalInterleavingSchedule::from_lowered_plan(
        &other_plan,
        ReplaySeed::required(Some(8)).unwrap(),
        StateSpaceBudget::bounded_steps(8).unwrap(),
    )
    .unwrap();

    let denial = execution_sources_with_schedule(&plan, &other_schedule, observed_trace(&plan))
        .expect_err("schedule replay identity from another plan must not satisfy source binding");

    assert_eq!(
        denial,
        CounterMismatchEvidence::ExecutedEvidencePlanMismatch
    );
}

#[test]
fn executed_counter_evidence_denies_source_bundle_reused_for_another_plan() {
    let source_plan = lower_physical_isolation_shortcut_plan();
    let target_plan = lower_physical_isolation_plan();
    let sources = execution_sources_for_plan(&source_plan, observed_trace(&source_plan)).unwrap();

    let denial = PhysicalExecutedCounterEvidence::from_execution_sources(&target_plan, sources)
        .expect_err("plan-bound source bundle must not certify another plan");

    assert_eq!(
        denial,
        CounterMismatchEvidence::ExecutedEvidencePlanMismatch
    );
}

#[test]
fn counter_evidence_denies_missing_duplicate_unexpected_and_under_strength_rows() {
    let plan = lower_physical_isolation_plan();
    let mut rows = hostile_satisfied_rows(&plan);
    rows.retain(|row| row.kind() != CounterContractKind::ReplayIdentityExact);
    let missing = reject_hostile_counter_evidence_for_readmission(
        &plan,
        rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        missing,
        CounterMismatchEvidence::MissingCounterSpec {
            kind: CounterContractKind::ReplayIdentityExact,
        }
    );

    let mut duplicate_rows = hostile_satisfied_rows(&plan);
    duplicate_rows.push(HostileCounterEvidenceRow::new(
        CounterContractKind::ActorStepExact,
        CounterExpectationKind::Exact,
        2,
    ));
    let duplicate = reject_hostile_counter_evidence_for_readmission(
        &plan,
        duplicate_rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        duplicate,
        CounterMismatchEvidence::DuplicateCounterRow {
            kind: CounterContractKind::ActorStepExact,
        }
    );

    let mut unexpected_rows = hostile_satisfied_rows(&plan);
    unexpected_rows.push(HostileCounterEvidenceRow::new(
        CounterContractKind::ResidentBytes,
        CounterExpectationKind::Bounded,
        1,
    ));
    let unexpected = reject_hostile_counter_evidence_for_readmission(
        &plan,
        unexpected_rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        unexpected,
        CounterMismatchEvidence::UnexpectedCounterRow {
            kind: CounterContractKind::ResidentBytes,
        }
    );

    let mut under_strength_rows = hostile_satisfied_rows(&plan);
    replace_row(
        &mut under_strength_rows,
        HostileCounterEvidenceRow::new(
            CounterContractKind::ActorStepExact,
            CounterExpectationKind::Positive,
            2,
        ),
    );
    let under_strength = reject_hostile_counter_evidence_for_readmission(
        &plan,
        under_strength_rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        under_strength,
        CounterMismatchEvidence::UnderStrengthEvidence {
            kind: CounterContractKind::ActorStepExact,
            required: CounterExpectationKind::Exact,
            actual: CounterExpectationKind::Positive,
        }
    );
}

#[test]
fn counter_evidence_denies_value_and_envelope_violations() {
    let plan = lower_shortcut_plan();
    let mut rows = hostile_satisfied_rows(&plan);
    replace_row(
        &mut rows,
        HostileCounterEvidenceRow::new(
            CounterContractKind::ForbiddenShortcutExact,
            CounterExpectationKind::Exact,
            1,
        ),
    );
    let nonzero_forbidden = reject_hostile_counter_evidence_for_readmission(
        &plan,
        rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        nonzero_forbidden,
        CounterMismatchEvidence::NonZeroForbiddenCounter {
            kind: CounterContractKind::ForbiddenShortcutExact,
            actual: 1,
        }
    );

    let plan = lower_physical_isolation_plan();
    let mut rows = hostile_satisfied_rows(&plan);
    replace_row(
        &mut rows,
        HostileCounterEvidenceRow::new(
            CounterContractKind::AllocationBytes,
            CounterExpectationKind::Bounded,
            64 * 1024 + 1,
        ),
    );
    let over_bound = reject_hostile_counter_evidence_for_readmission(
        &plan,
        rows,
        hostile_resource_observation_within_envelope(&plan),
    )
    .unwrap_err();
    assert_eq!(
        over_bound,
        CounterMismatchEvidence::BoundedCounterExceeded {
            kind: CounterContractKind::AllocationBytes,
            maximum: 64 * 1024,
            actual: 64 * 1024 + 1,
        }
    );

    let envelope = plan.resource_envelope();
    let envelope_denial = reject_hostile_counter_evidence_for_readmission(
        &plan,
        hostile_satisfied_rows(&plan),
        HostileResourceEnvelopeObservation::new(
            plan.profile(),
            envelope
                .allocation()
                .budget(AllocationScope::Foreground)
                .as_bytes(),
            envelope.resident_bytes().as_bytes(),
            u64::from(envelope.max_pinned_pages()) + 1,
            u64::from(envelope.max_dirty_pages()),
            u64::from(envelope.io_queue().max_queue_depth()),
            u64::from(envelope.io_queue().max_interference_events()),
        ),
    )
    .unwrap_err();
    assert_eq!(
        envelope_denial,
        CounterMismatchEvidence::ResourceEnvelopeExceeded {
            kind: CounterContractKind::PagePins,
            maximum: u64::from(envelope.max_pinned_pages()),
            actual: u64::from(envelope.max_pinned_pages()) + 1,
        }
    );
}

#[test]
fn over_exact_implementation_sensitive_cost_counters_deny() {
    for kind in [
        CounterContractKind::AllocationBytes,
        CounterContractKind::PagePins,
        CounterContractKind::IoQueueDepth,
        CounterContractKind::ResidentBytes,
        CounterContractKind::DirtyPages,
        CounterContractKind::IoInterferenceEvents,
        CounterContractKind::LatchWaits,
        CounterContractKind::Retries,
        CounterContractKind::BlockedReclaimAttempts,
        CounterContractKind::ReplayedPages,
    ] {
        let denial = PhysicalCounterContract::try_new(kind, PhysicalCounterExpectation::exact(1))
            .expect_err("implementation-sensitive cost exactness must be denied");

        assert_eq!(denial.kind(), kind);
    }
}
