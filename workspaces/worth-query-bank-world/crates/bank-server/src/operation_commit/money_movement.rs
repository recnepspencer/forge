use bank_domain::proposals::BankProposedEffect;
use bank_domain::schema::*;
use worth_query_host::facade::declaration::application_schema::OperationEmits;
use worth_query_host::facade::domain::{
    OperationCreates, OperationLinks, OperationReads, OperationWrites,
};

use super::journal::{lower_journal, resolve_journal_accounts};
use super::{application_idempotency, BankCommitPreparationDenial, BankMutationCommitOutcome};
use crate::{BankAuthorizedProposal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn commit_opening_funding(
        &self,
        proposal: BankAuthorizedProposal<
            ApplyOpeningFundingOperation,
            ApplyOpeningFunding,
            Institution,
            bank_domain::model::InstitutionId,
        >,
    ) -> Result<BankMutationCommitOutcome, BankCommitPreparationDenial> {
        commit_journal_proposal(self, proposal)
    }

    pub fn commit_deposit(
        &self,
        proposal: BankAuthorizedProposal<
            DepositOperation,
            Deposit,
            Institution,
            bank_domain::model::InstitutionId,
        >,
    ) -> Result<BankMutationCommitOutcome, BankCommitPreparationDenial> {
        commit_journal_proposal(self, proposal)
    }

    pub fn commit_withdrawal(
        &self,
        proposal: BankAuthorizedProposal<
            WithdrawOperation,
            Withdraw,
            Institution,
            bank_domain::model::InstitutionId,
        >,
    ) -> Result<BankMutationCommitOutcome, BankCommitPreparationDenial> {
        commit_journal_proposal(self, proposal)
    }

    pub fn commit_send_money(
        &self,
        proposal: BankAuthorizedProposal<SendMoneyOperation, SendMoney, Account, AccountId>,
    ) -> Result<BankMutationCommitOutcome, BankCommitPreparationDenial> {
        commit_journal_proposal(self, proposal)
    }
}

fn commit_journal_proposal<Operation, Input, Scope, ScopeIdentity>(
    runtime: &BankIdentityRuntime,
    proposal: BankAuthorizedProposal<Operation, Input, Scope, ScopeIdentity>,
) -> Result<BankMutationCommitOutcome, BankCommitPreparationDenial>
where
    ScopeIdentity: Copy,
    Input: Clone + Send + Sync + 'static,
    AccountIdentity: OperationReads<Operation>,
    AccountingRevision: OperationReads<Operation>,
    JournalEntry: OperationCreates<Operation>,
    Posting: OperationCreates<Operation>,
    JournalIdentityField: OperationWrites<Operation>,
    JournalPurpose: OperationWrites<Operation>,
    PostingIdentityField: OperationWrites<Operation>,
    PostingAmount: OperationWrites<Operation>,
    PostingAccountSequence: OperationWrites<Operation>,
    Purpose: OperationWrites<Operation>,
    AccountingRevision: OperationWrites<Operation>,
    JournalPosting: OperationLinks<Operation>,
    PostingAccount: OperationLinks<Operation>,
    AccountActivityEffect: OperationEmits<Operation>,
{
    let (admission, invariant, projection) = proposal.into_parts();
    let [BankProposedEffect::AppendJournal(journal)] = invariant.effects() else {
        return Err(BankCommitPreparationDenial::InvalidProposalShape);
    };
    let (_, _, query_admission) = admission.into_parts();
    let mut reads = runtime
        .application_runtime()
        .begin_projected_application_read_attempt(query_admission, projection)?;
    let accounts = resolve_journal_accounts(&mut reads, journal)?;
    let mut effects = reads
        .complete_projected_dependencies()?
        .begin_effect_program();
    lower_journal(&mut effects, journal, accounts)?;
    let program = effects.finish()?;
    let idempotency = application_idempotency(&invariant);
    Ok(runtime
        .application_runtime()
        .compare_and_commit_application(program, idempotency)
        .into())
}

use bank_domain::model::AccountId;
