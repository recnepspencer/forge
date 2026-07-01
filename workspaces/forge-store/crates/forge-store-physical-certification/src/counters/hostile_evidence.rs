use crate::{PhysicalSimulationPlan, PhysicalSimulationProfile};

use super::{
    evidence::{
        require_expected_counter_rows, require_resource_observation_within_envelope,
        sorted_counter_rows, PhysicalResourceEnvelopeObservation,
    },
    CounterContractKind, CounterExpectationKind, CounterMismatchEvidence,
    PhysicalCounterEvidenceRow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostileCounterEvidenceRow {
    kind: CounterContractKind,
    strength: CounterExpectationKind,
    observed_count: u64,
    previous_observed_count: Option<u64>,
}

impl HostileCounterEvidenceRow {
    pub const fn new(
        kind: CounterContractKind,
        strength: CounterExpectationKind,
        observed_count: u64,
    ) -> Self {
        Self {
            kind,
            strength,
            observed_count,
            previous_observed_count: None,
        }
    }

    pub const fn with_previous_observed_count(mut self, previous_observed_count: u64) -> Self {
        self.previous_observed_count = Some(previous_observed_count);
        self
    }

    pub const fn kind(self) -> CounterContractKind {
        self.kind
    }

    fn into_row(self) -> PhysicalCounterEvidenceRow {
        PhysicalCounterEvidenceRow::from_hostile_readmission(
            self.kind,
            self.strength,
            self.observed_count,
            self.previous_observed_count,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostileResourceEnvelopeObservation {
    observation: PhysicalResourceEnvelopeObservation,
}

impl HostileResourceEnvelopeObservation {
    pub const fn new(
        profile: PhysicalSimulationProfile,
        allocation_bytes: u64,
        resident_bytes: u64,
        pinned_pages: u64,
        dirty_pages: u64,
        io_queue_depth: u64,
        io_interference_events: u64,
    ) -> Self {
        Self {
            observation: PhysicalResourceEnvelopeObservation::new(
                profile,
                allocation_bytes,
                resident_bytes,
                pinned_pages,
                dirty_pages,
                io_queue_depth,
                io_interference_events,
            ),
        }
    }

    const fn into_observation(self) -> PhysicalResourceEnvelopeObservation {
        self.observation
    }
}

pub fn reject_hostile_counter_evidence_for_readmission(
    plan: &PhysicalSimulationPlan,
    rows: impl IntoIterator<Item = HostileCounterEvidenceRow>,
    resource_observation: HostileResourceEnvelopeObservation,
) -> Result<(), CounterMismatchEvidence> {
    require_resource_observation_within_envelope(plan, resource_observation.into_observation())?;
    let rows = sorted_counter_rows(rows.into_iter().map(HostileCounterEvidenceRow::into_row))?;
    require_expected_counter_rows(plan, &rows)
}
