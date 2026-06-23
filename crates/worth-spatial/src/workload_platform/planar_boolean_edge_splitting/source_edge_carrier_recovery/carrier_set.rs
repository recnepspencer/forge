use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

use super::counters::PlanarBooleanSplitSourceEdgeCarrierCounters;
use super::denial::PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial;
use super::identity::recovered_carrier_set_identity;
use super::input::PlanarBooleanSplitSourceEdgeCarrierRecoveryInput;
use super::recovered_carrier::PlanarBooleanSplitSourceEdgeCarrier;
use super::recovery::recover_source_edge_carriers;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitSourceEdgeCarrierSet {
    carrier_set_identity: String,
    scope_admission_identity: String,
    split_request_identity: String,
    event_ledger_identity: String,
    segment_carrier_set_identity: String,
    candidate_index_product_identity: String,
    query_index_plan_digest: String,
    carriers: Vec<PlanarBooleanSplitSourceEdgeCarrier>,
    carrier_offsets: BTreeMap<String, usize>,
    source_edge_offsets: BTreeMap<String, Vec<usize>>,
    counters: PlanarBooleanSplitSourceEdgeCarrierCounters,
}

impl PlanarBooleanSplitSourceEdgeCarrierSet {
    pub fn recover(
        input: PlanarBooleanSplitSourceEdgeCarrierRecoveryInput<'_>,
    ) -> Result<Self, PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
        recover_source_edge_carriers(input)
    }

    pub(crate) fn new(
        scope_admission_identity: String,
        split_request_identity: String,
        event_ledger_identity: String,
        segment_carrier_set_identity: String,
        candidate_index_product_identity: String,
        query_index_plan_digest: String,
        carriers: Vec<PlanarBooleanSplitSourceEdgeCarrier>,
        counters: PlanarBooleanSplitSourceEdgeCarrierCounters,
    ) -> Self {
        let carrier_offsets = carriers
            .iter()
            .enumerate()
            .map(|(offset, carrier)| (carrier.carrier_identity().to_string(), offset))
            .collect::<BTreeMap<_, _>>();
        let mut source_edge_offsets: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (offset, carrier) in carriers.iter().enumerate() {
            source_edge_offsets
                .entry(source_edge_lookup_key(
                    carrier.operand_side(),
                    carrier.source_edge_identity(),
                ))
                .or_default()
                .push(offset);
        }
        let set = Self {
            carrier_set_identity: String::new(),
            scope_admission_identity,
            split_request_identity,
            event_ledger_identity,
            segment_carrier_set_identity,
            candidate_index_product_identity,
            query_index_plan_digest,
            carriers,
            carrier_offsets,
            source_edge_offsets,
            counters,
        };
        Self {
            carrier_set_identity: recovered_carrier_set_identity(&set),
            ..set
        }
    }

    pub fn carrier_set_identity(&self) -> &str {
        &self.carrier_set_identity
    }

    pub fn scope_admission_identity(&self) -> &str {
        &self.scope_admission_identity
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn segment_carrier_set_identity(&self) -> &str {
        &self.segment_carrier_set_identity
    }

    pub fn candidate_index_product_identity(&self) -> &str {
        &self.candidate_index_product_identity
    }

    pub fn query_index_plan_digest(&self) -> &str {
        &self.query_index_plan_digest
    }

    pub fn carriers(&self) -> &[PlanarBooleanSplitSourceEdgeCarrier] {
        &self.carriers
    }

    pub fn carrier_for_identity(
        &self,
        carrier_identity: &str,
    ) -> Option<&PlanarBooleanSplitSourceEdgeCarrier> {
        self.carrier_offsets
            .get(carrier_identity)
            .and_then(|offset| self.carriers.get(*offset))
    }

    pub fn carriers_for_source_edge(
        &self,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        source_edge_identity: &str,
    ) -> Vec<&PlanarBooleanSplitSourceEdgeCarrier> {
        self.source_edge_offsets
            .get(&source_edge_lookup_key(operand_side, source_edge_identity))
            .into_iter()
            .flat_map(|offsets| offsets.iter())
            .filter_map(|offset| self.carriers.get(*offset))
            .collect()
    }

    pub fn counters(&self) -> PlanarBooleanSplitSourceEdgeCarrierCounters {
        self.counters
    }
}

fn source_edge_lookup_key(
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_edge_identity: &str,
) -> String {
    format!("{}:{source_edge_identity}", operand_side.query_key())
}
