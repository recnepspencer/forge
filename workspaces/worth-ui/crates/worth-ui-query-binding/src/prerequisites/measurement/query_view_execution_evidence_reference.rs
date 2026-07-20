use std::sync::Arc;

use worth_query::facade::foundation::ConsumedNativeValueView;

use super::{
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceOrder,
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactSettlement,
};

/// Read-only sharing of one exact Query-owned settlement at the UI plan edge.
///
/// This reference deliberately exposes native values and compact coordinates,
/// but no method can recover the retained consumed-projection authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryViewExecutionEvidenceReference {
    settlement: Arc<WorthUiQueryMeasurementFactSettlement>,
}

impl WorthUiQueryViewExecutionEvidenceReference {
    pub(crate) fn new(settlement: Arc<WorthUiQueryMeasurementFactSettlement>) -> Self {
        Self { settlement }
    }

    pub fn definition(&self) -> &crate::WorthUiQueryViewDefinition {
        self.settlement.definition()
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        self.settlement.receipt().observations()
    }

    pub fn native_fact_count(&self) -> usize {
        let facts = self
            .settlement
            .receipt()
            .query_authority()
            .authority()
            .facts();
        facts.display_fields().len() + facts.derived_fields().len()
    }

    pub fn native_fact(&self, index: usize) -> Option<ConsumedNativeValueView<'_>> {
        let facts = self
            .settlement
            .receipt()
            .query_authority()
            .authority()
            .facts();
        facts
            .display_fields()
            .iter()
            .chain(facts.derived_fields())
            .nth(index)
            .map(|fact| fact.native_value())
    }

    pub fn source_generation(&self) -> WorthUiQueryAllocationSourceGeneration {
        self.settlement.allocation_source_generation()
    }

    pub fn source_order(&self) -> WorthUiQueryAllocationSourceOrder {
        self.settlement.allocation_source_order()
    }

    pub fn is_partial(&self) -> bool {
        self.settlement.is_partial()
    }

    pub fn evidence_identity_digest(&self) -> u64 {
        self.settlement
            .allocation_source_identity()
            .authority_index_key()
            .identity_digest()
            ^ self.source_generation().as_u64().rotate_left(17)
            ^ self.source_order().as_u64().rotate_left(31)
            ^ self
                .settlement
                .definition()
                .digest()
                .as_u64()
                .rotate_left(47)
    }
}
