use bank_domain::model::InstitutionId;
use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankSnapshot,
};
use bank_domain::schema::{Institution, ReverseJournal, ReverseJournalOperation};

use crate::{BankAdmittedOperation, BankAuthorizedProposal, BankOperationProposals};

impl BankOperationProposals {
    pub fn prepare_reverse_journal(
        snapshot: &BankSnapshot,
        admission: BankAdmittedOperation<
            ReverseJournalOperation,
            ReverseJournal,
            Institution,
            InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &ReverseJournal,
    ) -> Result<
        BankAuthorizedProposal<ReverseJournalOperation, ReverseJournal, Institution, InstitutionId>,
        BankProposalDenial,
    > {
        if let Some(journal) = snapshot.journal_entry(input.journal) {
            let matches_scope = journal.postings().iter().all(|posting| {
                snapshot
                    .account(posting.account())
                    .is_some_and(|account| account.institution() == admission.scope())
            });
            if !matches_scope {
                return Err(BankProposalDenial::ScopeInputMismatch);
            }
        }
        let invariant = BankProposalEngine::prepare_reverse_journal(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }
}
