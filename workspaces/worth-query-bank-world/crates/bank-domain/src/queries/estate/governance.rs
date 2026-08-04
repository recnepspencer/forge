use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;

use crate::{
    authorization::ViewEstateCase,
    estate::{EstateAction, EstateCapabilityPurpose, EstateCaseId, RestrictedBankField},
    reads::EstateGovernanceContext,
    schema::{BankSchema, EstateCase},
};

use super::{governance_disclosure::governance_disclosure, governance_shape::governance_shape};

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

    pub const fn capability_request(self) -> EstateAction {
        EstateAction::ViewRestrictedEstate {
            estate: self.estate,
            field: RestrictedBankField::GovernanceMetadata,
            purpose: EstateCapabilityPurpose::EstateAdministration,
        }
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
        governance_disclosure(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewEstateCase::reference(),
    )
    .build()
    .expect("bank estate governance query is statically canonical")
}
