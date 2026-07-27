use bank_domain::model::{
    AccountAuthorizationId, AccountId, BankPrincipalId, CustomerRole, Money, PaymentId, USD,
};
use bank_domain::schema::*;
use worth_foundational::facade::{AspectValue, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    ApplicationEffectRef, ApplicationEntityRef, ApplicationFieldRef, ApplicationOperationRef,
    ApplicationRelationRef, ApplicationSchemaAuthoringDenialKind, DeclaredApplicationCurrency,
    EqualityPredicate, ReadOnly, ReadWrite, TypedApplicationValue,
};
use worth_query_host::facade::domain::{
    WorthQueryInstallationAdmissionProfile, WorthQueryInstallationGeneration,
    WorthQueryInstallationRuntimeIdentity, WorthQueryInstalledApplicationSchema,
    WorthQueryInstalledPackageIndex, WorthQueryPortableDomainIdentity,
    WorthQueryPortableDomainPackage,
};

#[test]
fn installed_read_and_directional_traversal_authoring_are_usable() {
    let (index, bank) = installed_bank();
    let read = bank
        .query(Account::reference())
        .project(AccountIdentity::reference())
        .project(AccountDisplayName::reference())
        .project(AvailableBalance::reference())
        .where_equal(Status::reference(), AccountStatus::Open)
        .build()
        .unwrap();
    assert_eq!(read.entity(), "Account");
    assert_eq!(read.projections().len(), 3);
    assert_eq!(
        read.binding().unwrap().runtime_ordinal(),
        index.runtime_ordinal()
    );

    let owned = bank
        .query(Principal::reference())
        .traverse(PersonalOwner::reference())
        .project(AccountIdentity::reference())
        .build()
        .unwrap();
    assert_eq!(owned.current_entity(), "Account");
    assert_eq!(owned.traversals()[0].relation(), "PersonalOwner");
    index.validate_application_schema(&bank).unwrap();
}

#[test]
fn installed_money_mutation_and_effect_program_are_usable() {
    let (_, bank) = installed_bank();
    let (from, recipient, amount) = transfer_values();
    let mutation = bank
        .operation(SendMoneyOperation::reference())
        .input(SendMoney {
            from,
            recipient,
            amount,
        })
        .create(JournalEntry::reference())
        .create(Posting::reference())
        .set(
            PostingAmount::reference(),
            Money::<USD>::from_signed_minor(-1_250),
        )
        .build()
        .unwrap();
    assert_eq!(mutation.creates(), &["JournalEntry", "Posting"]);
    assert_eq!(
        mutation.binding().unwrap().schema_identity(),
        BankSchema::declaration().unwrap().identity()
    );

    let effects = bank
        .effects(SendMoneyOperation::reference())
        .emit(
            AccountActivityEffect::reference(),
            ActivityEvent {
                account: from,
                journal_sequence: 7,
            },
        )
        .build()
        .unwrap();
    assert_eq!(effects.effects().len(), 1);
}

#[test]
fn installed_approval_grant_and_revoke_programs_are_usable() {
    let (_, bank) = installed_bank();
    let (account, principal, _) = transfer_values();
    let approval = bank
        .operation(ApprovePaymentOperation::reference())
        .input(ApprovePayment {
            payment: PaymentId::new(9).unwrap(),
            approver: principal,
        })
        .create(Approval::reference())
        .link(PaymentApproval::reference())
        .link(ApprovalPrincipal::reference())
        .set(PaymentStatusField::reference(), PaymentStatus::Committed)
        .build()
        .unwrap();
    assert_eq!(approval.links().len(), 2);

    let grant = bank
        .operation(GrantAccountAuthorizationOperation::reference())
        .input(GrantAccountAuthorization {
            account,
            principal,
            role: CustomerRole::Viewer,
        })
        .create(AccountAuthorization::reference())
        .link(AccountAuthorizedUser::reference())
        .link(AuthorizationAccount::reference())
        .set(AuthorizationRole::reference(), CustomerRole::Viewer)
        .build()
        .unwrap();
    assert_eq!(grant.links().len(), 2);

    let revoke = bank
        .operation(RevokeAccountAuthorizationOperation::reference())
        .input(RevokeAccountAuthorization {
            authorization: AccountAuthorizationId::new(4).unwrap(),
        })
        .unlink(AccountAuthorizedUser::reference())
        .unlink(AuthorizationAccount::reference())
        .delete(AccountAuthorization::reference())
        .build()
        .unwrap();
    assert_eq!(revoke.unlinks().len(), 2);
    assert_eq!(revoke.deletes(), &["AccountAuthorization"]);
}

#[test]
fn forged_entity_and_relation_names_are_denied() {
    let (_, bank) = installed_bank();
    let forged_entity =
        ApplicationEntityRef::<BankSchema, Account>::from_schema_identifier("ForgedAccount");
    assert_denial(
        bank.query(forged_entity).build().unwrap_err().kind(),
        ApplicationSchemaAuthoringDenialKind::UnknownEntity,
    );

    let forged_relation =
        ApplicationRelationRef::<BankSchema, PersonalOwner, Principal, Account>::from_schema_identifiers(
            "PersonalOwner",
            "Account",
            "Principal",
        );
    assert_denial(
        bank.query(Principal::reference())
            .traverse(forged_relation)
            .build()
            .unwrap_err()
            .kind(),
        ApplicationSchemaAuthoringDenialKind::UnknownRelation,
    );
}

#[test]
fn forged_field_capability_type_and_currency_are_denied_independently() {
    let (_, bank) = installed_bank();
    let (from, recipient, amount) = transfer_values();
    let forged_write = ApplicationFieldRef::<
        BankSchema,
        Account,
        AccountState,
        PostingAmount,
        Money<USD>,
        ReadWrite,
        EqualityPredicate,
        DeclaredApplicationCurrency<UsdCurrency, USD>,
    >::from_schema_identifiers("Account", "AccountState", "AvailableBalance");
    let denial = bank
        .operation(SendMoneyOperation::reference())
        .input(SendMoney {
            from,
            recipient,
            amount,
        })
        .set(forged_write, Money::<USD>::from_signed_minor(99))
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::FieldNotWritable,
    );

    assert_forged_currency_denied(&bank);
    assert_forged_value_type_denied(&bank, from, recipient, amount);
}

#[test]
fn forged_operation_input_effect_and_payload_are_denied_independently() {
    let (_, bank) = installed_bank();
    let (from, recipient, amount) = transfer_values();
    let forged_operation =
        ApplicationOperationRef::<BankSchema, SendMoneyOperation, SendMoney>::from_schema_identifier(
            "ForgedOperation",
        );
    let denial = bank
        .operation(forged_operation)
        .input(SendMoney {
            from,
            recipient,
            amount,
        })
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::UnknownOperation,
    );

    assert_forged_operation_input_denied(&bank);
    assert_forged_effect_denied(&bank, from);
    assert_forged_effect_payload_denied(&bank);
}

fn assert_forged_currency_denied(bank: &WorthQueryInstalledApplicationSchema<BankSchema>) {
    let field = ApplicationFieldRef::<
        BankSchema,
        Account,
        AccountState,
        AvailableBalance,
        Money<USD>,
        ReadOnly,
        worth_query_decl::facade::application_schema::NoEqualityPredicate,
    >::from_schema_identifiers("Account", "AccountState", "AvailableBalance");
    let denial = bank
        .query(Account::reference())
        .project(field)
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::FieldCurrencyMismatch,
    );
}

fn assert_forged_value_type_denied(
    bank: &WorthQueryInstalledApplicationSchema<BankSchema>,
    from: AccountId,
    recipient: BankPrincipalId,
    amount: Money<USD>,
) {
    let field = ApplicationFieldRef::<
        BankSchema,
        Account,
        AccountState,
        PostingAmount,
        AlternateMoney,
        ReadWrite,
        EqualityPredicate,
    >::from_schema_identifiers("Account", "AccountState", "AvailableBalance");
    let denial = bank
        .operation(SendMoneyOperation::reference())
        .input(SendMoney {
            from,
            recipient,
            amount,
        })
        .set(field, AlternateMoney(99))
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::FieldValueTypeMismatch,
    );
}

fn assert_forged_operation_input_denied(bank: &WorthQueryInstalledApplicationSchema<BankSchema>) {
    let operation =
        ApplicationOperationRef::<BankSchema, SendMoneyOperation, String>::from_schema_identifier(
            "SendMoneyOperation",
        );
    let denial = bank
        .operation(operation)
        .input(String::from("wrong-input"))
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::OperationInputTypeMismatch,
    );
}

fn assert_forged_effect_denied(
    bank: &WorthQueryInstalledApplicationSchema<BankSchema>,
    account: AccountId,
) {
    let effect = ApplicationEffectRef::<BankSchema, AccountActivityEffect, ActivityEvent>::
        from_schema_identifier("ForgedEffect");
    let denial = bank
        .effects(SendMoneyOperation::reference())
        .emit(
            effect,
            ActivityEvent {
                account,
                journal_sequence: 8,
            },
        )
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::UnknownEffect,
    );
}

fn assert_forged_effect_payload_denied(bank: &WorthQueryInstalledApplicationSchema<BankSchema>) {
    let effect =
        ApplicationEffectRef::<BankSchema, AccountActivityEffect, String>::from_schema_identifier(
            "AccountActivityEffect",
        );
    let denial = bank
        .effects(SendMoneyOperation::reference())
        .emit(effect, String::from("wrong-payload"))
        .build()
        .unwrap_err();
    assert_denial(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::EffectPayloadTypeMismatch,
    );
}

fn installed_bank() -> (
    WorthQueryInstalledPackageIndex,
    WorthQueryInstalledApplicationSchema<BankSchema>,
) {
    let package =
        WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new("bank", 1, 0))
            .application_schema(BankSchema::declaration().unwrap())
            .validate()
            .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .admit(package)
        .unwrap();
    let index = WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap();
    let bank = index
        .bind_application_schema(BankSchema::declaration().unwrap())
        .unwrap();
    (index, bank)
}

fn transfer_values() -> (AccountId, BankPrincipalId, Money<USD>) {
    (
        AccountId::new(1).unwrap(),
        BankPrincipalId::new(2).unwrap(),
        Money::<USD>::from_minor(1_250).unwrap(),
    )
}

fn assert_denial(
    actual: ApplicationSchemaAuthoringDenialKind,
    expected: ApplicationSchemaAuthoringDenialKind,
) {
    assert_eq!(actual, expected);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AlternateMoney(i64);

impl TypedApplicationValue for AlternateMoney {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self.0)
    }
}
