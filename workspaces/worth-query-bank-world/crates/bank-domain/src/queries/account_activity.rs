use worth_query_decl::facade::worth_query_application_query;
use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryLiveCauseBinding, ApplicationQueryLiveResourceContract,
        ApplicationQueryOrderingDirection, ApplicationQueryResultFieldRef,
        ApplicationQueryResultRelationRef, ApplicationQueryResultShapeBuilder, ExactlyOneResult,
        ManyResults, OptionalOneResult, ReverseResultTraversal,
    },
    application_schema::{
        DeclaredApplicationCurrency, EqualityPredicate, NoApplicationCurrency, NoEqualityPredicate,
        ReadOnly, ReadWrite,
    },
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::authorization::ViewAccount;
use crate::model::{
    AccountId, AccountJournalRevision, JournalEntryId, PostingId, SignedMoney, USD,
};
use crate::reads::AccountActivityItem;
use crate::schema::{
    Account, AccountActivityEffect, AccountIdentity, ActivityEvent, BankSchema, Identity,
    JournalEntry, JournalIdentity, JournalIdentityField, JournalPosting, JournalPurpose,
    JournalReversal, JournalState, Posting, PostingAccount, PostingAccountSequence, PostingAmount,
    PostingIdentity, PostingIdentityField, PostingPurpose, PostingValue, Purpose, UsdCurrency,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountActivityQueryParameters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountActivityQueryResult {
    account: AccountId,
    entries: Vec<AccountActivityItem>,
}

impl AccountActivityQueryResult {
    pub const fn account(&self) -> AccountId {
        self.account
    }

    pub fn entries(&self) -> &[AccountActivityItem] {
        &self.entries
    }
}

worth_query_application_query!(
    pub AccountActivityQuery in BankSchema,
    parameters AccountActivityQueryParameters,
    result AccountActivityQueryResult,
    scope Account,
    name "account_activity"
);

struct AccountIdentitySlot;
struct AccountPostingsSlot;
struct PostingSequenceSlot;
struct PostingIdentitySlot;
struct PostingAmountSlot;
struct PostingPurposeSlot;
struct PostingJournalSlot;
struct JournalIdentitySlot;
struct JournalPurposeSlot;
struct JournalReversalSlot;
struct ReversalIdentitySlot;

type AccountIdentitySelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    AccountIdentitySlot,
    BankSchema,
    Account,
    Identity,
    AccountIdentity,
    AccountId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PostingSequenceSelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    PostingSequenceSlot,
    BankSchema,
    Posting,
    PostingValue,
    PostingAccountSequence,
    AccountJournalRevision,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PostingIdentitySelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    PostingIdentitySlot,
    BankSchema,
    Posting,
    PostingIdentity,
    PostingIdentityField,
    PostingId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PostingAmountSelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    PostingAmountSlot,
    BankSchema,
    Posting,
    PostingValue,
    PostingAmount,
    SignedMoney<USD>,
    ReadWrite,
    NoEqualityPredicate,
    DeclaredApplicationCurrency<UsdCurrency, USD>,
>;

type PostingPurposeSelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    PostingPurposeSlot,
    BankSchema,
    Posting,
    PostingValue,
    Purpose,
    PostingPurpose,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type JournalIdentitySelector<Slot> = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    Slot,
    BankSchema,
    JournalEntry,
    JournalIdentity,
    JournalIdentityField,
    JournalEntryId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type JournalPurposeSelector = ApplicationQueryResultFieldRef<
    AccountActivityQuery,
    JournalPurposeSlot,
    BankSchema,
    JournalEntry,
    JournalState,
    JournalPurpose,
    PostingPurpose,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
>;

pub fn account_activity_definition() -> ApplicationQueryDefinition<
    BankSchema,
    AccountActivityQuery,
    AccountActivityQueryParameters,
    AccountActivityQueryResult,
    Account,
> {
    let reversal = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        AccountActivityQuery,
        JournalEntry,
        (),
    >::new(JournalEntry::reference())
    .field(reversal_identity());
    let journal = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        AccountActivityQuery,
        JournalEntry,
        (),
    >::new(JournalEntry::reference())
    .field(journal_identity())
    .field(journal_purpose())
    .relation(journal_reversal(), reversal);
    let posting =
        ApplicationQueryResultShapeBuilder::<BankSchema, AccountActivityQuery, Posting, ()>::new(
            Posting::reference(),
        )
        .field(posting_identity())
        .field(posting_sequence())
        .field(posting_amount())
        .field(posting_purpose())
        .relation(posting_journal(), journal);
    let shape = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        AccountActivityQuery,
        Account,
        AccountActivityQueryResult,
    >::new(Account::reference())
    .field(account_identity())
    .relation(account_postings(), posting)
    .build();
    ApplicationQueryDefinitionBuilder::requires_ability(
        AccountActivityQuery::reference(),
        Account::reference(),
        Account::reference(),
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(3, 3, 8),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot()
            .with_historical()
            .with_live(),
        ViewAccount::reference(),
    )
    .order_by(
        posting_sequence(),
        ApplicationQueryOrderingDirection::Ascending,
    )
    .continue_by(account_postings())
    .live_by::<Posting, AccountActivityLiveCause, _, _, _, _, _, _, _, _>(
        account_identity(),
        posting_identity(),
        ApplicationQueryLiveResourceContract::bounded(64, 2_048, 4_096),
    )
    .build()
    .expect("bank account activity query is statically canonical")
}

impl WorthQueryApplicationProjection<BankSchema, AccountActivityQuery>
    for AccountActivityQueryResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, AccountActivityQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let account = row.field(account_identity())?;
        let entries = row
            .many(account_postings())?
            .iter()
            .map(|posting| project_posting(account, &posting))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { account, entries })
    }
}

fn project_posting(
    account: AccountId,
    posting: &WorthQueryApplicationProjectionRow<'_, BankSchema, AccountActivityQuery>,
) -> Result<AccountActivityItem, WorthQueryApplicationProjectionDenial> {
    let _posting_id = posting.field(posting_identity())?;
    let sequence = posting.field(posting_sequence())?;
    let amount = posting.field(posting_amount())?;
    let purpose = posting.field(posting_purpose())?;
    let journal = posting.one(posting_journal())?;
    let journal_id = journal.field(journal_identity())?;
    let journal_purpose = journal.field(journal_purpose())?;
    if journal_purpose != purpose {
        return Err(WorthQueryApplicationProjectionDenial::reject(
            "posting and journal purpose disagree",
        ));
    }
    let reversal_of = journal
        .optional(journal_reversal())?
        .map(|reversal| reversal.field(reversal_identity()))
        .transpose()?;
    Ok(AccountActivityItem::from_projection(
        account,
        sequence,
        journal_id,
        purpose,
        amount,
        reversal_of,
    ))
}

fn account_identity() -> AccountIdentitySelector {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

fn posting_sequence() -> PostingSequenceSelector {
    ApplicationQueryResultFieldRef::new("sequence", PostingAccountSequence::reference())
}

fn posting_identity() -> PostingIdentitySelector {
    ApplicationQueryResultFieldRef::new("posting", PostingIdentityField::reference())
}

pub struct AccountActivityLiveCause;

impl ApplicationQueryLiveCauseBinding<BankSchema, AccountActivityQuery, Account, Posting>
    for AccountActivityLiveCause
{
    type Effect = AccountActivityEffect;
    type Payload = ActivityEvent;
    type ScopeIdentity = AccountId;
    type TargetIdentity = PostingId;

    fn effect() -> worth_query_decl::facade::application_schema::ApplicationEffectRef<
        BankSchema,
        Self::Effect,
        Self::Payload,
    > {
        AccountActivityEffect::reference()
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.account
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.posting
    }
}

fn posting_amount() -> PostingAmountSelector {
    ApplicationQueryResultFieldRef::new("amount", PostingAmount::reference())
}

fn posting_purpose() -> PostingPurposeSelector {
    ApplicationQueryResultFieldRef::new("purpose", Purpose::reference())
}

fn journal_identity() -> JournalIdentitySelector<JournalIdentitySlot> {
    ApplicationQueryResultFieldRef::new("journal", JournalIdentityField::reference())
}

fn reversal_identity() -> JournalIdentitySelector<ReversalIdentitySlot> {
    ApplicationQueryResultFieldRef::new("reversal_of", JournalIdentityField::reference())
}

fn journal_purpose() -> JournalPurposeSelector {
    ApplicationQueryResultFieldRef::new("purpose", JournalPurpose::reference())
}

fn account_postings() -> ApplicationQueryResultRelationRef<
    AccountActivityQuery,
    AccountPostingsSlot,
    BankSchema,
    PostingAccount,
    Posting,
    Account,
    ReverseResultTraversal,
    ManyResults,
> {
    ApplicationQueryResultRelationRef::reverse_many("postings", PostingAccount::reference())
}

fn posting_journal() -> ApplicationQueryResultRelationRef<
    AccountActivityQuery,
    PostingJournalSlot,
    BankSchema,
    JournalPosting,
    JournalEntry,
    Posting,
    ReverseResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_one("journal", JournalPosting::reference())
}

fn journal_reversal() -> ApplicationQueryResultRelationRef<
    AccountActivityQuery,
    JournalReversalSlot,
    BankSchema,
    JournalReversal,
    JournalEntry,
    JournalEntry,
    worth_query_decl::facade::application_query::ForwardResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::forward_optional("reversal_of", JournalReversal::reference())
}

#[cfg(test)]
mod tests;
