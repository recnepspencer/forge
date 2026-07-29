use worth_query_decl::facade::worth_query_operation_requires;

use crate::schema::{
    ApplyOpeningFundingOperation, ApprovePaymentOperation, AuditInstitutionActivityOperation,
    CreateBusinessAccountOperation, CreatePersonalAccountOperation, DepositOperation,
    DiscoverAccountsOperation, GrantAccountAuthorizationOperation,
    InitiateBusinessPaymentOperation, ReadAccountActivityOperation,
    ReadAccountAuthorizedUsersOperation, ReadAccountDetailOperation, ReadAccountSummaryOperation,
    ReadPaymentOperation, ReadPendingPaymentsOperation, RejectPaymentOperation,
    ReverseJournalOperation, RevokeAccountAuthorizationOperation, SendMoneyOperation,
    WithdrawOperation,
};

use super::{
    ApproveBusinessFunds, AuditInstitution, DiscoverOwnAccounts, InitiateBusinessFunds,
    ManageAccountAccess, OpenAccount, SendPersonalFunds, ServiceInstitutionAccount, ViewAccount,
    ViewAccountAccess, ViewPayment,
};

worth_query_operation_requires!(
    CreatePersonalAccountOperation => [OpenAccount]
);
worth_query_operation_requires!(
    CreateBusinessAccountOperation => [OpenAccount]
);
worth_query_operation_requires!(
    ApplyOpeningFundingOperation => [ServiceInstitutionAccount]
);
worth_query_operation_requires!(DepositOperation => [ServiceInstitutionAccount]);
worth_query_operation_requires!(WithdrawOperation => [ServiceInstitutionAccount]);
worth_query_operation_requires!(SendMoneyOperation => [SendPersonalFunds]);
worth_query_operation_requires!(
    InitiateBusinessPaymentOperation => [InitiateBusinessFunds]
);
worth_query_operation_requires!(ApprovePaymentOperation => [ApproveBusinessFunds]);
worth_query_operation_requires!(RejectPaymentOperation => [ApproveBusinessFunds]);
worth_query_operation_requires!(
    GrantAccountAuthorizationOperation => [ManageAccountAccess]
);
worth_query_operation_requires!(
    RevokeAccountAuthorizationOperation => [ManageAccountAccess]
);
worth_query_operation_requires!(ReverseJournalOperation => [ServiceInstitutionAccount]);
worth_query_operation_requires!(DiscoverAccountsOperation => [DiscoverOwnAccounts]);
worth_query_operation_requires!(ReadAccountSummaryOperation => [ViewAccount]);
worth_query_operation_requires!(ReadAccountDetailOperation => [ViewAccount]);
worth_query_operation_requires!(ReadAccountAuthorizedUsersOperation => [ViewAccountAccess]);
worth_query_operation_requires!(ReadAccountActivityOperation => [ViewAccount]);
worth_query_operation_requires!(ReadPendingPaymentsOperation => [DiscoverOwnAccounts]);
worth_query_operation_requires!(ReadPaymentOperation => [ViewPayment]);
worth_query_operation_requires!(AuditInstitutionActivityOperation => [AuditInstitution]);
