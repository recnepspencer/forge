use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryInfluenceContract,
    ApplicationQueryLaneEligibility,
};
use worth_query_decl::facade::worth_query_application_query;

use crate::{
    authorization::ViewEstateLegalCompliance,
    estate::{EstateAction, EstateCapabilityPurpose, EstateCaseId, RestrictedBankField},
    schema::{BankSchema, EstateCase, ViewEstateLegalComplianceCapability},
};

use super::{
    legal_compliance_projection::EstateLegalComplianceResult, legal_compliance_selectors::*,
    legal_compliance_shape::legal_compliance_shape,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateLegalComplianceQueryParameters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateLegalComplianceRequest {
    estate: EstateCaseId,
}

impl EstateLegalComplianceRequest {
    pub const fn estate(self) -> EstateCaseId {
        self.estate
    }

    pub const fn capability_request(self) -> EstateAction {
        EstateAction::ViewRestrictedEstate {
            estate: self.estate,
            field: RestrictedBankField::LegalDocument,
            purpose: EstateCapabilityPurpose::LegalCompliance,
        }
    }
}

pub const fn estate_legal_compliance(estate: EstateCaseId) -> EstateLegalComplianceRequest {
    EstateLegalComplianceRequest { estate }
}

worth_query_application_query!(
    pub EstateLegalComplianceQuery in BankSchema,
    parameters EstateLegalComplianceQueryParameters,
    result EstateLegalComplianceResult,
    scope EstateCase,
    name "estate_legal_compliance"
);

pub fn estate_legal_compliance_definition() -> ApplicationQueryDefinition<
    BankSchema,
    EstateLegalComplianceQuery,
    EstateLegalComplianceQueryParameters,
    EstateLegalComplianceResult,
    EstateCase,
> {
    ApplicationQueryDefinitionBuilder::requires_ability(
        EstateLegalComplianceQuery::reference(),
        EstateCase::reference(),
        EstateCase::reference(),
        legal_compliance_shape(),
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(3, 16, 16),
        disclosure_contract(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ViewEstateLegalCompliance::reference(),
    )
    .build()
    .expect("bank estate legal-compliance query is statically canonical")
}

fn disclosure_contract() -> ApplicationQueryDisclosureContract {
    let field = RestrictedBankField::LegalDocument;
    let influence = ApplicationQueryInfluenceContract::forbid_all();
    ApplicationQueryDisclosureContract::governed_by(
        "estate-legal-compliance",
        ViewEstateLegalComplianceCapability::reference(),
    )
    .disclose_field_by(estate_identity(), field, influence.clone())
    .disclose_relation_by(estate_authorities(), field, influence.clone())
    .disclose_field_by(authority_identity(), field, influence.clone())
    .disclose_field_by(authority_kind(), field, influence.clone())
    .disclose_field_by(authority_recognized(), field, influence.clone())
    .disclose_relation_by(authority_holder(), field, influence.clone())
    .disclose_field_by(authority_holder_identity(), field, influence)
}
