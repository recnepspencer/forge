use super::{
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceIdentity,
    WorthUiQueryAllocationSourceOrder, WorthUiQueryMeasurementConsumptionIdentity,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactSettlement,
};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationConsumptionIdentity {
    measurement_consumption_identity: WorthUiQueryMeasurementConsumptionIdentity,
    source_identity: WorthUiQueryAllocationSourceIdentity,
    source_generation: WorthUiQueryAllocationSourceGeneration,
    source_order: WorthUiQueryAllocationSourceOrder,
    query_basis_digest: Box<str>,
    projection_contract_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
}

/// Query-owned proof that allocation invalidation is derived from admitted
/// projection consumption rather than a copied payload or digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryAllocationInvalidationBasis {
    consumption_identity: WorthUiQueryAllocationConsumptionIdentity,
    source_identity: WorthUiQueryAllocationSourceIdentity,
    source_generation: WorthUiQueryAllocationSourceGeneration,
    source_order: WorthUiQueryAllocationSourceOrder,
    partial: bool,
    query_basis_digest: Box<str>,
    projection_contract_digest: Box<str>,
    consumed_families: Arc<[WorthUiQueryMeasurementFactFamily]>,
    projection_consumption_receipt_digest: Box<str>,
    observations: Arc<[WorthUiQueryMeasurementFactObservation]>,
}

impl WorthUiQueryAllocationInvalidationBasis {
    pub(crate) fn from_settlement(settlement: &WorthUiQueryMeasurementFactSettlement) -> Self {
        let query_basis_digest: Box<str> = settlement
            .receipt()
            .prerequisites()
            .resolution_report()
            .basis_digest()
            .as_str()
            .into();
        let projection_contract_digest: Box<str> =
            settlement.receipt().projection_contract_digest().into();
        let projection_consumption_receipt_digest: Box<str> = settlement
            .receipt()
            .projection_consumption_receipt_digest()
            .into();
        Self {
            consumption_identity: WorthUiQueryAllocationConsumptionIdentity {
                measurement_consumption_identity: settlement
                    .receipt()
                    .consumption_identity()
                    .clone(),
                source_identity: settlement.allocation_source_identity().clone(),
                source_generation: settlement.allocation_source_generation(),
                source_order: settlement.allocation_source_order(),
                query_basis_digest: query_basis_digest.clone(),
                projection_contract_digest: projection_contract_digest.clone(),
                projection_consumption_receipt_digest: projection_consumption_receipt_digest
                    .clone(),
            },
            source_identity: settlement.allocation_source_identity().clone(),
            source_generation: settlement.allocation_source_generation(),
            source_order: settlement.allocation_source_order(),
            partial: settlement.is_partial(),
            query_basis_digest,
            projection_contract_digest,
            consumed_families: settlement.receipt().consumed_families_arc(),
            projection_consumption_receipt_digest,
            observations: settlement.receipt().observations_arc(),
        }
    }

    pub fn consumption_identity(&self) -> &WorthUiQueryAllocationConsumptionIdentity {
        &self.consumption_identity
    }

    pub fn source_identity(&self) -> &WorthUiQueryAllocationSourceIdentity {
        &self.source_identity
    }
    pub fn source_generation(&self) -> WorthUiQueryAllocationSourceGeneration {
        self.source_generation
    }
    pub fn source_order(&self) -> WorthUiQueryAllocationSourceOrder {
        self.source_order
    }
    pub fn is_partial(&self) -> bool {
        self.partial
    }
    pub fn consumed_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.consumed_families
    }
    pub fn query_basis_digest(&self) -> &str {
        &self.query_basis_digest
    }
    pub fn projection_contract_digest(&self) -> &str {
        &self.projection_contract_digest
    }
    pub fn projection_consumption_receipt_digest(&self) -> &str {
        &self.projection_consumption_receipt_digest
    }
    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }
}

impl WorthUiQueryAllocationConsumptionIdentity {
    pub fn measurement_consumption_identity(&self) -> &WorthUiQueryMeasurementConsumptionIdentity {
        &self.measurement_consumption_identity
    }
    pub fn source_identity(&self) -> &WorthUiQueryAllocationSourceIdentity {
        &self.source_identity
    }
    pub fn source_generation(&self) -> WorthUiQueryAllocationSourceGeneration {
        self.source_generation
    }
    pub fn source_order(&self) -> WorthUiQueryAllocationSourceOrder {
        self.source_order
    }
    pub fn query_basis_digest(&self) -> &str {
        &self.query_basis_digest
    }
    pub fn projection_contract_digest(&self) -> &str {
        &self.projection_contract_digest
    }
    pub fn projection_consumption_receipt_digest(&self) -> &str {
        &self.projection_consumption_receipt_digest
    }
}
