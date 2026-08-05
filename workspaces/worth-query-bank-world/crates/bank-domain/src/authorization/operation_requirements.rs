use worth_query_decl::facade::worth_query_operation_requires;

use crate::schema::{
    ApplyOpeningFundingOperation, ApprovePaymentOperation, CreateBusinessAccountOperation,
    CreatePersonalAccountOperation, DepositOperation, GrantAccountAuthorizationOperation,
    InitiateBusinessPaymentOperation, RejectPaymentOperation, ReverseJournalOperation,
    RevokeAccountAuthorizationOperation, SendMoneyOperation, WithdrawOperation,
};

use super::{
    ApproveBusinessFunds, InitiateBusinessFunds, ManageAccountAccess, OpenAccount,
    SendPersonalFunds, ServiceInstitutionAccount,
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
