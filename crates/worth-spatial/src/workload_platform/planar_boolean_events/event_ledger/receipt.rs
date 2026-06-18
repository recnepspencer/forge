use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCollinearRelation, PlanarBooleanEventGroup, PlanarBooleanIntervalEvent,
    PlanarBooleanPointEvent, PlanarBooleanSegmentCarrier,
};

use super::counters::PlanarBooleanEventLedgerCounters;
use super::ordered_events::PlanarBooleanOrderedEventSet;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEventLedgerReceipt {
    reduced_pair_identity: String,
    event_extraction_request_identity: String,
    segment_carrier_set_identity: String,
    segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
    segment_pair_enumeration_identity: String,
    predicate_binding_identity: String,
    point_event_extraction_identity: String,
    collinear_relation_receipt_identity: String,
    interval_event_extraction_identity: String,
    point_events: Vec<PlanarBooleanPointEvent>,
    interval_events: Vec<PlanarBooleanIntervalEvent>,
    relation_diagnostics: Vec<PlanarBooleanCollinearRelation>,
    event_groups: Vec<PlanarBooleanEventGroup>,
    ordered_events: PlanarBooleanOrderedEventSet,
    counters: PlanarBooleanEventLedgerCounters,
    event_ledger_identity: String,
    downstream_consumption_identity: String,
}

impl PlanarBooleanEventLedgerReceipt {
    pub(crate) fn new(input: PlanarBooleanEventLedgerReceiptInput) -> Self {
        Self {
            reduced_pair_identity: input.reduced_pair_identity,
            event_extraction_request_identity: input.event_extraction_request_identity,
            segment_carrier_set_identity: input.segment_carrier_set_identity,
            segment_carriers: input.segment_carriers,
            segment_pair_enumeration_identity: input.segment_pair_enumeration_identity,
            predicate_binding_identity: input.predicate_binding_identity,
            point_event_extraction_identity: input.point_event_extraction_identity,
            collinear_relation_receipt_identity: input.collinear_relation_receipt_identity,
            interval_event_extraction_identity: input.interval_event_extraction_identity,
            point_events: input.point_events,
            interval_events: input.interval_events,
            relation_diagnostics: input.relation_diagnostics,
            event_groups: input.event_groups,
            ordered_events: input.ordered_events,
            counters: input.counters,
            event_ledger_identity: input.event_ledger_identity,
            downstream_consumption_identity: input.downstream_consumption_identity,
        }
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn event_extraction_request_identity(&self) -> &str {
        &self.event_extraction_request_identity
    }

    pub fn segment_carrier_set_identity(&self) -> &str {
        &self.segment_carrier_set_identity
    }

    pub fn segment_carriers(&self) -> &[PlanarBooleanSegmentCarrier] {
        &self.segment_carriers
    }

    pub fn segment_pair_enumeration_identity(&self) -> &str {
        &self.segment_pair_enumeration_identity
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn point_event_extraction_identity(&self) -> &str {
        &self.point_event_extraction_identity
    }

    pub fn collinear_relation_receipt_identity(&self) -> &str {
        &self.collinear_relation_receipt_identity
    }

    pub fn interval_event_extraction_identity(&self) -> &str {
        &self.interval_event_extraction_identity
    }

    pub fn point_events(&self) -> &[PlanarBooleanPointEvent] {
        &self.point_events
    }

    pub fn interval_events(&self) -> &[PlanarBooleanIntervalEvent] {
        &self.interval_events
    }

    pub fn relation_diagnostics(&self) -> &[PlanarBooleanCollinearRelation] {
        &self.relation_diagnostics
    }

    pub fn event_groups(&self) -> &[PlanarBooleanEventGroup] {
        &self.event_groups
    }

    pub fn ordered_events(&self) -> &PlanarBooleanOrderedEventSet {
        &self.ordered_events
    }

    pub fn counters(&self) -> PlanarBooleanEventLedgerCounters {
        self.counters
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanEventLedgerReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventLedger
    }

    fn evidence_identity(&self) -> &str {
        self.event_ledger_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_event_ledger(self.counters)
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanEventLedgerReceipt {}

pub(crate) struct PlanarBooleanEventLedgerReceiptInput {
    pub(crate) reduced_pair_identity: String,
    pub(crate) event_extraction_request_identity: String,
    pub(crate) segment_carrier_set_identity: String,
    pub(crate) segment_carriers: Vec<PlanarBooleanSegmentCarrier>,
    pub(crate) segment_pair_enumeration_identity: String,
    pub(crate) predicate_binding_identity: String,
    pub(crate) point_event_extraction_identity: String,
    pub(crate) collinear_relation_receipt_identity: String,
    pub(crate) interval_event_extraction_identity: String,
    pub(crate) point_events: Vec<PlanarBooleanPointEvent>,
    pub(crate) interval_events: Vec<PlanarBooleanIntervalEvent>,
    pub(crate) relation_diagnostics: Vec<PlanarBooleanCollinearRelation>,
    pub(crate) event_groups: Vec<PlanarBooleanEventGroup>,
    pub(crate) ordered_events: PlanarBooleanOrderedEventSet,
    pub(crate) counters: PlanarBooleanEventLedgerCounters,
    pub(crate) event_ledger_identity: String,
    pub(crate) downstream_consumption_identity: String,
}
