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
    NonCanonicalBasisAuthority,
    NonCanonicalSourceIdentity,
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
    canonical_basis_identity: [u8; 32],
    canonical_source_identity: [u8; 32],
    projection_contract_identity: u64,
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
        let authority_index_key = WorthUiQueryAuthorityIndexKey::from_authority(&query_authority)?;
        Ok(Self {
            query_authority,
            authority_index_key,
            consumed_families: parts.consumed_families.into(),
            observations: parts.observations.into(),
            refinement_counters: parts.refinement_counters,
        })
    }

    pub fn binds_prerequisites(&self, prerequisites: &WorthUiQueryPrerequisiteEvidence) -> bool {
        self.query_authority.binds_prerequisites(prerequisites)
    }

    pub fn query_authority(&self) -> &WorthUiQueryAuthorityHandle {
        &self.query_authority
    }

    pub fn authority_index_key(&self) -> &WorthUiQueryAuthorityIndexKey {
        &self.authority_index_key
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
    pub(crate) fn from_authority(
        authority: &WorthUiQueryAuthorityHandle,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let canonical_basis_identity =
            authority
                .authority()
                .basis_authority()
                .canonical_digest()
                .ok_or(WorthUiQueryMeasurementFactReceiptError::NonCanonicalBasisAuthority)?;
        let canonical_source_identity = authority
            .authority()
            .source_identity()
            .evidence_identity()
            .ok_or(WorthUiQueryMeasurementFactReceiptError::NonCanonicalSourceIdentity)?;
        Ok(Self {
            canonical_basis_identity: *canonical_basis_identity.value().bytes(),
            canonical_source_identity: *canonical_source_identity
                .canonical_digest()
                .value()
                .bytes(),
            projection_contract_identity:
                super::WorthUiQueryProjectionContractIdentity::from_authority(authority.authority())
                    .as_u64(),
        })
    }

    pub const fn canonical_basis_identity(&self) -> &[u8; 32] {
        &self.canonical_basis_identity
    }

    pub const fn canonical_source_identity(&self) -> &[u8; 32] {
        &self.canonical_source_identity
    }

    pub const fn projection_contract_identity(&self) -> u64 {
        self.projection_contract_identity
    }

    pub fn identity_digest(&self) -> u64 {
        fold_bytes(0x776f_7274_6875_6921, &self.canonical_basis_identity)
            ^ fold_bytes(0x7175_6572_795f_7569, &self.canonical_source_identity).rotate_left(17)
            ^ self.projection_contract_identity.rotate_left(37)
    }
}

fn fold_bytes(mut identity: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        identity ^= u64::from(*byte);
        identity = identity.wrapping_mul(0x100_0000_01b3);
    }
    identity
}
