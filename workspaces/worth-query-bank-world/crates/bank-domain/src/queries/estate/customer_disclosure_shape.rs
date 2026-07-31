use worth_query_decl::facade::application_query::{
    ApplicationQueryResultShapeBuilder, TypedApplicationQueryResultShape,
};

use crate::schema::{BankSchema, EstateCase, Principal};

use super::customer_disclosure::{
    EstateCustomerDisclosure, EstateCustomerDisclosureQuery,
};
use super::customer_disclosure_selectors::{customer_identity, estate_customer};

pub(super) fn customer_disclosure_shape() -> TypedApplicationQueryResultShape<
    BankSchema,
    EstateCustomerDisclosureQuery,
    EstateCase,
    EstateCustomerDisclosure,
> {
    let customer = ApplicationQueryResultShapeBuilder::<
        BankSchema,
        EstateCustomerDisclosureQuery,
        Principal,
        (),
    >::new(Principal::reference())
    .field(customer_identity());
    ApplicationQueryResultShapeBuilder::new(EstateCase::reference())
        .relation(estate_customer(), customer)
        .build()
}
