# Public Consumer Contract

## Dependency boundary

Bank product code consumes Query only through `worth-query-decl` and
`worth-query-host`. It does not import Query authority packages, the Query
monolith, replay, Relational, Signal, or the runtime bridge.

Runtime replay remains confined to a later certification consumer.

## Read target

```rust,ignore
let bank = installed.bind(BankSchema::declaration())?;

let accounts = bank
    .query(Principal::reference())
    .traverse(PersonalOwner::reference())
    .project(AccountIdentity::reference())
    .project(AccountDisplayName::reference())
    .project(AvailableBalance::reference())
    .where_equal(Status::reference(), AccountStatus::Open)
    .build()?;
```

## Mutation target

```rust,ignore
let transfer = bank
    .operation(SendMoneyOperation::reference())
    .input(SendMoney {
        from,
        recipient,
        amount: Money::<USD>::from_minor(cents)?,
    })
    .create(JournalEntry::reference())
    .create(Posting::reference())
    .set(PostingAmount::reference(), debit)
    .build()?;
```

## Domain effect target

```rust,ignore
let effects = bank
    .effects(SendMoneyOperation::reference())
    .emit(
        AccountActivityEffect::reference(),
        ActivityEvent {
            account: from,
            journal_sequence,
        },
    )
    .build()?;
```

## Approval and authorization authoring targets

```rust,ignore
let approval = bank
    .operation(ApprovePaymentOperation::reference())
    .input(ApprovePayment { payment, approver })
    .set(PaymentStatusField::reference(), PaymentStatus::Committed)
    .build()?;

let authorization = bank
    .operation(GrantAccountAuthorizationOperation::reference())
    .input(GrantAccountAuthorization {
        account,
        principal,
        role: CustomerRole::Viewer,
    })
    .create(AccountAuthorization::reference())
    .set(AuthorizationRole::reference(), CustomerRole::Viewer)
    .build()?;
```

These four blocks are Phase 1 compile targets and are exercised by
`public_consumer_transcript.rs`. They author installed-bound declarations; they
do not claim authentication, policy admission, execution, or commit authority.

## Later approval and live execution target

```rust,ignore
match bank.execute(initiation).as_principal(principal_proof).await? {
    InitiatedPayment::Committed(receipt) => publish(receipt),
    InitiatedPayment::ApprovalRequired(pending) => {
        pending.approve_as(other_principal_proof).await?
    }
    InitiatedPayment::Denied(reason) => deny(reason),
}

let activity = bank
    .query(Account::reference())
    .as_principal(principal_proof)
    .subscribe(LiveDelivery::server_sent_events())
    .await?;
```

Authentication, authorization, execution, and live delivery become runnable
only in their owning later phases. Phase 1 must not simulate them through a
local executor.

## Prohibitions

Application paths contain no semantic aspect, field, relation, operation,
policy, or currency strings. A dynamic extension, if admitted later, uses an
explicit dynamic key and schema-readmission result. Caller-cast values and raw
`AspectValue` construction are not the ordinary bank API.
