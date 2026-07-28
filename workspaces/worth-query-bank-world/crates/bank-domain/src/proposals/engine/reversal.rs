use crate::accounting::{BankJournalEntry, BankPosting};
use crate::model::SignedMoney;
use crate::proposals::{
    account_activity_effects, complete_proposal, ensure_open, BankIdempotencyIntent,
    BankIdempotencyKey, BankInvariantApprovedProposal, BankOperationScopeBinding,
    BankProposalDenial, BankProposedEffect, BankSnapshot, CanonicalProposalPayload,
};
use crate::schema::{PostingPurpose, ReversalReason, ReverseJournal};

use super::BankProposalEngine;

impl BankProposalEngine {
    pub fn prepare_reverse_journal(
        snapshot: &BankSnapshot,
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        input: &ReverseJournal,
    ) -> Result<BankInvariantApprovedProposal, BankProposalDenial> {
        if snapshot.is_reversed(input.journal) {
            return Err(BankProposalDenial::JournalAlreadyReversed(input.journal));
        }
        let original = snapshot
            .journal_entry(input.journal)
            .cloned()
            .ok_or(BankProposalDenial::UnknownJournal(input.journal))?;
        let payload = CanonicalProposalPayload::new()
            .u64(input.journal.get())
            .byte(reversal_reason_tag(input.reason));
        let intent =
            BankIdempotencyIntent::derive(binding, key, "reverse-journal", payload.as_bytes());

        let mut proposed = snapshot.clone();
        let journal_id = proposed.allocate_journal_id()?;
        let mut postings = Vec::with_capacity(original.postings().len());
        for original_posting in original.postings() {
            ensure_open(&proposed, original_posting.account())?;
            let reversed_amount = original_posting
                .amount()
                .minor_units()
                .checked_neg()
                .ok_or(BankProposalDenial::ArithmeticOverflow)?;
            postings.push(BankPosting::new(
                proposed.allocate_posting_id()?,
                original_posting.account(),
                SignedMoney::from_minor(reversed_amount),
            ));
        }
        let reversal = BankJournalEntry::new(
            journal_id,
            PostingPurpose::Reversal,
            postings,
            Some(input.journal),
        );
        proposed.append_journal(reversal.clone());
        proposed.mark_reversed(input.journal);
        let mut effects = vec![BankProposedEffect::ReverseJournal {
            original: input.journal,
            reversal: reversal.clone(),
        }];
        effects.extend(account_activity_effects(&reversal));
        complete_proposal(snapshot, proposed, intent, effects)
    }
}

const fn reversal_reason_tag(reason: ReversalReason) -> u8 {
    match reason {
        ReversalReason::Duplicate => 1,
        ReversalReason::OperatorCorrection => 2,
        ReversalReason::ExternalReturn => 3,
    }
}
