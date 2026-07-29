use worth_query_decl::facade::worth_query_operation_reads;

use super::super::*;

worth_query_operation_reads!(
    DiscoverAccountsOperation => [
        PrincipalIdentityField,
        PersonalOwner,
        AccountAuthorizedUser,
        AuthorizationAccount,
        AuthorizationRole,
        BusinessOwner,
        BusinessAccount,
        AccountIdentity,
        AccountDisplayName,
        AccountingRevision,
        InstitutionAccount,
        InstitutionIdentityField,
        Kind,
        BusinessIdentityField,
        Status
    ]
);

macro_rules! account_read_capabilities {
    ($operation:ident) => {
        worth_query_operation_reads!(
            $operation => [
                AccountIdentity,
                AccountDisplayName,
                AccountingRevision,
                InstitutionAccount,
                InstitutionIdentityField,
                Kind,
                PersonalOwner,
                PrincipalIdentityField,
                BusinessAccount,
                BusinessIdentityField,
                Status,
                PostingAccount,
                JournalPosting,
                JournalIdentityField,
                JournalPurpose,
                PostingIdentityField,
                Purpose,
                PostingAmount,
                PostingAccountSequence,
                JournalReversal
            ]
        );
    };
}

account_read_capabilities!(ReadAccountSummaryOperation);
account_read_capabilities!(ReadAccountDetailOperation);
account_read_capabilities!(ReadAccountActivityOperation);

worth_query_operation_reads!(
    ReadAccountAuthorizedUsersOperation => [
        AccountIdentity,
        AccountDisplayName,
        AccountingRevision,
        InstitutionAccount,
        InstitutionIdentityField,
        Kind,
        PersonalOwner,
        PrincipalIdentityField,
        BusinessAccount,
        BusinessIdentityField,
        Status,
        AccountAuthorizedUser,
        AuthorizationAccount,
        AccountAuthorizationIdentity,
        AuthorizationRole
    ]
);

macro_rules! payment_read_capabilities {
    ($operation:ident) => {
        worth_query_operation_reads!(
            $operation => [
                PaymentIdentityField,
                PaymentAmount,
                PaymentStatusField,
                PaymentSource,
                PaymentDestination,
                PaymentBusiness,
                PaymentInitiator,
                PaymentApproval,
                ApprovalPrincipal,
                AccountIdentity,
                AccountDisplayName,
                AccountingRevision,
                InstitutionAccount,
                InstitutionIdentityField,
                Kind,
                PersonalOwner,
                PrincipalIdentityField,
                BusinessAccount,
                BusinessIdentityField,
                Status
            ]
        );
    };
}

payment_read_capabilities!(ReadPaymentOperation);

worth_query_operation_reads!(
    ReadPendingPaymentsOperation => [
        PrincipalIdentityField,
        AccountAuthorizedUser,
        AuthorizationAccount,
        AuthorizationRole,
        AccountIdentity,
        PaymentSource,
        PaymentIdentityField,
        PaymentAmount,
        PaymentStatusField,
        PaymentDestination,
        PaymentBusiness,
        BusinessIdentityField,
        PaymentInitiator,
        PaymentApproval,
        ApprovalPrincipal
    ]
);

worth_query_operation_reads!(
    AuditInstitutionActivityOperation => [
        InstitutionIdentityField,
        InstitutionAccount,
        AccountIdentity,
        AccountDisplayName,
        AccountingRevision,
        Kind,
        Status,
        PersonalOwner,
        PrincipalIdentityField,
        BusinessAccount,
        BusinessIdentityField,
        JournalPosting,
        PostingAccount,
        JournalIdentityField,
        JournalPurpose,
        PostingIdentityField,
        PostingAmount,
        PostingAccountSequence,
        Purpose,
        JournalReversal
    ]
);
