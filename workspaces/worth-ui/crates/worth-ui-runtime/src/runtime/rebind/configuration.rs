use crate::runtime::observation::UiObservationProfile;

const PLATFORM_PULSE_PROFILE_REVISION: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindBudgetInput {
    pub changed_facts: usize,
    pub affected_aspects: usize,
    pub distinct_consumers: usize,
    pub graph_and_mounted_entries: usize,
    pub measurement_and_allocation_entries: usize,
    pub query_binding_transitions: usize,
    pub obligations: usize,
    pub native_surfaces: usize,
    pub prepared_presentation_bytes: usize,
    pub terminal_decision_records: usize,
    pub evidence_linkage_entries: usize,
    pub causal_neighborhood_bytes: usize,
    pub comparison_structural_entries: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindConcurrencyInput {
    pub pending_plans: usize,
    pub effecting_rebinds: usize,
    pub completion_handles: usize,
    pub recovery_handles: usize,
    pub retained_comparison_snapshots: usize,
    pub retained_rebind_receipts: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindLimit {
    ChangedFacts,
    AffectedAspects,
    DistinctConsumers,
    GraphAndMountedEntries,
    MeasurementAndAllocationEntries,
    QueryBindingTransitions,
    Obligations,
    NativeSurfaces,
    PreparedPresentationBytes,
    TerminalDecisionRecords,
    EvidenceLinkageEntries,
    CausalNeighborhoodBytes,
    ComparisonStructuralEntries,
    PendingPlans,
    EffectingRebinds,
    CompletionHandles,
    RecoveryHandles,
    RetainedComparisonSnapshots,
    RetainedRebindReceipts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindProfileConstructionDenial {
    EmptyLimit(UiRebindLimit),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindProfile {
    budget: UiRebindBudgetInput,
    concurrency: UiRebindConcurrencyInput,
}

impl UiRebindProfile {
    pub fn bounded(
        budget: UiRebindBudgetInput,
        concurrency: UiRebindConcurrencyInput,
    ) -> Result<Self, UiRebindProfileConstructionDenial> {
        validate_budget(budget)?;
        validate_concurrency(concurrency)?;
        Ok(Self {
            budget,
            concurrency,
        })
    }

    pub fn platform_pulse() -> Self {
        Self {
            budget: UiRebindBudgetInput {
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
            },
            concurrency: UiRebindConcurrencyInput {
                pending_plans: 2,
                effecting_rebinds: 1,
                completion_handles: 1,
                recovery_handles: 1,
                retained_comparison_snapshots: 2,
                retained_rebind_receipts: 1,
            },
        }
    }

    pub const fn budget(self) -> UiRebindBudgetInput {
        self.budget
    }

    pub const fn concurrency(self) -> UiRebindConcurrencyInput {
        self.concurrency
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiChangeProfile {
    revision: u16,
    observation: UiObservationProfile,
    rebind: UiRebindProfile,
}

impl UiChangeProfile {
    pub const fn new(observation: UiObservationProfile, rebind: UiRebindProfile) -> Self {
        Self {
            revision: PLATFORM_PULSE_PROFILE_REVISION,
            observation,
            rebind,
        }
    }

    pub fn platform_pulse() -> Self {
        Self::new(
            UiObservationProfile::platform_pulse(),
            UiRebindProfile::platform_pulse(),
        )
    }

    pub const fn revision(self) -> u16 {
        self.revision
    }

    pub const fn observation(self) -> UiObservationProfile {
        self.observation
    }

    pub const fn rebind(self) -> UiRebindProfile {
        self.rebind
    }
}

fn validate_budget(budget: UiRebindBudgetInput) -> Result<(), UiRebindProfileConstructionDenial> {
    let limits = [
        (budget.changed_facts, UiRebindLimit::ChangedFacts),
        (budget.affected_aspects, UiRebindLimit::AffectedAspects),
        (budget.distinct_consumers, UiRebindLimit::DistinctConsumers),
        (
            budget.graph_and_mounted_entries,
            UiRebindLimit::GraphAndMountedEntries,
        ),
        (
            budget.measurement_and_allocation_entries,
            UiRebindLimit::MeasurementAndAllocationEntries,
        ),
        (
            budget.query_binding_transitions,
            UiRebindLimit::QueryBindingTransitions,
        ),
        (budget.obligations, UiRebindLimit::Obligations),
        (budget.native_surfaces, UiRebindLimit::NativeSurfaces),
        (
            budget.prepared_presentation_bytes,
            UiRebindLimit::PreparedPresentationBytes,
        ),
        (
            budget.terminal_decision_records,
            UiRebindLimit::TerminalDecisionRecords,
        ),
        (
            budget.evidence_linkage_entries,
            UiRebindLimit::EvidenceLinkageEntries,
        ),
        (
            budget.causal_neighborhood_bytes,
            UiRebindLimit::CausalNeighborhoodBytes,
        ),
        (
            budget.comparison_structural_entries,
            UiRebindLimit::ComparisonStructuralEntries,
        ),
    ];
    validate_limits(&limits)
}

fn validate_concurrency(
    concurrency: UiRebindConcurrencyInput,
) -> Result<(), UiRebindProfileConstructionDenial> {
    let limits = [
        (concurrency.pending_plans, UiRebindLimit::PendingPlans),
        (
            concurrency.effecting_rebinds,
            UiRebindLimit::EffectingRebinds,
        ),
        (
            concurrency.completion_handles,
            UiRebindLimit::CompletionHandles,
        ),
        (concurrency.recovery_handles, UiRebindLimit::RecoveryHandles),
        (
            concurrency.retained_comparison_snapshots,
            UiRebindLimit::RetainedComparisonSnapshots,
        ),
        (
            concurrency.retained_rebind_receipts,
            UiRebindLimit::RetainedRebindReceipts,
        ),
    ];
    validate_limits(&limits)
}

fn validate_limits(
    limits: &[(usize, UiRebindLimit)],
) -> Result<(), UiRebindProfileConstructionDenial> {
    limits
        .iter()
        .find_map(|(value, limit)| (*value == 0).then_some(*limit))
        .map_or(Ok(()), |limit| {
            Err(UiRebindProfileConstructionDenial::EmptyLimit(limit))
        })
}
