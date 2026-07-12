use worth_query::facade::{
    MaterializedProjectionContract, ProjectionContractSourcePosture, ProjectionFactKind,
    WorthQueryConsumedProjectionAuthority,
};

use super::{WorthUiQueryMeasurementFactFamily, WorthUiQueryPrerequisiteEvidence};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactEligibilityError {
    NonQueryOwnedProjectionSource,
    BasisDigestMismatch,
    ProjectionConsumptionNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactEligibility {
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    projection_contract_digest: Box<str>,
    available_families: Box<[WorthUiQueryMeasurementFactFamily]>,
}

impl WorthUiQueryMeasurementFactEligibility {
    pub(crate) fn from_projection_contract(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        contract: &MaterializedProjectionContract,
    ) -> Result<Self, WorthUiQueryMeasurementFactEligibilityError> {
        validate_projection_contract(&prerequisites, contract)?;
        let prerequisites = prerequisites.bound_to_projection_contract(contract.contract_digest());

        let mut available_families = contract
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
        available_families.sort_unstable();
        available_families.dedup();

        Ok(Self {
            prerequisites,
            projection_contract_digest: contract.contract_digest().into(),
            available_families: available_families.into_boxed_slice(),
        })
    }

    pub(crate) fn from_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthUiQueryMeasurementFactEligibilityError> {
        Self::from_projection_contract(prerequisites, authority.contract())
    }

    pub(crate) fn bind_projection_contract(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        contract: &MaterializedProjectionContract,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryMeasurementFactEligibilityError> {
        validate_projection_contract(&prerequisites, contract)?;
        Ok(prerequisites.bound_to_projection_contract(contract.contract_digest()))
    }

    pub(crate) fn bind_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryMeasurementFactEligibilityError> {
        Self::bind_projection_contract(prerequisites, authority.contract())
    }

    #[cfg(feature = "certification-construction")]
    pub(crate) fn for_certification(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl Into<Box<str>>,
        mut available_families: Vec<WorthUiQueryMeasurementFactFamily>,
    ) -> Self {
        available_families.sort_unstable();
        available_families.dedup();
        let projection_contract_digest = projection_contract_digest.into();
        Self {
            prerequisites: prerequisites
                .bound_to_projection_contract(projection_contract_digest.as_ref()),
            projection_contract_digest,
            available_families: available_families.into_boxed_slice(),
        }
    }

    pub fn prerequisites(&self) -> &WorthUiQueryPrerequisiteEvidence {
        &self.prerequisites
    }

    pub fn projection_contract_digest(&self) -> &str {
        &self.projection_contract_digest
    }

    pub fn available_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.available_families
    }
}

fn validate_projection_contract(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
    contract: &MaterializedProjectionContract,
) -> Result<(), WorthUiQueryMeasurementFactEligibilityError> {
    if contract.source_posture() != ProjectionContractSourcePosture::QueryOwnedReceiptSource {
        return Err(WorthUiQueryMeasurementFactEligibilityError::NonQueryOwnedProjectionSource);
    }
    if contract.basis_digest() != Some(prerequisites.resolution_report().basis_digest().as_str()) {
        return Err(WorthUiQueryMeasurementFactEligibilityError::BasisDigestMismatch);
    }
    Ok(())
}
