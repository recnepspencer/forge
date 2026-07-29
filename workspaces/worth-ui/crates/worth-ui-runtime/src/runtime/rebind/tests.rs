use super::{
    UiChangeProfile, UiRebindBudgetInput, UiRebindConcurrencyInput, UiRebindLimit, UiRebindProfile,
    UiRebindProfileConstructionDenial,
};
use crate::runtime::observation::{UiObservationProfile, UiObservationProfileInput};

#[test]
fn platform_pulse_change_profile_has_the_governed_exact_limits() {
    let profile = UiChangeProfile::platform_pulse();
    let observation = profile.observation();
    let budget = profile.rebind().budget();
    let concurrency = profile.rebind().concurrency();

    assert_eq!(profile.revision(), 1);
    assert_eq!(observation.admitted_per_turn(), 8);
    assert_eq!(observation.retained_bytes_per_turn(), 65_536);
    assert_eq!(observation.queued_during_effecting_rebind(), 16);
    assert_eq!(
        budget,
        UiRebindBudgetInput {
            changed_facts: 16,
            affected_aspects: 16,
            distinct_consumers: 64,
            graph_and_mounted_entries: 128,
            measurement_and_allocation_entries: 64,
            query_binding_transitions: 16,
            obligations: 64,
            native_surfaces: 1,
            prepared_presentation_bytes: 4_194_304,
            terminal_decision_records: 64,
            evidence_linkage_entries: 512,
            causal_neighborhood_bytes: 262_144,
            comparison_structural_entries: 128,
        }
    );
    assert_eq!(
        concurrency,
        UiRebindConcurrencyInput {
            pending_plans: 2,
            effecting_rebinds: 1,
            completion_handles: 1,
            recovery_handles: 1,
            retained_comparison_snapshots: 2,
            retained_rebind_receipts: 1,
        }
    );
}

#[test]
fn every_empty_rebind_axis_returns_its_exact_typed_denial() {
    let limits = [
        UiRebindLimit::ChangedFacts,
        UiRebindLimit::AffectedAspects,
        UiRebindLimit::DistinctConsumers,
        UiRebindLimit::GraphAndMountedEntries,
        UiRebindLimit::MeasurementAndAllocationEntries,
        UiRebindLimit::QueryBindingTransitions,
        UiRebindLimit::Obligations,
        UiRebindLimit::NativeSurfaces,
        UiRebindLimit::PreparedPresentationBytes,
        UiRebindLimit::TerminalDecisionRecords,
        UiRebindLimit::EvidenceLinkageEntries,
        UiRebindLimit::CausalNeighborhoodBytes,
        UiRebindLimit::ComparisonStructuralEntries,
        UiRebindLimit::PendingPlans,
        UiRebindLimit::EffectingRebinds,
        UiRebindLimit::CompletionHandles,
        UiRebindLimit::RecoveryHandles,
        UiRebindLimit::RetainedComparisonSnapshots,
        UiRebindLimit::RetainedRebindReceipts,
    ];

    for limit in limits {
        let (budget, concurrency) = inputs_with_empty(limit);
        assert_eq!(
            UiRebindProfile::bounded(budget, concurrency),
            Err(UiRebindProfileConstructionDenial::EmptyLimit(limit))
        );
    }
}

#[test]
fn named_smaller_profile_uses_the_same_validated_construction_path() {
    let observation = UiObservationProfile::bounded(UiObservationProfileInput {
        admitted_per_turn: 2,
        retained_bytes_per_turn: 1_024,
        queued_during_effecting_rebind: 1,
    })
    .expect("certification observation profile should be valid");
    let (budget, concurrency) = baseline_inputs();
    let rebind = UiRebindProfile::bounded(budget, concurrency)
        .expect("certification rebind profile should be valid");

    let profile = UiChangeProfile::new(observation, rebind);

    assert_eq!(profile.observation(), observation);
    assert_eq!(profile.rebind(), rebind);
}

fn inputs_with_empty(limit: UiRebindLimit) -> (UiRebindBudgetInput, UiRebindConcurrencyInput) {
    let (mut budget, mut concurrency) = baseline_inputs();
    match limit {
        UiRebindLimit::ChangedFacts => budget.changed_facts = 0,
        UiRebindLimit::AffectedAspects => budget.affected_aspects = 0,
        UiRebindLimit::DistinctConsumers => budget.distinct_consumers = 0,
        UiRebindLimit::GraphAndMountedEntries => budget.graph_and_mounted_entries = 0,
        UiRebindLimit::MeasurementAndAllocationEntries => {
            budget.measurement_and_allocation_entries = 0;
        }
        UiRebindLimit::QueryBindingTransitions => budget.query_binding_transitions = 0,
        UiRebindLimit::Obligations => budget.obligations = 0,
        UiRebindLimit::NativeSurfaces => budget.native_surfaces = 0,
        UiRebindLimit::PreparedPresentationBytes => budget.prepared_presentation_bytes = 0,
        UiRebindLimit::TerminalDecisionRecords => budget.terminal_decision_records = 0,
        UiRebindLimit::EvidenceLinkageEntries => budget.evidence_linkage_entries = 0,
        UiRebindLimit::CausalNeighborhoodBytes => budget.causal_neighborhood_bytes = 0,
        UiRebindLimit::ComparisonStructuralEntries => budget.comparison_structural_entries = 0,
        UiRebindLimit::PendingPlans => concurrency.pending_plans = 0,
        UiRebindLimit::EffectingRebinds => concurrency.effecting_rebinds = 0,
        UiRebindLimit::CompletionHandles => concurrency.completion_handles = 0,
        UiRebindLimit::RecoveryHandles => concurrency.recovery_handles = 0,
        UiRebindLimit::RetainedComparisonSnapshots => {
            concurrency.retained_comparison_snapshots = 0;
        }
        UiRebindLimit::RetainedRebindReceipts => concurrency.retained_rebind_receipts = 0,
    }
    (budget, concurrency)
}

fn baseline_inputs() -> (UiRebindBudgetInput, UiRebindConcurrencyInput) {
    (
        UiRebindBudgetInput {
            changed_facts: 1,
            affected_aspects: 1,
            distinct_consumers: 1,
            graph_and_mounted_entries: 1,
            measurement_and_allocation_entries: 1,
            query_binding_transitions: 1,
            obligations: 1,
            native_surfaces: 1,
            prepared_presentation_bytes: 1,
            terminal_decision_records: 1,
            evidence_linkage_entries: 1,
            causal_neighborhood_bytes: 1,
            comparison_structural_entries: 1,
        },
        UiRebindConcurrencyInput {
            pending_plans: 1,
            effecting_rebinds: 1,
            completion_handles: 1,
            recovery_handles: 1,
            retained_comparison_snapshots: 1,
            retained_rebind_receipts: 1,
        },
    )
}
