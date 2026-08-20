use worth_query::facade::domain::{
    bind_primary_runtime_granular_invalidations, maintain_primary_runtime_granular_batch,
    maintain_primary_runtime_granular_invalidations, WorthQueryPrimaryGranularMaintenanceDenial,
    WorthQueryPrimaryGranularMaintenanceOutcome,
};
use worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationOutcome;

use crate::host_world::CourtroomWorld;
use crate::performed_identity_observer::PerformedIdentityObserver;
use crate::production_evidence::{
    CertificationComparatorPolicy, CertificationExecutionLane, OwnerPerformedCounterRows,
    PerformedScenarioEvidence, PerformedScenarioEvidenceParts,
};
use crate::query_runtime_world::{build_primary_query_world, IntentSourceProjection};
use crate::world::{GranularInvalidationScenario, GranularInvalidationWorldDefinition};

pub fn run_correspondence_certification(seed: u64) -> PerformedScenarioEvidence {
    let declared = GranularInvalidationWorldDefinition::for_scenario(
        GranularInvalidationScenario::CorrespondenceRebindRestore,
        seed,
    );
    let mut identity_observer = PerformedIdentityObserver::default();
    let mut host = CourtroomWorld::publish("blocked");
    let mut query = build_primary_query_world(&host);
    let old_binding = bind_primary_runtime_granular_invalidations(
        &query.live,
        host.application.granular_invalidation_installation(),
    );
    let mut baseline = observe(&mut host);
    assert!(baseline.take_granular_invalidation_batch().is_empty());
    host.amend_intent(1, "active", &format!("waiting-{seed}"));
    let mut delayed = observe(&mut host);
    let delayed_batch = delayed.take_granular_invalidation_batch();
    identity_observer
        .observe_lower_truth(
            &declared.mutations[0],
            host.intent_record_identity(),
            &delayed_batch,
        )
        .expect("the delayed batch must retain its original source mutation");

    let reconstruction = host
        .application
        .reinstall_conditional_runtime()
        .expect("the current runtime must reconstruct");
    let current = host.application.granular_invalidation_installation();
    query
        .workspace
        .rebind_primary_graph_source(
            &reconstruction,
            IntentSourceProjection::new(
                host.intent_record_identity(),
                std::sync::Arc::clone(&query.observations),
            ),
        )
        .expect("Query must rebind to the reconstructed source");
    let current_binding = bind_primary_runtime_granular_invalidations(&query.live, current.clone());
    assert!(matches!(
        maintain_primary_runtime_granular_batch(
            &query.live,
            &mut query.workspace,
            &current_binding,
            delayed_batch,
        ),
        Err(WorthQueryPrimaryGranularMaintenanceDenial::Admission(_))
    ));
    identity_observer.observe_rebind_denial("stale-batch");

    host.amend_intent(2, "active", "ready");
    let mut current_observation = observe(&mut host);
    assert!(matches!(
        maintain_primary_runtime_granular_invalidations(
            &query.live,
            &mut query.workspace,
            &old_binding,
            &mut current_observation,
        ),
        Err(WorthQueryPrimaryGranularMaintenanceDenial::ForeignPrimaryRuntime)
    ));
    identity_observer.observe_rebind_denial("old-binding");
    let batch = current_observation.take_granular_invalidation_batch();
    let installation = batch.installation().clone();
    let lower = batch.observation();
    let outcome = maintain_primary_runtime_granular_batch(
        &query.live,
        &mut query.workspace,
        &current_binding,
        batch,
    )
    .expect("the rebound current delivery must maintain");
    let WorthQueryPrimaryGranularMaintenanceOutcome::Performed(performed) = outcome else {
        panic!("the rebound current delivery must publish")
    };
    identity_observer
        .observe_impacts(
            &declared.mutations[1],
            host.intent_record_identity(),
            performed.impact_observations(),
            &query.signal_installations,
        )
        .expect("the restored impact must retain the current source mutation");
    identity_observer.observe_primary_publication(&performed);
    let mut counters = OwnerPerformedCounterRows::default();
    counters.observe(
        lower.bridge_performed(),
        lower.signal_performed(),
        performed.admission_counters(),
        Some(performed.maintenance_counters()),
    );
    PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario: GranularInvalidationScenario::CorrespondenceRebindRestore,
        seed,
        policy: CertificationComparatorPolicy::Exact,
        diagnostics_tier: query.diagnostics_tier,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: &installation,
        current_installation: &current,
        observer: identity_observer,
        counters,
    })
    .expect("the correspondence case must retain its restored runtime lineage")
}

fn observe(
    world: &mut CourtroomWorld,
) -> worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationReceipt<
    crate::adapters::CourtroomClock,
> {
    match world
        .application
        .conditional_clock(&world.clock)
        .unwrap()
        .observe()
    {
        WorthQueryConditionalClockObservationOutcome::Accepted(receipt) => receipt,
        _ => panic!("the due lifecycle certification observation must be accepted"),
    }
}
