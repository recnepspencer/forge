use worth_query_decl::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchemaDeclarationBuilder, OperationReads,
};

use super::super::authentication::PrincipalIdentityField;
use super::super::fields::{
    AccountDisplayName, AccountIdentity, AccountingRevision, BusinessIdentityField,
    InstitutionIdentityField, JournalIdentityField, JournalPurpose, Kind, PostingAmount,
    PostingIdentityField, Purpose, Status,
};
use super::super::relations::{
    BusinessAccount, InstitutionAccount, JournalPosting, JournalReversal, PersonalOwner,
    PostingAccount,
};
use super::super::BankSchema;

pub(super) fn install_account_projection_reads<Operation, Input>(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
    operation: ApplicationOperationRef<BankSchema, Operation, Input>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema>
where
    AccountIdentity: OperationReads<Operation>,
    AccountingRevision: OperationReads<Operation>,
    InstitutionAccount: OperationReads<Operation>,
    InstitutionIdentityField: OperationReads<Operation>,
    Kind: OperationReads<Operation>,
    PersonalOwner: OperationReads<Operation>,
    BusinessAccount: OperationReads<Operation>,
    PrincipalIdentityField: OperationReads<Operation>,
    BusinessIdentityField: OperationReads<Operation>,
    Status: OperationReads<Operation>,
    AccountDisplayName: OperationReads<Operation>,
{
    schema
        .operation_read_field(operation, AccountIdentity::reference())
        .operation_read_field(operation, AccountingRevision::reference())
        .operation_read_field(operation, InstitutionIdentityField::reference())
        .operation_read_field(operation, Kind::reference())
        .operation_read_field(operation, PrincipalIdentityField::reference())
        .operation_read_field(operation, BusinessIdentityField::reference())
        .operation_read_field(operation, Status::reference())
        .operation_read_field(operation, AccountDisplayName::reference())
        .operation_read_relation(operation, InstitutionAccount::reference())
        .operation_read_relation(operation, PersonalOwner::reference())
        .operation_read_relation(operation, BusinessAccount::reference())
}

pub(super) fn install_accounting_projection_reads<Operation, Input>(
    schema: ApplicationSchemaDeclarationBuilder<BankSchema>,
    operation: ApplicationOperationRef<BankSchema, Operation, Input>,
) -> ApplicationSchemaDeclarationBuilder<BankSchema>
where
    AccountIdentity: OperationReads<Operation>,
    AccountingRevision: OperationReads<Operation>,
    InstitutionAccount: OperationReads<Operation>,
    InstitutionIdentityField: OperationReads<Operation>,
    Kind: OperationReads<Operation>,
    PersonalOwner: OperationReads<Operation>,
    BusinessAccount: OperationReads<Operation>,
    PrincipalIdentityField: OperationReads<Operation>,
    BusinessIdentityField: OperationReads<Operation>,
    Status: OperationReads<Operation>,
    AccountDisplayName: OperationReads<Operation>,
    PostingAccount: OperationReads<Operation>,
    JournalPosting: OperationReads<Operation>,
    JournalIdentityField: OperationReads<Operation>,
    JournalPurpose: OperationReads<Operation>,
    PostingIdentityField: OperationReads<Operation>,
    Purpose: OperationReads<Operation>,
    PostingAmount: OperationReads<Operation>,
    JournalReversal: OperationReads<Operation>,
{
    install_account_projection_reads(schema, operation)
        .operation_read_field(operation, JournalIdentityField::reference())
        .operation_read_field(operation, JournalPurpose::reference())
        .operation_read_field(operation, PostingIdentityField::reference())
        .operation_read_field(operation, Purpose::reference())
        .operation_read_field(operation, PostingAmount::reference())
        .operation_read_relation(operation, PostingAccount::reference())
        .operation_read_relation(operation, JournalPosting::reference())
        .operation_read_relation(operation, JournalReversal::reference())
}
