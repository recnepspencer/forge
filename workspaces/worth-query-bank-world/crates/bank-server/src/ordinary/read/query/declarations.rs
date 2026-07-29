pub mod queries {
    use bank_domain::model::{AccountId, InstitutionId, JournalEntryId, PaymentId};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountDiscovery;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountSummary {
        pub(in crate::ordinary::read::query) account: AccountId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountDetail {
        pub(in crate::ordinary::read::query) account: AccountId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountAuthorizedUsers {
        pub(in crate::ordinary::read::query) account: AccountId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountActivity {
        pub(in crate::ordinary::read::query) account: AccountId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct AccountActivityCause {
        pub(in crate::ordinary::read::query) account: AccountId,
        pub(in crate::ordinary::read::query) journal: JournalEntryId,
        pub(in crate::ordinary::read::query) journal_sequence: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct AccountActivityPage {
        pub(in crate::ordinary::read::query) account: AccountId,
        pub(in crate::ordinary::read::query) cursor: Option<crate::BankActivityCursor>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct PendingPayments;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Payment {
        pub(in crate::ordinary::read::query) payment: PaymentId,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct InstitutionAudit {
        pub(in crate::ordinary::read::query) institution: InstitutionId,
    }

    pub const fn accounts() -> AccountDiscovery {
        AccountDiscovery
    }

    pub const fn account_summary(account: AccountId) -> AccountSummary {
        AccountSummary { account }
    }

    pub const fn account_detail(account: AccountId) -> AccountDetail {
        AccountDetail { account }
    }

    pub const fn account_authorized_users(account: AccountId) -> AccountAuthorizedUsers {
        AccountAuthorizedUsers { account }
    }

    pub const fn account_activity(account: AccountId) -> AccountActivity {
        AccountActivity { account }
    }

    pub(crate) const fn account_activity_cause(
        account: AccountId,
        journal: JournalEntryId,
        journal_sequence: u64,
    ) -> AccountActivityCause {
        AccountActivityCause {
            account,
            journal,
            journal_sequence,
        }
    }

    pub const fn account_activity_page(account: AccountId) -> AccountActivityPage {
        AccountActivityPage {
            account,
            cursor: None,
        }
    }

    pub const fn pending_payments() -> PendingPayments {
        PendingPayments
    }

    pub const fn payment(payment: PaymentId) -> Payment {
        Payment { payment }
    }

    pub const fn institution_audit(institution: InstitutionId) -> InstitutionAudit {
        InstitutionAudit { institution }
    }

    impl AccountActivityPage {
        pub const fn after(mut self, cursor: crate::BankActivityCursor) -> Self {
            self.cursor = Some(cursor);
            self
        }
    }

    impl AccountActivity {
        pub(crate) const fn account(self) -> AccountId {
            self.account
        }
    }
}
