use forge_query::facade::{
    CompletedProjectionFactConsumption, ProjectionContractSourcePosture,
    ProjectionFactConsumptionAttempt, ProjectionFactKind,
};

use super::{WorthUiQueryMeasurementFactFamily, WorthUiQueryPrerequisiteEvidence};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactReceiptError {
    NonQueryOwnedProjectionSource,
    BasisDigestMismatch,
    ProjectionConsumptionNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactReceipt {
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    projection_contract_digest: Box<str>,
    projection_consumption_declaration_digest: Box<str>,
    projection_consumption_receipt_digest: Box<str>,
    projection_fact_set_digest: Box<str>,
    projection_source_identity: Box<str>,
    consumed_families: Box<[WorthUiQueryMeasurementFactFamily]>,
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
        validate_projection_contract(&prerequisites, completed)?;
        let prerequisites = prerequisites
            .bound_to_projection_contract(completed.contract().contract_digest());
        let mut consumed_families = completed
            .contract()
            .fact_families()
            .iter()
            .filter_map(|fact_family| match fact_family.kind() {
                ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                    Some(WorthUiQueryMeasurementFactFamily::ScrollContentExtent)
                }
                ProjectionFactKind::EntityIdentity
                | ProjectionFactKind::ViewLocalIdentity
                | ProjectionFactKind::TargetIdentity
                | ProjectionFactKind::SourceReference
                | ProjectionFactKind::EffectContinuity
                | ProjectionFactKind::Membership
                | ProjectionFactKind::RelationEndpoint => None,
            })
            .collect::<Vec<_>>();
        consumed_families.sort_unstable();
        consumed_families.dedup();

        Ok(Self {
            prerequisites,
            projection_contract_digest: completed.contract().contract_digest().into(),
            projection_consumption_declaration_digest: completed
                .receipt()
                .declaration_digest()
                .into(),
            projection_consumption_receipt_digest: completed.receipt().receipt_digest().into(),
            projection_fact_set_digest: completed.receipt().fact_set_digest().into(),
            projection_source_identity: completed.receipt().source_identity().into(),
            consumed_families: consumed_families.into_boxed_slice(),
        })
    }

    pub fn prerequisites(&self) -> &WorthUiQueryPrerequisiteEvidence {
        &self.prerequisites
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

    pub fn consumed_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.consumed_families
    }

    #[cfg(feature = "certification-construction")]
    pub(crate) fn for_certification(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl Into<Box<str>>,
        projection_consumption_declaration_digest: impl Into<Box<str>>,
        projection_consumption_receipt_digest: impl Into<Box<str>>,
        projection_fact_set_digest: impl Into<Box<str>>,
        projection_source_identity: impl Into<Box<str>>,
        mut consumed_families: Vec<WorthUiQueryMeasurementFactFamily>,
    ) -> Self {
        consumed_families.sort_unstable();
        consumed_families.dedup();
        let projection_contract_digest = projection_contract_digest.into();
        Self {
            prerequisites: prerequisites
                .bound_to_projection_contract(projection_contract_digest.as_ref()),
            projection_contract_digest,
            projection_consumption_declaration_digest: projection_consumption_declaration_digest
                .into(),
            projection_consumption_receipt_digest: projection_consumption_receipt_digest.into(),
            projection_fact_set_digest: projection_fact_set_digest.into(),
            projection_source_identity: projection_source_identity.into(),
            consumed_families: consumed_families.into_boxed_slice(),
        }
    }
}

fn validate_projection_contract(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
    completed: &CompletedProjectionFactConsumption,
) -> Result<(), WorthUiQueryMeasurementFactReceiptError> {
    if completed.contract().source_posture() != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::NonQueryOwnedProjectionSource);
    }
    if completed.contract().basis_digest()
        != Some(prerequisites.resolution_report().basis_digest().as_str())
    {
        return Err(WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch);
    }
    Ok(())
}
