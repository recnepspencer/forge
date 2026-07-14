use super::{
    WorthUiQueryAllocationSourceGeneration, WorthUiQueryAllocationSourceIdentity,
    WorthUiQueryAllocationSourceOrder, WorthUiQueryAuthorityHandle, WorthUiQueryAuthorityIndexKey,
    WorthUiQueryMeasurementFactFamily, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactSettlement,
};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryAllocationConsumptionIdentity {
    authority_index_key: WorthUiQueryAuthorityIndexKey,
    source_identity: WorthUiQueryAllocationSourceIdentity,
    source_generation: WorthUiQueryAllocationSourceGeneration,
    source_order: WorthUiQueryAllocationSourceOrder,
}

/// Query-owned proof that allocation invalidation is derived from admitted
/// projection consumption rather than a copied payload or digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryAllocationInvalidationBasis {
    query_authority: WorthUiQueryAuthorityHandle,
    consumption_identity: WorthUiQueryAllocationConsumptionIdentity,
    source_identity: WorthUiQueryAllocationSourceIdentity,
    source_generation: WorthUiQueryAllocationSourceGeneration,
    source_order: WorthUiQueryAllocationSourceOrder,
    partial: bool,
    consumed_families: Arc<[WorthUiQueryMeasurementFactFamily]>,
    observations: Arc<[WorthUiQueryMeasurementFactObservation]>,
}

impl WorthUiQueryAllocationInvalidationBasis {
    pub(crate) fn from_settlement(settlement: &WorthUiQueryMeasurementFactSettlement) -> Self {
        Self {
            query_authority: settlement.receipt().query_authority().clone(),
            consumption_identity: WorthUiQueryAllocationConsumptionIdentity {
                authority_index_key: settlement.receipt().authority_index_key().clone(),
                source_identity: settlement.allocation_source_identity().clone(),
                source_generation: settlement.allocation_source_generation(),
                source_order: settlement.allocation_source_order(),
            },
            source_identity: settlement.allocation_source_identity().clone(),
            source_generation: settlement.allocation_source_generation(),
            source_order: settlement.allocation_source_order(),
            partial: settlement.is_partial(),
            consumed_families: settlement.receipt().consumed_families_arc(),
            observations: settlement.receipt().observations_arc(),
        }
    }

    pub fn consumption_identity(&self) -> &WorthUiQueryAllocationConsumptionIdentity {
        &self.consumption_identity
    }

    pub fn query_authority(&self) -> &WorthUiQueryAuthorityHandle {
        &self.query_authority
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
        self.query_authority
            .authority()
            .contract()
            .basis_digest()
            .unwrap_or_default()
    }
    pub fn projection_contract_digest(&self) -> &str {
        self.query_authority
            .authority()
            .contract()
            .contract_digest()
    }
    pub fn projection_consumption_receipt_digest(&self) -> &str {
        self.query_authority.authority().receipt().receipt_digest()
    }
    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }
}

impl WorthUiQueryAllocationConsumptionIdentity {
    pub fn authority_index_key(&self) -> &WorthUiQueryAuthorityIndexKey {
        &self.authority_index_key
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
        self.authority_index_key.query_basis_digest()
    }
    pub fn projection_contract_digest(&self) -> &str {
        self.authority_index_key.projection_contract_digest()
    }
    pub fn projection_consumption_receipt_digest(&self) -> &str {
        self.authority_index_key
            .projection_consumption_receipt_digest()
    }
}
