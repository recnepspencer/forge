use bank_domain::proposals::BankSnapshot;
use bank_domain::schema::{
    CreateBusinessAccount, CreateBusinessAccountOperation, CreatePersonalAccount,
    CreatePersonalAccountOperation,
};

use super::bounded::{BoundedProjectionState, InstitutionEntity, ProjectionReader};
use super::BankProjectionDenial;

pub(crate) fn project_personal_account_creation(
    reader: &mut ProjectionReader<'_, '_, CreatePersonalAccountOperation>,
    institution: &InstitutionEntity,
    input: &CreatePersonalAccount,
) -> Result<BankSnapshot, BankProjectionDenial> {
    let mut state = BoundedProjectionState::new(reader)?;
    state.project_admitted_institution(reader, institution, input.institution)?;
    let owner = state.project_principal(reader, input.owner)?;
    state.project_primary_account(reader, &owner)?;
    state
        .finish()
        .build()
        .map_err(BankProjectionDenial::InvalidDomainState)
}

pub(crate) fn project_business_account_creation(
    reader: &mut ProjectionReader<'_, '_, CreateBusinessAccountOperation>,
    institution: &InstitutionEntity,
    input: &CreateBusinessAccount,
) -> Result<BankSnapshot, BankProjectionDenial> {
    let mut state = BoundedProjectionState::new(reader)?;
    state.project_admitted_institution(reader, institution, input.institution)?;
    let business = state.project_business(reader, input.business)?;
    state.project_business_account(reader, &business)?;
    state
        .finish()
        .build()
        .map_err(BankProjectionDenial::InvalidDomainState)
}
