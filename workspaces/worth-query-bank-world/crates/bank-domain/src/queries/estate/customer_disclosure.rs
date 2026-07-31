use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;

use crate::authorization::ViewEstateCase;
use crate::estate::{BankDisclosure, EstateCaseId};
use crate::model::BankPrincipalId;
use crate::schema::{
    BankSchema, EstateCase, ViewEstateIdentityVerificationCapability,
};

use super::customer_disclosure_selectors::{customer_identity, estate_customer};
use super::customer_disclosure_shape::customer_disclosure_shape;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCustomerDisclosureQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCustomerDisclosureRequest {
    estate: EstateCaseId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCustomerDisclosure {
    customer: BankDisclosure<BankPrincipalId>,
}

impl EstateCustomerDisclosureRequest {
    pub const fn estate(self) -> EstateCaseId {
        self.estate
    }
}

impl EstateCustomerDisclosure {
    pub const fn new(customer: BankDisclosure<BankPrincipalId>) -> Self {
        Self { customer }
    }

    pub const fn customer(self) -> BankDisclosure<BankPrincipalId> {
        self.customer
    }
}

pub const fn estate_customer_identity(
    estate: EstateCaseId,
) -> EstateCustomerDisclosureRequest {
    EstateCustomerDisclosureRequest { estate }
}

worth_query_application_query!(
    pub EstateCustomerDisclosureQuery in BankSchema,
    parameters EstateCustomerDisclosureQueryParameters,
    result EstateCustomerDisclosure,
    scope EstateCase,
    name "estate_customer_identity"
);

pub fn estate_customer_disclosure_definition() -> ApplicationQueryDefinition<
    BankSchema,
    EstateCustomerDisclosureQuery,
    EstateCustomerDisclosureQueryParameters,
    EstateCustomerDisclosure,
    EstateCase,
> {
    let influence = ApplicationQueryInfluenceContract::forbid_all();
    let disclosure = ApplicationQueryDisclosureContract::governed_by(
        "estate-customer-identity",
        ViewEstateIdentityVerificationCapability::reference(),
    )
    .disclose_relation_by(
        estate_customer(),
        crate::estate::RestrictedBankField::CustomerIdentity,
        influence.clone(),
    )
    .disclose_field_by(
        customer_identity(),
        crate::estate::RestrictedBankField::CustomerIdentity,
        influence,
    );
    ApplicationQueryDefinitionBuilder::requires_ability(
        EstateCustomerDisclosureQuery::reference(),
        EstateCase::reference(),
        EstateCase::reference(),
        customer_disclosure_shape(),
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(1, 1, 1),
        disclosure,
        ApplicationQueryBasisSupport::current_and_pinned().with_preview(),
        ApplicationQueryLaneEligibility::one_shot()
            .with_historical()
            .with_preview(),
        ViewEstateCase::reference(),
    )
    .build()
    .expect("estate customer disclosure query is statically canonical")
}
