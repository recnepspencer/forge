use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiAdmittedHostFrameObservationReceipt, WorthUiProjectionFamily, WorthUiRuntimeFactFamily,
    WorthUiRuntimeFactId, WorthUiRuntimeHost, WorthUiSemanticSliceInventory,
};

use super::super::digest::digest_parts;
use super::counters::WorthUiHostObservationRebindCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHostObservationRebindReceipt {
    previous_observation_digest: u64,
    next_observation_digest: u64,
    changed_facts: Vec<WorthUiRuntimeFactId>,
    preserved_facts: Vec<WorthUiRuntimeFactId>,
    consuming_projection_families: Vec<WorthUiProjectionFamily>,
    counters: WorthUiHostObservationRebindCounters,
    receipt_digest: u64,
}

impl WorthUiRuntimeHost {
    pub fn rebind_host_measurement_observations(
        &self,
        previous: &WorthUiAdmittedHostFrameObservationReceipt,
        next: &WorthUiAdmittedHostFrameObservationReceipt,
    ) -> WorthUiHostObservationRebindReceipt {
        WorthUiHostObservationRebindReceipt::new(previous, next)
    }
}

impl WorthUiHostObservationRebindReceipt {
    fn new(
        previous: &WorthUiAdmittedHostFrameObservationReceipt,
        next: &WorthUiAdmittedHostFrameObservationReceipt,
    ) -> Self {
        let changed_facts = host_measurement_changed_facts(previous, next);
        let preserved_facts = host_measurement_preserved_facts(previous, next);
        let consuming_projection_families =
            host_measurement_consuming_projection_families(&changed_facts);
        let counters = WorthUiHostObservationRebindCounters::new(
            changed_facts.len(),
            preserved_facts.len(),
            consuming_projection_families.len(),
        );
        let receipt_digest = digest_parts(
            [
                "host_observation_rebind".to_owned(),
                previous.receipt_digest().to_string(),
                next.receipt_digest().to_string(),
            ]
            .into_iter()
            .chain(changed_facts.iter().map(|fact| fact.identity().to_owned()))
            .chain(
                preserved_facts
                    .iter()
                    .map(|fact| fact.identity().to_owned()),
            )
            .chain(
                consuming_projection_families
                    .iter()
                    .map(|family| format!("{family:?}")),
            ),
        );
        Self {
            previous_observation_digest: previous.receipt_digest(),
            next_observation_digest: next.receipt_digest(),
            changed_facts,
            preserved_facts,
            consuming_projection_families,
            counters,
            receipt_digest,
        }
    }

    pub fn previous_observation_digest(&self) -> u64 {
        self.previous_observation_digest
    }

    pub fn next_observation_digest(&self) -> u64 {
        self.next_observation_digest
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn preserved_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.preserved_facts
    }

    pub fn consuming_projection_families(&self) -> &[WorthUiProjectionFamily] {
        &self.consuming_projection_families
    }

    pub fn counters(&self) -> WorthUiHostObservationRebindCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn host_measurement_changed_facts(
    previous: &WorthUiAdmittedHostFrameObservationReceipt,
    next: &WorthUiAdmittedHostFrameObservationReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    let previous_facts = fact_set(previous.consumed_facts());
    fact_set(next.consumed_facts())
        .difference(&previous_facts)
        .cloned()
        .collect()
}

fn host_measurement_preserved_facts(
    previous: &WorthUiAdmittedHostFrameObservationReceipt,
    next: &WorthUiAdmittedHostFrameObservationReceipt,
) -> Vec<WorthUiRuntimeFactId> {
    let previous_facts = fact_set(previous.consumed_facts());
    fact_set(next.consumed_facts())
        .intersection(&previous_facts)
        .cloned()
        .collect()
}

fn fact_set(facts: &[WorthUiRuntimeFactId]) -> BTreeSet<WorthUiRuntimeFactId> {
    facts.iter().cloned().collect()
}

fn host_measurement_consuming_projection_families(
    changed_facts: &[WorthUiRuntimeFactId],
) -> Vec<WorthUiProjectionFamily> {
    if !changed_facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::HostMeasurementObservation)
    {
        return Vec::new();
    }
    WorthUiSemanticSliceInventory::current()
        .consumers_for_runtime_fact_family(WorthUiRuntimeFactFamily::HostMeasurementObservation)
        .projection_families()
        .to_vec()
}
