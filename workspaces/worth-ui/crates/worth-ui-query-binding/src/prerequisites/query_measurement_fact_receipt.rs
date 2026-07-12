use std::sync::Arc;
use worth_query::facade::{CompletedProjectionFactConsumption, ProjectionFactConsumptionAttempt};

use super::receipt_construction::{
    collect_verified_partial_receipt_parts, collect_verified_receipt_parts,
    VerifiedMeasurementFactReceiptParts,
};
use super::{
    WorthUiQueryMeasurementFactObservation, WorthUiQueryMeasurementFactObservationError,
    WorthUiQueryPrerequisiteEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactReceiptError {
    NonQueryOwnedProjectionSource,
    BasisDigestMismatch,
    ProjectionConsumptionNotAdmitted,
    Observation(WorthUiQueryMeasurementFactObservationError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactReceipt {
    consumption_identity: WorthUiQueryMeasurementConsumptionIdentity,
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    projection_contract_digest: Box<str>,
    projection_consumption_declaration_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
    projection_fact_set_digest: Box<str>,
    projection_source_identity: Box<str>,
    consumed_families: Arc<[super::WorthUiQueryMeasurementFactFamily]>,
    observations: Arc<[WorthUiQueryMeasurementFactObservation]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiQueryMeasurementConsumptionIdentity {
    projection_source_identity: Box<str>,
    query_basis_digest: Box<str>,
    projection_contract_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
}

impl WorthUiQueryMeasurementFactReceipt {
    pub(crate) fn from_projection_consumption_attempt(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        consumption: &ProjectionFactConsumptionAttempt,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let completed = consumption
            .completed()
            .ok_or(WorthUiQueryMeasurementFactReceiptError::ProjectionConsumptionNotAdmitted)?;
        Self::from_completed_projection_consumption(prerequisites, completed)
    }

    pub(crate) fn from_completed_projection_consumption(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        completed: &CompletedProjectionFactConsumption,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        Self::from_verified_parts(collect_verified_receipt_parts(prerequisites, completed)?)
    }

    pub(crate) fn from_partial_projection_consumption(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        completed: &CompletedProjectionFactConsumption,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        Self::from_verified_parts(collect_verified_partial_receipt_parts(
            prerequisites,
            completed,
        )?)
    }

    fn from_verified_parts(
        parts: VerifiedMeasurementFactReceiptParts,
    ) -> Result<Self, WorthUiQueryMeasurementFactReceiptError> {
        let consumption_identity = WorthUiQueryMeasurementConsumptionIdentity {
            projection_source_identity: parts.projection_source_identity.clone().into(),
            query_basis_digest: parts
                .prerequisites
                .resolution_report()
                .basis_digest()
                .as_str()
                .into(),
            projection_contract_digest: parts.projection_contract_digest.clone().into(),
            projection_consumption_receipt_digest: parts
                .projection_consumption_receipt_digest
                .clone()
                .into(),
        };
        Ok(Self {
            consumption_identity,
            prerequisites: parts.prerequisites,
            projection_contract_digest: parts.projection_contract_digest.into(),
            projection_consumption_declaration_digest: parts
                .projection_consumption_declaration_digest
                .into(),
            projection_consumption_receipt_digest: parts
                .projection_consumption_receipt_digest
                .into(),
            projection_fact_set_digest: parts.projection_fact_set_digest.into(),
            projection_source_identity: parts.projection_source_identity.into(),
            consumed_families: parts.consumed_families.into(),
            observations: parts.observations.into(),
        })
    }

    pub fn prerequisites(&self) -> &WorthUiQueryPrerequisiteEvidence {
        &self.prerequisites
    }

    pub fn consumption_identity(&self) -> &WorthUiQueryMeasurementConsumptionIdentity {
        &self.consumption_identity
    }

    pub fn projection_contract_digest(&self) -> &str {
        &self.projection_contract_digest
    }

    pub fn projection_consumption_declaration_digest(&self) -> &str {
        &self.projection_consumption_declaration_digest
    }

    pub fn projection_consumption_receipt_digest(&self) -> &str {
        &self.projection_consumption_receipt_digest
    }

    pub fn projection_fact_set_digest(&self) -> &str {
        &self.projection_fact_set_digest
    }

    pub fn projection_source_identity(&self) -> &str {
        &self.projection_source_identity
    }

    pub fn consumed_families(&self) -> &[super::WorthUiQueryMeasurementFactFamily] {
        &self.consumed_families
    }

    pub fn observations(&self) -> &[WorthUiQueryMeasurementFactObservation] {
        &self.observations
    }

    pub(crate) fn consumed_families_arc(&self) -> Arc<[super::WorthUiQueryMeasurementFactFamily]> {
        Arc::clone(&self.consumed_families)
    }

    pub(crate) fn observations_arc(&self) -> Arc<[WorthUiQueryMeasurementFactObservation]> {
        Arc::clone(&self.observations)
    }

    #[cfg(feature = "certification-construction")]
    pub(crate) fn for_certification(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl Into<Box<str>>,
        projection_consumption_declaration_digest: impl Into<Box<str>>,
        projection_consumption_receipt_digest: impl Into<Box<str>>,
        projection_fact_set_digest: impl Into<Box<str>>,
        projection_source_identity: impl Into<Box<str>>,
        mut consumed_families: Vec<super::WorthUiQueryMeasurementFactFamily>,
        observations: Vec<WorthUiQueryMeasurementFactObservation>,
    ) -> Self {
        consumed_families.sort_unstable();
        consumed_families.dedup();
        let projection_contract_digest = projection_contract_digest.into();
        let prerequisites =
            prerequisites.bound_to_projection_contract(projection_contract_digest.as_ref());
        let projection_consumption_receipt_digest = projection_consumption_receipt_digest.into();
        let projection_source_identity = projection_source_identity.into();
        let consumption_identity = WorthUiQueryMeasurementConsumptionIdentity {
            projection_source_identity: projection_source_identity.clone(),
            query_basis_digest: prerequisites
                .resolution_report()
                .basis_digest()
                .as_str()
                .into(),
            projection_contract_digest: projection_contract_digest.clone(),
            projection_consumption_receipt_digest: projection_consumption_receipt_digest.clone(),
        };
        Self {
            consumption_identity,
            prerequisites,
            projection_contract_digest,
            projection_consumption_declaration_digest: projection_consumption_declaration_digest
                .into(),
            projection_consumption_receipt_digest,
            projection_fact_set_digest: projection_fact_set_digest.into(),
            projection_source_identity,
            consumed_families: consumed_families.into(),
            observations: observations.into(),
        }
    }
}

impl WorthUiQueryMeasurementConsumptionIdentity {
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
