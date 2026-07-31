use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;

use crate::authorization::ViewEstateCase;
use crate::estate::EstateCaseId;
use crate::reads::EstateCaseOverview;
use crate::schema::{BankSchema, EstateCase};

use super::overview_shape::overview_shape;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCaseOverviewQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateCaseOverviewRequest {
    estate: EstateCaseId,
}

impl EstateCaseOverviewRequest {
    pub const fn new(estate: EstateCaseId) -> Self {
        Self { estate }
    }

    pub const fn estate(self) -> EstateCaseId {
        self.estate
    }
}

pub const fn estate_case(estate: EstateCaseId) -> EstateCaseOverviewRequest {
    EstateCaseOverviewRequest::new(estate)
}

worth_query_application_query!(
    pub EstateCaseOverviewQuery in BankSchema,
    parameters EstateCaseOverviewQueryParameters,
    result EstateCaseOverview,
    scope EstateCase,
    name "estate_case_overview"
);

pub fn estate_case_overview_definition() -> ApplicationQueryDefinition<
    BankSchema,
    EstateCaseOverviewQuery,
    EstateCaseOverviewQueryParameters,
    EstateCaseOverview,
    EstateCase,
> {
    ApplicationQueryDefinitionBuilder::requires_ability(
        EstateCaseOverviewQuery::reference(),
        EstateCase::reference(),
        EstateCase::reference(),
        overview_shape(),
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(3, 16, 28),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned().with_preview(),
        ApplicationQueryLaneEligibility::one_shot().with_preview(),
        ViewEstateCase::reference(),
    )
    .build()
    .expect("bank estate overview query is statically canonical")
}
