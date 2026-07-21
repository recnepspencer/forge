use worth_query::facade::foundation::{
    MaterializedProjectionContract, ProjectionContractSourcePosture, ProjectionFactKind,
    WorthQueryConsumedProjectionAuthority,
};

use super::{WorthUiQueryMeasurementFactFamily, WorthUiQueryPrerequisiteEvidence};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryMeasurementFactEligibilityError {
    NonQueryOwnedProjectionSource,
    BasisDigestMismatch,
    ProjectionContractMismatch,
    ProjectionConsumptionNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryMeasurementFactEligibility {
    prerequisites: WorthUiQueryPrerequisiteEvidence,
    projection_contract_identity: super::WorthUiQueryProjectionContractIdentity,
    available_families: Box<[WorthUiQueryMeasurementFactFamily]>,
}

impl WorthUiQueryMeasurementFactEligibility {
    fn from_projection_contract_unchecked(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthUiQueryMeasurementFactEligibilityError> {
        let projection_contract_identity =
            super::WorthUiQueryProjectionContractIdentity::from_authority(authority);
        let prerequisites =
            prerequisites.bound_to_projection_contract(projection_contract_identity);
        let contract: &MaterializedProjectionContract = authority.contract();

        let mut available_families = contract
            .fact_families()
            .iter()
            .filter_map(|fact_family| match fact_family.kind() {
                ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedField => {
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
            projection_contract_identity,
            available_families: available_families.into_boxed_slice(),
        })
    }

    pub(crate) fn from_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<Self, WorthUiQueryMeasurementFactEligibilityError> {
        validate_query_authority(&prerequisites, authority)?;
        Self::from_projection_contract_unchecked(prerequisites, authority)
    }

    pub(crate) fn bind_query_authority(
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &WorthQueryConsumedProjectionAuthority,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryMeasurementFactEligibilityError> {
        validate_query_authority(&prerequisites, authority)?;
        Ok(prerequisites.bound_to_projection_contract(
            super::WorthUiQueryProjectionContractIdentity::from_authority(authority),
        ))
    }

    pub fn prerequisites(&self) -> &WorthUiQueryPrerequisiteEvidence {
        &self.prerequisites
    }

    pub const fn projection_contract_identity(
        &self,
    ) -> super::WorthUiQueryProjectionContractIdentity {
        self.projection_contract_identity
    }

    pub fn available_families(&self) -> &[WorthUiQueryMeasurementFactFamily] {
        &self.available_families
    }
}

fn validate_query_authority(
    prerequisites: &WorthUiQueryPrerequisiteEvidence,
    authority: &WorthQueryConsumedProjectionAuthority,
) -> Result<(), WorthUiQueryMeasurementFactEligibilityError> {
    if authority.contract().source_posture()
        != ProjectionContractSourcePosture::QueryOwnedReceiptSource
    {
        return Err(WorthUiQueryMeasurementFactEligibilityError::NonQueryOwnedProjectionSource);
    }
    if !authority.binds_resolved_basis(prerequisites.basis()) {
        return Err(WorthUiQueryMeasurementFactEligibilityError::BasisDigestMismatch);
    }
    if !prerequisites.accepts_projection_contract(
        super::WorthUiQueryProjectionContractIdentity::from_authority(authority),
    ) {
        return Err(WorthUiQueryMeasurementFactEligibilityError::ProjectionContractMismatch);
    }
    Ok(())
}
