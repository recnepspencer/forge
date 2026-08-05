use bank_domain::proposals::BankSnapshot;
use bank_domain::schema::{ReverseJournal, ReverseJournalOperation};

use super::bounded::{BoundedProjectionState, InstitutionEntity, ProjectionReader};
use super::BankProjectionDenial;

pub(crate) fn project_journal_reversal(
    reader: &mut ProjectionReader<'_, '_, ReverseJournalOperation>,
    institution: &InstitutionEntity,
    institution_id: bank_domain::model::InstitutionId,
    input: &ReverseJournal,
) -> Result<BankSnapshot, BankProjectionDenial> {
    let mut state = BoundedProjectionState::new(reader)?;
    state.project_admitted_institution(reader, institution, institution_id)?;
    state.project_journal_neighborhood(reader, input.journal)?;
    state
        .finish()
        .build()
        .map_err(BankProjectionDenial::InvalidDomainState)
}
