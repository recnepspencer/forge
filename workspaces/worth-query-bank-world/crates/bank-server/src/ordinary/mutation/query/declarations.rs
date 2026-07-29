pub mod mutations {
    use bank_domain::schema::*;

    macro_rules! mutation {
        ($Name:ident, $Input:ty, $constructor:ident) => {
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct $Name {
                pub(in crate::ordinary::mutation::query) input: $Input,
            }

            pub const fn $constructor(input: $Input) -> $Name {
                $Name { input }
            }
        };
    }

    mutation!(
        CreatePersonalAccountMutation,
        CreatePersonalAccount,
        create_personal_account
    );
    mutation!(
        CreateBusinessAccountMutation,
        CreateBusinessAccount,
        create_business_account
    );
    mutation!(
        OpeningFundingMutation,
        ApplyOpeningFunding,
        apply_opening_funding
    );
    mutation!(DepositMutation, Deposit, deposit);
    mutation!(WithdrawalMutation, Withdraw, withdraw);
    mutation!(SendMoneyMutation, SendMoney, send_money);
    mutation!(
        InitiateBusinessPaymentMutation,
        InitiateBusinessPayment,
        initiate_business_payment
    );
    mutation!(ApprovePaymentMutation, ApprovePayment, approve_payment);
    mutation!(RejectPaymentMutation, RejectPayment, reject_payment);
    mutation!(
        GrantAccountAccessMutation,
        GrantAccountAuthorization,
        grant_account_access
    );
    mutation!(
        RevokeAccountAccessMutation,
        RevokeAccountAuthorization,
        revoke_account_access
    );
    mutation!(ReverseJournalMutation, ReverseJournal, reverse_journal);
}
