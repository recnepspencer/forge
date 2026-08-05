use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    TypedApplicationIdentityValue, TypedApplicationReadableValue, TypedApplicationValue,
    TypedCurrencyApplicationValue,
};

use crate::model::{
    AccountAuthorizationId, AccountId, AccountJournalRevision, AccountName, BankPrincipalId,
    BusinessId, Currency, CustomerRole, EmployeeAssignmentId, EmployeeRole, InstitutionId,
    JournalEntryId, Money, PaymentId, PostingId, SignedMoney,
};
use crate::proposals::{BankIdempotencyIntent, BankIdempotencyKeyIdentity};

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
    EstateDisbursement,
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

        impl TypedApplicationReadableValue for $Type {
            fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                let AspectValue::String(InternedString::Raw(value)) = value else {
                    return None;
                };
                match value.as_str() {
                    $($value => Some($Variant),)+
                    _ => None,
                }
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
    PostingPurpose::EstateDisbursement => "estate-disbursement",
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
    EmployeeRole::BranchManager => "branch-manager",
    EmployeeRole::EstateSpecialist => "estate-specialist",
    EmployeeRole::Compliance => "compliance",
    EmployeeRole::Legal => "legal",
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

            impl TypedApplicationReadableValue for $Type {
                fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                    match value {
                        AspectValue::UInt64(value) => <$Type>::new(*value),
                        _ => None,
                    }
                }
            }
        )+
    };
}

identity_application_value!(
    BankPrincipalId,
    BusinessId,
    EmployeeAssignmentId,
    InstitutionId,
);

impl TypedApplicationIdentityValue for BankPrincipalId {}

macro_rules! created_identity_application_value {
    ($($Type:ty),+ $(,)?) => {
        $(
            impl TypedApplicationValue for $Type {
                const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

                fn into_foundational_value(self) -> AspectValue {
                    AspectValue::String(InternedString::from(self.canonical_text()))
                }
            }

            impl TypedApplicationReadableValue for $Type {
                fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                    match value {
                        AspectValue::String(InternedString::Raw(value)) => {
                            <$Type>::from_canonical_text(value)
                        }
                        _ => None,
                    }
                }
            }
        )+
    };
}

created_identity_application_value!(
    AccountId,
    AccountAuthorizationId,
    PaymentId,
    JournalEntryId,
    PostingId,
);

impl TypedApplicationValue for AccountJournalRevision {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::UInt64(self.get())
    }
}

impl TypedApplicationReadableValue for AccountJournalRevision {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => Some(AccountJournalRevision::from_posting_count(*value)),
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

impl TypedApplicationReadableValue for AccountName {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::String(InternedString::Raw(value)) => AccountName::new(value.clone()).ok(),
            _ => None,
        }
    }
}

impl<C: Currency> TypedApplicationValue for Money<C> {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self.minor_units())
    }
}

impl<C: Currency> TypedApplicationReadableValue for Money<C> {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::Int64(value) => Money::from_minor(*value).ok(),
            _ => None,
        }
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

impl<C: Currency> TypedApplicationReadableValue for SignedMoney<C> {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::Int64(value) => Some(SignedMoney::from_minor(*value)),
            _ => None,
        }
    }
}

impl<C: Currency> worth_query_decl::facade::application_schema::TypedApplicationSignedAggregateValue
    for SignedMoney<C>
{
    fn from_aggregate_i64(value: i64) -> Self {
        SignedMoney::from_minor(value)
    }
}

impl<C: Currency> TypedCurrencyApplicationValue for SignedMoney<C> {
    type Currency = C;
}

macro_rules! idempotency_application_value {
    ($($Type:ty),+ $(,)?) => {
        $(
            impl TypedApplicationValue for $Type {
                const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

                fn into_foundational_value(self) -> AspectValue {
                    AspectValue::String(InternedString::from(self.canonical_text()))
                }
            }

            impl TypedApplicationReadableValue for $Type {
                fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                    match value {
                        AspectValue::String(InternedString::Raw(value)) => {
                            <$Type>::from_canonical_text(value)
                        }
                        _ => None,
                    }
                }
            }
        )+
    };
}

idempotency_application_value!(BankIdempotencyKeyIdentity, BankIdempotencyIntent);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estate_disbursement_is_a_distinct_round_tripping_posting_purpose() {
        let encoded = PostingPurpose::EstateDisbursement.into_foundational_value();

        assert_eq!(
            PostingPurpose::from_foundational_value(&encoded),
            Some(PostingPurpose::EstateDisbursement)
        );
        assert_ne!(
            encoded,
            PostingPurpose::Transfer.into_foundational_value(),
            "estate disbursement must not masquerade as an ordinary transfer"
        );
    }
}
