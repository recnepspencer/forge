use worth_query::facade::domain::{
    bind_shared_primary_runtime_granular_invalidations,
    perform_prepared_shared_primary_runtime_granular_maintenance,
    prepare_shared_primary_runtime_granular_batch,
    WorthQuerySharedPrimaryGranularMaintenanceOutcome,
    WorthQuerySharedPrimaryGranularSelectionOutcome,
};
use worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationOutcome;

use super::{host::FinancialCourtroomWorld, query};
use crate::production_evidence::{
    CertificationComparatorPolicy, CertificationExecutionLane, OwnerPerformedCounterRows,
    PerformedScenarioEvidence, PerformedScenarioEvidenceParts,
};
use crate::performed_identity_observer::PerformedIdentityObserver;
use crate::world::{GranularInvalidationScenario, GranularInvalidationWorldDefinition};

pub fn run_shared_certification(seed: u64) -> PerformedScenarioEvidence {
    let declared = GranularInvalidationWorldDefinition::for_scenario(
        GranularInvalidationScenario::SharedLeaseDisclosureNoninterference,
        seed,
    );
    let mut host = FinancialCourtroomWorld::publish_curve();
    let mut query = query::build_shared_curve(&host);
    assert!(matches!(
        host.application
            .conditional_clock(&host.curve_clock)
            .unwrap()
            .observe(),
        WorthQueryConditionalClockObservationOutcome::Accepted(_)
    ));
    host.amend_curve(2, 4_250 + seed, 5_100 + seed);
    host.curve_gate.release();
    host.curve_clock_control.push(2, 11);
    let mut receipt = match host
        .application
        .conditional_clock(&host.curve_clock)
        .unwrap()
        .observe()
    {
        WorthQueryConditionalClockObservationOutcome::Accepted(receipt) => receipt,
        _ => panic!("the shared certification mutation must be observed"),
    };
    let batch = receipt.take_granular_invalidation_batch();
    let installation = batch.installation().clone();
    let lower = batch.observation();
    let current = host.application.granular_invalidation_installation();
    let binding = bind_shared_primary_runtime_granular_invalidations(
        &query.subject,
        current.clone(),
    );
    let selected = prepare_shared_primary_runtime_granular_batch(
        &[&query.subject, &query.candidate],
        &mut query.workspace,
        &binding,
        batch,
    )
    .expect("the current shared mutation must select both consumers");
    let WorthQuerySharedPrimaryGranularSelectionOutcome::Prepared(prepared) = selected else {
        panic!("the current shared mutation must prepare maintenance")
    };
    let disposed = match query.candidate.dispose(&mut query.workspace) {
        worth_query::facade::domain::WorthQuerySharedProjectionDisposalOutcome::Disposed(value) => {
            value
        }
        _ => panic!("the governed certification lease must revoke"),
    };
    assert!(!disposed.release().owner_terminal());
    let outcome = perform_prepared_shared_primary_runtime_granular_maintenance(
        prepared,
        &[&query.subject],
        &mut query.workspace,
        &binding,
    )
    .expect("the surviving shared consumer must publish");
    let WorthQuerySharedPrimaryGranularMaintenanceOutcome::Performed(performed) = outcome else {
        panic!("the surviving shared consumer must perform")
    };
    assert_eq!(performed.publications().len(), 1);
    assert_eq!(performed.denied_consumer_count(), 1);
    assert_eq!(performed.maintenance_counters().authorization_denials(), 1);
    let mut observer = PerformedIdentityObserver::default();
    observer
        .observe_impacts(
            &declared.mutations[0],
            host.record_identity(),
            performed.impact_observations(),
            &query.signal_installations,
        )
        .expect("the shared impacts must retain the named source mutation");
    for publication in performed.publications() {
        observer.observe_shared_publication(publication);
    }
    observer.observe_authorization_denials(performed.denied_consumer_count());
    let mut counters = OwnerPerformedCounterRows::default();
    counters.observe(
        lower.bridge_performed(),
        lower.signal_performed(),
        performed.admission_counters(),
        Some(performed.maintenance_counters()),
    );
    PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario: GranularInvalidationScenario::SharedLeaseDisclosureNoninterference,
        seed,
        policy: CertificationComparatorPolicy::Exact,
        diagnostics_tier: query.diagnostics_tier,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: &installation,
        current_installation: &current,
        observer,
        counters,
    })
    .expect("the shared case must retain one runtime lineage")
}
