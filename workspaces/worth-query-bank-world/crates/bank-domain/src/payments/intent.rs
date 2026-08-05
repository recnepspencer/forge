use crate::model::{AccountId, BankPrincipalId, BusinessId, Money, PaymentId, USD};
use crate::schema::PaymentStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BusinessPayment {
    id: PaymentId,
    business: BusinessId,
    source: AccountId,
    destination: AccountId,
    initiator: BankPrincipalId,
    amount: Money<USD>,
    status: PaymentStatus,
    deciding_principal: Option<BankPrincipalId>,
}

pub struct BusinessPaymentProjection {
    pub id: PaymentId,
    pub business: BusinessId,
    pub source: AccountId,
    pub destination: AccountId,
    pub initiator: BankPrincipalId,
    pub amount: Money<USD>,
    pub status: PaymentStatus,
    pub deciding_principal: Option<BankPrincipalId>,
}

impl BusinessPayment {
    pub const fn from_projection(projection: BusinessPaymentProjection) -> Self {
        let BusinessPaymentProjection {
            id,
            business,
            source,
            destination,
            initiator,
            amount,
            status,
            deciding_principal,
        } = projection;
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

    pub(crate) const fn pending(
        id: PaymentId,
        business: BusinessId,
        source: AccountId,
        destination: AccountId,
        initiator: BankPrincipalId,
        amount: Money<USD>,
    ) -> Self {
        Self {
            id,
            business,
            source,
            destination,
            initiator,
            amount,
            status: PaymentStatus::ApprovalRequired,
            deciding_principal: None,
        }
    }

    pub(crate) fn with_decision(&self, status: PaymentStatus, principal: BankPrincipalId) -> Self {
        let mut decided = self.clone();
        decided.status = status;
        decided.deciding_principal = Some(principal);
        decided
    }

    pub const fn id(&self) -> PaymentId {
        self.id
    }

    pub const fn business(&self) -> BusinessId {
        self.business
    }

    pub const fn source(&self) -> AccountId {
        self.source
    }

    pub const fn destination(&self) -> AccountId {
        self.destination
    }

    pub const fn initiator(&self) -> BankPrincipalId {
        self.initiator
    }

    pub const fn amount(&self) -> Money<USD> {
        self.amount
    }

    pub const fn status(&self) -> PaymentStatus {
        self.status
    }

    pub const fn deciding_principal(&self) -> Option<BankPrincipalId> {
        self.deciding_principal
    }
}
