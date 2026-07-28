use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    TypedApplicationIdentityValue, TypedApplicationValue, TypedCurrencyApplicationValue,
};

use crate::model::{
    AccountId, AccountName, BankPrincipalId, BusinessId, Currency, CustomerRole, EmployeeRole,
    InstitutionId, Money, PaymentId, SignedMoney,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    Personal,
    Business,
    InstitutionCash,
    InstitutionSettlement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountStatus {
    Open,
    Frozen,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    ApprovalRequired,
    Committed,
    Rejected,
    Reversed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostingPurpose {
    OpeningFunding,
    Deposit,
    Withdrawal,
    Transfer,
    Reversal,
}

macro_rules! string_application_value {
    ($Type:ty, {$($Variant:path => $value:literal),+ $(,)?}) => {
        impl TypedApplicationValue for $Type {
            const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

            fn into_foundational_value(self) -> AspectValue {
                let value = match self {
                    $($Variant => $value),+
                };
                AspectValue::String(InternedString::from(value))
            }
        }
    };
}

string_application_value!(AccountKind, {
    AccountKind::Personal => "personal",
    AccountKind::Business => "business",
    AccountKind::InstitutionCash => "institution-cash",
    AccountKind::InstitutionSettlement => "institution-settlement",
});
string_application_value!(AccountStatus, {
    AccountStatus::Open => "open",
    AccountStatus::Frozen => "frozen",
    AccountStatus::Closed => "closed",
});
string_application_value!(PaymentStatus, {
    PaymentStatus::Pending => "pending",
    PaymentStatus::ApprovalRequired => "approval-required",
    PaymentStatus::Committed => "committed",
    PaymentStatus::Rejected => "rejected",
    PaymentStatus::Reversed => "reversed",
});
string_application_value!(PostingPurpose, {
    PostingPurpose::OpeningFunding => "opening-funding",
    PostingPurpose::Deposit => "deposit",
    PostingPurpose::Withdrawal => "withdrawal",
    PostingPurpose::Transfer => "transfer",
    PostingPurpose::Reversal => "reversal",
});
string_application_value!(CustomerRole, {
    CustomerRole::PersonalOwner => "personal-owner",
    CustomerRole::BusinessOwner => "business-owner",
    CustomerRole::Initiator => "initiator",
    CustomerRole::Approver => "approver",
    CustomerRole::Viewer => "viewer",
});
string_application_value!(EmployeeRole, {
    EmployeeRole::Teller => "teller",
    EmployeeRole::Auditor => "auditor",
});

macro_rules! identity_application_value {
    ($($Type:ty),+ $(,)?) => {
        $(
            impl TypedApplicationValue for $Type {
                const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

                fn into_foundational_value(self) -> AspectValue {
                    AspectValue::UInt64(self.get())
                }
            }
        )+
    };
}

identity_application_value!(
    AccountId,
    BankPrincipalId,
    BusinessId,
    InstitutionId,
    PaymentId
);

impl TypedApplicationIdentityValue for BankPrincipalId {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => BankPrincipalId::new(*value),
            _ => None,
        }
    }
}

impl TypedApplicationValue for AccountName {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(InternedString::from(self.into_string()))
    }
}

impl<C: Currency> TypedApplicationValue for Money<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self.minor_units())
    }
}

impl<C: Currency> TypedCurrencyApplicationValue for Money<C> {
    type Currency = C;
}

impl<C: Currency> TypedApplicationValue for SignedMoney<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self.minor_units())
    }
}

impl<C: Currency> TypedCurrencyApplicationValue for SignedMoney<C> {
    type Currency = C;
}
