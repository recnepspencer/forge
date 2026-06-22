use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitSourceEdgeCarrierSet;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventLedgerReceipt, PlanarBooleanIntervalEvent, PlanarBooleanPointEvent,
};

use super::builder::build_participation_index;
use super::carrier_event_row::PlanarBooleanSplitEventParticipationRow;
use super::counters::PlanarBooleanSplitEventParticipationCounters;
use super::denial::PlanarBooleanSplitEventParticipationDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitEventParticipationIndex {
    index_identity: String,
    event_ledger_identity: String,
    recovered_carrier_set_identity: String,
    rows: Vec<PlanarBooleanSplitEventParticipationRow>,
    carrier_row_offsets: BTreeMap<String, usize>,
    point_events_by_identity: BTreeMap<String, PlanarBooleanPointEvent>,
    interval_events_by_identity: BTreeMap<String, PlanarBooleanIntervalEvent>,
    counters: PlanarBooleanSplitEventParticipationCounters,
}

impl PlanarBooleanSplitEventParticipationIndex {
    pub(crate) fn new(
        index_identity: String,
        event_ledger_identity: String,
        recovered_carrier_set_identity: String,
        rows: Vec<PlanarBooleanSplitEventParticipationRow>,
        point_events_by_identity: BTreeMap<String, PlanarBooleanPointEvent>,
        interval_events_by_identity: BTreeMap<String, PlanarBooleanIntervalEvent>,
        counters: PlanarBooleanSplitEventParticipationCounters,
    ) -> Self {
        let carrier_row_offsets = rows
            .iter()
            .enumerate()
            .map(|(offset, row)| (row.carrier_identity().to_string(), offset))
            .collect();
        Self {
            index_identity,
            event_ledger_identity,
            recovered_carrier_set_identity,
            rows,
            carrier_row_offsets,
            point_events_by_identity,
            interval_events_by_identity,
            counters,
        }
    }

    pub fn from_recovered_carriers(
        recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
        ledger: &PlanarBooleanEventLedgerReceipt,
    ) -> Result<Self, PlanarBooleanSplitEventParticipationDenial> {
        build_participation_index(recovered_carriers, ledger)
    }

    pub fn index_identity(&self) -> &str {
        &self.index_identity
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn recovered_carrier_set_identity(&self) -> &str {
        &self.recovered_carrier_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanSplitEventParticipationRow] {
        &self.rows
    }

    pub(crate) fn point_event(&self, event_identity: &str) -> Option<&PlanarBooleanPointEvent> {
        self.point_events_by_identity.get(event_identity)
    }

    pub(crate) fn interval_event(
        &self,
        event_identity: &str,
    ) -> Option<&PlanarBooleanIntervalEvent> {
        self.interval_events_by_identity.get(event_identity)
    }

    pub fn row_for_carrier(
        &self,
        carrier_identity: &str,
    ) -> Option<&PlanarBooleanSplitEventParticipationRow> {
        self.carrier_row_offsets
            .get(carrier_identity)
            .and_then(|offset| self.rows.get(*offset))
    }

    pub fn counters(&self) -> PlanarBooleanSplitEventParticipationCounters {
        self.counters
    }
}
