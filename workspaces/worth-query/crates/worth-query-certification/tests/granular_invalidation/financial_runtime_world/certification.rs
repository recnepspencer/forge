use worth_query::facade::domain::{
    bind_primary_runtime_granular_invalidations, maintain_primary_runtime_granular_batch,
    WorthQueryPrimaryGranularMaintenanceOutcome,
};
use worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationOutcome;
use worth_runtime_bridge::facade::BridgeDiagnosticsTier;

use super::{host::FinancialCourtroomWorld, query};
use crate::production_evidence::{
    CertificationComparatorPolicy, CertificationExecutionLane, OwnerPerformedCounterRows,
    PerformedScenarioEvidence, PerformedScenarioEvidenceParts,
};
use crate::performed_identity_observer::PerformedIdentityObserver;
use crate::world::{GranularInvalidationScenario, GranularInvalidationWorldDefinition};

pub fn assert_mixed_runtime_evidence_denied() {
    let left = FinancialCourtroomWorld::publish_curve();
    let right = FinancialCourtroomWorld::publish_curve();
    let left_installation = left.application.granular_invalidation_installation();
    let right_installation = right.application.granular_invalidation_installation();
    let denied = PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario: GranularInvalidationScenario::CurveDetailToLiveRisk,
        seed: 17,
        policy: CertificationComparatorPolicy::Exact,
        diagnostics_tier: BridgeDiagnosticsTier::Standard,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: &left_installation,
        current_installation: &right_installation,
        observer: PerformedIdentityObserver::default(),
        counters: OwnerPerformedCounterRows::default(),
    });
    assert_eq!(
        denied.expect_err("foreign runtime evidence must not be constructible"),
        "scenario mixed primary runtime installations"
    );
}

pub fn run_curve_certification(seed: u64) -> PerformedScenarioEvidence {
    run_curve_with_query(
        seed,
        GranularInvalidationScenario::CurveDetailToLiveRisk,
        query::build_curve,
    )
}

pub fn run_curve_with_opaque_query_substitution(seed: u64) -> PerformedScenarioEvidence {
    run_curve_with_query(
        seed,
        GranularInvalidationScenario::CurveDetailToLiveRisk,
        query::build_opaque_curve,
    )
}

pub fn run_quote_certification(seed: u64) -> PerformedScenarioEvidence {
    let declared = GranularInvalidationWorldDefinition::for_scenario(
        GranularInvalidationScenario::SuppressedQuoteNoQueryPatch,
        seed,
    );
    let mut host = FinancialCourtroomWorld::publish_quote();
    let mut query = query::build_quote(&host);
    host.quote_gate.release();
    accept_baseline(&mut host, false);
    let current = host.application.granular_invalidation_installation();
    let binding = bind_primary_runtime_granular_invalidations(&query.live, current.clone());
    let small_revision = 2;
    host.amend_quote(small_revision, 102, 5_100 + seed);
    host.quote_clock_control.push(small_revision, 11);
    let mut small_receipt = observe_quote(&mut host);
    let small_batch = small_receipt.take_granular_invalidation_batch();
    let installation = small_batch.installation().clone();
    let small_lower = small_batch.observation();
    let small = maintain_primary_runtime_granular_batch(
        &query.live,
        &mut query.workspace,
        &binding,
        small_batch,
    )
    .expect("the tolerance-suppressed quote is a lawful no-change");
    let WorthQueryPrimaryGranularMaintenanceOutcome::NoRelevantChange(small) = small else {
        panic!("the small quote move must be suppressed")
    };
    assert_eq!(small.suppressed_impact_count(), 1);
    let mut observer = PerformedIdentityObserver::default();
    observer
        .observe_suppression(
            &declared.mutations[0],
            host.record_identity(),
            &small,
            &query.signal_installations,
        )
        .expect("the suppressed quote must retain its source mutation");

    let large_revision = 3;
    host.amend_quote(large_revision, 110, 5_100 + seed);
    host.quote_clock_control.push(large_revision, 12);
    let mut large_receipt = observe_quote(&mut host);
    let large_batch = large_receipt.take_granular_invalidation_batch();
    assert!(current.is_same_current_runtime_as(large_batch.installation()));
    let large_lower = large_batch.observation();
    let large = maintain_primary_runtime_granular_batch(
        &query.live,
        &mut query.workspace,
        &binding,
        large_batch,
    )
    .expect("the meaningful quote move must maintain");
    let WorthQueryPrimaryGranularMaintenanceOutcome::Performed(large) = large else {
        panic!("the meaningful quote move must publish")
    };
    observer
        .observe_impacts(
            &declared.mutations[1],
            host.record_identity(),
            large.impact_observations(),
            &query.signal_installations,
        )
        .expect("the meaningful quote must retain its source mutation");
    observer.observe_primary_publication(&large);
    let mut counters = OwnerPerformedCounterRows::default();
    counters.observe(
        small_lower.bridge_performed(),
        small_lower.signal_performed(),
        small.admission_counters(),
        None,
    );
    counters.observe(
        large_lower.bridge_performed(),
        large_lower.signal_performed(),
        large.admission_counters(),
        Some(large.maintenance_counters()),
    );
    PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario: GranularInvalidationScenario::SuppressedQuoteNoQueryPatch,
        seed,
        policy: CertificationComparatorPolicy::Tolerance {
            epsilon: 5,
            provider_identity: "worth.query.financial.quote-tolerance-5",
        },
        diagnostics_tier: query.diagnostics_tier,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: &installation,
        current_installation: &current,
        observer,
        counters,
    })
    .expect("the quote case must retain one runtime lineage")
}

pub fn run_opaque_certification(seed: u64) -> PerformedScenarioEvidence {
    run_curve_with_query(
        seed,
        GranularInvalidationScenario::OpaqueRegionPlatformTwin,
        query::build_opaque_curve,
    )
}

fn run_curve_with_query(
    seed: u64,
    scenario: GranularInvalidationScenario,
    build_query: fn(&FinancialCourtroomWorld) -> query::FinancialQueryWorld,
) -> PerformedScenarioEvidence {
    let declared = GranularInvalidationWorldDefinition::for_scenario(
        scenario,
        seed,
    );
    let mut host = FinancialCourtroomWorld::publish_curve();
    let mut query = build_query(&host);
    accept_baseline(&mut host, true);
    host.amend_curve(2, 4_250 + seed, 5_100 + seed);
    host.curve_gate.release();
    host.curve_clock_control.push(2, 11);
    let mut receipt = observe_curve(&mut host);
    let batch = receipt.take_granular_invalidation_batch();
    let installation = batch.installation().clone();
    let lower = batch.observation();
    assert_eq!(lower.direct_truth_delivery_count(), 3);
    assert_eq!(lower.signal_performed_delivery_count(), 1);
    let current = host.application.granular_invalidation_installation();
    let binding = bind_primary_runtime_granular_invalidations(&query.live, current.clone());
    let outcome = maintain_primary_runtime_granular_batch(
        &query.live,
        &mut query.workspace,
        &binding,
        batch,
    )
    .expect("the curve certification query must maintain");
    let WorthQueryPrimaryGranularMaintenanceOutcome::Performed(performed) = outcome else {
        panic!("the curve certification query must publish")
    };
    assert_eq!(performed.admitted_impact_count(), 3);
    assert_eq!(performed.consumer_publication_count(), 1);
    let mut observer = PerformedIdentityObserver::default();
    observer
        .observe_impacts(
            &declared.mutations[0],
            host.record_identity(),
            performed.impact_observations(),
            &query.signal_installations,
        )
        .expect("the opaque impacts must retain the named source mutation");
    observer.observe_primary_publication(&performed);
    let mut counters = OwnerPerformedCounterRows::default();
    counters.observe(
        lower.bridge_performed(),
        lower.signal_performed(),
        performed.admission_counters(),
        Some(performed.maintenance_counters()),
    );
    PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario,
        seed,
        policy: CertificationComparatorPolicy::Exact,
        diagnostics_tier: query.diagnostics_tier,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: &installation,
        current_installation: &current,
        observer,
        counters,
    })
    .expect("the curve certification query must retain one runtime lineage")
}

fn accept_baseline(host: &mut FinancialCourtroomWorld, curve: bool) {
    let outcome = if curve {
        host.application
            .conditional_clock(&host.curve_clock)
            .unwrap()
            .observe()
    } else {
        host.application
            .conditional_clock(&host.quote_clock)
            .unwrap()
            .observe()
    };
    assert!(matches!(
        outcome,
        WorthQueryConditionalClockObservationOutcome::Accepted(_)
    ));
}

fn observe_curve(
    host: &mut FinancialCourtroomWorld,
) -> worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationReceipt<
    crate::adapters::CourtroomClock,
> {
    match host
        .application
        .conditional_clock(&host.curve_clock)
        .unwrap()
        .observe()
    {
        WorthQueryConditionalClockObservationOutcome::Accepted(receipt) => receipt,
        _ => panic!("the due curve certification observation must be accepted"),
    }
}

fn observe_quote(
    host: &mut FinancialCourtroomWorld,
) -> worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationReceipt<
    crate::adapters::CourtroomClock,
> {
    match host
        .application
        .conditional_clock(&host.quote_clock)
        .unwrap()
        .observe()
    {
        WorthQueryConditionalClockObservationOutcome::Accepted(receipt) => receipt,
        _ => panic!("the due quote certification observation must be accepted"),
    }
}
