use bank_domain::model::{InstitutionId, JournalEntryId};
use bank_domain::schema::{ReversalReason, ReverseJournal};

/// Construct a compensating reverse-journal input for an original disbursement.
pub fn compensating_reverse_journal(
    institution: InstitutionId,
    original: JournalEntryId,
) -> ReverseJournal {
    ReverseJournal {
        institution,
        journal: original,
        reason: ReversalReason::OperatorCorrection,
    }
}
