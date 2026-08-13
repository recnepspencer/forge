use bank_domain::schema::ReverseJournal;
use worth_query_host::facade::provisional_aftermath::WorthQueryUndoProgressionHandoff;

use super::BankUndoRetry;
use crate::estate_progression::{BankCompensationUndoAdmission, BankRecordedInverseUndoAdmission};

pub(super) fn compensation_retry(
    handoff: WorthQueryUndoProgressionHandoff,
    reverse_journal: ReverseJournal,
) -> BankUndoRetry {
    let (query, _) = handoff.into_parts();
    BankUndoRetry::Compensation(BankCompensationUndoAdmission::new(query, reverse_journal))
}

pub(super) fn recorded_inverse_retry(handoff: WorthQueryUndoProgressionHandoff) -> BankUndoRetry {
    let (query, _) = handoff.into_parts();
    BankUndoRetry::RecordedInverse(BankRecordedInverseUndoAdmission::new(query))
}
