use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

use crate::authorization::ViewPayment;
use crate::model::PaymentId;
use crate::reads::PaymentSummary;
use crate::schema::{BankSchema, PaymentIntent};

use super::payment_summary_projection::{payment_summary_shape, project_payment_summary};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaymentDetailQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PaymentDetailRequest {
    payment: PaymentId,
}

impl PaymentDetailRequest {
    pub const fn new(payment: PaymentId) -> Self {
        Self { payment }
    }

    pub const fn payment(self) -> PaymentId {
        self.payment
    }
}

pub const fn payment(payment: PaymentId) -> PaymentDetailRequest {
    PaymentDetailRequest::new(payment)
}

worth_query_application_query!(
    pub PaymentDetailQuery in BankSchema,
    parameters PaymentDetailQueryParameters,
    result PaymentSummary,
    scope PaymentIntent,
    name "payment_detail"
);

pub fn payment_detail_definition() -> ApplicationQueryDefinition<
    BankSchema,
    PaymentDetailQuery,
    PaymentDetailQueryParameters,
    PaymentSummary,
    PaymentIntent,
> {
    ApplicationQueryDefinitionBuilder::requires_ability(
        PaymentDetailQuery::reference(),
        PaymentIntent::reference(),
        PaymentIntent::reference(),
        payment_summary_shape().build(),
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(2, 6, 8),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewPayment::reference(),
    )
    .build()
    .expect("bank payment detail query is statically canonical")
}

impl WorthQueryApplicationProjection<BankSchema, PaymentDetailQuery> for PaymentSummary {
    fn project(
        row: &WorthQueryApplicationProjectionRow<'_, BankSchema, PaymentDetailQuery>,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        project_payment_summary(row)
    }
}
