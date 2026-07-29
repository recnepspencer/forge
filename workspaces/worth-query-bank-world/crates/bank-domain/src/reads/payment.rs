use crate::model::{AccountId, BankPrincipalId, BusinessId, Money, PaymentId, USD};
use crate::schema::PaymentStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaymentSummary {
    id: PaymentId,
    business: BusinessId,
    source: AccountId,
    destination: AccountId,
    initiator: BankPrincipalId,
    amount: Money<USD>,
    status: PaymentStatus,
    deciding_principal: Option<BankPrincipalId>,
}

impl PaymentSummary {
    pub const fn from_projection(
        id: PaymentId,
        business: BusinessId,
        source: AccountId,
        destination: AccountId,
        initiator: BankPrincipalId,
        amount: Money<USD>,
        status: PaymentStatus,
        deciding_principal: Option<BankPrincipalId>,
    ) -> Self {
        Self {
            id,
            business,
            source,
            destination,
            initiator,
            amount,
            status,
            deciding_principal,
        }
    }

    pub const fn id(self) -> PaymentId {
        self.id
    }

    pub const fn business(self) -> BusinessId {
        self.business
    }

    pub const fn source(self) -> AccountId {
        self.source
    }

    pub const fn destination(self) -> AccountId {
        self.destination
    }

    pub const fn initiator(self) -> BankPrincipalId {
        self.initiator
    }

    pub const fn amount(self) -> Money<USD> {
        self.amount
    }

    pub const fn status(self) -> PaymentStatus {
        self.status
    }

    pub const fn deciding_principal(self) -> Option<BankPrincipalId> {
        self.deciding_principal
    }
}
