use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef,
        ApplicationQueryResultShapeBuilder, ExactlyOneResult, ForwardResultTraversal,
        OptionalOneResult, ReverseResultTraversal,
    },
    application_schema::{
        DeclaredApplicationCurrency, EqualityPredicate, NoApplicationCurrency, NoEqualityPredicate,
        ReadOnly, ReadWrite,
    },
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjectionDenial, WorthQueryApplicationProjectionRow,
};

use crate::model::{AccountId, BankPrincipalId, BusinessId, Money, PaymentId, USD};
use crate::reads::PaymentSummary;
use crate::schema::{
    Account, AccountIdentity, Approval, ApprovalPrincipal, BankSchema, Business, BusinessIdentity,
    BusinessIdentityField, PaymentAmount, PaymentApproval, PaymentBusiness, PaymentDestination,
    PaymentIdentity, PaymentIdentityField, PaymentInitiator, PaymentIntent, PaymentSource,
    PaymentState, PaymentStatus, PaymentStatusField, PaymentValue, Principal, PrincipalIdentity,
    PrincipalIdentityField, UsdCurrency,
};

pub(super) struct PaymentIdentitySlot;
struct PaymentAmountSlot;
struct PaymentStatusSlot;
struct PaymentSourceSlot;
struct SourceIdentitySlot;
struct PaymentDestinationSlot;
struct DestinationIdentitySlot;
struct PaymentBusinessSlot;
struct BusinessIdentitySlot;
struct PaymentInitiatorSlot;
struct InitiatorIdentitySlot;
struct PaymentApprovalSlot;
struct ApprovalPrincipalSlot;
struct DecidingPrincipalIdentitySlot;

type PaymentIdentitySelector<Query> = ApplicationQueryResultFieldRef<
    Query,
    PaymentIdentitySlot,
    BankSchema,
    PaymentIntent,
    PaymentIdentity,
    PaymentIdentityField,
    PaymentId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PaymentAmountSelector<Query> = ApplicationQueryResultFieldRef<
    Query,
    PaymentAmountSlot,
    BankSchema,
    PaymentIntent,
    PaymentValue,
    PaymentAmount,
    Money<USD>,
    ReadWrite,
    NoEqualityPredicate,
    DeclaredApplicationCurrency<UsdCurrency, USD>,
>;

type PaymentStatusSelector<Query> = ApplicationQueryResultFieldRef<
    Query,
    PaymentStatusSlot,
    BankSchema,
    PaymentIntent,
    PaymentState,
    PaymentStatusField,
    PaymentStatus,
    ReadWrite,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type AccountIdentitySelector<Query, Slot> = ApplicationQueryResultFieldRef<
    Query,
    Slot,
    BankSchema,
    Account,
    crate::schema::Identity,
    AccountIdentity,
    AccountId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type BusinessIdentitySelector<Query> = ApplicationQueryResultFieldRef<
    Query,
    BusinessIdentitySlot,
    BankSchema,
    Business,
    BusinessIdentity,
    BusinessIdentityField,
    BusinessId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

type PrincipalIdentitySelector<Query, Slot> = ApplicationQueryResultFieldRef<
    Query,
    Slot,
    BankSchema,
    Principal,
    PrincipalIdentity,
    PrincipalIdentityField,
    BankPrincipalId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
>;

pub(super) fn payment_summary_shape<Query, Result>(
) -> ApplicationQueryResultShapeBuilder<BankSchema, Query, PaymentIntent, Result>
where
    Query: 'static,
{
    let source = ApplicationQueryResultShapeBuilder::<BankSchema, Query, Account, ()>::new(
        Account::reference(),
    )
    .field(source_identity());
    let destination = ApplicationQueryResultShapeBuilder::<BankSchema, Query, Account, ()>::new(
        Account::reference(),
    )
    .field(destination_identity());
    let business = ApplicationQueryResultShapeBuilder::<BankSchema, Query, Business, ()>::new(
        Business::reference(),
    )
    .field(business_identity());
    let initiator = ApplicationQueryResultShapeBuilder::<BankSchema, Query, Principal, ()>::new(
        Principal::reference(),
    )
    .field(initiator_identity());
    let deciding_principal =
        ApplicationQueryResultShapeBuilder::<BankSchema, Query, Principal, ()>::new(
            Principal::reference(),
        )
        .field(deciding_principal_identity());
    let approval = ApplicationQueryResultShapeBuilder::<BankSchema, Query, Approval, ()>::new(
        Approval::reference(),
    )
    .relation(approval_principal(), deciding_principal);
    ApplicationQueryResultShapeBuilder::new(PaymentIntent::reference())
        .field(payment_identity())
        .field(payment_amount())
        .field(payment_status())
        .relation(payment_source(), source)
        .relation(payment_destination(), destination)
        .relation(payment_business(), business)
        .relation(payment_initiator(), initiator)
        .relation(payment_approval(), approval)
}

pub(super) fn project_payment_summary<Query>(
    row: &WorthQueryApplicationProjectionRow<'_, BankSchema, Query>,
) -> Result<PaymentSummary, WorthQueryApplicationProjectionDenial>
where
    Query: 'static,
{
    let deciding_principal = row
        .optional(payment_approval())?
        .map(|approval| {
            approval
                .one(approval_principal())?
                .field(deciding_principal_identity())
        })
        .transpose()?;
    Ok(PaymentSummary::from_projection(
        row.field(payment_identity())?,
        row.one(payment_business())?.field(business_identity())?,
        row.one(payment_source())?.field(source_identity())?,
        row.one(payment_destination())?
            .field(destination_identity())?,
        row.one(payment_initiator())?.field(initiator_identity())?,
        row.field(payment_amount())?,
        row.field(payment_status())?,
        deciding_principal,
    ))
}

pub(super) fn payment_identity<Query>() -> PaymentIdentitySelector<Query> {
    ApplicationQueryResultFieldRef::new("payment", PaymentIdentityField::reference())
}

fn payment_amount<Query>() -> PaymentAmountSelector<Query> {
    ApplicationQueryResultFieldRef::new("amount", PaymentAmount::reference())
}

fn payment_status<Query>() -> PaymentStatusSelector<Query> {
    ApplicationQueryResultFieldRef::new("status", PaymentStatusField::reference())
}

fn source_identity<Query>() -> AccountIdentitySelector<Query, SourceIdentitySlot> {
    ApplicationQueryResultFieldRef::new("source", AccountIdentity::reference())
}

fn destination_identity<Query>() -> AccountIdentitySelector<Query, DestinationIdentitySlot> {
    ApplicationQueryResultFieldRef::new("destination", AccountIdentity::reference())
}

fn business_identity<Query>() -> BusinessIdentitySelector<Query> {
    ApplicationQueryResultFieldRef::new("business", BusinessIdentityField::reference())
}

fn initiator_identity<Query>() -> PrincipalIdentitySelector<Query, InitiatorIdentitySlot> {
    ApplicationQueryResultFieldRef::new("initiator", PrincipalIdentityField::reference())
}

fn deciding_principal_identity<Query>(
) -> PrincipalIdentitySelector<Query, DecidingPrincipalIdentitySlot> {
    ApplicationQueryResultFieldRef::new("deciding_principal", PrincipalIdentityField::reference())
}

fn payment_source<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    PaymentSourceSlot,
    BankSchema,
    PaymentSource,
    PaymentIntent,
    Account,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("source", PaymentSource::reference())
}

fn payment_destination<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    PaymentDestinationSlot,
    BankSchema,
    PaymentDestination,
    PaymentIntent,
    Account,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("destination", PaymentDestination::reference())
}

fn payment_business<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    PaymentBusinessSlot,
    BankSchema,
    PaymentBusiness,
    PaymentIntent,
    Business,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("business", PaymentBusiness::reference())
}

fn payment_initiator<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    PaymentInitiatorSlot,
    BankSchema,
    PaymentInitiator,
    Principal,
    PaymentIntent,
    ReverseResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::reverse_one("initiator", PaymentInitiator::reference())
}

fn payment_approval<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    PaymentApprovalSlot,
    BankSchema,
    PaymentApproval,
    PaymentIntent,
    Approval,
    ForwardResultTraversal,
    OptionalOneResult,
> {
    ApplicationQueryResultRelationRef::forward_optional("approval", PaymentApproval::reference())
}

fn approval_principal<Query>() -> ApplicationQueryResultRelationRef<
    Query,
    ApprovalPrincipalSlot,
    BankSchema,
    ApprovalPrincipal,
    Approval,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one(
        "deciding_principal",
        ApprovalPrincipal::reference(),
    )
}
