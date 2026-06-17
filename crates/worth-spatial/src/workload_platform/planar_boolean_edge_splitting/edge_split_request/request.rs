use super::counters::PlanarBooleanEdgeSplitRequestCounters;
use super::denial::PlanarBooleanEdgeSplitRequestDenial;
use super::identity::edge_split_request_identity;
use super::input::PlanarBooleanEdgeSplitRequestInput;
use super::validation::validate_edge_split_request;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitRequest {
    split_request_identity: String,
    event_ledger_identity: String,
    downstream_consumption_identity: String,
    reduced_pair_identity: String,
    event_extraction_request_identity: String,
    segment_carrier_set_identity: String,
    segment_pair_enumeration_identity: String,
    candidate_index_consumption_gate_identity: String,
    candidate_index_product_identity: String,
    query_index_plan_digest: String,
    counters: PlanarBooleanEdgeSplitRequestCounters,
}

impl PlanarBooleanEdgeSplitRequest {
    pub fn admit(
        input: PlanarBooleanEdgeSplitRequestInput<'_>,
    ) -> Result<Self, PlanarBooleanEdgeSplitRequestDenial> {
        validate_edge_split_request(&input)?;
        let event_ledger = input.event_ledger();
        let candidate_index_gate = input.candidate_index_gate();
        let request = Self {
            split_request_identity: String::new(),
            event_ledger_identity: event_ledger.event_ledger_identity().to_string(),
            downstream_consumption_identity: event_ledger
                .downstream_consumption_identity()
                .to_string(),
            reduced_pair_identity: event_ledger.reduced_pair_identity().to_string(),
            event_extraction_request_identity: event_ledger
                .event_extraction_request_identity()
                .to_string(),
            segment_carrier_set_identity: event_ledger.segment_carrier_set_identity().to_string(),
            segment_pair_enumeration_identity: event_ledger
                .segment_pair_enumeration_identity()
                .to_string(),
            candidate_index_consumption_gate_identity: candidate_index_gate
                .gate_identity()
                .to_string(),
            candidate_index_product_identity: candidate_index_gate
                .candidate_index_product_identity()
                .to_string(),
            query_index_plan_digest: candidate_index_gate.query_index_plan_digest().to_string(),
            counters: PlanarBooleanEdgeSplitRequestCounters::new(
                event_ledger.segment_carriers().len(),
                event_ledger.point_events().len(),
                event_ledger.interval_events().len(),
                event_ledger.event_groups().len(),
            ),
        };
        Ok(Self {
            split_request_identity: edge_split_request_identity(&request),
            ..request
        })
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
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

    pub fn segment_pair_enumeration_identity(&self) -> &str {
        &self.segment_pair_enumeration_identity
    }

    pub fn candidate_index_consumption_gate_identity(&self) -> &str {
        &self.candidate_index_consumption_gate_identity
    }

    pub fn candidate_index_product_identity(&self) -> &str {
        &self.candidate_index_product_identity
    }

    pub fn query_index_plan_digest(&self) -> &str {
        &self.query_index_plan_digest
    }

    pub fn counters(&self) -> PlanarBooleanEdgeSplitRequestCounters {
        self.counters
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanEdgeSplitRequest {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        self.split_request_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}
