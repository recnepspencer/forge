use worth_query::facade::domain::{
    bind_primary_runtime_granular_invalidations,
    maintain_primary_runtime_granular_collection_batch,
    WorthQueryPrimaryGranularMaintenanceOutcome, WorthQueryPrimaryGranularMaintenancePerformed,
    WorthQueryPrimaryRuntimeInvalidationBinding, WorthQuerySemanticDependencyRole as Role,
};
use worth_query_execution::facade::primary_graph::{
    WorthQueryGranularInvalidationInstallation, WorthQueryGranularInvalidationObservation,
};
use worth_query_host::facade::primary_graph::WorthQueryConditionalClockObservationOutcome;

use super::{host::FinancialCourtroomWorld, query};
use crate::production_evidence::{
    CertificationComparatorPolicy, CertificationExecutionLane, OwnerPerformedCounterRows,
    PerformedScenarioEvidence, PerformedScenarioEvidenceParts,
};
use crate::performed_identity_observer::PerformedIdentityObserver;
use crate::world::{GranularInvalidationScenario, GranularInvalidationWorldDefinition};

pub fn run_portfolio_certification(seed: u64) -> PerformedScenarioEvidence {
    let declared = GranularInvalidationWorldDefinition::for_scenario(
        GranularInvalidationScenario::OrderedPortfolioMembership,
        seed,
    );
    run_portfolio_certification_with_world(seed, declared)
}

pub fn run_portfolio_with_relational_record_substitution(
    seed: u64,
) -> (
    GranularInvalidationWorldDefinition,
    PerformedScenarioEvidence,
) {
    let mut declared = GranularInvalidationWorldDefinition::for_scenario(
        GranularInvalidationScenario::OrderedPortfolioMembership,
        seed,
    );
    declared.mutations[0].relational_record_identity =
        worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 99, 1);
    let evidence = run_portfolio_certification_with_world(seed, declared.clone());
    (declared, evidence)
}

fn run_portfolio_certification_with_world(
    seed: u64,
    declared: GranularInvalidationWorldDefinition,
) -> PerformedScenarioEvidence {
    let mut host = FinancialCourtroomWorld::publish_portfolio();
    let mut query = query::build_portfolio_with_unrelated_rows(&host, 64);
    assert!(matches!(
        host.application
            .conditional_clock(&host.portfolio_clock)
            .unwrap()
            .observe(),
        WorthQueryConditionalClockObservationOutcome::Accepted(_)
    ));
    host.portfolio_gate.release();
    let current = host.application.granular_invalidation_installation();
    let binding =
        bind_primary_runtime_granular_invalidations(&query.live, current.clone());
    let mut counters = OwnerPerformedCounterRows::default();
    let mut installation = None;
    let mut observer = PerformedIdentityObserver::default();

    let steps = [
        PortfolioStep::Value(2, 5_100 + seed),
        PortfolioStep::Value(3, 5_100),
        PortfolioStep::Desk(4, "credit"),
        PortfolioStep::Desk(5, "rates"),
        PortfolioStep::Rank(6, 3),
        PortfolioStep::Rank(7, 100_000 + seed),
    ];
    for (step, mutation) in steps.into_iter().zip(&declared.mutations) {
        step.apply(&mut host);
        let (step_installation, lower, performed) =
            perform(&mut host, &mut query, &binding);
        installation.get_or_insert(step_installation);
        assert_step_roles(step, &performed);
        observer
            .observe_impacts(
                mutation,
                host.record_identity(),
                performed.impact_observations(),
                &query.signal_installations,
            )
            .expect("the portfolio impact must retain its named source mutation");
        observer.observe_primary_publication(&performed);
        counters.observe(
            lower.bridge_performed(),
            lower.signal_performed(),
            performed.admission_counters(),
            Some(performed.maintenance_counters()),
        );
    }
    PerformedScenarioEvidence::from_performed(PerformedScenarioEvidenceParts {
        scenario: GranularInvalidationScenario::OrderedPortfolioMembership,
        seed,
        policy: CertificationComparatorPolicy::Exact,
        diagnostics_tier: query.diagnostics_tier,
        execution_lane: CertificationExecutionLane::Scheduled,
        batch_installation: installation.as_ref().expect("portfolio installation"),
        current_installation: &current,
        observer,
        counters,
    })
    .expect("the portfolio case must retain one runtime lineage")
}

#[derive(Clone, Copy)]
enum PortfolioStep {
    Value(u64, u64),
    Desk(u64, &'static str),
    Rank(u64, u64),
}

impl PortfolioStep {
    fn apply(self, host: &mut FinancialCourtroomWorld) {
        let revision = match self {
            Self::Value(revision, value) => {
                host.amend_portfolio_value(revision, value);
                revision
            }
            Self::Desk(revision, desk) => {
                host.amend_portfolio_desk(revision, desk);
                revision
            }
            Self::Rank(revision, rank) => {
                host.amend_portfolio_rank(revision, rank);
                revision
            }
        };
        host.portfolio_clock_control.push(revision, revision + 9);
    }
}

fn perform(
    host: &mut FinancialCourtroomWorld,
    query: &mut query::FinancialQueryWorld,
    binding: &WorthQueryPrimaryRuntimeInvalidationBinding,
) -> (
    WorthQueryGranularInvalidationInstallation,
    WorthQueryGranularInvalidationObservation,
    WorthQueryPrimaryGranularMaintenancePerformed,
) {
    let WorthQueryConditionalClockObservationOutcome::Accepted(mut receipt) = host
        .application
        .conditional_clock(&host.portfolio_clock)
        .unwrap()
        .observe()
    else {
        panic!("the current portfolio certification mutation must be observed")
    };
    let batch = receipt.take_granular_invalidation_batch();
    let installation = batch.installation().clone();
    let lower = batch.observation();
    let outcome = maintain_primary_runtime_granular_collection_batch(
        &query.live,
        query.collection.as_mut().expect("portfolio collection state"),
        &mut query.workspace,
        binding,
        batch,
    )
    .expect("the portfolio certification mutation must maintain");
    let WorthQueryPrimaryGranularMaintenanceOutcome::Performed(performed) = outcome else {
        panic!("the portfolio certification mutation must publish")
    };
    (installation, lower, performed)
}

fn assert_step_roles(step: PortfolioStep, performed: &WorthQueryPrimaryGranularMaintenancePerformed) {
    let roles = performed.deliveries()[0].roles();
    let expected: &[Role] = match step {
        PortfolioStep::Value(..) => &[Role::ProjectedValue],
        PortfolioStep::Desk(..) => &[
            Role::ProjectedValue,
            Role::SelectionOrMembership,
            Role::Grouping,
        ],
        PortfolioStep::Rank(..) => &[
            Role::ProjectedValue,
            Role::Ordering,
            Role::WindowBoundary,
        ],
    };
    assert!(expected.iter().all(|role| roles.contains(role)));
    assert_eq!(performed.consumer_publication_count(), 1);
}
