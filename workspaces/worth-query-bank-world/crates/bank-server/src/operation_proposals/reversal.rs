use bank_domain::model::InstitutionId;
use bank_domain::proposals::{BankIdempotencyKey, BankProposalDenial, BankProposalEngine};
use bank_domain::schema::{Institution, ReverseJournal, ReverseJournalOperation};

use crate::bank_projection::project_journal_reversal;
use crate::{
    BankAdmittedOperation, BankAuthorizedProposal, BankIdentityRuntime, BankOperationProposalError,
    BankOperationProposals,
};

impl BankOperationProposals {
    pub fn prepare_reverse_journal(
        runtime: &BankIdentityRuntime,
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
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime.invariant_projection().project_admitted_operation(
            admission.query(),
            |reader, institution| {
                project_journal_reversal(reader, institution, admission.scope(), input)
            },
        )?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        if let Some(journal) = snapshot.journal_entry(input.journal) {
            let matches_scope = journal.postings().iter().all(|posting| {
                snapshot
                    .account(posting.account())
                    .is_some_and(|account| account.institution() == input.institution)
            });
            if !matches_scope {
                return Err(BankProposalDenial::ScopeInputMismatch.into());
            }
        }
        let invariant = BankProposalEngine::prepare_reverse_journal(
            &snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }
}
