use std::sync::Arc;

use super::receipt_construction::{
    collect_authority_partial_receipt_parts, collect_authority_receipt_parts,
    collect_verified_partial_receipt_parts, collect_verified_receipt_parts,
    VerifiedMeasurementFactReceiptParts,
};
use super::{
    WorthUiQueryAuthorityHandle, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryPrerequisiteEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactReceiptError {
    NonQueryOwnedProjectionSource,
    BasisDigestMismatch,
    ProjectionContractMismatch,
    ProjectionConsumptionNotAdmitted,
    Observation(WorthUiQueryMeasurementFactObservationError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactReceipt {
    query_authority: WorthUiQueryAuthorityHandle,
    authority_index_key: WorthUiQueryAuthorityIndexKey,
    consumed_families: Arc<[super::WorthUiQueryMeasurementFactFamily]>,
    observations: Arc<[WorthUiQueryMeasurementFactObservation]>,
    refinement_counters: super::WorthUiQueryMeasurementRefinementCounters,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Derived lookup key. This is never sufficient to mint or validate authority;
/// operational consumers must retain [`WorthUiQueryAuthorityHandle`].
pub struct WorthUiQueryAuthorityIndexKey {
    projection_source_identity: Box<str>,
    query_basis_digest: Box<str>,
    projection_contract_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
}

impl WorthUiQueryMeasurementFactReceipt {
    pub(crate) fn from_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        query_authority: WorthUiQueryAuthorityHandle,
        partial: bool,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let parts = if partial {
            collect_verified_partial_receipt_parts(prerequisites, query_authority.authority())?
        } else {
            collect_verified_receipt_parts(prerequisites, query_authority.authority())?
        };
        Self::from_verified_parts(query_authority, parts)
    }

    pub(crate) fn from_installed_query_authority(
        query_authority: WorthUiQueryAuthorityHandle,
        partial: bool,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let parts = if partial {
            collect_authority_partial_receipt_parts(query_authority.authority())?
        } else {
            collect_authority_receipt_parts(query_authority.authority())?
        };
        Self::from_verified_parts(query_authority, parts)
    }

    fn from_verified_parts(
        query_authority: WorthUiQueryAuthorityHandle,
        parts: VerifiedMeasurementFactReceiptParts,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let authority_index_key = WorthUiQueryAuthorityIndexKey {
            projection_source_identity: parts.projection_source_identity.clone().into(),
            query_basis_digest: query_authority
                .authority()
                .contract()
                .basis_digest()
                .unwrap_or_default()
                .into(),
            projection_contract_digest: parts.projection_contract_digest.clone().into(),
            projection_consumption_receipt_digest: parts
                .projection_consumption_receipt_digest
                .clone()
                .into(),
        };
        Ok(Self {
            query_authority,
            authority_index_key,
            consumed_families: parts.consumed_families.into(),
            observations: parts.observations.into(),
            refinement_counters: parts.refinement_counters,
        })
    }

    pub fn binds_prerequisites(&self, prerequisites: &WorthUiQueryPrerequisiteEvidence) -> bool {
        self.query_authority
            .authority()
            .binds_resolved_basis(prerequisites.basis())
            && prerequisites.accepts_projection_contract(self.projection_contract_digest())
    }

    pub fn query_authority(&self) -> &WorthUiQueryAuthorityHandle {
        &self.query_authority
    }

    pub fn authority_index_key(&self) -> &WorthUiQueryAuthorityIndexKey {
        &self.authority_index_key
    }

    pub fn projection_contract_digest(&self) -> &str {
        self.query_authority
            .authority()
            .contract()
            .contract_digest()
    }

    pub fn projection_consumption_declaration_digest(&self) -> &str {
        self.query_authority
            .authority()
            .receipt()
            .declaration_digest()
    }

    pub fn projection_consumption_receipt_digest(&self) -> &str {
        self.query_authority.authority().receipt().receipt_digest()
    }

    pub fn projection_fact_set_digest(&self) -> &str {
        self.query_authority.authority().receipt().fact_set_digest()
    }

    pub fn projection_source_identity(&self) -> &str {
        self.query_authority.authority().source_identity().as_str()
    }

    pub fn consumed_families(&self) -> &[super::WorthUiQueryMeasurementFactFamily] {
        &self.consumed_families
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }

    pub fn refinement_counters(&self) -> super::WorthUiQueryMeasurementRefinementCounters {
        self.refinement_counters
    }

    pub(crate) fn consumed_families_arc(&self) -> Arc<[super::WorthUiQueryMeasurementFactFamily]> {
        Arc::clone(&self.consumed_families)
    }

    pub(crate) fn observations_arc(&self) -> Arc<[WorthUiQueryMeasurementFactObservation]> {
        Arc::clone(&self.observations)
    }
}

impl WorthUiQueryAuthorityIndexKey {
    pub fn projection_source_identity(&self) -> &str {
        &self.projection_source_identity
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
