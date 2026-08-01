use worth_query_decl::facade::application_schema::TypedApplicationReadableValue;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationDisclosed, WorthQueryApplicationOmission, WorthQueryApplicationProjection,
    WorthQueryApplicationProjectionDenial, WorthQueryApplicationProjectionRow,
};

use crate::estate::{BankDisclosure, RestrictedBankField};
use crate::schema::BankSchema;

use super::customer_disclosure::{EstateCustomerDisclosure, EstateCustomerDisclosureQuery};
use super::customer_disclosure_selectors::{customer_identity, estate_customer};

impl WorthQueryApplicationProjection<BankSchema, EstateCustomerDisclosureQuery>
    for EstateCustomerDisclosure
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, EstateCustomerDisclosureQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        let customer = match row.disclosed_one(estate_customer())? {
            WorthQueryApplicationDisclosed::Disclosed(customer) => {
                match customer.disclosed_field(customer_identity())? {
                    WorthQueryApplicationDisclosed::Disclosed(identity) => {
                        BankDisclosure::Disclosed(identity)
                    }
                    WorthQueryApplicationDisclosed::Omitted(omission) => omission_value(omission)?,
                }
            }
            WorthQueryApplicationDisclosed::Omitted(omission) => omission_value(omission)?,
        };
        Ok(EstateCustomerDisclosure::new(customer))
    }
}

fn omission_value<T>(
    omission: WorthQueryApplicationOmission,
) -> Result<BankDisclosure<T>, WorthQueryApplicationProjectionDenial> {
    let field = RestrictedBankField::from_foundational_value(omission.required_disclosure())
        .filter(|field| *field == RestrictedBankField::CustomerIdentity)
        .ok_or_else(|| WorthQueryApplicationProjectionDenial::reject("customer-disclosure"))?;
    Ok(BankDisclosure::Omitted(field.classification()))
}
