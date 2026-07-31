use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;

use crate::{
    authorization::ViewEstateCase,
    estate::EstateCaseId,
    reads::EstateGovernanceContext,
    schema::{BankSchema, EstateCase, ViewEstateAdministrationCapability},
};

use super::governance_shape::governance_shape;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateGovernanceQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateGovernanceRequest {
    estate: EstateCaseId,
}

impl EstateGovernanceRequest {
    pub const fn estate(self) -> EstateCaseId {
        self.estate
    }
}

pub const fn estate_governance_context(estate: EstateCaseId) -> EstateGovernanceRequest {
    EstateGovernanceRequest { estate }
}

worth_query_application_query!(
    pub EstateGovernanceQuery in BankSchema,
    parameters EstateGovernanceQueryParameters,
    result EstateGovernanceContext,
    scope EstateCase,
    name "estate_governance_context"
);

pub fn estate_governance_definition() -> ApplicationQueryDefinition<
    BankSchema,
    EstateGovernanceQuery,
    EstateGovernanceQueryParameters,
    EstateGovernanceContext,
    EstateCase,
> {
    ApplicationQueryDefinitionBuilder::requires_ability(
        EstateGovernanceQuery::reference(),
        EstateCase::reference(),
        EstateCase::reference(),
        governance_shape(),
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(3, 9, 21),
        ApplicationQueryDisclosureContract::governed_by(
            "estate-governance-context",
            ViewEstateAdministrationCapability::reference(),
        ),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewEstateCase::reference(),
    )
    .build()
    .expect("bank estate governance query is statically canonical")
}
