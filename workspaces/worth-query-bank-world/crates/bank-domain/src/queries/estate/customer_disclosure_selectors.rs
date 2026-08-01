use worth_query_decl::facade::application_query::{
    ApplicationQueryResultFieldRef, ApplicationQueryResultRelationRef, ExactlyOneResult,
    ForwardResultTraversal,
};
use worth_query_decl::facade::application_schema::{
    EqualityPredicate, NoApplicationCurrency, ReadOnly,
};

use crate::model::BankPrincipalId;
use crate::schema::{
    BankSchema, EstateCase, EstateDeceased, Principal, PrincipalIdentity, PrincipalIdentityField,
};

use super::customer_disclosure::EstateCustomerDisclosureQuery;

pub(super) struct CustomerRelationSlot;
pub(super) struct CustomerIdentitySlot;

pub(super) fn estate_customer() -> ApplicationQueryResultRelationRef<
    EstateCustomerDisclosureQuery,
    CustomerRelationSlot,
    BankSchema,
    EstateDeceased,
    EstateCase,
    Principal,
    ForwardResultTraversal,
    ExactlyOneResult,
> {
    ApplicationQueryResultRelationRef::forward_one("customer", EstateDeceased::reference())
}

pub(super) fn customer_identity() -> ApplicationQueryResultFieldRef<
    EstateCustomerDisclosureQuery,
    CustomerIdentitySlot,
    BankSchema,
    Principal,
    PrincipalIdentity,
    PrincipalIdentityField,
    BankPrincipalId,
    ReadOnly,
    EqualityPredicate,
    NoApplicationCurrency,
> {
    ApplicationQueryResultFieldRef::new("customer_identity", PrincipalIdentityField::reference())
}
