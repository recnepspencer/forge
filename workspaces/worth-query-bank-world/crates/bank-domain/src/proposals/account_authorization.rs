use crate::model::{AccountAuthorizationId, AccountId, BankPrincipalId, CustomerRole};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BankAccountAuthorization {
    id: AccountAuthorizationId,
    account: AccountId,
    principal: BankPrincipalId,
    role: CustomerRole,
}

impl BankAccountAuthorization {
    pub(crate) const fn new(
        id: AccountAuthorizationId,
        account: AccountId,
        principal: BankPrincipalId,
        role: CustomerRole,
    ) -> Self {
        Self {
            id,
            account,
            principal,
            role,
        }
    }

    pub const fn id(self) -> AccountAuthorizationId {
        self.id
    }

    pub const fn account(self) -> AccountId {
        self.account
    }

    pub const fn principal(self) -> BankPrincipalId {
        self.principal
    }

    pub const fn role(self) -> CustomerRole {
        self.role
    }
}
